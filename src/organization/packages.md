# Packages

When you start your project, the very first thing you will likely do is create a
new package. A package is a unit in which Rust organizes code, it consists of
metadata (such as a `Cargo.toml`) and _crates_. You can think of it like a Ruby
gem, a Python package, or a Node module. Packages allow you to use the Cargo
build system to compile it, run tests and manage dependencies.

```admonish info
Sometimes, the terms *package* and *crate* are used interchangeably.
```

A crate is a compilation unit. Unlike C, C++ or Java, which compile individual
_files_, in Rust an entire crate is always compiled in one go. This means you
don't have to worry about the ordering of includes, and it means that all
definitions are always visible. It also makes it easier for the compiler to
implement certain optimizations, such as inlining code.

### Contents of a Package

At the very minimum, a Rust package contains metadata (in the `Cargo.toml` file)
and a single library or binary crate, otherwise there is nothing to compile.
Generally, you do not need to configure Cargo to tell it where the crates are:
it automatically detects them based on their standard locations. You can,
however, override this and place your source files in non-standard locations,
but this is not recommended. For example, if you have a `src/lib.rs` file in
your package, Cargo recognizes this as your library crate.

<center>

![Rust crate layout](rust-package.svg)

</center>

Every package needs to have either a library crate or a binary crate. It may
also contain other, supporting crates, such as integration tests, benchmarks,
examples. Having first-class support for these is a big bonus, because it means
you can run `cargo test` in any Rust project and Cargo will know where the tests
are and is able to run them.

| File path       | Autodetected crate type                   |
| --------------- | ----------------------------------------- |
| `src/lib.rs`    | Library crate                             |
| `src/main.rs`   | Binary crate, named after name of package |
| `src/bin/*.rs`  | Binary crate, named after filename        |
| `examples/*.rs` | Example                                   |
| `bench/*.rs`    | Benchmark                                 |
| `tests/*.rs`    | Integration test                          |
| `build.rs`      | Build script                              |

Generally, the library crate of every package is where you want to keep all of
your logic. This is because this code is what all the other crates link to by
default. So, if you write an integration test, it cannot "see" what is inside
your binary crates. In many projects, the binary crate at `src/main.rs` is just
a small shell that parses command-line arguments, sets up logging and calls into
the library crate to do the hard work.

### Metadata

Every crate contains some metadata, in the `Cargo.toml` file. This contains
everything cargo needs to know to build the crate, such as its name, and a list
of all dependencies it needs to build. It also contains metadata necessary for
publishing it on [crates.io][], Rust's crate registry, such as its version, list
of authors, license, and description. Finally, this file can also contain
metadata for other tooling, some of which we will discuss in this book. An
example file might look like this:

```toml
{{#include ../../examples/example-crate/Cargo.toml}}
```

Dependencies can have optional features. This ensures a faster compilation, by
only compiling them when they are explicitly enabled.

Cargo has built-in support for [semantic versioning][semver], so the versions
listed here are constraints. For example, when you specify version `1.0.12`, it
really means that your crate will work with any version `>=1.0.12` and `<1.1.0`,
because semver considers changes in the patch level (the third number) as
non-breaking changes.

This means that when you build your crate, Rust has to resolve the version
numbers. It stores those resolved version numbers in a separate file,
`Cargo.lock`. This is to ensure that you get reproducible builds: if two people
build the project, they always use exactly the same versions of dependencies.
You have to manually tell Cargo to go look if there are newer versions of
dependencies that are within the constraints, using `cargo update`. This and
some issues around it will be covered in later chapters.

Here is an example of what this looks like:

```toml
{{#include ../../examples/example-crate/Cargo.lock}}
```

### Library and Binaries

Besides these two files, crates also contain Rust source code in various places.
We will list the default locations for these here, but the locations can be
configured and overridden in the metadata.

Every crate can define (at most) one library. The entrypoint for this is in
`src/lib.rs`. When you use a crate as a dependency, this is what other crates
can see. Even if your project is primarily an executable and not a library, you
should try to put most of the code into this library section, because this is
what is visible to example code and integration tests. I call this
_library-first development_.

- articles for library-first development

Besides a single library, crates can also define binaries. These must contain a
`main()` function, and are compiled into executables. The default location for
binaries is `src/main.rs`, and it will produce an executable with the same name
as the crate. You can create additional ones under `src/bin/<name>.rs`, which
will create executables with the same name.

