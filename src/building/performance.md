# Performance

Rust's default compilation settings are designed to balance compile speed and
runtime performance. For development builds (`cargo build`), Cargo prioritizes
fast compilation so you can iterate quickly. For release builds
(`cargo build --release`), it enables optimizations that produce faster binaries
at the cost of longer compile times. But the default release profile is still
fairly conservative, and there are several options you can tune to get more
performance out of your code.

Cargo has several built-in profiles (`dev`, `release`, `test`, `bench`), but the
two you interact with most are `dev` and `release`. The `dev` profile is used by
default, and `release` is used when you pass the `--release` flag. You can
override the settings of any built-in profile, and you can also define your own
custom profiles. The default release profile looks like this:

```toml
[profile.release]
opt-level = 3
debug = false
split-debuginfo = '...'  # Platform-specific.
strip = "none"
debug-assertions = false
overflow-checks = false
lto = false
panic = 'unwind'
incremental = false
codegen-units = 16
rpath = false
```

Most of the performance-relevant options here are `opt-level`, `lto`,
`codegen-units`, and `panic`. The sections below explain the most impactful
changes you can make.

## Codegen Units

By default, the release profile splits each crate into 16 _codegen units_ that
are compiled in parallel. This speeds up compilation, but it limits the
optimizer's ability to perform cross-function optimizations like inlining,
because each codegen unit is optimized independently.

Setting `codegen-units = 1` forces the compiler to process each crate as a
single unit, giving the optimizer a complete view of all the code. This
typically produces faster binaries at the cost of longer compile times.

```toml
[profile.release]
codegen-units = 1
```

## Link-Time Optimization

When you enable Link-Time Optimization (LTO), you ask the compiler to run extra
optimization passes not when building the individual crates, but when linking
your crates together into a binary. At this point, the compiler can see exactly
which code is actually getting called and which is not.

LTO allows the compiler to eliminate dead code and inline functions across crate
boundaries, which can improve both binary size and runtime speed. There are two
variants:

- `lto = "full"` merges all codegen units from all crates into a single module
  and optimizes it as a whole. This produces the best results but is the slowest
  to compile.
- `lto = "thin"` performs LTO on a per-module basis using summaries of each
  module rather than merging everything together. It captures most of the
  benefit of full LTO with significantly less compile time overhead. This is a
  good default if full LTO makes your build too slow.

```toml
[profile.release]
lto = "full"
```

Combining `codegen-units = 1` with `lto = "full"` gives the optimizer the
broadest possible view of your code. This is the most impactful configuration
change you can make for runtime performance, and it is what most projects should
use for production builds.

## Target Features

When you compile your Cargo crate, it will generate code for some specific
platform. Typically, you will generate code for the `x86_64-unknown-linux-gnu`
target. That first part of the triple, `x86_64` (commonly called `amd64`) is the
architecture (type of processor) that your code will run on.

Modern AMD64 processors have an array of extensions that can speed up certain
operations, such as hardware support for AES through AES-NI, or support for SIMD
with AVX2. In order for your program to remain compatible with many processors,
Cargo will, by default, not make use of these added instructions, unless you
tell it to.

You can enable these extra instructions (called _target features_) by adding
them to your Cargo configuration at `.cargo/config.toml` within your repository.

```toml
[target.x86_64-unknown-linux-gnu]
rustflags = ["-C target-feature=+avx2"]
```

If you know that your binary will only run on the machine it's being compiled on
(for example, a server you control), you can tell the compiler to use whatever
features the current CPU supports:

```toml
[target.x86_64-unknown-linux-gnu]
rustflags = ["-C target-cpu=native"]
```

```admonish warning
Be careful with `target-cpu=native` in CI or cross-compilation setups. The
compiler will emit instructions specific to whichever CPU the build machine
has. If you build on a machine with AVX-512 and deploy to one without it,
your binary will crash with an illegal instruction error.
```

Note that these flags only affect which instructions Cargo will natively emit.
Some crates also detect CPU features at runtime and switch to whichever
implementation works best on your chipset, regardless of what target features
you compile with.

