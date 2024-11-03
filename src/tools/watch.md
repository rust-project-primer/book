# Watch Files

While you are developing, having a quick feedback loop can be invaluable. What
this means is that the time between you writing some code, and getting feedback
if it is syntactically incorrect or if it breaks unit tests should be as short
as possible.

Often times, the [development environment](../development-environment/index.md)
you use can give you fast feedback on syntaxtical issues. Some even let you
define shortcuts for quickly running unit tests or other actions.

Another approach is to use a tool that watches your code for changes, and runs
some command whenever you make a change. There are some situations where this
is useful:

- You want to run some custom tests on the code
- You want to rebuild and relaunch your application, so that you can test it
  interactively.

If you build [Web Frontends](../ecosystem/web-frontend.md) in Rust and use the
[Trunk](https://trunkrs.dev) tool to build them, you will get this for free.
When you run Trunk in *serve* mode, it will automatically rebuild your frontend
and reload your browser whenever it detects a change, to minimze your
development feedback loop.

## [Cargo Watch](https://github.com/watchexec/cargo-watch)

Cargo Watch is a generic tool you can use to watch your Rust projects and execute
commands whenever a file changes. 

You can install it using Cargo:

    cargo install cargo-watch

By default, it will run `cargo check` when a change is detected:

    # run `cargo check` whenever files change
    cargo watch

You can customize it to run any command you like. Using the `-x` flag, you can tell
it to run any other Cargo subcommand. You can also directly give it a command to run.

    cargo watch -x test
    cargo watch -- just test

It also supports command chaining, where you specify multiple Cargo subcommands
to run.  When doing so, it will run each of them in the order you specify them,
when they are successful.

    cargo watch -x check -x test -x run

The repository and help text explain more commands that you can use, such as
specifying which files to watch.

## Reading

[Chapter 1: Setup - Toolchains, IDEs, CI](https://www.lpalmieri.com/posts/2020-06-06-zero-to-production-1-setup-toolchain-ides-ci/#5-2-cargo-watch) by Luca Palmieri

*In this chapter of his book, Luca explains how to setup a real-life Rust
project. He explains that using `cargo watch` can reduce the perceived
compilation time, because it triggers immediately after you change and
save a file.*

[Cargo Issue #9339: Add Cargo watch](https://github.com/rust-lang/cargo/issues/9339)

*In this issue on the Cargo repository, there is some discussion going on to
incorporate file watching functionality natively into Cargo.*
