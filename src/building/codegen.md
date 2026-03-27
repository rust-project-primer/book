# Codegen Backend

Rust uses LLVM as its default code generation backend. LLVM produces highly
optimized binaries and supports a wide range of targets, but it is designed to
produce _fast binaries_, not to _produce binaries fast_. For release builds this
is the right tradeoff. For development, where you care about iteration speed
more than runtime performance, LLVM's thoroughness becomes a cost.

The Rust compiler supports alternative codegen backends that make a different
tradeoff: faster compilation at the expense of less optimized output. For
development builds — where most of what you run is unit tests — this can
meaningfully improve the edit-compile-test cycle.

## Cranelift

[Cranelift][cranelift] is a compiler backend originally developed for the
[Wasmtime](https://wasmtime.dev/) WebAssembly runtime. The Rust compiler team
has adopted it as an [alternative codegen backend][rustc-cranelift]. Because
Cranelift focuses on generating code quickly rather than optimizing it
aggressively, it compiles faster than LLVM at the cost of producing slower
binaries.

To use Cranelift, install the preview component on a nightly toolchain:

```bash
rustup component add rustc-codegen-cranelift-preview --toolchain nightly
```

Then build with it:

```bash
CARGO_PROFILE_DEV_CODEGEN_BACKEND=cranelift cargo +nightly build -Zcodegen-backend
```

Cranelift currently requires a nightly toolchain. The speedup depends on the
project, but as a rough benchmark:

| Crate   | LLVM  | Cranelift | Speedup |
| ------- | ----- | --------- | ------- |
| ripgrep | 7.50s | 5.72s     | ~24%    |

The benefit is most noticeable for larger projects where LLVM's optimization
passes dominate compile time. For small crates, the difference may be negligible
because most time is spent in the frontend (parsing, type checking, borrow
checking) rather than code generation.

[cranelift]: https://cranelift.dev/
[rustc-cranelift]: https://github.com/rust-lang/rustc_codegen_cranelift

<!-- TODO: benchmark Cranelift with more crates (clap, ripgrep, regex). Update
     the table with results and note exact compiler/crate versions used for
     reproducibility. -->

## Reading

```reading
style: article
title: Cranelift code generation comes to Rust
url: https://lwn.net/Articles/964735/
author: Daroc Alden
archived: lwn-rust-cranelift.pdf
---
Covers the history of Cranelift (built for Wasmtime, adopted by the Rust
compiler team), how it differs from LLVM architecturally (single-pass vs
multi-pass optimization), and what it means for Rust developers. Explains that
Cranelift is not a replacement for LLVM — it targets development builds where
compilation speed matters more than runtime performance.
```
