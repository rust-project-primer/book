# Cargo

Cargo is the default build system for Rust projects. It makes it easy to create
build and test Rust code, manages dependencies from [crates.io][], and allows
you to publish your own crates there. It uses semantic versioning to resolve
dependency version from constraints you define and uses a lockfile to ensure
you are always building with the same dependency versions. Since `rustc` is LLVM-based,
it is also easy to cross-compile your Rust code for other targets, see the
[list of supported Rust targets][rust-targets].

Cargo supports installing other tools that integrate into it and extend it
with new subcommands. This guide mentions several of such tools, such as `cargo-hack`
or `cargo-llvm-cov`.

One nice property of having Cargo as the default build system for all Rust
projects is that you can typically clone any repository that contains a Rust
crate and run `cargo build` to build it, or `cargo test` to run tests.
This is quite different to languages such as C, C++ or JavaScript that have
a more fragmented build ecosystem.

## What Cargo Lacks

If you only use built-in commands and only build Rust code, then Cargo is a
great build system for Rust projects. However, there are some features it does
not have.

If you rely on plugins to build your project, such as `trunk` for building
WebAssembly-powered web frontend applications powered by Rust, Cargo will not
install it automatically.  Rather, developers need to install it manually by
running `cargo install trunk`.

If you rely on native dependencies, such as OpenSSL or other libraries, Cargo
will not handle installing them on your behalf. There are some workarounds for
this, for example some crates like `rusqlite` ship the C code and have a
feature flag where Cargo will build the required library from source if you
request it.

If you need to execute build steps, such as compiling C code or your have some
parts of your project that use for example JavaScript, there is only
rudimentary support for doing so with Cargo.

In short, Cargo is great at all things Rust, but it does not help you much if
you mix other languages into your project. And that is by design: Cargo's goal
is not to reinvent the world. It does one thing, and it does it well, which is
build Rust code.

The next sections discuss some approaches that you can use to use Cargo in
situations that it is not designed for, but that yet seem to work.

## Complex build steps

Cargo is great at building Rust code, but has few features for building
projects that involve other languages. This makes sense, because such functionality
is not needed by it. 

Cargo does come with some support for running arbitrary steps at build time,
through the use of *build scripts*. These are little Rust programs that you can
write that are executed at build time and let you do anything you like, including
building other code. It also supports linking with C/C++ libraries by having
these build scripts emit some data that Cargo parses.

The other sections of this chapter are only relevant to you if your project
consists of a mixture of languages, and building it is sufficiently complex
that it cannot trivially be expressed or implemented in a `build.rs` file (such
as: it needs external dependencies).

### `build.rs` to define custom build actions



If you have a few more complex steps that you need to do when building your
code, you can always use a build script.

Build scripts in Cargo are little Rust programs defined in a `build.rs` in the crate
root which are compiled and run before your crate is compiled. They are able to do
some build steps (such as compile an external, vendored C library) and they can
emit some information to Cargo, for example to tell it to link against a specific
library.

Build scripts receive a [number of environment variables][build-script-input]
as inputs, and output [some metadata][build-script-output] that controls
Cargo's behaviour. 


A simple build script might look like this:

```rust
fn main() {
}
```

For common tasks such as building C code, generating bindings for native
libraries there are crates that allow you to write build scripts easily, these
are presented in the next sections.

### Compiling C/C++ Code

If you have some C or C++ code that you want built with your crate, you can use
the [`cc`][cc] crate to do so. It is a helper library that you can call inside
your build script to run the native C/C++ compiler to compile some code, link
it into a static archive and tell Cargo to link it when building your crate. It
also has support for compiling CUDA code.

A basic use of this crate looks by adding something like this to the `main`
function of your build script:

```rust
cc::Build::new()
    .file("foo.c")
    .file("bar.c")
    .compile("foo");
```

The crate will take care of the rest of finding a suitable compiler and
communicating to Cargo that you wish to link the `foo` library.

Here is an example of how this looks like. In this crate, a build script is
used to compile and link some C code, and the unsafe C API is wrapped and
exposed as a native Rust function.

```files
path = "levenshtein"
git_ignore = true
files = ["!.git"]
default_file = "src/lib.rs"
```

