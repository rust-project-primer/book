# Code Documentation

Rust has first-class support for code documentation through
[rustdoc][rustdoc-book], which parses documentation comments in your source code
and generates searchable, cross-linked HTML. For published crates,
[docs.rs][docs-rs] builds and hosts this documentation automatically. The result
is that most Rust libraries have browsable API documentation available without
any extra effort from the author — and with some effort, that documentation can
be genuinely good.

## Doc Comments

Rustdoc recognizes two kinds of documentation comments. Outer doc comments
(`///`) document the item that follows them — a function, struct, enum, trait,
or module. Inner doc comments (`//!`) document the item that contains them,
which in practice means the crate root (`lib.rs` or `main.rs`) or a module file.

```rust
//! This crate provides utilities for parsing configuration files.
//!
//! It supports TOML, JSON, and YAML formats, with automatic
//! type-safe deserialization using serde.

/// Parse a configuration file at the given path.
///
/// Returns an error if the file does not exist or contains
/// invalid syntax for the detected format.
pub fn parse_config(path: &std::path::Path) -> Result<Config, Error> {
    // ...
#     todo!()
}
```

Doc comments support Markdown: headings, lists, links, emphasis, and fenced code
blocks. Rustdoc adds some extensions on top of standard Markdown, most notably
intra-doc links and doc tests (both covered below).

## Intra-Doc Links

Rustdoc can resolve links to other items in your crate (or its dependencies)
using Rust path syntax inside square brackets. This is more robust than linking
to a URL, because the compiler checks that the target exists and will warn if it
breaks.

```rust
/// Parses the configuration and returns a [`Config`] struct.
///
/// For format-specific options, see [`Config::format`].
/// For error handling, see the [`Error`] type.
pub fn parse_config(path: &std::path::Path) -> Result<Config, Error> {
    // ...
#     todo!()
}
```

These links resolve to the correct page in the generated documentation. You can
link to types, functions, methods, modules, traits, and even specific trait
implementations. The full syntax is documented in the [rustdoc
book][intra-doc-links].

```admonish info
If you want to ensure that your intra-doc links are not broken, Clippy
has a lint for it:
[`doc_broken_link`](https://rust-lang.github.io/rust-clippy/stable/index.html?search=document#doc_broken_link).
```

## Doc Tests

Code blocks in doc comments are compiled and run as tests by `cargo test`. This
means your examples are checked by the compiler — they cannot silently fall out
of date when you change the API. A doc test that fails to compile or panics at
runtime will fail the test suite.

````rust
/// Add two numbers together.
///
/// ```
/// assert_eq!(my_crate::add(2, 3), 5);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
````

Lines prefixed with `#` are compiled but hidden from the rendered documentation.
This is useful for boilerplate like imports, error handling, or setup code that
would distract from the example:

````rust
/// Connect to the database and run a query.
///
/// ```
/// # use my_crate::Database;
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let db = Database::connect("localhost:5432")?;
/// let rows = db.query("SELECT 1")?;
/// assert_eq!(rows.len(), 1);
/// # Ok(())
/// # }
/// ```
pub fn connect(addr: &str) -> Result<Database, Error> {
    // ...
#     todo!()
}
````

You can annotate code blocks to change how they are handled. `should_panic`
marks a test that is expected to panic. `no_run` compiles the code but does not
execute it, which is useful for examples that require network access or specific
hardware. `ignore` skips compilation entirely — use it sparingly, since it
defeats the purpose of doc tests. `compile_fail` asserts that the code does
_not_ compile, which is useful for documenting what a type system prevents.

## Sections

Rustdoc recognizes certain conventional headings in doc comments and gives them
special treatment in the rendered output. The most important ones:

- **`# Examples`** — rendered prominently and expected by convention on public
  API items. Clippy's `missing_doc_code_examples` lint (currently nightly-only)
  checks for this.
- **`# Panics`** — documents the conditions under which a function panics.
- **`# Errors`** — documents the error variants a function can return.
- **`# Safety`** — required on `unsafe` functions to document the invariants the
  caller must uphold.

These headings appear in a consistent location in the generated documentation,
making it easy for readers to find the information they need.

```admonish info
If you want to enforce these, Clippy has lints for them:
[`missing_errors_doc`](https://rust-lang.github.io/rust-clippy/stable/index.html?search=document#missing_errors_doc),
[`missing_panics_doc`](https://rust-lang.github.io/rust-clippy/stable/index.html?search=document#missing_panics_doc) and
[`missing_safety_doc`](https://rust-lang.github.io/rust-clippy/stable/index.html?search=document#missing_safety_doc).
```

## Writing Good Documentation