- graphic: executables linking against library

While Rust supports writing unit tests directly in the code, sometimes you want
to write tests from the perspective of an external user using your library
(without visibility into private functions). For this reason, you can write
integration tests, in `tests/<name>.rs`. These are compiled as if they were an
external crate which links to your crate, and as such only have access to the
public API.

- graphic: tests linking against library

Finally, Rust has a large focus on making it easy to write documentation. In
fact, support for generating documentation is a built-in feature. In some cases,
writing code is the best kind of documentation. For this reason, Cargo has
first-class support for keeping code examples. If you put examples into
`examples/<name>.rs`, they can be built and run by cargo using
`cargo build --examples` and `cargo run --example <name>`. There is even a
feature in Rust's built-in support for documentation, where it will pick up and
reference examples in the code documentation automatically.

- graphic: examples linking against library

See also:
[Package Layout](https://doc.rust-lang.org/cargo/guide/project-layout.html).

## Creating a crate

You can use `cargo new` to create an empty crate. You have the choice of
creating a library crate (using the `--lib` switch) or a binary crate. Using
`cargo` is recommended over creating a new crate manually, because it will
usually set useful defaults.

```
# create a binary-only crate
cargo new example-crate

# create a library crate
cargo new --lib example-crate
```

This is what an example crate layout looks like, after adding some dependencies.
You can see what the metadata and the source code looks like.

```files
path = "example-crate"
git_ignore = true
default_file = "Cargo.toml"
```

A more full-fledged example makes use of both the library and executables, has
some documentation strings, tests and examples in it, along with complete crate
metadata.

````admonish info
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
````

## Crate Features

Rust crates can declare optional dependencies. These are additive, meaning that
enabling them should not break anything. The reason for this is that Rust
performs feature unification: if you have multiple dependencies in your
dependency that depend on a single crate, it will only be built once with the
features unified.

- dependency tree: feature unification

This is a good way to add additional, optional features to your crates while
keeping compilation times short for those who don't use them. If you have a
dependency, you can enable them by setting the `features` key:

```toml
[dependencies]
serde = { version = "1.0.182", features = ["derive"] }
```

For your own crates, you can declare optional features using the `features`
section in the metadata. Using features, you can enable optional dependencies,
and inside your code you can disable parts (functions, structs, modules)
depending on them.

```toml
[features]
default = []
cool-feature = ["serde"]
```

Once you have declared a feature like this, you can use it to conditionally
include code in your project, using the cfg attribute.

```rust
#[cfg(feature = "cool-feature")]
fn only_visible_when_cool_feature_enabled() {
    // ...
}
```

Doing this can have some advantages, for example it lets you keep compilation
times short for developers because they can build a subset of the project for
testing purposes. However, it also requires some care, because you often need to
be careful to make sure features don't conflict with each other, see
[Chapter 6.10: Crate Features](checks/features.md).

## Crate Size

As mentioned earlier, in Rust a crate is a compilation unit. When you make a
change in one file, the entire crate needs to be rebuilt. While it makes sense
initially to start a project out with one crate, as the project grows it may
make sense to split it up into multiple, smaller crates. This allows for faster
development cycles.

The next section discusses how this can be done, and what mechanisms Rust
supports for doing so.

## Reading

```reading
style: book
title: "Chapter 3.2.1: Cargo Targets"
url: https://doc.rust-lang.org/cargo/reference/cargo-targets.html
author: The Cargo Book
---
In this section of the Cargo book, all of the possible targets that Cargo can build
for a crate are defined.
```

```reading
style: book
title: "Chapter 3.1: Specifying Dependencies"
url: https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html
author: The Cargo Book
---
In this section of the book, it is explained how dependencies are specified in Cargo.
```

```reading
style: article
title: Default to Large Modules
url: https://two-wrongs.com/decompose-into-larger-modules
author: Chris
---
In this article, Chris argues that it is best to default to large modules,
because the cost of designing useful abstractions for the interaction is high,
and it is possible to split larger modules into smaller ones later when the
code is more stable.
```

```reading
style: book
title: Features
url: https://doc.rust-lang.org/cargo/reference/features.html
author: Cargo Project
---
In this chapter of the cargo book, features are discussed. Specifically, it explains
how Cargo resolves crate features, and performs feature unification.
```

[semver]: https://semver.org/
[crates.io]: https://crates.io/
