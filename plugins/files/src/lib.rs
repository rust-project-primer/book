use anyhow::{bail, Context as _, Result};
use camino::Utf8PathBuf;
use ignore::{overrides::OverrideBuilder, WalkBuilder};
use log::*;
use mdbook_preprocessor::{
    book::{Book, BookItem, Chapter},
    errors::Result as MdbookResult,
    Preprocessor, PreprocessorContext,
};
use pulldown_cmark::{CodeBlockKind, CowStr, Event, Options, Parser, Tag};
use pulldown_cmark_to_cmark::cmark;
use serde::Deserialize;
use std::{collections::BTreeMap, fmt::Write};
use tera::Tera;
use uuid::Uuid;

/// Configuration for an invocation of files
#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Files {
    /// Path to files
    pub path: Utf8PathBuf,

    /// Add a glob to the set of overrides.
    ///
    /// Globs provided here have precisely the same semantics as a single line in a gitignore file,
    /// where the meaning of `!` is inverted: namely, `!` at the beginning of a glob will ignore a
    /// file. Without `!`, all matches of the glob provided are treated as whitelist matches.
    #[serde(default)]
    pub files: Vec<String>,

    /// When specified, path to the file that is opened by default.
    #[serde(default)]
    pub default_file: Option<Utf8PathBuf>,

    /// Process ignores case insensitively
    #[serde(default)]
    pub ignore_case_insensitive: bool,

    /// Do not cross file system boundaries.
    ///
    /// When this option is enabled, directory traversal will not descend into directories that are
    /// on a different file system from the root path.
    #[serde(default)]
    pub same_file_system: bool,

    /// Select the file type given by name.
    #[serde(default)]
    pub types: Vec<String>,

    /// Enables ignoring hidden files.
    #[serde(default)]
    pub hidden: bool,

    /// Whether to follow symbolic links or not.
    #[serde(default)]
    pub follow_links: bool,

    /// Enables reading `.ignore` files.
    ///
    /// `.ignore` files have the same semantics as gitignore files and are supported by search
    /// tools such as ripgrep and The Silver Searcher.
    #[serde(default)]
    pub dot_ignore: bool,

    /// Enables reading a global `gitignore` file, whose path is specified in git’s `core.excludesFile`
    /// config option.
    #[serde(default)]
    pub git_global: bool,

    /// Enables reading `.git/info/exclude` files.
    #[serde(default)]
    pub git_exclude: bool,

    /// Enables reading `.gitignore` files.
    #[serde(default)]
    pub git_ignore: bool,

    /// Whether a git repository is required to apply git-related ignore rules (global rules,
    /// .gitignore and local exclude rules).
    #[serde(default)]
    pub require_git: bool,

    /// Enables reading ignore files from parent directories.
    #[serde(default)]
    pub git_ignore_parents: bool,

    /// The maximum depth to recurse.
    #[serde(default)]
    pub max_depth: Option<usize>,

    /// Whether to ignore files above the specified limit.
    #[serde(default)]
    pub max_filesize: Option<u64>,

    #[serde(default)]
    pub height: Option<String>,
}

/// Configuration for the plugin
#[derive(Deserialize)]
pub struct Config {
    pub prefix: Utf8PathBuf,
}

#[derive(Clone, Debug, Copy)]
pub struct Context<'a> {
    prefix: &'a Utf8PathBuf,
    tera: &'a Tera,
}

pub struct Instance<'a> {
    context: Context<'a>,
    data: Files,
    uuid: Uuid,
}

#[derive(Clone, Debug)]
pub enum TreeNode {
    Directory(BTreeMap<String, TreeNode>),
    File(Uuid),
}

impl Default for TreeNode {
    fn default() -> Self {
        TreeNode::Directory(Default::default())
    }
}

impl TreeNode {
    fn insert(&mut self, path: &[&str], uuid: Uuid) {
        match self {
            TreeNode::Directory(files) if path.len() == 1 => {
                files.insert(path[0].into(), TreeNode::File(uuid));
            }
            TreeNode::Directory(files) => {
                files
                    .entry(path[0].into())
                    .or_default()
                    .insert(&path[1..], uuid);
            }
            TreeNode::File(_file) => panic!("entry exists"),
        }
    }

