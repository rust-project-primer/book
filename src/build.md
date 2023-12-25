# Build system

Build systems orchestrate the build process. They track tasks and dependencies, and
make sure that the tasks are run in the right order, rerun when inputs have changed.
Usually, this is not really something you need to think of when you are working in a
Rust project, because you have [Cargo][cargo]

> ToDo: graph here

However, build systems become interesting to your Rust project when one of two things
happen:

- Inside your project, you have multi-language parts. For example, a frontend
  written in TypeScript, a backend component written in Kotlin, a C library,
  some Python tooling.
- Inside your project, you have cross-target dependencies. For example, you
  have a project fully written in Rust, and the backend wants to embed the
  frontend compiled to WebAssembly using a tool such as `trunk` for ease of
  deployment.
- You depend on some external dependency which is not written in Rust, and you
  want to be sure you can use reproducibly it on all platforms. For example,
  you depend on the presence of `sqlite` in a specific version.

Many build systems also offer fully reproducible builds by requiring all build
inputs and tools to be pinned down by hash, which enables distributed caching
which is a big quality of life improvement for developers as it leads to faster
development times.

This chapter discusses some build systems that play nice with Rust.  Note that
build systems are not necessarily mutually-exclusive: most of the time, even
when using a build system that is not Cargo, you will still have the necessary
Cargo manifests in the project that allows standard Cargo tooling to work.

## Reading

- [Multi-language build system options](https://cxx.rs/building.html)

[cargo]: https://doc.rust-lang.org/cargo/
