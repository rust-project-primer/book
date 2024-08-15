# Cargo

Cargo is the default build system for Rust projects. It makes it easy to create
build and test Rust code, manages dependencies from [crates.io][], and allows
you to publish your own crates there. It uses semantic versioning to resolve
dependency version from constraints you define and uses a lockfile to ensure
you are always building with the same dependency versions. Since `rustc` is LLVM-based,
it is also easy to cross-compile your Rust code for other targets, see the
[list of supported Rust targets][rust-targets].

Cargo supports installing other tools that integrate into it and extend it
with new subcommands. This guide mentions several of such tools, for example:

- `cargo hack`, for testing feature permutations,
- `cargo llvm-cov`, for determining test coverage,

One nice property of having Cargo as the default build system for all Rust
projects is that you can typically clone any repository that contains a Rust
crate and run `cargo build` to build it, or `cargo test` to run tests.
This is quite different to languages such as C, C++ or JavaScript that have
a more fragmented build ecosystem.

## Complex build steps

Cargo is great at building Rust code, but has few features for building
projects that involve other languages. This makes sense, because such functionality
is not needed by it. 

Cargo does come with some support for running arbitrary steps at build time,
through the use of *build scripts*. These are little Rust programs that you can
write that are executed at build time and let you do anything you like, including
building other code. It also supports linking with C/C++ libraries by having
these build scripts emit some data that Cargo parses.

### Using build scripts with Cargo

Issues typically come up whenever you need to interface with other programming
languages. While there is support for doing that, it can sometimes be tricky to
get it right and have it work across multiple platforms and versions.

If you have a few more complex steps that you need to do when building your
code, you can always use a build script.

The other sections of this chapter are only relevant to you if your project
consists of a mixture of languages, and building it is sufficiently complex
that it cannot trivially be expressed or implemented in a `build.rs` file (such
as: it needs external dependencies).

```admonish
Build scripts in Cargo are little Rust programs defined in a `build.rs` in the crate
root which are compiled and run before your crate is compiled. They are able to do
some build steps (such as compile an external, vendored C library) and they can
emit some information to Cargo, for example to tell it to link against a specific
library.
```

There are some common patterns that people do in build scripts:

- Use the `cc` or `cmake` crates to compile C/C++ code and then link it with your program.

~~~admonish example title="Using a build script with Cargo"
TODO
~~~

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

[crates.io]: https://crates.io
[rust-targets]: http://example.com
