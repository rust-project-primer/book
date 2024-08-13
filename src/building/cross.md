# Cross-Compiling

Thanks to Rust's use of LLVM, it comes with good support for cross-compiling built-in.
If you use Rustup, you can easily add other targets:

    rustup target add wasm32-unknown-unknown

It gets a bit more tricky when you want to link your binary, or when you want
to link with native libraries, because you need versions of them for the target
architecture.  But most of the hard work with cross-compiling is already
handled by Cargo, rustc and LLVM.

## Debian

If you use Debian, or a derivative, you can get cross-compilation working relatively
easily by installing a suitable cross compiler. You can even install QEMU to be able
to run (emulate) the resulting executables, to be able to run things like unit or
integration tests.

* todo: example

## [Cross](https://github.com/cross-rs/cross)

Cross advertises itself as a "zero-setup" tool for cross-compilation and
cross-testing of Rust crates. Under the hood, it uses Docker containers to run
the compilation steps with the right toolchains and libraries preinstalled.

The idea with it is that it acts as a replacement for Cargo.

## Reading

[Guide to cross-compilation in Rust](https://blog.logrocket.com/guide-cross-compilation-rust/) by Greg Stoll

*In this article, Greg explains how to cross-compile Rust crates using the cross project.*

https://actually.fyi/posts/zig-makes-rust-cross-compilation-just-work/