Note that in order to make the C function "visible" from Rust, you need to
declare it in an `extern "C"` block. It needs a function definition that
matches the one in the C header. Writing this by hand is error-prone, and can
lead to unsafety issues.

This example also shows how this unsafe C function is wrapped into a safe Rust
function. Doing so involves dealing with raw pointers, and it is easy to get
something wrong. It is important to write good [Unit
Tests](../testing/unit-tests.md), and often it can help to use [Dynamic
Analysis](../testing/dynamic-analysis.md) to make sure you did it correctly.

[cc]: https://docs.rs/cc/latest/cc/

### Compiling CMake projects

- use the `cmake` crate

### Generating Bindings for C/C++ Libraries

- using rust-bindgen

[build-script-input]: https://doc.rust-lang.org/cargo/reference/environment-variables.html#environment-variables-cargo-sets-for-build-scripts
[build-script-output]: https://doc.rust-lang.org/cargo/reference/build-scripts.html#outputs-of-the-build-script

## Caching builds

You may find that Rust takes a long time to compile, which is certainly the
case.  You can partially mitigate this by using a build cache, which is a
service that will cache the compiled artifacts and allow you to compile
considerably faster.  One tool that lets you do this is `sccache`, which is
discussed in [a future chapter](../building/cache.md).

## Toolchain Pinning

If you depend on specific Cargo or Rust features, you may find that you can
run into issues if people with older toolchain versions try to build your code.
For this reason, it is sometimes useful to pin a specific version of the Rust
toolchain in a project, to make sure everyone is using the same versions.

There are two mechanisms that you can use here, depending on where you
want this pinning to work:

- You can use a `rust-toolchain.toml` file to pin the Rust version for the
  current project. This file is picked up by `rustup`, which most people use
  to manage and update their Rust toolchain. When running any Cargo command
  in a project that has such a file, `rustup` will ensure that the specified
  toolchain version is installed on the system and will only use that.
- Conversely, if you are building a library and you want *users* of your
  library (as in, people that depend on your library *as a dependency*, but
  do not directly work on it) to use a specified minimum Rust toolchain
  version, you can set the MSRV in the Cargo metadata. This means that users
  of your library that are on older Rust versions will get an error or a warning
  when they try to add your library as a dependency.

### Pinning the toolchain version for projects

The way you can solve this is by putting a `rust-toolchain.toml` file into the
repository. This will instruct `rustup` to fetch the exact toolchain mentioned
in this file whenever you run any operations in the project.

Typically, such a file simply looks like this:

```toml
[toolchain]
channel = "1.75"
components = ["rustfmt", "clippy"]
```

Keep in mind that this file is only picked up by people who use `rustup` to manage
their Rust toolchains.

~~~admonish note
Putting `rust-toolchain.toml` file in your project lets you specify exactly which
version of the Rust compiler is used by the people working on the project.
~~~

~~~admonish example title="Specifying Rust toolchain version using a `rust-toolchain.toml` file"
TODO
~~~

### Specifying the minimum toolchain version for library crates

However, this `rust-toolchain.toml` file is only consulted when you are building
the current project. What if your crate is used as a dependency by other crates?
How can you communicate that it needs a certain version of the Rust compiler?

For this, Cargo has the option of specifying a MSRV for each crate. This is the
minimum version of the Rust compiler that the crate will build with.

In a later chapter, we will show you how you can determine the MSRV
programmatically and how you can test it to make sure that the version you put
there actually works.

~~~admonish info title="Specifying the MSRV for library crates"
If you build library crates, you should specify the minimum version of the Rust
toolchain that is needed to build your library. This helps other crate authors
by telling them which version of Rust they need to use your library. You
should always specify this.
~~~

~~~admonish example title="Specifying the MSRV for library crates"
TODO
~~~

## Convenience Commands

Cargo has a useful selection of convenience commands built-in to it that make
using it to manage Rust projects easy.

https://blog.logrocket.com/demystifying-cargo-in-rust/

## Initializing Cargo project

To quickly create a Cargo project, you can use `cargo new`. By default, it will
create a binary crate, but you can use the `--lib` flag to create a library crate
instead.

```
cargo new my-crate
```