In theory, when you enable target features, the compiler is able to use them to
produce faster code. This process is called
[automatic vectorization](https://en.wikipedia.org/wiki/Automatic_vectorization).
In practise, this might not make much of a difference. Either you have number
crunching code, and you really care about the memory layout, and use SIMD calls
to precisely speed it up, or you have mixed code with memory layouts that poorly
vectorize. That is why generally, you don't need to worry about enabling target
CPU features, and if you do, you already know about it.

## Profile-Guided Optimization

Profile-Guided Optimization (PGO) is an approach to give the compiler better
context for optimizing your program, by first compiling it with instrumentation,
running representative workloads (with the instrumentation tracking which
branches are taken, and which functions are commonly used), and then
re-compiling your program with this information.

If the compiler knows which branches are commonly taken, and which functions are
commonly used, it is sometimes able to emit code that runs faster. Typical
improvements range from 5% to 20% depending on the workload.

The process has four steps:

1. Build with instrumentation enabled:
   ```bash
   RUSTFLAGS="-Cprofile-generate=/tmp/pgo-data" \
       cargo build --release
   ```
2. Run the instrumented binary with a representative workload. This generates
   `.profraw` files in the directory you specified.
3. Merge the raw profiling data into a single file using LLVM's profdata tool:
   ```bash
   llvm-profdata merge -o /tmp/pgo-data/merged.profdata \
       /tmp/pgo-data/*.profraw
   ```
4. Rebuild using the merged profiling data:
   ```bash
   RUSTFLAGS="-Cprofile-use=/tmp/pgo-data/merged.profdata" \
       cargo build --release
   ```

The [`cargo-pgo`][cargo-pgo] tool simplifies this workflow by managing the
instrumentation, profiling, merging, and rebuild steps for you.

These kinds of optimizations are commonly applied for large GUI applications,
for example the Chromium and Firefox browsers use them. For them it makes sense,
if a build takes multiple hours because they need to generate this profdata, but
they deploy their software out to billions of devices, and it makes their
browsers run 3% faster, that is worth it. For your garden-variety backend Rust
project, you likely don't need it.

<!-- todo: find out what other projects use PGO, and why -->

## Post-Link Optimization

Post-link optimization tools optimize binaries _after_ they have been fully
compiled and linked. The most notable tool in this space is [BOLT][bolt],
developed by Meta. BOLT works similarly to PGO: you first run your binary with a
profiling tool to collect data about which code paths are hot, and then BOLT
reorganizes the binary's layout to improve instruction cache locality.

The key advantage of BOLT over PGO is that it operates on the final binary, so
it can optimize across all code including the standard library and C
dependencies that the Rust compiler never sees. BOLT can be combined with PGO
for additional gains. The `cargo-pgo` tool supports both PGO and BOLT workflows.

## Allocators

In programs that perform a lot of heap allocations, the allocator can become a
bottleneck. The default allocator in Rust is the system allocator (typically
glibc's `malloc` on Linux), which is a general-purpose allocator designed for
correctness and broad compatibility. Specialized allocators can improve
performance for specific workloads.

Two popular alternative allocators in the Rust ecosystem are
[`jemalloc`][jemalloc-rs] and [`mimalloc`][mimalloc-rs].

jemalloc, originally developed for FreeBSD, is designed for multi-threaded
applications. It uses thread-local caches to reduce contention and has better
fragmentation behavior for long-running services. You can use it in Rust through
the `tikv-jemallocator` crate:

```toml
[dependencies]
tikv-jemallocator = "0.6"
```

```rust
use tikv_jemallocator::Jemalloc;

#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;
```

mimalloc, developed by Microsoft Research, is a compact general-purpose
allocator that focuses on performance and low memory overhead. It tends to
perform particularly well in workloads with many small allocations:

```toml
[dependencies]
mimalloc = "0.1"
```

```rust
use mimalloc::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;
```

Switching the allocator is a simple change, but the performance impact varies
significantly depending on your workload. It is worth benchmarking your specific
application with different allocators before committing to one. For server
workloads with high allocation rates and multiple threads, jemalloc or mimalloc
often provide measurable improvements. For single-threaded or low-allocation
workloads, the system allocator is usually fine.

## Reading

```reading
style: book
title: "Profiles"
url: https://doc.rust-lang.org/cargo/reference/profiles.html
author: The Cargo Book
---
Official documentation for Cargo profiles, explaining how to configure build
settings for different compilation modes including debug, release, and custom
profiles.
```

```reading
style: article
title: "Optimizing Rust programs with PGO and BOLT using cargo-pgo"
url: https://kobzol.github.io/rust/cargo/2023/07/28/rust-cargo-pgo.html
author: Jakub Beránek
---
Jakub demonstrates how to combine Profile-Guided Optimization (PGO) with BOLT
post-link optimization to achieve significant performance improvements in Rust
programs.
```

```reading
style: book
title: "Profile-guided Optimization"
url: https://doc.rust-lang.org/rustc/profile-guided-optimization.html
author: The rustc book
---
Official documentation explaining how to use Profile-Guided Optimization (PGO)
with rustc to optimize program performance based on runtime profiling data.
```

```reading
style: article
title: "Exploring PGO for the Rust compiler"
url: https://blog.rust-lang.org/inside-rust/2020/11/11/exploring-pgo-for-the-rust-compiler.html
author: Rust Team
---
Blog post discussing the Rust team's exploration of using Profile-Guided
Optimization to improve the performance of the Rust compiler itself.
```

```reading
style: article
title: "cargo-pgo"
url: https://github.com/Kobzol/cargo-pgo
author: Jakub Beránek
---
A Cargo subcommand for easier use of Profile-Guided Optimization (PGO) and
post-link optimization (BOLT) with Rust programs.
```

```reading
style: article
title: "BOLT"
url: https://github.com/llvm/llvm-project/tree/main/bolt
author: LLVM Project
---
Binary Optimization and Layout Tool (BOLT), a post-link optimizer developed by
Meta that can improve performance by optimizing application layout based on
profiling data.
```

```reading
style: book
title: "Optimized build"
url: https://rustc-dev-guide.rust-lang.org/building/optimized-build.html
author: rustc dev guide
---
Guide explaining how to build optimized versions of the Rust compiler itself,
including using PGO and other optimization techniques.
```

[cargo-pgo]: https://github.com/Kobzol/cargo-pgo
[bolt]: https://github.com/llvm/llvm-project/tree/main/bolt
[jemalloc-rs]: https://github.com/tikv/jemallocator
[mimalloc-rs]: https://github.com/purpleprotocol/mimalloc_rust
