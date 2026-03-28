use anyhow::{Context as _, Result};
use mdbook_preprocessor::{
    Preprocessor, PreprocessorContext,
    book::{Book, BookItem, Chapter},
};
use pulldown_cmark::{CodeBlockKind, CowStr, Event, Options, Parser, Tag};
use pulldown_cmark_to_cmark::cmark;

/// Parse a fenced code block info string to check for the `framed` attribute.
///
/// Returns `Some((language, title))` if `framed` is present, where `title` is
/// the optional `title="..."` value. Returns `None` if `framed` is absent.
fn parse_framed(info: &str) -> Option<(String, Option<String>)> {
    let mut parts = InfoParser::new(info);
    let language = parts.next_token().unwrap_or_default();

    let mut found_framed = false;
    let mut title = None;

    while let Some(token) = parts.next_token() {
        if token == "framed" {
            found_framed = true;
        } else if let Some(value) = token.strip_prefix("title=") {
            title = Some(unquote(value));
        }
    }

    if found_framed {
        Some((language, title))
    } else {
        None
    }
}

/// Simple tokenizer for the info string that handles quoted values.
struct InfoParser<'a> {
    remaining: &'a str,
}

impl<'a> InfoParser<'a> {
    fn new(s: &'a str) -> Self {
        Self { remaining: s.trim() }
    }

    fn next_token(&mut self) -> Option<String> {
        self.remaining = self.remaining.trim_start();
        if self.remaining.is_empty() {
            return None;
        }

        // Find the end of this token, respecting quoted values after `=`.
        let mut chars = self.remaining.char_indices();
        let mut end = self.remaining.len();
        let mut in_key = true;

        while let Some((i, ch)) = chars.next() {
            if in_key {
                if ch == '=' {
                    in_key = false;
                    // Check if next char is a quote
                    if let Some(&(_, q)) = chars.clone().peekable().peek() {
                        if q == '"' || q == '\'' {
                            let quote = q;
                            chars.next(); // consume opening quote
                            // Scan until closing quote
                            for (j, c) in chars.by_ref() {
                                if c == quote {
                                    end = j + 1;
                                    break;
                                }
                            }
                            break;
                        }
                    }
                } else if ch.is_whitespace() {
                    end = i;
                    break;
                }
            } else if ch.is_whitespace() {
                end = i;
                break;
            }
        }

        let token = &self.remaining[..end];
        self.remaining = &self.remaining[end..];
        Some(token.to_string())
    }
}

fn unquote(s: &str) -> String {
    let s = s.trim();
    if (s.starts_with('"') && s.ends_with('"')) || (s.starts_with('\'') && s.ends_with('\'')) {
        s[1..s.len() - 1].to_string()
    } else {
        s.to_string()
    }
}

fn escape_html(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        // Encode newlines so the entire HTML stays on one line.
        // CommonMark terminates type-6 HTML blocks (starting with <div>)
        // at blank lines, which would break the frame if the code content
        // contains empty lines. &#10; renders as a newline inside <pre>.
        .replace('\n', "&#10;")
}

