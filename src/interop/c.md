# C

Interfacing with C code is commonly necessary in order to use C libraries.
Many interpreters for programming languages, compression libraries, database
connection libraries expose a C API that they expect you to use.

Similarly, exposing a Rust library as a C API is commonly done to make it
easy for other tools to use it. The C ABI is often the lowest common
denominator for interfacing with different languages and tools.

### Typical architecture

When you interact with native libraries, there is a common pattern of building
it with two crates: in one crate, you have raw bindings for the C API to unsafe
Rust, and in a separate library you write a safe Rust wrapper.

## bindgen

Bindgen is a tool that makes it easy for you to create Rust bindings to a C
library. It works by parsing a C header, and emitting matching Rust bindings
for it, mapping the C types to appropriate Rust types. This way, you get a Rust
module that has contains every function and data type that the C header defines,
as `unsafe` Rust functions.

### How it works


### Tools you might need

You need a C library to link with in the first place. You have a choice of
either trying to rely on the operating system (or Nix) for getting it for you,
or you can compile the C library yourself from source. There are some crates
that you can use for this:

- `pkg-config` is a crate that lets you use pkg-config to find the library, if
  it is installed on the machine.
- `cc` is a crate that you can use to compile C sources with whichever compiler
  is on the system.
- `cmake` is a crate you can use to compile C sources that use CMake.

This compilation usually happens in the `build.rs` script that you have. A
common pattern is to try to find the library on the system, but also having a
feature that can be enabled that will build it from source using vendored
sources.

### How to use it

### Example: rusqlite

[bindgen]: https://github.com/rust-lang/rust-bindgen

## cbindgen

cbindgen is, in some ways, the opposite tool to bindgen. It allows you to export
a Rust API into a C or C++ API. In this section, we will only discuss its abilities
to export a C++ binding, for the C++ part see the [C++ section](./cpp.md).

### How to use it


### Example: tquic

[cbindgen]: https://github.com/mozilla/cbindgen

## Examples

- rusqlite

## Reading

~~~reading
style: article
title: How to create a C binding to a Rust library
url: https://developers.redhat.com/articles/2022/09/05/how-create-c-binding-rust-library#memory_leak_test_for_the_c_binding
author: Gris Ge
---
~~~

~~~reading
style: article
title: "Rust FFI and bindgen: Integrating embedded C code in Rust"
url: https://blog.theembeddedrustacean.com/rust-ffi-and-bindgen-integrating-embedded-c-code-in-rust
author: Omar Hiari
---
~~~

~~~reading
style: book
title: The bindgen User Guide
url: https://rust-lang.github.io/rust-bindgen/
author: Rust Project
---
The bindgen User Guide shows how to set up bindgen, and how to use it in a Rust
project.
~~~

~~~reading
style: article
title: cbindgen User Guide
url: https://github.com/mozilla/cbindgen/blob/master/docs.md
author: Mozilla
---
The cbindgen User Guide shows how to set up cbindgen, and how to use it in a Rust
project.
~~~

~~~reading
style: book
title: Foreign Function Interface
url: https://doc.rust-lang.org/nomicon/ffi.html
author: Rust Project
---
In this chapter of the Rust Nomicon, Foreign Function Interfaces are explained.
The chapter outlines how Rust can bind with other languages, such as C, and
gives some examples.
~~~

~~~reading
style: article
title: Rust to C - FFI Guide
url: https://github.com/Quin-Darcy/rust-c-ffi-guide
author: Quin Darcy
---
Quin shows how to call C code from Rust in this example repository. He has set up
an example whereby some C library is called from Rust, and walks through how it
works.
~~~
