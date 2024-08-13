# Development Environment

Developers have varying preferences for their development environments. Some
prefer light-weight setups, using [vim][], [neovim][], or [helix][]. The
advantage of these setups is that they are lightweight, portable and support
any language, especially newer ones that don't have IDE support yet. Many
people that are proficient in Rust like to explore newer programming languages,
you will find many that use these kinds of setup.

However, many developers prefer development environments that are more tightly
integrated into the languages they use. This section focusses on some
environments that make it easy to get started with Rust. The core piece that
makes this possible is [rust-analyzer][], which is a project that understands
Rust projects and implements the [Language Server Protocol][lsp], which is a
way for IDEs to understand them too and display type annotations, warnings,
errors, suggestions.

In general, any IDE that supports the LSP protocol can be used for Rust
development using [rust-analyzer][]. The only exception is Rust Rover, which
implements it's own parser for Rust projects.

The [Are We IDE Yet][areweideyet] page tracks the progress of Rust integrations. 

[areweideyet]: https://areweideyet.com/
[vim]: https://vim.org
[neovim]: https://neovim.org
[rust-analyzer]: https://github.com/rust-lang/rust-analyzer
[lsp]: https://microsoft.github.io/language-server-protocol/


[zed]: https://zed.dev/
[helix]: https://github.com/helix-editor/helix