fn map_markdown(markdown: &str) -> Result<String> {
    let mut parser = Parser::new_ext(markdown, Options::all());
    let mut events = vec![];

    loop {
        let next = parser.next();
        match next {
            None => break,
            Some(Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(ref info))))
                if parse_framed(info).is_some() =>
            {
                let (language, title) = parse_framed(info).unwrap();

                // Collect the code content
                let code = match parser.next() {
                    Some(Event::Text(code)) => code.to_string(),
                    other => unreachable!("Expected text in code block, got {other:?}"),
                };

                // Consume the end tag
                parser.next();

                // Build the frame HTML
                let title_html = match &title {
                    Some(t) => format!(
                        r#"<span class="mdbook-frames-title">{}</span>"#,
                        escape_html(t)
                    ),
                    None => String::new(),
                };

                // Emit the entire framed block as a single HTML event.
                // Mixing HTML events with markdown code block events causes
                // pulldown-cmark-to-cmark to serialize them separately, breaking
                // the nesting.
                let lang_attr = if language.is_empty() {
                    String::new()
                } else {
                    format!(r#" class="language-{language}""#)
                };

                let html = format!(
                    r#"<div class="mdbook-frames-window"><div class="mdbook-frames-titlebar">{title_html}<div class="mdbook-frames-buttons"><span class="mdbook-frames-button mdbook-frames-close"></span><span class="mdbook-frames-button mdbook-frames-minimize"></span><span class="mdbook-frames-button mdbook-frames-maximize"></span></div></div><div class="mdbook-frames-content"><pre><code{lang_attr}>{}</code></pre></div></div>"#,
                    escape_html(&code),
                );

                events.push(Event::Html(CowStr::Boxed(html.into())));
            }
            Some(event) => events.push(event),
        }
    }

    let mut buf = String::with_capacity(markdown.len());
    let output = cmark(events.iter(), &mut buf).map(|_| buf)?;
    Ok(output)
}

fn map_chapter(mut chapter: Chapter) -> Result<Chapter> {
    chapter.content = map_markdown(&chapter.content)
        .with_context(|| format!("mapping chapter {:?}", chapter.name))?;
    chapter.sub_items = std::mem::take(&mut chapter.sub_items)
        .into_iter()
        .map(map_book_item)
        .collect::<Result<_, _>>()?;
    Ok(chapter)
}

fn map_book_item(item: BookItem) -> Result<BookItem> {
    match item {
        BookItem::Chapter(chapter) => Ok(BookItem::Chapter(map_chapter(chapter)?)),
        other => Ok(other),
    }
}

#[derive(Clone, Debug, Default)]
pub struct FramesPreprocessor;

impl Preprocessor for FramesPreprocessor {
    fn name(&self) -> &str {
        "frames"
    }

    fn run(&self, _ctx: &PreprocessorContext, book: Book) -> Result<Book> {
        let mut book = book;
        book.items = std::mem::take(&mut book.items)
            .into_iter()
            .map(map_book_item)
            .collect::<Result<_, _>>()?;
        Ok(book)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_framed_basic() {
        let result = parse_framed("bash framed");
        assert_eq!(result, Some(("bash".into(), None)));
    }

    #[test]
    fn parse_framed_with_title() {
        let result = parse_framed(r#"rust framed title="Cargo.toml""#);
        assert_eq!(result, Some(("rust".into(), Some("Cargo.toml".into()))));
    }

    #[test]
    fn parse_framed_title_single_quotes() {
        let result = parse_framed("bash framed title='my terminal'");
        assert_eq!(result, Some(("bash".into(), Some("my terminal".into()))));
    }

    #[test]
    fn parse_not_framed() {
        let result = parse_framed("rust");
        assert_eq!(result, None);
    }

    #[test]
    fn parse_framed_no_language() {
        let result = parse_framed("framed");
        // "framed" is consumed as the language since it's the first token,
        // but framed flag is not found separately
        assert_eq!(result, None);
    }

    #[test]
    fn markdown_passthrough() {
        let input = "```rust\nfn main() {}\n```\n";
        let output = map_markdown(input).unwrap();
        assert!(!output.contains("mdbook-frames"));
    }

    #[test]
    fn markdown_framed() {
        let input = "```bash framed\ncargo build\n```\n";
        let output = map_markdown(input).unwrap();
        assert!(output.contains("mdbook-frames-window"));
        assert!(output.contains("mdbook-frames-titlebar"));
        assert!(output.contains("cargo build"));
    }

    #[test]
    fn markdown_framed_with_title() {
        let input = "```toml framed title=\"Cargo.toml\"\n[package]\nname = \"foo\"\n```\n";
        let output = map_markdown(input).unwrap();
        assert!(output.contains("Cargo.toml"));
        assert!(output.contains("mdbook-frames-title"));
    }
}
