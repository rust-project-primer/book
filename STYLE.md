# Style Guide

This book should be a primer on starting, maintaining and understanding
real-world Rust projects. It tries to cover the most common problems one might
encounter in a real-world Rust project, and give some advice and context for how
to solve them.

The main focus of this book is to give an overview of how to solve common
problems, referring to existing resources for more detailed information.

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
detail.

## Writing Style

The writing style should be clear and concise, using simple language so that
people new to Rust can understand it. It should avoid flowery language and
define the technical terms that it uses.

The book should not be overly verbose or be a tutorial. It should be a reference
book that provides a quick overview of how to solve common problems. It may
contain examples, by directly including Rust code in code tags, or by including
an example Rust project using `mdbook-files`.

Including an entire project with `mdbook-files` works by placing the example
Rust project inside `examples/`, and referencing it in the section using the
`mdbook-files` syntax. The `path` key is relative to the `examples/` directory.

    ```files
    path = "levenshtein"
    git_ignore = true
    files = ["!.git"]
    default_file = "src/lib.rs"
    ```

It is possible to use git-submodules to include examples that live in other
repositories.

Using bullet-point lists is allowed, but writing should prefer using prose
(paragraphs), because they break up the reading flow less.

## Resources

There is some special syntax for referring to existing resources, which is
provided by the `mdbook-reading` plugin. Every external resource has a block
like this:

    ```reading
    style: article
    title: "The Title"
    author: "The Author"
    url: https://example.common
    archived: name-of-exported.pdf
    ---
    Summary of contents of this resource.
    ```

This will automatically insert an admonition with the title, a link to the
archived resource, and the author name, along with a summary of the contents.
When referencing resources, the full name of the author should be quoted (rather
than a handle), if this is known.

When possible, resource should be archived. You can do this by using your
browser's "Print to PDF" functionality. The PDF should be in A4 format, saved
without headers or footers. The resulting PDF should be saved inside
`src/archived/`, and referenced by the filename of the PDF in the `archived: `
field of the resource block. This archiving is intended to protect against
bit-rot, since a lot of the resources are on people's personal blogs which may
disappear or be restructured over time.