    pub fn render(&self) -> Result<String> {
        let mut output = String::new();
        match self {
            TreeNode::File(_) => bail!("root node cannot be file"),
            TreeNode::Directory(files) => Self::render_files(&mut output, files)?,
        }
        Ok(output)
    }

    fn render_files(output: &mut dyn Write, files: &BTreeMap<String, TreeNode>) -> Result<()> {
        write!(output, "<ul>")?;
        for (path, node) in files {
            node.render_inner(output, path)?;
        }
        write!(output, "</ul>")?;
        Ok(())
    }

    fn render_inner(&self, output: &mut dyn Write, name: &str) -> Result<()> {
        match self {
            TreeNode::File(uuid) => {
                write!(
                    output,
                    r#"<li id="button-{uuid}" class="mdbook-files-button">{name}</li>"#
                )?;
            }
            TreeNode::Directory(files) => {
                write!(
                    output,
                    r#"<li class="mdbook-files-folder"><span>{name}/</span>"#
                )?;
                Self::render_files(output, files)?;
                write!(output, "</li>")?;
            }
        }
        Ok(())
    }
}

pub type FilesMap = BTreeMap<Utf8PathBuf, Uuid>;

impl<'a> Instance<'a> {
    fn parent(&self) -> Utf8PathBuf {
        self.context.prefix.join(&self.data.path)
    }

    fn files(&self) -> Result<FilesMap> {
        let mut paths: FilesMap = Default::default();
        let parent = self.parent();
        let mut overrides = OverrideBuilder::new(&parent);
        for item in &self.data.files {
            overrides.add(item)?;
        }
        let overrides = overrides.build()?;
        let mut walker = WalkBuilder::new(&parent);
        walker
            .standard_filters(false)
            .ignore_case_insensitive(self.data.ignore_case_insensitive)
            .same_file_system(self.data.same_file_system)
            .require_git(self.data.require_git)
            .hidden(self.data.hidden)
            .ignore(self.data.dot_ignore)
            .git_ignore(self.data.git_ignore)
            .git_exclude(self.data.git_exclude)
            .git_global(self.data.git_global)
            .parents(self.data.git_ignore_parents)
            .follow_links(self.data.follow_links)
            .max_depth(self.data.max_depth)
            .overrides(overrides)
            .max_filesize(self.data.max_filesize);

        let walker = walker.build();

        for path in walker {
            let path = path?;
            if path.file_type().unwrap().is_file() {
                paths.insert(path.path().to_path_buf().try_into()?, Uuid::new_v4());
            }
        }

        info!("Found {} matching files", paths.len());
        if paths.is_empty() {
            bail!("No files matched");
        }

        Ok(paths)
    }

