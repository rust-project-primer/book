# Style Guide

This book should be a primer on starting, maintaining and understanding
real-world Rust projects. It tries to cover the most common problems one might
encounter in a real-world Rust project, and give some advice and context for how
to solve them. It should give advice on structuring them and on tools used to
solve common problems. It should reference existing articles and guides for more
detailed information. The book uses American English throughout.

## Structure

The structure of the book should not be overly nested. For the time being, it is
limited to chapters and sections. Chapters organize it into high-level topics,
while sections focus on a specific sub-topic.

Every chapter should start with a structured overview that explains the topic
and previews the subchapters with cross-references. The strongest chapter
readmes frame what the chapter covers, why it matters, and how the subchapters
relate to each other (see `src/releasing/readme.md` or `src/ci/readme.md` for
examples).

Every section should open by explaining the concept or problem directly, then
cover the tools and approaches for solving it. Where possible, it should give
examples and reference articles, tutorials or chapters in other books that go
into more detail. Every section should end with a `## Reading` section (always
that exact heading) that provides a list of resources for further learning.

## Writing Style

### Voice and Tone

The writing should use second person ("you") to directly address the reader,
creating a conversational but professional tone. The book acts as a guide for
the reader's Rust journey. Avoid overly academic or formal language in favor of
clear, practical explanations.

Examples:

- "When you compile something, you usually create an executable..."
- "If you want to use this, you first need to add support..."
- "You can install it with Cargo..."

### Opening Sections

Sections should open by explaining the concept or problem directly, not by
framing a hypothetical scenario. Do not use italicized "you notice that..."
style openings. The strongest sections get to the point immediately: what the
tool or concept is, why you would use it, and how it works.

Good:

> Semantic versioning encodes information into the version string. A version
> looks like `1.2.3`, where the three numbers are called major, minor, and
> patch.

Avoid:

> _You just released a new version of your crate, but a user reports that their
> code broke after upgrading. You realize you need a way to communicate breaking
> changes..._

### Technical Terminology

- Use backticks for code elements, file names, command names, crate names, and
  technical terms: `rustfmt`, `Cargo.toml`, `cargo build`
- Use italics (_text_) for emphasis on concepts: _cross compilation_,
  _derivation_
- Use bold (**text**) sparingly for major concepts or section highlights
- Define technical terms when first introduced in a section
- Assume basic Rust knowledge but explain more complex concepts
- Reference articles and chapters in other books for detailed explanations

### Lists and Structure

Bullet-point lists are fine for feature lists, options, or procedural steps, but
overusing them makes sections read like notes rather than prose. If a section is
mostly bullet points, rewrite the key content as paragraphs and keep lists for
items that genuinely benefit from the format (comparison tables, command
references, short enumerations). Use numbered lists only when order matters.

### Code Examples

Provide practical, working examples rather than abstract demonstrations. Include
complete command lines with flags, and show expected output when it helps
clarify behavior. Use realistic crate names and scenarios.

Code examples should live in the `examples/` directory and be pulled into
chapters using `{{#include ...}}` directives. This keeps examples buildable and
testable. Short inline code blocks (a few lines of configuration, a single
command) are fine directly in the markdown.

## Admonitions

Use the `mdbook-admonish` plugin for callouts with specific semantic meaning:

### Information Blocks

````
```admonish info
Additional context or background information that helps understand the topic.
````

### Notes

````
```admonish note
Important points to remember or clarifications.
```
````

### Tips

````
```admonish tip
Practical advice or best practices.
```
````

### Warnings

````
```admonish warning
Important caveats or things to be careful about.
```
````

### Examples

````
```admonish example title="Descriptive Title"
Placeholder for examples that will be filled in later, or brief example content.
```
````

## Incomplete Content

Use italicized placeholders for content to be written later:

- `_TODO_` for missing sections
- `_Todo_` for minor missing content
- More descriptive placeholders like
  `_TODO: Explain monomorphization and boxed trait objects_`

## Resources

Use the `mdbook-reading` plugin for external references:

````
```reading
style: article|book|video
title: "The Title"
author: "Full Author Name"
url: https://example.com
archived: filename.pdf (optional)
---
Brief description of the resource content and why it's relevant.
```
````

### Resource Guidelines

- Always provide full name of author when known (not just handles)
- Include archived PDF versions when possible to protect against link rot
- Write summaries based on the actual content of the article, not just restating
  the title. A good summary tells the reader what they will learn and why it is
  worth reading.
- Use appropriate style tags: `article`, `book`, `video`
- Archive PDFs should be in A4 format without headers/footers in `src/archived/`

## Markdown Conventions

### Links

Prefer reference-style links over inline links for better readability:

```markdown
See the [Rust Book][rust-book] for more information.

[rust-book]: https://doc.rust-lang.org/book/
```

### Headings

- Use a single top-level heading (`#`) per file
- Limit to sections (`##`) and subsections (`###`)
- Avoid deeply nested heading structures
- Use descriptive, concise noun phrases ("Link-Time Optimization", not "Enabling
  Link-Time Optimization")
- Tool headings use backtick style: `` ## `cargo-foo` ``, not "## Cargo Foo"

### Line Length

Wrap markdown at approximately 80 characters for better readability in text
editors and version control diffs.

### Code Blocks

Always specify language for syntax highlighting:

```toml
[profile.release]
opt-level = 3
```

## Examples and Projects

Use `mdbook-files` to include complete example projects:

````
```files
path = "project-name"
git_ignore = true
files = ["!.git"]
default_file = "src/lib.rs"
```
````

Projects should be placed in the `examples/` directory and can use git
submodules for external repositories.

## Diagrams

Create diagrams using [draw.io][] with these specifications:

- Use [Routed Gothic][routed-gothic] font at 14pt for consistency
- Export as SVG with embedded diagram data for future editing
- Enable transparency and automatic appearance for dark mode compatibility
- Keep diagrams simple and primarily black-and-white
- Avoid excessive use of color

[draw.io]: https://app.diagrams.net/
[routed-gothic]: https://webonastick.com/fonts/routed-gothic/
