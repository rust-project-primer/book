# Performance

_You've shipped your Rust application to production and it's working well, but you notice that it's
not performing as fast as you'd like. Your users are experiencing longer response times than
expected, and you suspect that the default compilation settings might not be optimal for your
production workload. You wonder what compilation options are available to squeeze more performance
out of your Rust code._

When you build your code, you have some opportunities to tweak how Cargo compiles your code that can
result in better performance.

By default, Cargo has two built-in profiles: `dev` and `release`. These come with sensible defaults,
but you can override them, or even create your own, custom profiles.

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

## Link-Time Optimizations

When you enable Link-Time Optimizations (LTO), you ask the compiler to run some extra optimization
passes not when building the individual crates, but when linking your crates together into a binary.
At this point, the compiler can see exactly which code is actually getting called, and which is not.

Link-Time Optimizations therefore allow you to reduce the code size, which can result in added
speed. They also allow for inlining things across crate boundaries, which can also give your code a
speed boost.

To enable LTO, you can set the `lto` property in your profile to `true` or to `"full"`.

```toml
[profile.release]
lto = "full"
```

## Enabling Target Features

When you compile your Cargo crate, it will generate code for some specific platform. Typically, you
will generate code for the `x86_64-unknown-linux-glibc` target. That first part of the triple,
`x86_64` (commonly called `amd64`) is the type of processor that your code will run.

Modern AMD64 processors have an array of extensions that can speed up certain operations, such as
hardware support for AES through AES-NI, or support for SIMD with AVX2. In order for your program to
remain compatible with many processors, Cargo will, by default, not make use of these added
instructions, unless you tell it to.

You can enable these extra instructions (called _target features_) by adding it into your Cargo
configuration at `.cargo/config.toml` within your repository.

```toml
[target.x86_64-unknown-linux-gnu]
rustflags = ["-C target-feature=+avx2"]
```

Note that these flags only affect which instructions Cargo will natively emit. Crates may check for
the presence of these features. But some crates will also detect them at runtime and switch to
whichever implementation works best on your chipset.

## Performance-Guided Optimization

Performance-Guided Optimization (PGO) is an approach to give the compiler better context for
optimizing your program, by first compiling it with instrumentation, running representative
workloads (with the instrumentation tracking which branches are taken, and which functions are
commonly used), and then re-compiling your program with this information.

If the compiler knows which branches are commonly taken, and which functions are commonly used, it
is sometimes able to emit code that runs faster.

## Post-Link Optimization

Post-Link Optimization is an approach whereby binaries are optimized after being fully compiled and
linked.

## Using a different allocator

In programs that perform a lot of allocations (which is most programs these days), the allocator can
be a bottleneck for performance.

## Reading

```reading
style: book
title: "Profiles"
url: https://doc.rust-lang.org/cargo/reference/profiles.html
author: The Cargo Book
---
Official documentation for Cargo profiles, explaining how to configure build settings for different compilation modes including debug, release, and custom profiles.
```

```reading
style: article
title: "Optimizing Rust programs with PGO and BOLT using cargo-pgo"
url: https://kobzol.github.io/rust/cargo/2023/07/28/rust-cargo-pgo.html
author: Jakub Beránek
---
Jakub demonstrates how to combine Profile-Guided Optimization (PGO) with BOLT post-link optimization to achieve significant performance improvements in Rust programs.
```

```reading
style: book
title: "Profile-guided Optimization"
url: https://doc.rust-lang.org/rustc/profile-guided-optimization.html
author: The rustc book
---
Official documentation explaining how to use Profile-Guided Optimization (PGO) with rustc to optimize program performance based on runtime profiling data.
```

```reading
style: article
title: "Exploring PGO for the Rust compiler"
url: https://blog.rust-lang.org/inside-rust/2020/11/11/exploring-pgo-for-the-rust-compiler.html
author: Rust Team
---
Blog post discussing the Rust team's exploration of using Profile-Guided Optimization to improve the performance of the Rust compiler itself.
```

```reading
style: article
title: "cargo-pgo"
url: https://github.com/Kobzol/cargo-pgo
author: Jakub Beránek
---
A Cargo subcommand for easier use of profile-guided optimization (PGO) and post-link optimization (PLO/BOLT) with Rust programs.
```

```reading
style: article
title: "BOLT"
url: https://github.com/llvm/llvm-project/tree/main/bolt
author: LLVM Project
---
Binary Optimization and Layout Tool (BOLT) - a post-link optimizer developed by Facebook that can improve performance by optimizing application layout based on profiling data.
```

```reading
style: book
title: "Optimized build"
url: https://rustc-dev-guide.rust-lang.org/building/optimized-build.html
author: rustc dev guide
---
Guide explaining how to build optimized versions of the Rust compiler itself, including using PGO and other optimization techniques.
```
