# Task Runners

Often times, there can be some manual tasks related to maintenance of
projects. A lot of these tasks are manual or semi-manual, in that they
can be captured in some kind of a script or tool invocation.

A common pattern that can be found in open-source software is the
use of Makefiles to automate tasks. 

## Just

Just ([crate](https://crates.io/crates/just), [repository](https://github.com/casey/just))
is a simple task runner with a syntax similar to Makefiles, but simpler and
with some extensions to allow passing arguments to tasks and to use comments
for self-documenting tasks.

## Cargo Make

`cargo-make` is a simple Cargo subcommand which performs a similar
function as Makefiles do, however it uses a TOML-based configuration

- <https://crates.io/crates/cargo-make>

## Cargo XTask

- <https://github.com/matklad/cargo-xtask>
