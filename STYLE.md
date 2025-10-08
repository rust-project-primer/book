# Style Guide

This book should be a primer on starting, maintaining and understanding
real-world Rust projects. It tries to cover the most common problems one might
encounter in a real-world Rust project, and give some advice and context for how
to solve them. It should give advise on structing them and on tools used to
solve common problems. It should reference existing articles and guides for more
detailed information.

## Structure

The structure of the book should not be overly nested. For the time being, it is
limited to chapters and sections. Chapters organize it into high-level topics,
while sections focus on a specific sub-topic.

Every chapter should start with a brief introduction and high-level explanation
of what topic it is about. If possible, it should link to articles that cover
this topic.

Every section should start with a brief introduction to explain what the problem
is that it is trying to solve. It should have subheadings for solutions to the
problem. Where possible, it should give examples of how to solve the problem,
and reference articles, tutorials or chapters in other books that go into more
detail. Every section should end with a _Reading_ section that provides a list
of resources for further learning.

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

### Problem-Driven Approach

Many sections begin with italicized scenario descriptions that set up real-world
problems the reader might face. These scenarios should be specific and
relatable, often framed as situations where something has gone wrong or a need
has emerged.

Format:

```
_Scenario description explaining the problem context and why the reader would
need the solution presented in this section._
```

Example:

```
_While Rust does have a local build cache in the `target` folder, you notice
that this is not always useful. Especially in the CI system, the entire project
is always rebuilt. It makes you wonder if there is some way to cache the build
artifacts in a global cache that your team and the CI server can make use of._
```

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

- Use bullet points for procedural steps, feature lists, and options
- Use numbered lists only when order matters (like installation steps)
- Prefer prose paragraphs over excessive bullet points for better flow
- Keep paragraphs focused on single concepts

### Code Examples

Provide practical, working examples rather than abstract demonstrations:

- Include complete command lines with flags
- Show both the command and expected output when helpful
- Use realistic crate names and scenarios
- Prefer inline code blocks over separate files unless the example is
  substantial

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
- Write concise but informative descriptions explaining the resource's value
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
- Use descriptive, concise headings

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
