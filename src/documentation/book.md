# Book

While having code-level documentation is useful for some cases, another important
aspect is having high-level documentation which explains:

- System architecture
- Crate architecture
- How to launch and use things

Not explicitly documenting these somewhere leads to having projects where this
important context lives in a few people's brains. It can block others in the
team from making changes by not knowing how things fit together.

## mdBook

In the Rust community, the [mdBook][mdbook] tool has become the standard way to write 
this kind of documentation. It consumes the documentation in the form of Markdown and
renders it nicely into a HTML book.

Ideally, inside every project you will want to have some kind of `book/` folder
containing this high-level documentation. You can even have multiple books or
sections, targeted at different audiences.

You can install `mdbook` like this:

```
cargo install mdbook
```

You can then initialize a new project like this:

```
mdbook init
```

Finally, you can build or serve your project locally like this:

```
mdbook build
mdbook serve
```

### Examples

### Usage

## Reading

```reading
style: book
title: mdBook Book
url: https://rust-lang.github.io/mdBook/
author: rust-lang
---
This is the official book of the mdBook project. It explains all the various
features that mdBook has, and how to use them.
```

```reading
style: article
title: "mdBook: Thirt-Party Plugins"
url: https://github.com/rust-lang/mdBook/wiki/Third-party-plugins
author: mdBook
---
A list of third-party plugins for mdBook, contains various preprocessors and backends.
```

[mdbook]: https://github.com/rust-lang/mdBook
