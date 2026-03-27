# Building

For most Rust projects, `cargo build --release` is all you need. Cargo handles
dependency resolution, compilation order, and linking. But as a project grows,
build concerns that were invisible on a small codebase start to matter: compile
times creep up, release binaries are larger than expected, CI spends minutes
rebuilding unchanged dependencies, and you need to ship binaries for platforms
you don't develop on.

This chapter covers the knobs you can turn and the tools you can use once the
defaults are no longer enough. The topics fall roughly into two categories.

The first is **build output**: controlling what the compiler produces. Cargo
profiles let you tune the tradeoff between binary size, runtime performance, and
compile time. [Binary Size](size.md) covers stripping, optimization levels, and
monomorphization. [Performance](performance.md) covers LTO, codegen units,
target features, PGO, and allocators. These two chapters are complementary — the
same profile options appear in both, but with different goals.

The second is **build process**: making compilation itself faster or more
capable. [Codegen](codegen.md) covers alternative compiler backends like
Cranelift that trade runtime performance for faster debug builds.
[Caching](cache.md) covers sccache for sharing compilation results across builds
and machines. [Linking](linker.md) covers faster linkers like mold and lld that
can cut link times dramatically. [Cross-Compiling](cross.md) covers building for
targets other than your development machine, including Docker-based and
Nix-based approaches.

[`cargo-wizard`](https://github.com/Kobzol/cargo-wizard) is a useful starting
point if you want to quickly apply preset profile configurations for faster
builds, smaller binaries, or better runtime performance without manually tuning
each option.

## Reading

```reading
style: article
title: Tips For Faster Rust Compile Times
url: https://corrode.dev/blog/tips-for-faster-rust-compile-times/
author: Matthias Endler
---
Comprehensive list of techniques for reducing Rust compile times, covering the
full range: updating the toolchain, enabling the parallel compiler frontend,
removing unused dependencies, diagnosing slow crates with `cargo build
--timings`, splitting large crates, workspace-level optimizations, and
compilation caching. A good starting point if you want to survey all available
options before diving into the specific chapters below.
```

```reading
style: article
title: Fast Rust Builds
url: https://matklad.github.io/2021/09/04/fast-rust-builds.html
author: Alex Kladov
archived: matklad-fast-rust-builds.pdf
---
Alex frames Rust's compile time problem honestly — the language has prioritized
execution speed and programmer productivity over compilation speed — and then
gives practical advice for working within that constraint. Covers CI pipeline
structure (separate check/test/lint jobs), pruning dependencies, avoiding
procedural macros where possible, and code patterns that compile faster.
```

```reading
style: article
title: Stupidly effective ways to optimize Rust compile time
url: https://xxchan.me/cs/2023/02/17/optimize-rust-comptime-en.html
author: Tianxiao Shen
archived: xxchan-optimize-rust-comptime.pdf
---
Practical tips from optimizing compilation for a real-world Rust project,
covering dependency management, workspace organization, and compiler flags.
Focuses on changes that are easy to apply and have outsized impact.
```

```reading
style: article
title: What part of Rust compilation is the bottleneck?
url: https://kobzol.github.io/rust/rustc/2024/03/15/rustc-what-takes-so-long.html
author: Jakub Beránek
archived: kobzol-rustc-what-takes-so-long.pdf
---
Profiles the Rust compiler across the 100 most popular crates to measure where
time is actually spent. The answer depends on context: the LLVM backend
dominates binary builds, while the frontend (type checking, borrow checking)
dominates library builds. For incremental debug builds, the linker is the main
bottleneck — which is why the [Linking](linker.md) chapter matters for
development iteration speed.
```
