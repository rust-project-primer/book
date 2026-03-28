# Tools

The preceding chapters cover tools tied to specific workflows: formatting and
linting in [Checks](../checks/readme.md), test runners in
[Testing](../testing/readme.md), profiling in [Measure](../measure/readme.md),
and so on. This chapter collects general-purpose development tools that are
useful across workflows but don't belong to any single one.

[Code Search](search.md) covers `ripgrep` and `ast-grep` for navigating large
codebases. [Task Runners](tasks.md) compares `just`, `cargo-make`, and the
`xtask` pattern for automating project-specific commands.
[Readme Generation](repository.md) covers tools that keep your `README.md` in
sync with your crate documentation. [Watch Files](watch.md) covers `cargo-watch`
and `bacon` for re-running commands on file changes. [Expand Macros](macros.md)
covers `cargo-expand` for inspecting what procedural and declarative macros
generate. [Debugging](debugger.md) covers debugger integration with `rust-gdb`
and `rust-lldb`.

## Reading

```reading
style: article
title: "Rust Tooling: 8 tools that will increase your productivity"
url: https://www.shuttle.rs/blog/2024/02/15/best-rust-tooling
author: Joshua Mo
archived: shuttle-best-rust-tooling.pdf
---
Joshua showcases and explains some tools for Rust developers that can increase
your productivity, and gives examples for how they can be used.
```

```reading
style: article
title: Awesome Rust Tools
url: https://github.com/unpluggedcoder/awesome-rust-tools
author: "@unpluggedcoder"
---
This is a list of awesome tools written in Rust. It showcases tools in various
categories, from general-purpose command-line tools to tools specifically for
Rust development, maintenance or navigation.
```

```reading
style: article
title: Cargo plugins
url: https://lib.rs/development-tools/cargo-plugins
author: lib.rs
---
This is a list of useful plugins for Cargo, sorted by their popularity (as
measured by the download count from the Rust crates registry).
```
