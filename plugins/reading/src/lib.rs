use anyhow::{Context as _, Result, bail};
use log::{error, warn};
use mdbook_preprocessor::{
    Preprocessor, PreprocessorContext,
    book::{Book, BookItem, Chapter},
};
use pulldown_cmark::{CodeBlockKind, CowStr, Event, Options, Parser, Tag, TagEnd};
use pulldown_cmark_to_cmark::cmark;
use serde::Deserialize;
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
        book.items = std::mem::take(&mut book.items)
            .into_iter()
            .map(|section| self.map_book_item(section, &mut errors))
            .collect::<Result<_, _>>()?;
        if !errors.is_empty() {
            for err in &errors {
                error!("{}", err);
            }
            let msg = format!("found {} archived file(s) that do not exist", errors.len());
            error!("{}", msg);
            bail!("{}", msg);
        }
        Ok(book)
    }

    fn map_book_item(&self, item: BookItem, errors: &mut Vec<String>) -> Result<BookItem> {
        let result = match item {
            BookItem::Chapter(chapter) => {
                let title = chapter.name.clone();
                let chapter = self
                    .map_chapter(chapter, errors)
                    .with_context(|| format!("mapping chapter {title:?}"))?;
                BookItem::Chapter(chapter)
            }
            other => other,
        };

        Ok(result)
    }

    fn map_chapter(&self, mut chapter: Chapter, errors: &mut Vec<String>) -> Result<Chapter> {
        let chapter_path = chapter
            .source_path
            .as_deref()
            .map(|p| p.display().to_string())
            .unwrap_or_else(|| chapter.name.clone());
        chapter.content = self.map_markdown(&chapter.content, &chapter_path, errors)?;
        chapter.sub_items = std::mem::take(&mut chapter.sub_items)
            .into_iter()
            .map(|item| self.map_book_item(item, errors))
            .collect::<Result<_, _>>()?;
        Ok(chapter)
    }

    fn map_markdown(
        &self,
        markdown: &str,
        chapter_path: &str,
        errors: &mut Vec<String>,
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
                            .map_code(code, chapter_path, errors)
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
    ) -> Result<Vec<Event<'static>>> {
        let (header, content) = code.split_once("---").unwrap();
        let header: Header = serde_yaml::from_str(header)?;

        // Validate archive status for articles
        if header.style == "article" {
            match &header.archived {
                None => {
                    warn!(
                        "{}: missing archived for article: {}",
                        chapter_path, header.url
                    );
                }
                Some(filename) => {
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

#[derive(Deserialize, Debug)]
pub struct Header {
    style: String,
    title: String,
    author: String,
    url: Url,
    archived: Option<String>,
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
        if let Some(archived) = &archived {
            let prefix = config.archives.as_deref().unwrap_or("");
            let archived = format!("{prefix}{archived}");
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
