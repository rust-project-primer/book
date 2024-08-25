# Code Search

When navigating large or unfamiliar code bases, it can often be useful to
search over the entire code base to find some patterns. This could be finding
any places where a specific crate is used, or finding some code patterns.

## Ripgrep

[ripgrep](https://github.com/BurntSushi/ripgrep) is a command-line tool for
searching code bases using [regular expressions][regex]. It is very fast,
making use of Rust's powerful [regex][rust-regex] crate. 

It understands git repositories and will respect `.gitignore` files, making it
particularly suitable for search software projects. Visual Studio Code's search
functionality [uses it behind the
scenes](https://github.com/microsoft/vscode-ripgrep).

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

## Reading

[ripgrep is faster than {grep, ag, git grep, ucg, pt, sift}](https://blog.burntsushi.net/ripgrep/) by Andrew Gallant

*Andrew, the author of ripgrep, introduces the tool in this article, explains how it
works and compares it to some common similar tools used by developers, showing how
it performs better and how it excels at dealing with Unicode, something other tools
struggle with.*

[regex]: https://en.wikipedia.org/wiki/Regular_expression
[rust-regex]: https://github.com/rust-lang/regex
