# Cross-Compiling

Cross-compilation is the process of compiling code on one platform to produce
executables for a different platform. Rust identifies platforms using [_target
triples_][triple] like `x86_64-unknown-linux-gnu` or `aarch64-apple-darwin`. The
compiler maintains a [list of supported targets][platform-support] organized into
tiers based on the level of support each receives.

<center>

![Cross compilation example](cross-compile.svg)

</center>

Common reasons to cross-compile include building for a platform variant (like
`x86_64-unknown-linux-musl` for statically linked binaries), targeting platforms
that cannot host a compiler (WebAssembly, embedded microcontrollers), and
producing builds for multiple architectures from a single CI fleet without
maintaining separate builder machines for each platform.

Because Rust uses LLVM as its compilation backend, it has good cross-compilation
support out of the box — LLVM's modular architecture makes it straightforward to
generate code for many different targets.

## Simple Cross-Compilation

The simplest case requires two steps: adding the target's standard library to
your toolchain, and telling Cargo to build for that target.

```bash
rustup target add aarch64-unknown-linux-gnu
cargo build --target aarch64-unknown-linux-gnu
```

Cargo places the resulting binaries in `target/<triple>/debug/` (or `release/`)
rather than the default `target/debug/` directory. You can also set a default
target in `.cargo/config.toml` so you don't need to pass `--target` every time:

```toml
[build]
target = "aarch64-unknown-linux-gnu"
```

## When It Gets Complicated

For pure Rust crates with no native dependencies, the simple approach often just
works. But three issues commonly arise:

- **Linking errors**: Rust can compile your code for the requested target, but
  your system linker may not be able to handle non-native object files. This
  typically manifests as `error: linking with 'cc' failed` with the linker
  complaining about _file in wrong format_.
- **Native dependencies**: If your crate links against C libraries (like
  OpenSSL), you need those libraries compiled for the target platform, not your
  host platform.
- **Running tests**: You cannot execute cross-compiled binaries natively, so
  running unit tests requires an emulator or a remote machine.

The rest of this chapter covers several approaches to solving these problems,
from manual Debian multiarch setup to fully automated tools like `cross`.

## Debian Multiarch

On Debian and its derivatives (Ubuntu, etc.), you can get cross-compilation
working by installing the target's GCC toolchain and any native libraries your
code needs in the target architecture. Linux also supports userspace emulation
through QEMU, which lets you run cross-compiled binaries as if they were native
— useful for running unit tests.

The process has four steps:

1. Install a GCC cross-compiler for the target (e.g. `gcc-aarch64-linux-gnu`).
2. Add the target as a dpkg architecture and install native dependencies in that
   architecture (e.g. `libssl-dev:arm64`).
3. Set environment variables to tell Cargo which linker to use and where
   `pkg-config` can find the target's libraries.