### Building and running Code and Examples

The main thing you likely use Cargo for is to build and run Rust code.
Cargo has two commands for this, `cargo build` and `cargo run`.

    cargo build
    cargo run

If you have multiple binaries and you want to build or run a specific one,
you can specify it using the `--bin` flag.

    cargo build --bin my_binary
    cargo run --bin my_binary

If you instead want to build or run an example, you can specify that using
the `--example` flag.

    cargo build --example my_example
    cargo run --example my_example

### Running Tests and Benchmarks

Besides building and running Rust code, you will likely also use Cargo to
run unit tests and benchmarks. It has built-in commands for this, too.

    cargo test
    cargo bench

As explained in the [Unit testing](../testing/unit.md) section, you can
also use the external tool `cargo-nextest` to run tests faster.

### Managing Dependencies

Cargo comes with built-in commands for managing dependencies. Originally, these commands
were part of [cargo-edit][], but due to their popularity the Cargo team has decided to adopt
them as first-class citizens and integrate them into Cargo.

```
cargo add serde
cargo remove serde
```

Recently, they also added support for Workspace dependencies. If you use `cargo-add` to add
a dependency to a crate, which already exists in the root workspace as a dependency, it will do the
right thing and add it as a workspace dependency to your Cargo manifst.

You can also use Cargo to query the dependency tree. This lets you see a list of all dependencies,
and their child dependencies. It lets you find out if you have duplicate dependencies (with different
versions), and when that is the case, why they get pulled in. For example, if you have one dependency
that uses `uuid v1.0.0`, but you depend on `uuid v0.7.0`, then you will end up with two versions of
the `uuid` crate that are being pulled in.

```
cargo tree
```

This command used to be a separate plugin called [cargo-tree][], but was
incorporated into Cargo by the team due to it being useful.

[cargo-edit]: https://github.com/killercup/cargo-edit
[cargo-tree]: https://github.com/sfackler/cargo-tree


### Building Documentation

```
cargo doc
```

### Installing Rust Tools

Besides just being a build system for Rust, Cargo also acts as a kind of package
manager. Any binary Rust crates that are published on a registry can be compiled
and installed using it. This is often used to install Cargo plugins or other supporting
tools.

    cargo install ripgrep


- mention cargo-binstall

## Profiling Builds

If you want to figure out what Cargo is spending most of the time on during
builds, you can use the built-in profiling support. This generates a HTML report
of the timings of the build and allows you to debug slow builds. However, it only
works using nightly Rust.

    cargo +nightly build --timings=html

## Conclusion

If your project can get away with only using it to define and run all of the
steps needed to build your project, then you should prefer it over using a
third-party build system. Everyone who writes Rust code uses Cargo, it is very
simple to use and comes with features that cover the majority of the use-cases
you might run into.

If you do have a multi-language project, or a project with complicated build
steps, you might soon find that build scripts are rather limited. Dependency
tracking is possible with them, but it feels hacky. They are not hermetic, and
there is no built-in caching that you can use.  In this case, you may find it
useful to take a look at the other popular build systems and determine if they
might help you achieve what you want in a way that is more robust or more
maintainable.

Do keep in mind that usually, using third-party build systems can be more pain
than using Cargo itself, because they need to reimplement some functionality
that you get for free when using Cargo. However, sometimes there are advantages
that they bring that outweigh the additional complexity.

## Reading

[The Cargo Book](https://doc.rust-lang.org/cargo)

*Reference guide for Cargo. This book discusses all features that Cargo has and
how they can be used.*

[The Cargo Book: Build
Scripts](https://doc.rust-lang.org/cargo/reference/build-scripts.html)

*Section in the Cargo Book that talks about using build scripts. It shows some
examples for how they can be used and explains what can be achieved with them.*

[The Missing Parts in Cargo](https://weihanglo.tw/posts/2024/the-missing-parts-in-cargo/)

*TODO*

[Foreign Function Interface](https://doc.rust-lang.org/nomicon/ffi.html) in *The Rustonomicon*

*This chapter in The Rustonomicon explains how to interact with foreign
functions, that is code written in C or C++, in Rust.*

[crates.io]: https://crates.io
[rust-targets]: http://example.com
