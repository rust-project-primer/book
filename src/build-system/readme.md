# Build system

*You are implementing a backend service in Rust which offers an API.  At some
point you realize that you need a frontend to configure it properly.
Helpfully, one of your colleagues implements a frontend for you in React.  You
notice that it would be convenient if the backend would serve the files of the
frontend, and you are looking for some way to tell Cargo to build and embed the
frontend files into the backend's binary.  How can you achieve this?*

With Cargo, Rust has some fanstastic tooling for building, cross-compiling
and testing Rust software. Cargo supports installing plugins that extend it's
functionality, a lot of which are discussed in this book. If your Rust
project has a relatively simple setup, where it consists only of Rust crates,
then Cargo is the ideal tool to get it to build:

```mermaid
graph LR
    lib_a_lib-->lib_a
    lib_b_lib-->lib_b
    lib_a-->bin
    lib_b-->bin
    bin_main-->bin
```

Things start to get tricky when you involve other languages (such as mixing
Rust with C, C++, TypeScript) or when the build involves building code for
different targets (for example, that some crates need to be built as WebAssembly
and the resulting code is needed by other build.

### Example architectures

For example, some projects may need to interface with some legacy C/C++ code.
In this case, building might involve compiling the library first:

```mermaid
graph LR
    a-->b
```

Another common pattern when building full-stack web applications with Rust
is that you might write the frontend in Rust and need to compile it to WebAssembly,
and the backend in Rust. You want the Rust backend to serve the frontend, so
it requires the WebAssembly output as a build input:

```mermaid
graph LR
    a-->b
```

If you build a traditional web application with a TypeScript frontend and a
Rust backend, you may need to run a TypeScript compiler for some part of your
code and use the output as the input for your backend.

```mermaid
graph LR
    a-->b
```

Other configurations are also possible, it depends on your particular need.

## Build Systems

Build systems are high-level tools to orchestrate the build process. They track
tasks and dependencies, and make sure that the build steps are run in the right
order and rerun when any of the inputs have changed. 

Good build systems will enforce [hygiene by sandboxing build steps][sandboxing]
to make sure you do not accidentally depend on inputs you have not declared.
This helps to avoid the "it works on my machine" syndrome, where your code
accidentally depends on some system state that is present on your machine but
not on other's.

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

Many build systems also offer fully reproducible builds by requiring all build
inputs and tools to be pinned down by hash, which enables distributed caching
which is a big quality of life improvement for developers as it leads to faster
development times.

This chapter discusses some build systems that play nice with Rust.  Note that
build systems are not necessarily mutually-exclusive: most of the time, even
when using a build system that is not Cargo, you will still have the necessary
Cargo manifests in the project that allows standard Cargo tooling to work.

## Reading

[Multi-language build system options](https://cxx.rs/building.html)

*TODO*

[Build systems a la carte]()

*Paper which explain build systems, and how they work.*

[cargo]: https://doc.rust-lang.org/cargo/
[sandboxing]: https://bazel.build/docs/sandboxing
