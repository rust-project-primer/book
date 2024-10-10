# Cross-Compiling

When you compile something, you usually create an executable that is able to
run on whichever platform you are compiling it on. This is called *native
compilation*. However, there are some situation where you want to be able to
generate binaries for a *different* platform than the one that you are doing
the compilation on. Doing this is called *cross compilation*.

<center>

![Cross compilation example](cross-compile.svg)

</center>

Rust has a concept of [*target triples*][triple]. This is how Rust identifies the
platform that you want the compiler to generate binaries for. For example, if
you are using Linux on an AMD64 system, your target triple might be
`x86_64-unknown-linux-gnu`.  From Rust's point of view, cross compilation is
whenever you ask the compiler to generate executables for a different target
triple than your native one. The Rust compiler maintains a [list of supported
targets](https://doc.rust-lang.org/rustc/platform-support.html).

In general, doing cross-compilation can be a bit of a hassle, but there are
some good reasons to do so. For example:

- If you want to build code for a variant of your native triple, for example
  using the `x86_64-unknown-linux-musl` target triple to generate an executable
  which uses [MUSL libc][musl] rather than the default glibc.
- If the target triple you are building for does not have a Rust toolchain.
  For example, you cannot do a native compilation for `wasm32-unknown-unknown`.
- If the target you are building for is severaly underpowered, such that you
  cannot compile on it natively. This tends to be the case for embedded systems
  such as `thumbv6m-none-eabi`.
- If you want to create builds for multiple platforms, but you don't want
  to purchase and maintain builder machines for every platform. Usually, having
  a fleet of Linux machines is cheaper and easier to maintain.

Rust uses [LLVM][llvm] to implement its compilation backend. LLVM
is written in a modular way that makes it easy to add support for code generation
for new targets. This means that Rust comes with good support for cross-compilation
out-of-the-box.

### Simple Cross-Compilation

If you want to use this, you first need to add support for the target you want
to build for to your installed Rust toolchain. What this does is download a
pre-built version of the Rust standard library for the target you specify.  If
you are using Rustup to manage your Rust toolchains, then doing so looks like
this:

```bash
# add support for WebAssembly
rustup target add wasm32-unknown-unknown

# add support for building binaries with musl libc
rustup target add x86_64-unknown-linux-musl
```

When you then want to build your code, all you need to do is tell Cargo to
build for the different target. You can do this by passing it the `--target`
command-line option, or you can define default target in your
`.cargo/config.toml` file.

```bash
cargo build --target wasm32-unknown-unknown
```

When you specify the target like this, Cargo will output the resulting binaries
in `target/wasm32-unknown-unknown/debug/` rather than the default
`target/debug` folder in your project.

### Issues with cross-compilation

In some cases, this is all you need to do. However, there are three issues
you may run into with this approach:

- **Linking errors**: You might run into linking errors, because while Rust
  can compile your crate for the target that you have requested, your
  system linker might not be able to deal with non-native object files.
- **Native dependencies**: If your applications links with any native libraries,
  then you need to have these native libraries compiled *for the target that
  you are compiling for*.
- **Inability to run tests**: You might not be able to execute the code
  you have just compiled, meaning that you cannot run your unit tests.

Linking issues usually manifest by an error like this, along with some output
from the linker complaining about *file in wrong format*.

    error: linking with `cc` failed: exit status: 1



In the rest of this section, I'll show you how to deal with this. There is no
real perfect solution, but several tools and approaches exist that should help
you get this working. The main challenge is getting a linker that can handle
the target executables, and getting the dependencies for the right target.

## Debian

If you use Debian, or some derivative distribution, you can typically get cross-compilation
(including native dependencies) working relatively easy by installing a few packages.
Linux even lets you install a userspace emulator (for example QEMU) to allow you to
run your binaries "as if" they were native, allowing you to run unit tests, for
example.

Generally, this can be done in four steps:

1. Install a compiler toolchain appropriate for your target. This is often
   `gcc-<triple>`, for example `gcc-aarch64-linux-gnu`.
2. Add the target as a dpkg architecture and install whatever native
   dependencies your code needs in that architecture. For example, if you
   require `libssl-dev` for ARM64, then you must install `libssl-dev:arm64`.
3. Set some environment variables to tell Cargo to use the correct linker, and
   to allow `pkg-config` to find your native dependencies.
4. If you want to be able to run the executables, install the
   `qemu-user-binfmt` package. This will install [binfmt
   handlers](https://www.kernel.org/doc/html/latest/admin-guide/binfmt-misc.html)
   which will use QEMU to emulate any non-native executables, allowing you to
   run unit tests.

### Example

## Docker

You can use Docker to build and image which contains the right packages and
environment variables to allow Rust to easily cross-compile your project for
another target. This is often useful in CI, where you can build a Docker
container from this and use it for the CI job where you cross-compile your
code.

To allow Docker to run native dependencies, you can use the
[`multiarch/qemu-user-static`](https://github.com/multiarch/qemu-user-static)
image, which you can set up like this. After you ran this, you have configured
Docker to be able to run non-native binaries, which stays enabled until a
reboot.

    docker run --rm --privileged multiarch/qemu-user-static --reset -p yes


### Example: Dockerfile for cross-compiling for ARM64

```docker
FROM rust

# install rustfmt and clippy
RUN rustup component add rustfmt
RUN rustup component add clippy

# install build-essential, pkg-config, cmake
RUN apt update && \
    apt install -y build-essential pkg-config cmake && \
    rm -rf /var/lib/apt/lists/*

# install arm64 cross-compiler
RUN dpkg --add-architecture arm64 && \
    apt update && \
    apt install -y \
        gcc-aarch64-linux-gnu \
        g++-aarch64-linux-gnu \
        libssl-dev:arm64 && \
    rm -rf /var/lib/apt/lists/*

# add arm32 target for rust
RUN rustup target add aarch64-unknown-linux-gnu

# tell rust to use this linker
ENV CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER=/usr/bin/aarch64-linux-gnu-gcc

# set pkg-config libdir to allow it to link aarch libraries
ENV PKG_CONFIG_LIBDIR=/usr/lib/aarch64-linux-gnu/pkgconfig
ENV PKG_CONFIG_ALLOW_CROSS=true
```

### Example: Dockerfile for cross-compiling for ARM32

```docker
FROM rust

# install rustfmt and clippy
RUN rustup component add rustfmt
RUN rustup component add clippy

# install build-essential, pkg-config, cmake
RUN apt update && \
    apt install -y build-essential pkg-config cmake && \
    rm -rf /var/lib/apt/lists/*

# install arm32 cross-compiler
RUN dpkg --add-architecture armhf && \
    apt update && \
    apt install -y \
        gcc-arm-linux-gnueabihf \
        g++-arm-linux-gnueabihf \
        libssl-dev:armhf && \
    rm -rf /var/lib/apt/lists/*

# add arm32 target for rust
RUN rustup target add arm-unknown-linux-gnueabihf

# tell rust to use this linker
ENV CARGO_TARGET_ARM_UNKNOWN_LINUX_GNUEABIHF_LINKER=/usr/bin/arm-linux-gnueabihf-gcc

# set pkg-config libdir to allow it to link aarch libraries
ENV PKG_CONFIG_LIBDIR=/usr/lib/arm-linux-gnueabihf/pkgconfig
ENV PKG_CONFIG_ALLOW_CROSS=true
```

### Example: Dockerfile for cross-compiling for RISC-V

```docker
FROM rust

# install rustfmt and clippy
RUN rustup component add rustfmt
RUN rustup component add clippy

# install build-essential, pkg-config, cmake
RUN apt update && \
    apt install -y build-essential pkg-config cmake && \
    rm -rf /var/lib/apt/lists/*

# install arm32 cross-compiler
RUN apt update && \
    apt install -y debian-ports-archive-keyring && \
    dpkg --add-architecture riscv64 && \
    echo "deb [arch=riscv64] http://deb.debian.org/debian-ports sid main" >> /etc/apt/sources.list && \
    apt update && \
    apt install -y \
        gcc-riscv64-linux-gnu \
        g++-riscv64-linux-gnu && \
    rm -rf /var/lib/apt/lists/*

# add arm32 target for rust
RUN rustup target add riscv64gc-unknown-linux-gnu

# tell rust to use this linker
ENV CARGO_TARGET_RISCV64GC_UNKNOWN_LINUX_GNU_LINKER=/usr/bin/riscv64-linux-gnu-gcc

# set pkg-config libdir to allow it to link aarch libraries
ENV PKG_CONFIG_LIBDIR=/usr/lib/riscv64-linux-gnu/pkgconfig
ENV PKG_CONFIG_ALLOW_CROSS=true
```

## Nix

You can use Nix to implement cross-compilation.

*Todo*

## Cargo Zigbuild

[cargo-zigbuild](https://github.com/rust-cross/cargo-zigbuild) is a Cargo subcommand
that lets you build Rust applications while using Zig as the linker.


## [Cross](https://github.com/cross-rs/cross)

Cross advertises itself as a "zero-setup" tool for cross-compilation and
cross-testing of Rust crates. Under the hood, it uses Docker containers to run
the compilation steps with the right toolchains and libraries preinstalled.

The idea with it is that it acts as a replacement for Cargo.

## Reading

[Platform Support](https://doc.rust-lang.org/rustc/platform-support.html) in *The rustc book*

*This chapter lists all targets which are supported by the Rust toolchain, along
with notes explaining what the targets are, and which tools are required for build
for them. It also gives information on the level of support for each target.*

[Guide to cross-compilation in Rust](https://blog.logrocket.com/guide-cross-compilation-rust/) by Greg Stoll

*In this article, Greg explains how to cross-compile Rust crates using the cross project.*

[Zig makes Rust cross-compilation just work](https://actually.fyi/posts/zig-makes-rust-cross-compilation-just-work/) by Max Hollmann

*Max explains how you can use Zig to simplify cross-compilation for Rust. Zig
comes with built-in support for compiling and linking for various targets
out-of-the-box, which means you don't need to install separate toolchains for
each target.*

[LLVM](https://aosabook.org/en/v1/llvm.html) in *The Architecture of Open Source Applications (Volume 1)*

*Chris explains the architecture of LLVM, and how its design choices make it
easy to use it as a library to build compilers, and to target a variety of
different targets. LLVM decouples the various stages of the compiler and uses a
serialisation format to communicate between them.*

[Cross-compilation](https://rust-lang.github.io/rustup/cross-compilation.html) in *The rustup book*

*This chapter explains the basics for how to do cross-compilation with rustup.*

[Configuration (target section)](https://doc.rust-lang.org/cargo/reference/config.html#target) in *The Cargo Book*

*This chapter in the book explains how you can configure Cargo to do cross-compilation,
by telling it which linker, rustflags and runner to use.*

[llvm]: https://llvm.org/
[musl]: https://musl.libc.org/
[triple]: https://doc.rust-lang.org/cargo/appendix/glossary.html?highlight=triple#target
