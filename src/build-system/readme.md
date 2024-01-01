# Build system

*You are implementing a backend service in Rust which offers an API.  At some
point you realize that you need a frontend to configure it properly.
Helpfully, one of your colleagues implements a frontend for you in React.  You
notice that it would be convenient if the backend would serve the files of the
frontend, and you are looking for some way to tell Cargo to build and embed the
frontend files into the backend's binary.  How can you achieve this?*

## Solution

Build systems orchestrate the build process. They track tasks and dependencies, and
make sure that the tasks are run in the right order, rerun when inputs have changed.
Often times, they also enforce [hygiene by sandboxing build steps][sandboxing]
to make sure you do not accidentally depend on inputs you have not declared.

```admonish
Usually, this is not really something you need to think of when you are working
in a Rust project, because you have [Cargo][cargo] acting as an excellent build
system.
```

However, build systems become interesting to your Rust project when one of
three things happen:

- Inside your project, you have **multi-language components**. For example, a frontend
  written in TypeScript, a backend component written in Kotlin, a C library,
  some Python tooling.
- Inside your project, you have **cross-target dependencies**. For example, you
  have a project fully written in Rust, and the backend wants to embed the
  frontend compiled to WebAssembly using a tool such as `trunk` for ease of
  deployment.
- You depend on some **external dependency** which is not written in Rust, and you
  want to be sure you can use reproducibly it on all platforms. For example,
  you depend on the presence of `sqlite` in a specific version.

> ToDo: graph here

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
[sandboxing]: https://bazel.build/docs/sandboxing