Having documentation is not the same as having _good_ documentation. The most
common failure in Rust crate documentation is restating what the reader can
already see: a doc comment on `struct Config` that says "The Config struct" adds
nothing. Good documentation describes _behavior_: what a function does, under
what conditions it fails, what invariants a type maintains, and how it relates
to other parts of the API.

A few patterns that consistently produce better documentation:

**Describe behavior, not names.** Instead of "Parses the input", explain what
format is expected, what happens with invalid input, and what the caller gets
back. The reader can see the function name — they came to the docs to learn what
the name doesn't tell them.

**Link to related items.** When a method returns a type, link to that type. When
two methods are complementary (like `lock` and `try_lock`), cross-reference
them. Intra-doc links make this easy and the compiler keeps them from going
stale. The standard library is a good model: the docs for `Option` link
extensively between `map`, `and_then`, `unwrap_or_else`, and related methods,
helping users find the right combinator.

**Show realistic examples.** An `# Examples` section with
`assert_eq!(add(2, 3), 5)` demonstrates that the function works, but it does not
help a reader who needs to understand how to use it in context. The best
examples show a small but realistic scenario: setting up the inputs, calling the
function, and handling the result. Hidden lines (`#`) keep the boilerplate out
of the way without removing it from compilation.

**Document failure modes.** If a function returns `Result`, the `# Errors`
section should list the conditions that produce each error variant. If a
function panics, the `# Panics` section should state when. These sections are
not just conventions — Clippy's `missing_errors_doc` and `missing_panics_doc`
lints (in the `pedantic` group) can check for them.

**Explain design choices.** Most crates document _what_ they do but not _why_. A
brief explanation of why an API is shaped a certain way — why there are two
duration types, why a particular trait does not implement `Copy`, why the
builder pattern was chosen over a constructor with many arguments — helps users
form a mental model that makes the rest of the API predictable. This kind of
explanation can live in the crate-level docs, in a dedicated module (see below),
or in a design document linked from the repository.

## Crate-Level Documentation

Crate-level documentation (the text that appears on the crate's front page on
docs.rs) is written with inner doc comments (`//!`) at the top of `lib.rs` or
`main.rs`. This is the first thing a potential user sees, and it should answer
three questions: what does this crate do, when should you use it, and how do you
get started? A good crate root includes a brief overview, a usage example, and
links to the most important types and modules.

For longer documentation, writing Markdown directly in a Rust source file can be
awkward. An alternative is to write the documentation in a separate Markdown
file and include it:

```rust
#![doc = include_str!("../README.md")]
```

This pulls the contents of `README.md` into the crate-level documentation at
compile time. It keeps your README and your crate documentation in sync — you
write the overview once and it appears both on GitHub and on docs.rs.

### The `_documentation` Module Pattern

