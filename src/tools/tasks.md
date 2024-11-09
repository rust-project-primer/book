# Task Runners

Often times, it can be useful to automate tasks in code projects. This could
involve:

- Automating processes, such as creating releases of the project,
- Launching services required for running or testing the software, such as a database
- Generating documentation
- Maintenance commands, such as checking for unused dependencies

These tasks can often be captured as command-line scripts of tool invocations.
To make developer's lives easier, it can be useful to use some tool to make it
easy to run these tasks. This is what task runners do, they allow you to define
a list of preset tasks along with the commands that need to be run, and give
developers an easy way to run them.

Some IDEs are even able to parse these definitions and offer some graphical
interface for invoking them.

A common pattern that can be found in open-source software is the [use of
Makefiles to automate tasks](https://ivan.sh/make/). However, Makefiles are
often not ideal, requiring some workarounds such as making the tasks as
`.PHONY` to work.

High-level build system such as Bazel typically have some built-in support for
creating custom tasks that can be run, and don't benefit as much from task
runners described in this section.

## Just

[Just](https://github.com/casey/just)
is a simple task runner with a syntax similar to Makefiles, but simpler
and with some extensions to allow passing arguments to tasks and to use
comments for self-documenting tasks.

To get started, you can install it using Cargo:

    cargo install just

To use it, all you need to do is create a `Justfile` in your project, which
contains all of the tasks. A sample justfile might look like this:

```just
# release this version
release:
  just test
  cargo publish

# run unit and integration tests, starts database before tests
test:
  docker start database
  cargo test
  docker stop database
```

With this definition, you can run the tasks like this:

    just release
    just test

You can also list all available tasks:

    just --list

Just has support for tasks taking arguments, integrations with various IDEs,
some built-in functions, support for variables and much more. The [Just
Programmer's Manual](https://just.systems/man/en/) describes all of the
features it has to offer.

## Cargo Make

[cargo-make](https://github.com/sagiegurari/cargo-make)
is a Rust task runner and build tool. It lets you define tasks in
a `Makefile.toml`. It supports task dependencies and has some built-in features
that are useful in Rust projects, such as the ability to install crates.

You can install it using Cargo:

    cargo install cargo-make

Once installed, you can create a `Makefile.toml` in your repository to define
the tasks you want it to do.

```toml
# generate coverage, will install cargo-llvm-cov if it doesn't exist
[tasks.coverage]
install_crate = "cargo-llvm-cov"
command = "cargo"
args = ["llvm-cov", "--html"]
```

With this definition, running the `coverage` task will ensure that
`cargo-llvm-cov` is installed, and run it to produce a HTML coverage report.

    cargo make coverage

Tasks can also have dependencies on other tasks, and these dependencies can be
set conditionally, such as per-platform, allowing you to write
platform-specific or environment-specific implementations for tasks.

## Cargo XTask

[Cargo XTask](https://github.com/matklad/cargo-xtask)
is less of a tool and more a pattern for defining bespoke tasks
for Rust projects. The advantage of it is that you write the tasks themselves
in Rust, and `cargo-xtask` is only used to run them.

## Reading

[Just use just](https://toniogela.dev/just/) by Tonio Gela

*Tonio explains what Just is, and how you can use it. He demonstrates the
features it has with some examples.*

[Automating your Rust workflows with
cargo-make](https://medium.com/@sagiegurari/automating-your-rust-workflows-with-cargo-make-part-1-of-5-introduction-and-basics-b19ced7e7057)
by Sagie Gur-Ari

*Sagie, the author of cargo-make, explains how you can use it to automate your
Rust workflows and gives some examples.*

[Make your own
make](https://matklad.github.io/2018/01/03/make-your-own-make.html) by Alex
Kladov

*Alex explains the idea of using Rust itself for the automation of steps in
this article. This idea is what `cargo-xtask` implements.*
