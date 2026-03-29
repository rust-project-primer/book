# Binary Size

When you compile Rust code, you have some control over the compiler as to what
it prioritizes when building your executables. Everything is a tradeoff, so when
you prioritize one aspect, you might see a regression in another aspect. Common
priorities are:

- **Speed**: You want your executables to run as fast as possible. This might
  lead to an increase in code size, because the compiler will use techniques
  like inlining or loop unrolling to achieve this.
- **Binary size**: You want your executables to be as small as possible, for
  example because you are targeting a resource-constrained platform like
  embedded microcontrollers with limited flash memory sizes, or you want to be
  able to easily distribute your binary. This might lead to a negative impact on
  performance.

### Compilation Profiles

In general, the way you exercise control over this is by creating _profiles_.
Every profile comes with a set of parameters that let you tweak how the compiler
performs. Typically, when you make debug builds, your main priority is fast
compilation times, so you are happy to sacrifice some runtime speed.

A profile definition looks like this:

```toml
[profile.release]
strip = true
opt-level = 3
```

## Runtime Speed

Optimizing for runtime speed is covered in detail in the
[Performance](./performance.md) chapter. In short, the main levers are: enabling
link-time optimizations (`lto = "full"`), reducing codegen units
(`codegen-units = 1`) so the optimizer can see more code at once, enabling
target-specific CPU features (like AVX2), and using profile-guided optimization
(PGO) to let the compiler make better decisions based on real workload data.

These optimizations tend to increase binary size and compile time. If you need
both speed and small binaries, you will need to find a balance that works for
your use case.

## Binary Size

There are some low-hanging fruits that can be configured to drastically reduce
binary size in Rust projects. Note that some of these have a cost, in that they
lead to longer compile times (for release builds). There are also some
structural decisions that can lead to smaller binary sizes.

### Configuration

The simplest way to reduce binary size is to set some options in the Cargo
profile:

```toml framed title="Cargo.toml"
[profile.release]
# Automatically strip symbols from the binary.
strip = true
# Optimize for size rather than speed.
opt-level = "z"
# Enable link-time optimization so the linker can remove unused code.
lto = true
# Use a single codegen unit so the optimizer can see all code at once.
codegen-units = 1
```

Each of these has a different effect. Stripping removes symbol names and debug
information from the final binary, which doesn't affect functionality at all but
can significantly reduce size. The `opt-level = "z"` flag tells the compiler to
prioritize size over speed in its optimization passes. Link-time optimization
allows the linker to perform whole-program analysis, removing dead code that
wouldn't be caught when crates are compiled individually. Reducing codegen units
to 1 gives the optimizer a broader view of the code, which helps with both dead
code elimination and inlining decisions.

```admonish note
The `opt-level = "z"` and `opt-level = "s"` options both optimize for size. The
difference is that `"z"` is more aggressive: it will disable loop vectorization
and make other tradeoffs that `"s"` won't. In practice, `"z"` produces smaller
binaries but may be noticeably slower for compute-heavy workloads. Start with
`"s"` and switch to `"z"` if you need to squeeze out more.
```

### Dependencies

Sometimes, the binary size is caused by some dependencies that you are using. To
analyze this, [`cargo-bloat`][cargo-bloat] can be used, which measures the
resulting binary and lists the amount that each dependency contributes to the
final binary size. In some cases, this can allow you to investigate if the
dependency could be replaced with a lighter one, or if there are any features
that could be disabled.

You can install and run it like this:

```bash
cargo install cargo-bloat
cargo bloat --release -n 10
```

This will show you the 10 largest functions in your binary, along with which
crate they come from. You can also use `--crates` to get a per-crate breakdown:

```bash
cargo bloat --release --crates
```

This is often more actionable: if a single dependency accounts for a large
fraction of your binary, you can investigate whether you actually need all of
its features, or whether a lighter alternative exists.

### Monomorphization

Rust generics are compiled through _monomorphization_: every time you use a
generic function or type with a concrete type parameter, the compiler generates
a specialized copy of the code for that specific type. This is what makes Rust
generics zero-cost at runtime, but it comes at a cost in binary size.

For example, consider a function like this:

```rust
fn process<T: Display>(items: &[T]) {
    for item in items {
        println!("{item}");
    }
}
```

If your code calls `process::<String>()`, `process::<i32>()`, and
`process::<PathBuf>()`, the compiler will generate three separate copies of the
function body. For small functions this is negligible, but for large generic
functions called with many different types, the duplicated code can add up.

One common strategy to reduce this is to factor out the type-independent parts
of a generic function into a non-generic inner function. This is sometimes
called the "outline" pattern:

```rust
fn process<T: Display>(items: &[T]) {
    // Only the formatting is generic
    let strings: Vec<String> = items.iter().map(|i| i.to_string()).collect();
    process_inner(&strings);
}

fn process_inner(items: &[String]) {
    for item in items {
        println!("{item}");
    }
}
```

Now only the thin conversion wrapper gets monomorphized for each type, while the
bulk of the work lives in a single copy of `process_inner`.

This pattern is common enough that the [`momo`][momo] crate automates it with a
procedural macro. It works for function parameters that use the `Into`, `AsRef`,
or `AsMut` traits. Instead of manually writing a wrapper and an inner function,
you annotate your function and `momo` generates the split for you:

```rust
use momo::momo;

#[momo]
fn read_file(path: impl Into<PathBuf>) -> std::io::Result<String> {
    // This body is only compiled once, with a concrete PathBuf.
    // momo generates a generic wrapper that calls .into() and
    // forwards to this inner function.
    std::fs::read_to_string(path)
}
```

This is particularly useful for public API functions that accept
`impl Into<String>` or `impl AsRef<Path>`, which are convenient for callers but
would otherwise generate a separate copy for every call site that passes a
different type.

### Trait objects

Another approach is to use trait objects (`dyn Trait`) instead of generics in
places where the performance cost of dynamic dispatch is acceptable. Instead of
generating a specialized copy for each type, a trait object uses a vtable for
method dispatch at runtime, meaning only one copy of the code exists in the
binary:

```rust
fn process(items: &[&dyn Display]) {
    for item in items {
        println!("{item}");
    }
}
```

This trades a small amount of runtime performance (one pointer indirection per
method call) for a reduction in binary size. For hot loops this may not be
worthwhile, but for code that isn't performance-critical (logging,
configuration, error formatting) it's a reasonable tradeoff.

The standard library itself uses this technique internally. For example,
`std::fmt` uses trait objects to avoid monomorphizing the formatting machinery
for every type that implements `Display`.

## Reading

```reading
style: article
title: Min Sized Rust
url: https://github.com/johnthagen/min-sized-rust
archived: min-sized-rust.pdf
author: John T. Hagen
---
This is a comprehensive guide to producing minimally sized binaries in Rust. It
starts with some low-hanging fruits and ends at building the standard library
from source to be able to do link-time optimization on it.
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
Comprehensive guide covering build configuration options for optimizing Rust
performance, including compiler flags, profile settings, and build-time
optimization techniques.
```

```reading
style: book
title: Type Sizes
url: https://nnethercote.github.io/perf-book/type-sizes.html
author: The Rust Performance Book
---
Explains how type sizes affect performance and memory usage in Rust, covering
techniques for measuring and optimizing data structure layouts to reduce binary
size and improve cache efficiency.
```

[cargo-bloat]: https://github.com/RazrFalcon/cargo-bloat
[momo]: https://docs.rs/momo