The [jiff](https://docs.rs/jiff) crate demonstrates a pattern for making
longer-form documentation discoverable through docs.rs. It creates a
`_documentation` module (the leading underscore sorts it to the top of the
module list) that includes separate Markdown files as submodules:

```rust
pub mod _documentation {
    #[doc = include_str!("../COMPARE.md")]
    pub mod comparison {}

    #[doc = include_str!("../DESIGN.md")]
    pub mod design {}
}
```

Each empty submodule renders as a page on docs.rs with the full content of the
included Markdown file. This makes design rationale, comparison guides, and
migration documentation part of the API docs rather than files buried in the
repository. The Markdown files remain the single source of truth — they are
readable on GitHub and rendered on docs.rs without duplication.

This pattern is worth considering for any crate where users benefit from
understanding the design philosophy or the differences between your crate and
alternatives. The [snafu](https://docs.rs/snafu) and
[clap](https://docs.rs/clap) crates use a similar approach for their user guides
and troubleshooting documentation.

The `_documentation` module pattern has the advantage of requiring no separate
hosting — everything lives on docs.rs alongside the API reference. The tradeoff
is that docs.rs renders plain Markdown without custom navigation, styling, or
search. For larger projects that need tutorials, guides, or structured
walkthroughs, a standalone [mdBook](book.md) hosted as a project website is
often a better fit.

## Feature-Gated Documentation

If your crate has optional features, you can annotate items so that docs.rs
shows which feature is required to use them:

```rust
#[doc(cfg(feature = "json"))]
pub fn parse_json(input: &str) -> Result<Config, Error> {
    // ...
#     todo!()
}
```

On docs.rs, this renders a badge next to the item indicating the required
feature. To build documentation for all features locally, pass `--all-features`
to `cargo doc`. docs.rs reads a `[package.metadata.docs.rs]` section in your
`Cargo.toml` to determine which features to enable when building:

```toml
[package.metadata.docs.rs]
all-features = true
```

## Scraped Examples

Rustdoc can automatically find uses of your public API items in the `examples/`
directory and display them inline in the generated documentation. This means
that if you have an example binary that calls `parse_config`, the docs page for
`parse_config` will show that usage in context, without you writing a separate
`# Examples` section. To enable this when building docs locally:

```bash
cargo doc --scrape-examples
```

docs.rs enables scraped examples automatically for published crates that have an
`examples/` directory. This is a good reason to write well-structured example
programs even beyond their value as standalone demos — they feed directly into
your API documentation. The [Bevy](https://docs.rs/bevy) game engine uses this
extensively: its hundreds of examples appear inline throughout the API docs,
giving users real-world usage patterns for every major type.

## Generating Documentation

Run `cargo doc` to build documentation for your crate and its dependencies. Add
`--open` to open it in a browser, and `--no-deps` to skip dependencies if you
only want your own crate's docs:

```bash
cargo doc --open --no-deps
```

To catch broken links and other documentation issues during development, build
with warnings turned into errors:

```bash
RUSTDOCFLAGS="-D warnings" cargo doc --no-deps
```

This is worth running in CI. The [GitHub Actions](../ci/github.md) and
[GitLab CI](../ci/gitlab.md) chapters include examples of documentation jobs
that use this flag. For publishing documentation to a hosted location, see the
GitHub Pages and GitLab Pages sections in those chapters.

## Enforcing Documentation

For libraries, enforcing that all public API items have documentation prevents
gaps from accumulating over time. The `missing_docs` lint checks for public
items without doc comments. Setting it to `deny` makes missing documentation a
compile error:

```rust
#![deny(missing_docs)]
```

This is a strong stance — it means no public function, struct, enum variant, or
trait method can be added without documentation. For projects that are still
evolving rapidly, `warn` is a softer alternative that surfaces the gaps without
blocking compilation. For established libraries, `deny` is the better default:
it is much easier to maintain documentation coverage than to backfill it later.

To also catch broken intra-doc links, enable the corresponding lint:

```rust
#![deny(rustdoc::broken_intra_doc_links)]
```

Combined with `RUSTDOCFLAGS="-D warnings"` in CI, this ensures that
documentation links stay valid as the codebase evolves.

Clippy has a number of lints that can be useful for enforcing documentation
style, see
[Clippy Lints](https://rust-lang.github.io/rust-clippy/stable/index.html) for
more context. Many of these are turned on when using the `clippy::pedantic` lint
level.

## Reading

```reading
style: book
title: The Rustdoc Book
url: https://doc.rust-lang.org/rustdoc/what-is-rustdoc.html
author: The Rust Project
---
The official reference for rustdoc. Covers doc comment syntax, doc tests,
intra-doc links, the `#[doc]` attribute, and configuration options. The
chapter on doc tests is particularly useful for understanding the annotation
syntax (`should_panic`, `no_run`, `compile_fail`, hidden lines with `#`).
```

```reading
style: book
title: "Rust API Guidelines: Documentation"
url: https://rust-lang.github.io/api-guidelines/documentation.html
author: The Rust Project
---
Guidelines for documenting Rust libraries, including conventions for crate-level
docs, the `# Examples`, `# Errors`, `# Panics`, and `# Safety` sections, and
what makes documentation effective for downstream users. Part of the broader
API Guidelines that cover naming, interoperability, and type safety.
```

```reading
style: article
title: "Making Great Docs with Rustdoc"
url: https://www.tangramvision.com/blog/making-great-docs-with-rustdoc
author: Tangram Vision
---
Practical advice on writing effective rustdoc documentation, from structuring
crate-level docs to writing good examples. Covers the `include_str!` pattern,
doc tests, and strategies for keeping documentation accurate as the codebase
changes.
```

```reading
style: article
title: "Rust Documentation Ecosystem Review"
url: https://www.harudagondi.space/blog/rust-documentation-ecosystem-review/
author: Gio Genre De Asis
---
A thorough evaluation of documentation quality across ~25 popular Rust crates
using the Diátaxis framework (tutorials, how-to guides, reference,
explanation). Scores each crate on comprehensiveness, discoverability,
approachability, and design philosophy. The jiff crate stands out for its
`_documentation` module pattern and design rationale docs; ratatui for its
iterative tutorials and website. A valuable read for understanding what
separates adequate documentation from genuinely helpful documentation.
```

[rustdoc-book]: https://doc.rust-lang.org/rustdoc/
[docs-rs]: https://docs.rs
[intra-doc-links]:
  https://doc.rust-lang.org/rustdoc/write-documentation/linking-to-items-by-name.html
