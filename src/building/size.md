# Binary Size

*One of your customers deploys the Rust-based tool that you are developing on
very small embedded systems which run in an industrial environment. They are
severely resource-constrained, making Rust an ideal language to target them.
However, the customer has started complaining that the binaries you are shipping
him are getting quite large. You are wondering if there are some strategies
you can use to reduce the size of the binaries.*

There are some low-hanging fruits that can be configured to drastically
reduce binary sized in Rust projects. Note that some of these have a cost,
in that they lead to longer compile times (for release builds). There are also
some structural decisions that can lead to smaller binary sizes.

### Configuration

The simplest way to reduce code size is to set some configuration in the
Cargo manifest.

```toml
[profile.release]
# Automatically strip symbols from the binary.
strip = true
opt-level = "z"  # Optimize for size.
# Enable link-time optimization
lto = true
```

### Dependencies

Sometimes, the binary size is caused by some dependencies that you are using.
To analyze this, [`cargo-bloat`][cargo-bloat] can be used, which measures the
resulting binary and lists the amount that each dependency contributes to the
final binary size. In some cases, this can allow you to investigate if the
dependency could be replaced with a lighter one, or if there are any features
that could be disabled.

### Structural

Rust's use of generics means there is a lot of monomorphization. 

*TODO: Explain monomorphization and boxed trait objects*


## Reading

[Min Sized Rust](https://github.com/johnthagen/min-sized-rust) by John T. Hagen

*This is a comprehensive guide to producing minimally sized binaries in Rust. It
starts with some low-hanging fruits and ends at building the standard library from
source to be able to do link-time optimization on it.*

[Thoughts on Rust bloat](https://raphlinus.github.io/rust/2019/08/21/rust-bloat.html) by Raph Levien

*Article discussing binary bloat in Rust and strategies that might help.*

[cargo-bloat]: https://github.com/RazrFalcon/cargo-bloat

