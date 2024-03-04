# Crates

In Rust, code is typically organized into *crates*. These are similar to *gems*
in the Ruby world, *packages* in Python. Within crates, code is structured
in modules, which are similar to C++ *namespaces*.

Crates consist of some Rust source code, a `Cargo.toml` file which holds
metadata, and a `Cargo.lock` file which is a lockfile for the dependencies.

Unless you want to fetch dependencies manually and build using raw `rustc`
invocations, your project should also be organized into one or more crates.

## Contents

Crates can consist of the following along with default locations for each:

- A single **library**, located in `src/lib.rs`.
- Zero or more **binaries**, located in `src/main.rs` or under `src/bin/`.
- Zero or more **examples**, located in `examples/`
- Zero or more (integration) **test binaries**, located under `tests/`.
- A list of **dependencies**, some of which may be optional.
- A list of **optional feautures**, which may enable optional dependencies.
- Crate metadata, such as crate name, version, description, homepage.
- Metadata for external tooling.

Every crate is compiled as **one unit**. This is different from languages such
as C++ or C, where every *file* is built as a unit. This lets you have circular
dependencies within your Rust crates.

## Creating a crate

You can use `cargo new` to create an empty crate. You have the choice of creating a library
crate (using the `--lib` switch) or a binary crate. Using `cargo` is recommended over
creating a new crate manually, because it will usually set useful defaults.

~~~admonish example title="Binary crate"
```
cargo new my-binary-crate
```
~~~

~~~admonish example title="Library crate"
```
cargo new --lib my-library-crate
```
~~~

~~~admonish info
Cargo has some neat features besides being able to create new crates for you.
It can also manage dependencies for you. For example, if you are inside a crate
and you would like to add `serde` to the list of dependencies, you can use
`cargo add` to add it:

```
cargo add serde --features derive
```

This will edit your `Cargo.toml` to add the dependency, without touching
anything else.  Comments and formatting is preserved. The Cargo team is quite
good at looking how people use it and extending it with functionality that is
commonly requested.
~~~
