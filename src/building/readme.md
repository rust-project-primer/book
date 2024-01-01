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


https://endler.dev/2020/rust-compile-times/


https://matklad.github.io/2021/09/04/fast-rust-builds.html



https://xxchan.me/cs/2023/02/17/optimize-rust-comptime-en.html
