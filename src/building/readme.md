# Building

Usually, building a Rust project is as simple as running the appropriate Cargo command,
and everything just works:

```
cargo build --release
```

However, doing builds on a larger scale can present with some more challenges. For example,
always building the same dependencies in CI can present some challenges. Some projects
want to provide builds for multiple architectures.

This chapter discusses some issues you might run into when building Rust code in
your project, and strategies for how you might solve that.

## Reading

[Tips For Faster Rust Compile Times](https://corrode.dev/blog/tips-for-faster-rust-compile-times/) by Matthias Endler

*Matthias goes through and extensive list of tips for getting faster Rust
compile times.  These include making sure your toolchain is up-to-date,
enabling the parallel compiler frontend, removing unused dependencies,
debugging dependency compile times, splitting large crates into smaller ones,
optimizing workspaces, compilation caching, and many more.*

[Fast Rust Builds](https://matklad.github.io/2021/09/04/fast-rust-builds.html) by Alex Kladov

*Alex explains some strategies to speed up Rust compilation. He explains that
the Rust programming language has prioritized execution speed and programmer
productivity over compilation speed. He gives recommendation for how to setup
your CI pipeline, pruning dependencies, what code styles lead to faster
compilation times.*

[Stupidly effective ways to optimize Rust compile time](https://xxchan.me/cs/2023/02/17/optimize-rust-comptime-en.html) by Tianxiao Shen

[What part of Rust compilation is the bottleneck?](https://kobzol.github.io/rust/rustc/2024/03/15/rustc-what-takes-so-long.html)
