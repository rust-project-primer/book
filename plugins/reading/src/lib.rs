use anyhow::{Context as _, Result, bail};
use log::{error, warn};
use mdbook_preprocessor::{
    Preprocessor, PreprocessorContext,
    book::{Book, BookItem, Chapter},
};
use pulldown_cmark::{CodeBlockKind, CowStr, Event, Options, Parser, Tag, TagEnd};
use pulldown_cmark_to_cmark::cmark;
use serde::Deserialize;
use std::collections::HashMap;
use std::path::PathBuf;
use url::Url;

fn default_label() -> String {
    "reading".into()
}

#[derive(Deserialize, Debug)]
pub struct Config {
    /// Base path where archives are stored.
    archives: Option<String>,
    /// Label to look for
    #[serde(default = "default_label")]
    label: String,
}

#[derive(Debug)]
pub struct Instance {
    config: Config,
    /// The directory where the book source files live (root + src).
    src_dir: PathBuf,
}

impl Instance {
    pub fn new(config: Config, src_dir: PathBuf) -> Self {
        Self { config, src_dir }
    }

    fn map(&self, book: Book) -> Result<Book> {
        let mut book = book;
        let mut errors: Vec<String> = Vec::new();
        let mut seen: HashMap<Url, SeenEntry> = HashMap::new();
        book.items = std::mem::take(&mut book.items)
            .into_iter()
            .map(|section| self.map_book_item(section, &mut errors, &mut seen))
            .collect::<Result<_, _>>()?;
        if !errors.is_empty() {
            for err in &errors {
                error!("{}", err);
            }
            let msg = format!("found {} error(s)", errors.len());
            error!("{}", msg);
            bail!("{}", msg);
        }
        Ok(book)
    }

    fn map_book_item(
        &self,
        item: BookItem,
        errors: &mut Vec<String>,
        seen: &mut HashMap<Url, SeenEntry>,
    ) -> Result<BookItem> {
        let result = match item {
            BookItem::Chapter(chapter) => {
                let title = chapter.name.clone();
                let chapter = self
                    .map_chapter(chapter, errors, seen)
                    .with_context(|| format!("mapping chapter {title:?}"))?;
                BookItem::Chapter(chapter)
            }
            other => other,
        };

        Ok(result)
    }

    fn map_chapter(
        &self,
        mut chapter: Chapter,
        errors: &mut Vec<String>,
        seen: &mut HashMap<Url, SeenEntry>,
    ) -> Result<Chapter> {
        let chapter_path = chapter
            .source_path
            .as_deref()
            .map(|p| p.display().to_string())
            .unwrap_or_else(|| chapter.name.clone());
        chapter.content = self.map_markdown(&chapter.content, &chapter_path, errors, seen)?;
        chapter.sub_items = std::mem::take(&mut chapter.sub_items)
            .into_iter()
            .map(|item| self.map_book_item(item, errors, seen))
            .collect::<Result<_, _>>()?;
        Ok(chapter)
    }

