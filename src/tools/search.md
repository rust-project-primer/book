# Code Search

Searching across a codebase is one of the most common tasks when navigating
unfamiliar code or tracking down all uses of a function, type, or dependency.
There are two approaches: text-based search (fast, works everywhere) and
structural search (syntax-aware, understands code structure).

## `ripgrep`

[`ripgrep`](https://github.com/BurntSushi/ripgrep) is a command-line tool for
searching code bases using [regular expressions][regex]. It is very fast, making
use of Rust's powerful [regex][rust-regex] crate. It understands git
repositories and respects `.gitignore` files, making it particularly suitable
for searching software projects.

```admonish note
If you use Visual Studio Code, you are already using ripgrep. VS Code uses
[ripgrep internally](https://github.com/microsoft/vscode-ripgrep) to implement
its search functionality.
```

You can install it with Cargo:

    cargo install ripgrep

Running this will install the `rg` binary, which you can use to search code
projects. You can then use it to search for patterns.

```
$ rg uuid::
database/src/main.rs
8:use uuid::Uuid;

protocol/src/types.rs
10:use uuid::Uuid;

common/src/entities.rs
12:use uuid::Uuid;
```

## `ast-grep`

[`ast-grep`](https://ast-grep.github.io/) (command: `sg`) is a structural search
tool. Where ripgrep matches text with regular expressions, ast-grep parses code
into an abstract syntax tree (AST) and matches against tree patterns. This makes
it syntax-aware: it can distinguish between a function call and a variable name
that happens to have the same text, and it ignores whitespace and formatting
differences that would break a regex.

You can install it with Cargo:

    cargo install ast-grep

ast-grep uses patterns that look like the code you are searching for, with
metavariables (prefixed with `$`) standing in for parts you don't care about.
For example, to find all places where `.unwrap()` is called on anything:

```
$ sg -p '$A.unwrap()' -l rust
src/config.rs
15:    let file = std::fs::read_to_string(path).unwrap();

src/main.rs
42:    let port = env::var("PORT").unwrap();
```

You can also use it for codemod-style replacements. For example, to replace all
`unwrap()` calls with `expect()`:

```bash
sg -p '$A.unwrap()' -r '$A.expect("todo: handle error")' -l rust
```

ast-grep supports Rust and 20+ other languages out of the box through
[tree-sitter](https://tree-sitter.github.io/) parsers. Beyond one-off searches,
it can also be used as a linter by defining custom rules in YAML configuration
files, and it has a language server for editor integration.

## Reading

```reading
style: article
title: "ripgrep is faster than {grep, ag, git grep, ucg, pt, sift}"
url: https://blog.burntsushi.net/ripgrep/
author: Andrew Gallant
archived: burntsushi-ripgrep-is-faster-than.pdf
---
Andrew, the author of ripgrep, introduces the tool in this article, explains how it
works and compares it to some common similar tools used by developers, showing how
it performs better and how it excels at dealing with Unicode, something other tools
struggle with.
```

[regex]: https://en.wikipedia.org/wiki/Regular_expression
[rust-regex]: https://github.com/rust-lang/regex