4. Optionally, install `qemu-user-binfmt` to enable transparent emulation of
   non-native binaries via
   [binfmt_misc](https://www.kernel.org/doc/html/latest/admin-guide/binfmt-misc.html).

### Example

To cross-compile for ARM64 on a Debian-based system:

```bash
# install the cross-compiler and target libraries
sudo dpkg --add-architecture arm64
sudo apt update
sudo apt install gcc-aarch64-linux-gnu libssl-dev:arm64

# add the Rust target
rustup target add aarch64-unknown-linux-gnu

# tell Cargo which linker to use
export CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER=aarch64-linux-gnu-gcc

# tell pkg-config where to find arm64 libraries
export PKG_CONFIG_LIBDIR=/usr/lib/aarch64-linux-gnu/pkgconfig
export PKG_CONFIG_ALLOW_CROSS=true

# build
cargo build --target aarch64-unknown-linux-gnu
```

To also run the resulting binary (for example, to execute unit tests), install
QEMU userspace emulation:

```bash
sudo apt install qemu-user-binfmt
cargo test --target aarch64-unknown-linux-gnu
```

## Docker

Docker is a natural fit for cross-compilation in CI: you build an image
containing the correct toolchain, cross-compiler, and native libraries, and use
it as the CI job's container. This avoids installing cross-compilation
dependencies on the host and makes the setup reproducible.

To enable running cross-compiled binaries inside Docker (for tests), register
QEMU's userspace emulators on the host:

    docker run --rm --privileged multiarch/qemu-user-static --reset -p yes

This uses the [`multiarch/qemu-user-static`](https://github.com/multiarch/qemu-user-static)
image to install binfmt handlers. The registration persists until reboot.

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

# add arm64 target for rust
RUN rustup target add aarch64-unknown-linux-gnu

# tell rust to use this linker
ENV CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER=/usr/bin/aarch64-linux-gnu-gcc

# set pkg-config libdir to allow it to find arm64 libraries
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

# set pkg-config libdir to allow it to find armhf libraries
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

# install riscv64 cross-compiler
RUN apt update && \
    apt install -y debian-ports-archive-keyring && \
    dpkg --add-architecture riscv64 && \
    echo "deb [arch=riscv64] http://deb.debian.org/debian-ports sid main" >> /etc/apt/sources.list && \
    apt update && \
    apt install -y \
        gcc-riscv64-linux-gnu \
        g++-riscv64-linux-gnu && \
    rm -rf /var/lib/apt/lists/*

# add riscv64 target for rust
RUN rustup target add riscv64gc-unknown-linux-gnu

# tell rust to use this linker
ENV CARGO_TARGET_RISCV64GC_UNKNOWN_LINUX_GNU_LINKER=/usr/bin/riscv64-linux-gnu-gcc

# set pkg-config libdir to allow it to find riscv64 libraries
ENV PKG_CONFIG_LIBDIR=/usr/lib/riscv64-linux-gnu/pkgconfig
ENV PKG_CONFIG_ALLOW_CROSS=true
```

## `cargo-zigbuild`

[`cargo-zigbuild`][cargo-zigbuild] uses Zig's bundled C compiler and linker as
the cross-compilation toolchain. Zig ships with pre-built sysroots for many
targets, so you don't need to install separate GCC cross-compilers or manage
multiarch packages. This makes it particularly easy to cross-compile for Linux
targets with different glibc versions or for musl.

```bash
cargo install cargo-zigbuild
cargo zigbuild --target aarch64-unknown-linux-gnu
```

The main advantage is simplicity: where the Debian approach requires installing
architecture-specific packages and setting environment variables,
`cargo-zigbuild` handles the linker and sysroot automatically. The limitation is
that it only helps with the C toolchain — if your project has complex native
dependencies (like OpenSSL with custom build scripts), you may still need a more
complete cross-compilation environment.

## `cross`

[`cross`][cross] is a drop-in replacement for Cargo that runs compilation inside
Docker containers with the correct toolchains and libraries preinstalled. It
supports both cross-compilation and cross-testing (via QEMU emulation inside the
container), and targets a wide range of platforms out of the box.

```bash
cargo install cross
cross build --target aarch64-unknown-linux-gnu
cross test --target aarch64-unknown-linux-gnu
```

Because `cross` runs everything inside a container, you don't need to install
any cross-compilation toolchains on your host system. The tradeoff is that
Docker must be available, and the container images can be large. For CI
environments where Docker is already available, `cross` is often the easiest
path to multi-platform builds.

## Nix

Nix has built-in cross-compilation support through its `pkgsCross`
infrastructure. When you import nixpkgs with a `crossSystem` different from the
`localSystem`, Nix automatically provides the correct toolchain, sysroot, and
spliced dependencies — packages are compiled for the right platform based on
whether they are build-time tools (`nativeBuildInputs`) or runtime dependencies
(`buildInputs`). This distinction is what makes Nix cross-compilation work
without manual environment variable juggling.

For Rust projects using [crane](https://crane.dev/), the approach is to import
nixpkgs with the cross system set, override crane's toolchain, and use
`pkgs.callPackage` so that Nix can splice dependencies correctly (build-time
tools like `pkg-config` run on the host, while libraries like OpenSSL are
compiled for the target). Crane's documentation has a
[worked example](https://crane.dev/examples/cross-rust-overlay.html) of this
approach.

<!-- TODO: add a tested flake.nix example for cross-compiling a Rust project
     with crane, similar to the crane docs example but adapted to this book's
     style. -->

[cargo-zigbuild]: https://github.com/rust-cross/cargo-zigbuild
[cross]: https://github.com/cross-rs/cross

## Reading

```reading
style: book
title: Cross-compilation
url: https://rust-lang.github.io/rustup/cross-compilation.html
author: The rustup book
---
Official introduction to cross-compilation with rustup: how to add targets,
what gets installed (pre-built standard library), and the basics of building
for a non-native target. Start here if you are new to cross-compilation in
Rust.
```

```reading
style: book
title: "Configuration: [target] section"
url: https://doc.rust-lang.org/cargo/reference/config.html#target
author: The Cargo Book
---
Reference for the `[target.<triple>]` section in `.cargo/config.toml`. Covers
how to set the linker, rustflags, and runner per target — the configuration
that makes cross-compilation work when you need a non-default linker or want to
run tests through an emulator.
```

```reading
style: article
title: Platform Support
url: https://doc.rust-lang.org/rustc/platform-support.html
author: The rustc book
---
Complete list of targets supported by the Rust toolchain, organized into three
tiers: Tier 1 (guaranteed to build and pass tests), Tier 2 (guaranteed to
build), and Tier 3 (community-maintained). Lists the required tools for each
target and notes any limitations.
```

```reading
style: article
title: Guide to cross-compilation in Rust
url: https://blog.logrocket.com/guide-cross-compilation-rust/
author: Greg Stoll
archived: logrocket-guide-cross-compilation-rust.pdf
---
Practical walkthrough of cross-compiling from Linux to Windows using the
`cross` tool. Demonstrates the full workflow including platform detection with
`cfg` attributes and shows how `cross` handles the Docker container setup
transparently.
```

```reading
style: article
title: Zig makes Rust cross-compilation just work
url: https://actually.fyi/posts/zig-makes-rust-cross-compilation-just-work/
author: Max Hollmann
---
Demonstrates wrapping Zig's compiler as the C compiler and linker for Rust
cross-compilation. Zig ships with pre-built sysroots for many targets, so no
separate GCC toolchain is needed. Shows the shell-script wrapper approach and
discusses limitations with Zig's self-hosted linker on certain targets (like
aarch64 macOS) that were still under development at the time of writing.
```

```reading
style: book
title: LLVM
url: https://aosabook.org/en/v1/llvm.html
author: The Architecture of Open Source Applications (Volume 1)
---
Explains the architecture of LLVM: how it decouples the compiler frontend,
optimizer, and backend using a common intermediate representation (LLVM IR),
and how this modularity makes it straightforward to add new targets. Useful
background for understanding why Rust's cross-compilation support is as broad
as it is.
```

[llvm]: https://llvm.org/
[musl]: https://musl.libc.org/
[platform-support]: https://doc.rust-lang.org/rustc/platform-support.html
[triple]:
  https://doc.rust-lang.org/cargo/appendix/glossary.html?highlight=triple#target
