# Development Environment

Developers have varying preferences for their development environments. Some
prefer light-weight setups, using [vim][], [neovim][], or [helix][]. The
advantage of these setups is that they are lightweight, portable and support
any language, especially newer ones that don't have IDE support yet. Many
people that are proficient in Rust like to explore newer programming languages,
you will find many that use these kinds of setup.

However, many developers prefer development environments that are more tightly
integrated into the languages they use. This section focusses on some
environments that make it easy to get started with Rust. 

The [Are We IDE Yet][areweideyet] page tracks the progress of Rust integrations. 

## Rust Analyzer

Language servers are tools that parse understand programming languages, and
expose this data to IDEs. Unline compilers, which run once and produce a
binary, language servers are designed to run continuously, generated metadata
such as inferred types of values, and implement high-level operations such as
refactoring code.

The original language server for Rust was called [Rust Language
Server](https://github.com/rust-lang/rls), and it used [rustc to parse
projects](https://github.com/rust-lang/rls/blob/master/architecture.md). This
approached worked initially, but there were issues with latency. Additionally,
rustc is not great at handling incomplete or broken code, which is important
for language servers as they run while you write code. As a result, RLS was
[deprecated](https://blog.rust-lang.org/2022/07/01/RLS-deprecation.html) in
2022.

- graph of [rls architecture](https://github.com/rust-lang/rls/blob/master/architecture.md)

As a result, a new approach was taken that used a custom parser to be more
error-resiliant than `rustc`, called
[rust-analyzer](https://blog.rust-lang.org/2022/02/21/rust-analyzer-joins-rust-org.html).

- graph of rust-analyzer architecture

The core piece that makes Rust IDEs possible is thus [rust-analyzer][], which
is a project that understands Rust projects and implements the [Language Server
Protocol][lsp], which is a way for IDEs to understand them too and display type
annotations, warnings, errors, suggestions.

In general, any IDE that supports the LSP protocol can be used for Rust
development using [rust-analyzer][]. The only exception is Rust Rover, which
implements it's own parser for Rust projects.

In general, you don't need to know much about Rust Analyzer to use it. In fact,
many Rust IDEs even bundle it, and will manage and update it for you. You will not
even be aware that it is running in the background. But there are some situations where
you might need to be aware of its existence. If you use build systems other than Cargo to
build your Rust project, for example, then Rust Analyzer might not be able to analyze
your project. There might also be cases where it has bugs, because it uses a different
parser for Rust than `rustc` has.

[Rust Analyzer Manual](https://rust-analyzer.github.io/manual.html)

[LSP could have been better](https://matklad.github.io/2023/10/12/lsp-could-have-been-better.html)

[Improving "Extract Function" in Rust Analyzer](https://dorianlistens.com/2022/07/improving-extract-function-in-rust-analyzer/)


[areweideyet]: https://areweideyet.com/
[vim]: https://vim.org
[neovim]: https://neovim.org
[rust-analyzer]: https://github.com/rust-lang/rust-analyzer
[lsp]: https://microsoft.github.io/language-server-protocol/


[zed]: https://zed.dev/
[helix]: https://github.com/helix-editor/helix
