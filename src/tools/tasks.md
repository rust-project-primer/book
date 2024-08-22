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

## [Just](https://github.com/casey/just)

Just is a simple task runner with a syntax similar to Makefiles, but simpler
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

`cargo-make` is a simple Cargo subcommand which performs a similar
function as Makefiles do, however it uses a TOML-based configuration

- <https://crates.io/crates/cargo-make>

## Cargo XTask

- <https://github.com/matklad/cargo-xtask>
