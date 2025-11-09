# Binary Size Optimization

When you compile Rust code, you have some control over the compiler as to what it prioritizes when
building your executables. Everything is a tradeoff, so when you prioritize one aspect, you might
see a regression in another aspect. Common priorities are:

- **Speed**: You want your executables to run as fast as possible. This might lead to an increase in
  code size, because the compiler will use techniques like inlining or loop unrolling to achieve
  this.
- **Binary size**: You want your executables to be as small as possible, for example because you are
  targeting a resource-constrained platform like embedded microcontrollers with limited flash memory
  sizes, or you want to be able to easily distribute your binary. This might lead to a negative
  impact on performance.

### Compilation Profiles

In general, the way you exercise control over this is by creating _profiles_. Every profile comes
with a set of parameters that let you tweak how the compiler performs. Typically, when you make
debug builds, your main priority is fast compilation times, so you are happy to sacrifice some
runtime speed.

A profile definition looks like this:

```toml
[profile.release]
strip = true
opt-level = 3
```

## Runtime speed

## Binary Size

_One of your customers deploys the Rust-based tool that you are developing on very small embedded
systems which run in an industrial environment. They are severely resource-constrained, making Rust
an ideal language to target them. However, the customer has started complaining that the binaries
you are shipping him are getting quite large. You are wondering if there are some strategies you can
use to reduce the size of the binaries._

There are some low-hanging fruits that can be configured to drastically reduce binary sized in Rust
projects. Note that some of these have a cost, in that they lead to longer compile times (for
release builds). There are also some structural decisions that can lead to smaller binary sizes.

### Configuration

The simplest way to reduce code size is to set some configuration in the Cargo manifest.

```toml
[profile.release]
# Automatically strip symbols from the binary.
strip = true
opt-level = "z"  # Optimize for size.
# Enable link-time optimization
lto = true
```

### Dependencies

Sometimes, the binary size is caused by some dependencies that you are using. To analyze this,
[`cargo-bloat`][cargo-bloat] can be used, which measures the resulting binary and lists the amount
that each dependency contributes to the final binary size. In some cases, this can allow you to
investigate if the dependency could be replaced with a lighter one, or if there are any features
that could be disabled.

### Structural

Rust's use of generics means there is a lot of monomorphization.

_TODO: Explain monomorphization and boxed trait objects_

## Reading

```reading
style: article
title: Min Sized Rust
url: https://github.com/johnthagen/min-sized-rust)
author: John T. Hagen
---
This is a comprehensive guide to producing minimally sized binaries in Rust. It
starts with some low-hanging fruits and ends at building the standard library from
source to be able to do link-time optimization on it.
```

```reading
style: article
title: Thoughts on Rust bloat
url: https://raphlinus.github.io/rust/2019/08/21/rust-bloat.html
author: Raph Levien
archived: raphlinus-rust-bloat.pdf
---
Article discussing binary bloat in Rust and strategies that might help.
```

```reading
style: article
title: Build Configuration
url: https://nnethercote.github.io/perf-book/build-configuration.html
author: The Rust Performance Book
---
Comprehensive guide covering build configuration options for optimizing Rust performance, including compiler flags, profile settings, and build-time optimization techniques.
```

```reading
style: book
title: Type Sizes
url: https://nnethercote.github.io/perf-book/type-sizes.html
author: The Rust Performance Book
---
Explains how type sizes affect performance and memory usage in Rust, covering techniques for measuring and optimizing data structure layouts to reduce binary size and improve cache efficiency.
```

[cargo-bloat]: https://github.com/RazrFalcon/cargo-bloat
