# Linking

*When examining the compile times, you notice that the majority of the build
time during development is spent on the linking stage.*

Rust link times can be large, especially for bigger projects. They can easily
get to and exceed the minute mark. Typically, there are two reasons why this
happens: the first one is that there is a lot to link, a big Rust backend
project can easily have ~700 dependencies that all need to be linked together
to create a single executable. 

In debug mode, Rust emits a lot of debug information, a Rust backend service
can easily have binaries which are 200 MB large, the majority of which is debug
information. All of this data needs to pass through the linker, and linkers are
not necessarily fast, the majority of them are essentially single-threaded.

## Stripping Symbols

Rust emits a lot of debug information, which can bloat binaries and put a lot
of strain on the linkers being used. Turning off or reducing the amount of debug
symbols produced leads to faster linking times.

To enable this, you can add a section such as this to the crate:

```toml
[profile.dev]
split-debuginfo = "packed"
```

## Using Faster Linker

If reducing the amount of debug information does not help enough, another
option is to use a different linker.

## Using Parallel Linker

Finally, it is possible to use a linker that works in parallel and provides
much faster build times, at the cost of needing to install it manually and not
it not supporting all platforms.
The [`mold`][mold] linker does just that and works on Linux. It has a cousin,
[`sold`][sold] which works on macOS. Currently, it is the fastest linker on the
market.

## Reading

[5.1 Faster Linking](https://www.lpalmieri.com/posts/2020-06-06-zero-to-production-1-setup-toolchain-ides-ci/#5-1-faster-linking) by Luca Palmieri

*Luca explains that when compiling Rust code, a large amount of time is spent
in the linking phase. The default linker that Rust uses, which depends on the
platform you are using, does a good job but is not the fastest. Luca explains
that LLVM's `lld` (on Windows and Linux) and Michael Eisel's `zld` (on macOS)
can get better link times and thereby cut down your compilation times.*

~~~admonish note
Since writing this article, Apple has improved the performance of `lld`, and
the author of `zld` [recommends using that
instead](https://eisel.me/lld). That is why `zld` is not mentioned in this
seciond.
~~~

[Tips For Faster Rust Compile
Times](https://corrode.dev/blog/tips-for-faster-rust-compile-times/) by
Matthias Endler

*Matthias goes through and extensive list of tips for getting faster Rust
compile times.  These include making sure your toolchain is up-to-date,
enabling the parallel compiler frontend, removing unused dependencies,
debugging dependency compile times, splitting large crates into smaller ones,
optimizing workspaces, compilation caching, and many more. He recommends trying
`mold`, `lld` or `zld` linkers to speed up your linking times.*


[Slightly faster linking for Rust](https://brokenco.de/2020/01/08/faster-rust-linking.html) by R. Tyler Croy

[Resolving Rust Symbols](https://blog.shrirambalaji.com/posts/resolving-rust-symbols/) by Shriram Balaji

*Shriram explains how Rust compilation, and specifically the linking phase
works. He uses diagrams to break down what the Rust compiler and LLVM do to
allow you to compile your Rust code, what object files are and what they store,
how name mangling works, and many other nuggets of information. This article is
incredibly useful if you want to understand on a deep level how linking works.*

[Enabling rust-lld on Linux](https://blog.rust-lang.org/2024/05/17/enabling-rust-lld-on-linux.html)

*In this blog article, the Rust team announces enabling the `lld` linker by
default for nightly Rust on Linux targets.*

[mold]: https://github.com/rui314/mold
[sold]: https://github.com/bluewhalesystems/sold
