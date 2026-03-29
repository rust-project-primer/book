# Task Runners

Every project accumulates commands that developers need to run repeatedly:
building releases, starting databases for tests, generating documentation,
checking for unused dependencies. A task runner gives these commands names and
makes them discoverable, so developers don't have to remember (or look up) the
exact invocations.

Many open-source projects use [Makefiles](https://ivan.sh/make/) for this, but
Makefiles were designed for build dependency tracking, not running tasks. They
require workarounds like `.PHONY` targets and have surprising behavior around
quoting and shell compatibility. The tools in this section are purpose-built for
task running.

Some IDEs can parse task runner definitions and offer a graphical interface for
invoking them. Build systems like Bazel and Buck2 have their own task
infrastructure and don't benefit as much from these tools.

## Just

[Just](https://github.com/casey/just) is a simple task runner with a syntax
similar to Makefiles, but simpler and with some extensions to allow passing
arguments to tasks and to use comments for self-documenting tasks.

To get started, you can install it using Cargo:

    cargo install just

To use it, all you need to do is create a `Justfile` in your project, which
contains all of the tasks. A sample justfile might look like this:

```just framed title="Justfile"
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

```bash
just release
just test
```

You can also list all available tasks:

```bash
$ just --list
Available recipes:
    release # release this version
    test    # run unit and integration tests, starts database before tests
```

A common pattern is setting up `just` so that it shows the available commands
when run with no arguments. You can do that like this:

```
# List available recipes
default:
    @just --list
```

Just has support for tasks taking arguments, integrations with various IDEs,
some built-in functions, support for variables and much more. The
[Just Programmer's Manual](https://just.systems/man/en/) describes all of the
features it has to offer.

## `cargo-make`

[`cargo-make`](https://github.com/sagiegurari/cargo-make) is a Rust task runner
and build tool. It lets you define tasks in a `Makefile.toml`. It supports task
dependencies and has some built-in features that are useful in Rust projects,
such as the ability to install crates.

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
set conditionally, such as per-platform, allowing you to write platform-specific
or environment-specific implementations for tasks.

## `cargo-xtask`

[`cargo-xtask`](https://github.com/matklad/cargo-xtask) is less of a tool and
more of a pattern. You add a `xtask` crate to your workspace that contains your
automation scripts written in Rust, and a Cargo alias that runs it:

```toml
# .cargo/config.toml
[alias]
xtask = "run --package xtask --"
```

The advantage is that your task definitions are type-checked Rust code with
access to all the crates in your ecosystem (file manipulation, HTTP requests,
argument parsing). The disadvantage is more boilerplate than a `Justfile` for
simple tasks. The `cargo-xtask` pattern works best for projects that already
have complex build logic or where tasks need to interact with Rust APIs
directly.

## Reading

```reading
style: article
title: Just use just
url: https://toniogela.dev/just/
author: Tonio Gela
archived: tonio-gela-just-use-just.pdf
---
Tonio explains what Just is, and how you can use it. He demonstrates the
features it has with some examples.
```

```reading
style: article
title: Automating your Rust workflows with cargo-make
url: https://medium.com/@sagiegurari/automating-your-rust-workflows-with-cargo-make-part-1-of-5-introduction-and-basics-b19ced7e7057
author: Sagie Gur-Ari
archived: sagie-gur-ari-automating-your-rust-workflows.pdf
---
Sagie, the author of cargo-make, explains how you can use it to automate your
Rust workflows and gives some examples.
```

```reading
style: article
title: Make your own make
url: https://matklad.github.io/2018/01/03/make-your-own-make.html
author: Alex Kladov
archived: matklad-make-your-own-make.pdf
---
Alex explains the idea of using Rust itself for the automation of steps in
this article. This idea is what `cargo-xtask` implements.
```
