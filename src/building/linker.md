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

[Tips For Faster Rust Compile Times](https://corrode.dev/blog/tips-for-faster-rust-compile-times/) by Matthias Endler

[Slightly faster linking for Rust](https://brokenco.de/2020/01/08/faster-rust-linking.html) by R. Tyler Croy

https://blog.shrirambalaji.com/posts/resolving-rust-symbols/

[mold]: https://github.com/rui314/mold
[sold]: https://github.com/bluewhalesystems/sold
