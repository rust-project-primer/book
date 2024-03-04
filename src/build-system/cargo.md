# Cargo

Cargo is the default build system for Rust projects. It comes with anything you
might need for a Rust project, has a very simple interface. 

## Toolchain Version

One issue that can arise is that different people might not have the same version
of Cargo and the Rust toolchain. This means that they might run into issues with 
certain operations, generally if their toolchain is behind that of the project.

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

## Complex build steps

Sometimes, your project might have more complicated build steps. For example,
you must link against a C/C++ library, or you must build some C functions.
Cargo has some support for that through the use of *build scripts*.

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

You may find that Rust takes a long time to compile, which is certainly the case.
You can partially mitigate this by using a build cache, which is a service
that will cache the compiled artifacts and allow you to compile considerably faster.
One tool that lets you do this is `sccache`, which is discussed in 
[a future chapter](../building/cache.md).

## Conclusion

Cargo is very simply and yet very powerful. If your project can get away with only
using it to define and run all of the steps needed to build your project, then you
should prefer it over using a third-party build system.

However, there are cases where you might feel like you need to push Cargo very
far to achieve what you are looking for. In this case, you may find it useful
to take a look at the other popular build systems and determine if they might help
you achieve what you want in a way that is more robust or more maintainable.

Do keep in mind that usually, using third-party build systems can be more pain
than using Cargo itself, because they need to reimplement some functionality that you
get for free when using Cargo. However, sometimes there are advantages that they
bring that outweigh the additional complexity.

## Reading

- [The Cargo Book](https://doc.rust-lang.org/cargo)
- [The Cargo Book: Build Scripts](https://doc.rust-lang.org/cargo/reference/build-scripts.html)

