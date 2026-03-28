# Watch Files

A short feedback loop between writing code and seeing whether it compiles or
passes tests is important for productive development. Your
[development environment](../development-environment/index.md) can give you
immediate feedback on syntax errors, but for running tests or rebuilding an
application on every change, a file watcher is useful.

If you build [web frontends](../ecosystem/web-frontend.md) in Rust using
[Trunk](https://trunkrs.dev), file watching is built in: Trunk's serve mode
rebuilds and reloads your browser automatically on every change.

## `cargo-watch`

[`cargo-watch`](https://github.com/watchexec/cargo-watch) is a tool you can use
to watch your Rust projects and execute commands whenever a file changes.

You can install it using Cargo:

    cargo install cargo-watch

By default, it will run `cargo check` when a change is detected:

    # run `cargo check` whenever files change
    cargo watch

You can customize it to run any command you like. Using the `-x` flag, you can
tell it to run any other Cargo subcommand. You can also directly give it a
command to run.

    cargo watch -x test
    cargo watch -- just test

It also supports command chaining, where you specify multiple Cargo subcommands
to run. When doing so, it will run each of them in the order you specify them,
when they are successful.

    cargo watch -x check -x test -x run

The repository and help text explain more commands that you can use, such as
specifying which files to watch.

## Reading

```reading
style: article
title: "Chapter 1: Setup - Toolchains, IDEs, CI"
url: https://www.lpalmieri.com/posts/2020-06-06-zero-to-production-1-setup-toolchain-ides-ci/#5-2-cargo-watch
author: Luca Palmieri
---
In this chapter of his book, Luca explains how to setup a real-life Rust
project. He explains that using `cargo watch` can reduce the perceived
compilation time, because it triggers immediately after you change and
save a file.
```

```reading
style: article
title: "Cargo Issue #9339: Add Cargo watch"
url: https://github.com/rust-lang/cargo/issues/9339
author: Patrick Hintermayer
---
In this issue on the Cargo repository, there is some discussion going on to
incorporate file watching functionality natively into Cargo.
```
