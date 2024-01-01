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
