# Code Search

Sometimes, especially when navigating large code bases, it can be useful to
quickly search over the entire code base to find some patterns. This could be finding
any places where a specific crate is used, or finding some code patterns.

[ripgrep](https://github.com/BurntSushi/ripgrep) is a command-line tool for searching
code bases using [regular expressions][regex]. It is very fast, making use of Rust's powerful
crate. It is also smart, as it understands git repositories and will respect `.gitignore`
files. Visual Studio Code's search functionality [uses it behind the scenes](https://github.com/microsoft/vscode-ripgrep).

You can install it with Cargo:

    cargo install ripgrep

Doing so will install the `rg` binary, which you can use to search code projects. You
can then use it to search for patterns.

```
$ rg uuid::
database/src/main.rs
8:use uuid::Uuid;

protocol/src/types.rs
10:use uuid::Uuid;

common/src/entities.rs
12:use uuid::Uuid;
```
