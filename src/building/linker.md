# Linking

Linking is often the bottleneck in Rust compile times, especially during
development. After the compiler has translated each crate into object files,
the linker combines them into a single executable. For a large project with
hundreds of dependencies, there is a lot of data to process. Debug builds make
this worse: Rust emits extensive debug information by default, and a typical
backend service can produce debug binaries of 200 MB or more. Most of that
volume passes through the linker, which on many systems defaults to GNU `ld` — a
capable but single-threaded linker that was not designed for this scale.

There are three approaches to reducing link times: reducing the amount of data
the linker has to process, using a faster linker, and using a parallel linker.

## Reducing Debug Information

By default, debug builds include full debug information so that tools like `gdb`
and `lldb` can provide useful stack traces and variable inspection. This debug
information is the largest contributor to binary size in dev builds and adds
significant work for the linker.

You can reduce this overhead by splitting debug information into a separate file
rather than embedding it in the binary, or by reducing the debug info level:

```toml
[profile.dev]
# Split debug info into a separate file (reduces linker work)
split-debuginfo = "packed"
```

```toml
[profile.dev]
# Reduce debug info to line tables only (faster linking, less useful debugger)
debug = "line-tables-only"
```

For release builds, stripping debug information entirely is covered in [Binary
Size](size.md).

## `rust-lld`

Starting with Rust 1.90, the Rust toolchain ships with and uses `rust-lld` (a
bundled copy of LLVM's LLD linker) by default on `x86_64-unknown-linux-gnu`.
LLD is significantly faster than GNU `ld` because it is designed for parallel
processing and has a more efficient architecture for handling large inputs.

Benchmarks from the Rust team show that LLD provides roughly 7x faster linking
on incremental rebuilds, translating to around a 40% reduction in end-to-end
compilation time for projects like ripgrep. For most developers on Linux x86_64,
this improvement happens automatically with no configuration needed.

## `mold`

[`mold`][mold] is a linker designed from scratch for parallelism. On Linux, it
is typically the fastest linker available, outperforming even LLD for large
projects. Its macOS counterpart is [`sold`][sold].

The tradeoff is that mold must be installed separately and does not support all
platforms. But if you are on Linux and link times are a bottleneck, mold is
worth trying.

To use mold, install it through your system package manager (e.g. `apt install
mold` on Debian/Ubuntu) and configure Cargo to use it:

```toml
[target.x86_64-unknown-linux-gnu]
linker = "clang"
rustflags = ["-C", "link-arg=-fuse-ld=mold"]
```

Alternatively, you can use mold's wrapper mode, which intercepts linker
invocations without changing your Cargo configuration:

```bash
mold -run cargo build
```

<!-- TODO: measure linking times across configurations: system linker (GNU ld),
     rust-lld, and mold, each with and without strip/split-debuginfo. A concrete
     comparison table would make this chapter much more actionable. -->

## Reading

```reading
style: article
title: Resolving Rust Symbols
url: https://blog.shrirambalaji.com/posts/resolving-rust-symbols/
author: Shriram Balaji
---
Walks through the entire Rust compilation pipeline from lexing through LLVM
code generation, then focuses on what the linker actually does: combining
object files, resolving symbols, and producing executables. Covers ELF object
file structure, symbol tables (strong vs weak symbols), name mangling (how
`Global` becomes `__ZN11foo6Global17ha2a12041c4e557c5E`), and how to manually
compile Rust files into static libraries and link them. Read this if you want
to understand what the linker is doing under the hood.
```

```reading
style: article
title: "5.1 Faster Linking"
url: https://www.lpalmieri.com/posts/2020-06-06-zero-to-production-1-setup-toolchain-ides-ci/#5-1-faster-linking
author: Luca Palmieri
---
Section from Luca's "Zero to Production" series showing how to configure
alternative linkers for Rust on different platforms. Predates the `rust-lld`
default but the configuration approach is still useful for opting into `mold`
or for platforms where LLD is not yet the default. Note: the article mentions
`zld` for macOS, which has since been deprecated in favor of Apple's improved
`lld`.
```

```reading
style: article
title: Slightly faster linking for Rust
url: https://brokenco.de/2020/01/08/faster-rust-linking.html
author: "R. Tyler Croy"
---
Short practical post showing a 70% reduction in link times (from ~10s to ~3s)
by switching from GNU `ld` to `lld` with a two-line `.cargo/config` change.
Good illustration of the kind of improvement alternative linkers provide.
```

```reading
style: article
title: Enabling rust-lld on Linux
url: https://blog.rust-lang.org/2024/05/17/enabling-rust-lld-on-linux.html
author: Rust Team
---
Announcement from the Rust team about enabling `rust-lld` (a bundled copy of
LLVM's LLD) by default on nightly for Linux targets. Explains the motivation:
LLD is faster than GNU `ld`, and bundling it means Rust controls the linker
version, avoiding compatibility issues with system linkers. This was the
precursor to the stable rollout in Rust 1.90.
```

```reading
style: article
title: Faster linking times with 1.90.0 stable on Linux using the LLD linker
url: https://blog.rust-lang.org/2025/09/01/rust-lld-on-1.90.0-stable/
author: Rémy Rakic
---
Announces the stable rollout of `rust-lld` as the default linker on
`x86_64-unknown-linux-gnu` in Rust 1.90. Reports 7x faster linking on
incremental rebuilds and a 40% reduction in end-to-end compilation time for
projects like ripgrep. Explains how to opt out if compatibility issues arise
and the plan to expand to other Linux targets.
```

```reading
style: article
title: Tips For Faster Rust Compile Times
url: https://corrode.dev/blog/tips-for-faster-rust-compile-times/
author: Matthias Endler
archived: corrode-faster-rust-compile-times.pdf
---
Broad survey of techniques for reducing Rust compile times. The linker section
recommends trying `mold`, `lld`, or `zld` (now deprecated) and shows how to
configure each. Also covers other approaches that complement faster linking:
removing unused dependencies, splitting large crates, and compilation caching.
```


[mold]: https://github.com/rui314/mold
[sold]: https://github.com/bluewhalesystems/sold