    fn left(&self, files: &FilesMap) -> Result<String> {
        let mut output = String::new();
        let parent = self.parent();
        output.push_str(r#"<div class="mdbook-files-left">"#);

        let mut root = TreeNode::default();
        for (path, uuid) in files.iter() {
            let path = path.strip_prefix(&parent)?;
            let path: Vec<_> = path.components().map(|c| c.as_str()).collect();
            root.insert(&path[..], *uuid);
        }

        let list = root.render()?;
        output.push_str(&list);
        output.push_str("</div>");
        Ok(output)
    }

    fn right(&self, files: &FilesMap) -> Result<Vec<Event<'static>>> {
        let mut events = vec![];
        events.push(Event::Html(CowStr::Boxed(
            r#"<div class="mdbook-files-right">"#.to_string().into(),
        )));

        for (path, uuid) in files {
            info!("Reading {path}");
            let contents = std::fs::read_to_string(path)?;
            let extension = path.extension().unwrap_or("");
            let tag = Tag::CodeBlock(CodeBlockKind::Fenced(CowStr::Boxed(extension.into())));

            events.push(Event::Html(CowStr::Boxed(
                format!(r#"<div id="file-{uuid}" class="mdbook-file visible">"#).into(),
            )));

            events.push(Event::Start(tag.clone()));
            events.push(Event::Text(CowStr::Boxed(contents.into())));
            events.push(Event::End(tag));

            events.push(Event::Html(CowStr::Boxed("</div>".to_string().into())));
        }

        events.push(Event::Html(CowStr::Boxed("</div>".to_string().into())));
        Ok(events)
    }

    fn events(&self) -> Result<Vec<Event<'static>>> {
        let paths = self.files()?;

        let mut events = vec![];

        let height = self.data.height.as_deref().unwrap_or("300px");
        events.push(Event::Html(CowStr::Boxed(
            format!(
                r#"<div id="files-{}" class="mdbook-files" style="height: {height};">"#,
                self.uuid
            )
            .into(),
        )));

        events.push(Event::Html(CowStr::Boxed(self.left(&paths)?.into())));
        events.append(&mut self.right(&paths)?);
        events.push(Event::Html(CowStr::Boxed("</div>".to_string().into())));

        let uuids: Vec<Uuid> = paths.values().copied().collect();
        let visible = match &self.data.default_file {
            Some(file) => paths.get(&self.parent().join(file)).unwrap(),
            None => &uuids[0],
        };

        let mut context = tera::Context::new();
        context.insert("uuids", &uuids);
        context.insert("visible", visible);

        let script = self.context.tera.render("script", &context)?;

        events.push(Event::Html(CowStr::Boxed(
            format!("<script>{script}</script>").into(),
        )));

        events.push(Event::HardBreak);
        Ok(events)
    }
}

impl<'b> Context<'b> {
    fn map(&self, book: Book) -> Result<Book> {
        let mut book = book;
        book.items = std::mem::take(&mut book.items)
            .into_iter()
            .map(|section| self.map_book_item(section))
            .collect::<Result<_, _>>()?;
        Ok(book)
    }

    fn map_book_item(&self, item: BookItem) -> Result<BookItem> {
        let result = match item {
            BookItem::Chapter(chapter) => BookItem::Chapter(self.map_chapter(chapter)?),
            other => other,
        };

        Ok(result)
    }

    fn map_code(&self, code: CowStr<'_>) -> Result<Vec<Event<'static>>> {
        Instance {
            data: toml::from_str(&code)?,
            uuid: Uuid::new_v4(),
            context: *self,
        }
        .events()
    }

    fn label(&self) -> &str {
        "files"
    }

    fn map_chapter(&self, mut chapter: Chapter) -> Result<Chapter> {
        chapter.content = self.map_markdown(&chapter.content)?;
        chapter.sub_items = std::mem::take(&mut chapter.sub_items)
            .into_iter()
            .map(|item| self.map_book_item(item))
            .collect::<Result<_, _>>()?;
        Ok(chapter)
    }

    fn map_markdown(&self, markdown: &str) -> Result<String> {
        let mut parser = Parser::new_ext(markdown, Options::all());
        let mut events = vec![];

        loop {
            let next = parser.next();
            match next {
                None => break,
                Some(Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(label))))
                    if &*label == self.label() =>
                {
                    let mapped = match parser.next() {
                        Some(Event::Text(code)) => self.map_code(code).context("Mapping code")?,
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
}

#[derive(Clone, Debug)]
pub struct FilesPreprocessor {
    templates: Tera,
}

impl Default for FilesPreprocessor {
    fn default() -> Self {
        Self::new()
    }
}

impl FilesPreprocessor {
    pub fn new() -> Self {
        let mut templates = Tera::default();
        templates
            .add_raw_template("script", include_str!("script.js.tera"))
            .unwrap();
        Self { templates }
    }
}

impl Preprocessor for FilesPreprocessor {
    fn name(&self) -> &str {
        "files"
    }

    fn run(&self, ctx: &PreprocessorContext, book: Book) -> MdbookResult<Book> {
        let config: Config = ctx
            .config
            .get(&format!("preprocessor.{}", self.name()))?
            .unwrap();
        let instance = Context {
            prefix: &config.prefix,
            tera: &self.templates,
        };
        instance.map(book)
    }
}