    fn map_markdown(
        &self,
        markdown: &str,
        chapter_path: &str,
        errors: &mut Vec<String>,
        seen: &mut HashMap<Url, SeenEntry>,
    ) -> Result<String> {
        let mut parser = Parser::new_ext(markdown, Options::all());
        let mut events = vec![];

        loop {
            let next = parser.next();
            match next {
                None => break,
                Some(Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(label))))
                    if *label == self.config.label =>
                {
                    let mapped = match parser.next() {
                        Some(Event::Text(code)) => self
                            .map_code(code, chapter_path, errors, seen)
                            .context("Mapping code")?,
                        other => unreachable!("Got {other:?}"),
                    };

                    for event in mapped.into_iter() {
                        events.push(event);
                    }

                    parser.next();
                }
                Some(event) => events.push(event),
            }
        }

        let mut buf = String::with_capacity(markdown.len());
        let output = cmark(events.iter(), &mut buf).map(|_| buf)?;
        Ok(output)
    }

    fn map_code(
        &self,
        code: CowStr<'_>,
        chapter_path: &str,
        errors: &mut Vec<String>,
        seen: &mut HashMap<Url, SeenEntry>,
    ) -> Result<Vec<Event<'static>>> {
        let (header, content) = code.split_once("---").unwrap();
        let header: Header = serde_yaml::from_str(header)?;

        let summary = content.trim().to_string();

        // Validate archive status for articles
        if header.style == "article" {
            // Check for duplicate URLs with conflicting metadata
            if let Some(prev) = seen.get(&header.url) {
                let mut mismatches = Vec::new();
                if prev.header.title != header.title {
                    mismatches.push(format!(
                        "title: {:?} vs {:?}",
                        header.title, prev.header.title
                    ));
                }
                if prev.header.author != header.author {
                    mismatches.push(format!(
                        "author: {:?} vs {:?}",
                        header.author, prev.header.author
                    ));
                }
                if prev.header.archived != header.archived {
                    mismatches.push(format!(
                        "archived: {:?} vs {:?}",
                        header.archived, prev.header.archived
                    ));
                }
                if prev.summary != summary {
                    mismatches.push("summary".to_string());
                }
                if !mismatches.is_empty() {
                    errors.push(format!(
                        "{}: mismatch with {} for {}: {}",
                        chapter_path,
                        prev.chapter,
                        header.url,
                        mismatches.join(", ")
                    ));
                }
            } else {
                seen.insert(
                    header.url.clone(),
                    SeenEntry {
                        chapter: chapter_path.to_string(),
                        header: header.clone(),
                        summary: summary.clone(),
                    },
                );
            }

            match &header.archived {
                None => {
                    warn!(
                        "{}: missing archived for article: {}",
                        chapter_path, header.url
                    );
                }
                Some(Archived::Disabled(false)) => {}
                Some(Archived::Disabled(true)) => {
                    warn!(
                        "{}: archived: true is not valid, use a filename: {}",
                        chapter_path, header.url
                    );
                }
                Some(Archived::File(filename)) => {
                    let prefix = self.config.archives.as_deref().unwrap_or("");
                    let prefix = prefix.trim_start_matches('/');
                    let archive_path = self.src_dir.join(prefix).join(filename);
                    if !archive_path.exists() {
                        errors.push(format!(
                            "{}: archived file not found: {} (expected at {})",
                            chapter_path,
                            filename,
                            archive_path.display()
                        ));
                    }
                }
            }
        }

        let title = header.title(&self.config);

        let events: Vec<Event<'static>> = vec![
            Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(title.into()))),
            Event::Text(content.to_string().into()),
            Event::End(TagEnd::CodeBlock),
        ];
        Ok(events)
    }
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum Archived {
    Disabled(bool),
    File(String),
}

/// Tracks the first occurrence of a reading block for a given URL, so we can
/// detect conflicting duplicates.
#[derive(Debug)]
struct SeenEntry {
    chapter: String,
    header: Header,
    summary: String,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct Header {
    style: String,
    title: String,
    author: String,
    url: Url,
    archived: Option<Archived>,
}

impl Header {
    pub fn title(&self, config: &Config) -> String {
        let Self {
            style,
            title,
            author,
            url,
            archived,
        } = &self;
        let mut title = format!("<a href='{url}'>{title}</a>");
        if let Some(Archived::File(filename)) = &archived {
            let prefix = config.archives.as_deref().unwrap_or("");
            let archived = format!("{prefix}{filename}");
            title.push_str(&format!(" (<a href='{archived}'>archived</a>)"));
        }
        title.push_str(&format!(" by {author}"));
        let output = format!("admonish {style} title=\"{title}\"");
        output
    }
}

#[derive(Clone, Debug, Default)]
pub struct ReadingPreprocessor;

impl Preprocessor for ReadingPreprocessor {
    fn name(&self) -> &str {
        "reading"
    }

    fn run(&self, ctx: &PreprocessorContext, book: Book) -> Result<Book> {
        let key = format!("preprocessor.{}", self.name());
        let config: Config = ctx.config.get(&key)?.unwrap();
        let src_dir = ctx.root.join(&ctx.config.book.src);
        let instance = Instance::new(config, src_dir);
        instance.map(book)
    }
}
