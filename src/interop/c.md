# C

The C language is a widely used, general-purpose programming language that is known for its
efficiency and portability, but also for its lack of memory safety, which is the cause of many
serious vulnerabilities. It is often used as a foundation for other programming languages and is a
common target for interoperability. Many operating systems are implemented in it, which often makes
it the lowest common denominator for interop with other ecosystems.

Interfacing with C code is commonly necessary in order to use C libraries. Many interpreters for
programming languages, compression libraries, database connection libraries expose a C API that they
expect you to use. The advantage of C is that is does not need a runtime, and many libraries are
written in a way to have few dependencies to be portable, making it easy to embed C libraries in
your Rust project. There is a large ecosystem of C libraries that can be used in Rust projects.

Going the other way, if you want your Rust libraries to be used by other projects that are not
Rust-based, the easiest way to achieve this is often by exposing a C API that can be used by other
languages.

Rust has built-in support for interfacing with C APIs using the `extern "C"` keyword. Rust can
interface with C APIs without runtime overhead. However, there is also good tooling that can help
you generate C bindings for Rust libraries, and maintain them over time.

The C ABI is a widely supported standard across programming languages. Rust can interface with it
without runtime overhead, thanks to its zero-cost abstraction principle.

### When to use C interop

Interoperating with C libraries is a risk factor, because they are not guaranteed to be thread-safe,
and may have undefined behavior in certain situations. If you can stick to only using native Rust
libraries, you should do so. However, there are situations where you do not have a choice, and that
is when doing interop is appropriate.

C interop is necessary in several scenarios:

- Using existing C libraries (compression algorithms, database drivers, etc.). In many cases, there
  are native Rust alternatives to popular C libraries that you should consider using instead, if
  they work for your use-case.
- Accessing operating system interfaces that expose C APIs, unless they have already been wrapped in
  a safe Rust API, such as `libc` or the `winapi` crates.
- Exposing Rust code to other languages through C as a common interface. You should check if some
  higher-level FFI tools work for your use-case, because they can help you preserve Rust's safety
  guarantees in some cases.
- Integrating Rust components inside C-based projects

If you do decide to use C interop, you should make sure to have good unit-tests to ensure that your
bindings are correct and safe. You should consider using tooling like
[Dynamic Analysis](../testing/dynamic-analysis.md) to check that your bindings do not violate memory
safety or introduce undefined behavior.

## Binding to C libraries

The Rust ecosystem follows a consistent pattern for C interop with a two-crate structure. Usually,
if you wrap a native C library named `libfoo.so`, you will create two Rust crates: `foo-sys`, which
exports the raw (unsafe) bindings for the library, and `foo`, which provides a safe wrapper around
those bindings. The `foo-sys` crate is also called the
[`-sys` package](https://doc.rust-lang.org/cargo/reference/build-scripts.html#-sys-packages).

<center>

![C Library Bindings](c-library-ffi.svg)

</center>

There are two reasons for doing this: it provides a clear separation between the unsafe and safe
interfaces. But more importantly, it allows other crates to directly access the raw unsafe bindings
if they need to. In Rust [only a single crate can link to a specific native library][cargo-links]
(in other words, you cannot have two crates that independently link with `libfoo.so`). So having
separate crates for this allows other crates to link with and directly access the unsafe bindings if
they need to, bypassing the safe interface.

<!-- example: manually written bindings -->

### Examples

**Examples**: `rusqlite`/`libsqlite3-sys`, `openssl`/`openssl-sys`, `flate2`/`libz-sys`

[cargo-links]: https://doc.rust-lang.org/cargo/reference/build-scripts.html#the-links-manifest-key

## Exchanging Data between Rust and C

### Data Types

When working with C interop, you have to keep in mind how C types map to Rust types (and vice
versa), and what the ownership, lifecycle and mutability constraints are. For many C types, the
`std::ffi` module and the `libc` crate provide safe abstractions over the raw C types that allow
them to be converted into native Rust types.

| C Type          | Rust Type                 | Notes                               |
| --------------- | ------------------------- | ----------------------------------- |
| `int`           | `i32`, `c_int`            | Integer, size is platform dependent |
| `char*`         | `*const c_char`, `CStr`   | Raw pointer (unsafe)                |
| `struct foo`    | `#[repr(C)]` struct       | Field-by-field mapping              |
| `void*`         | `*mut c_void`             | Type-erased pointer                 |
| `T(func*)(...)` | `extern "C" fn(...) -> T` | Function pointer                    |
| `char[N]`       | `[c_char; N]`             | Fixed-size array                    |
| `size_t`        | `usize`/`size_t`          | Platform-dependent size             |
| `bool`          | `bool`/`c_bool`           | C99 `_Bool` or custom               |

It's important to note that C strings (`char *`) don't map directly to Rust's `String`/`&str` type.
C strings are null-terminated char arrays, and are not necessarily UTF-8 encoded. Rust provides
utilities in the `std::ffi` module like `CString` and `CStr` to safely convert between C strings and
Rust strings.

### Memory Management

Memory management is something you have to watch out for. When data crosses the boundary:

- Memory allocated in Rust and passed to C must either be 'static or kept alive for the duration of
  C's usage
- Memory allocated in C and passed to Rust must be explicitly freed (typically by the side that
  allocated it)
- Ownership transfer must be clearly documented and handled correctly

## Exporting C Libraries

If you export C bindings to your Rust library, you will typically do one of two things:

- Create a separate crate for the exported bindings. This is what `rustls` does. This has the
  advantage of decoupling the Rust library from the FFI bindings, and lets you remove tooling and
  configuration necessary for generating the C library from your main Rust library.
- Create a feature flag in your library crate which enables the generation of C bindings.

To tell Cargo to export a C-compatible library, you need to specify the `crate-type` field in your
`Cargo.toml` file. Here, `staticlib` is used to generate a static library (`libfoo.a`), and `lib` is
used to generate a dynamic library (`libfoo.so`).

```toml
[lib]
name = "foo"
crate-type = ["lib", "staticlib"]
```

For example, the [`rustls`][rustls] crate exports its C bindings in the [`rustls-ffi`][rustls-ffi]
crate.

[rustls-ffi]: https://github.com/rustls/rustls-ffi
[rustls]: https://github.com/rustls/rustls

## bindgen

Bindgen generates Rust FFI bindings from C header files. It parses C headers and produceThis process
is typically managed through Cargo's build script system, with `build.rs` handling the generation of
bindings and configuration of the build environment.s matching Rust code with appropriate type
mappings.

It does not create a safe wrapper around the raw FFI bindings, but it allows you to keep the raw
bindings in sync with the C API by automatically generating them from the header, rather than having
to manually write and maintain them.

### How it works

Bindgen uses Clang to parse C/C++ headers and generates unsafe Rust bindings for them. It will
automatically map C primitive types to the appropriate Rust equivalents, for example `int` to `i32`,
`char` to `c_char`, or `char *` to `CStr`. It will convert C structs to Rust structs with
`#[repr(C)]` to preserve memory layout. It will translate enums with proper discriminant values, and
unions with proper layout and alignment. It finally generates raw `unsafe extern "C"` function
declarations that you can call from (unsafe) Rust.

Bindgen is typically integrated into your project's `build.rs` script. You can configure it to only
include specific types or functions (if you only want to expose a subset of the API). You can apply
custom attributes to generated types. It can handle opaque types, and rename symbols for a better
Rust integration

### Tools for C integration

Several crates help with C library integration:

- `pkg-config`: Finds system-installed libraries
- `cc`: Compiles C sources with system compiler
- `cmake`: Builds libraries that use the CMake build system

A common pattern in `build.rs` scripts is to try finding the library on the system first, with a
fallback feature to build from vendored sources.

<!-- code example: A build.rs script that first attempts to find a system library using pkg-config, then falls back to building from vendored sources -->

### Example: rusqlite

[rusqlite](https://github.com/rusqlite/rusqlite) demonstrates bindgen integration with SQLite:

- Uses `libsqlite3-sys` for raw bindings
- Provides both system linking and bundled SQLite options
- Converts C error codes to Rust Result types

<!-- code example: A simple example of using rusqlite with its safe interface, showing how C error codes are converted to Rust's Result type -->

[bindgen]: https://github.com/rust-lang/rust-bindgen

## cbindgen

cbindgen generates C (or C++) header files from Rust code, allowing you to expose Rust functions to
C. It creates the header files that describe your Rust API in terms C can understand.

To use cbindgen, you'll need to:

- Mark functions with `#[no_mangle]` and `pub extern "C"`
- Use `#[repr(C)]` for exported structs and enums
- Configure cbindgen through a build.rs script

cbindgen can handle Rust-specific types like `Option<T>` by generating appropriate C equivalents.
For example, an `Option<*mut T>` might be represented as a nullable pointer in C.

<!-- code example: A build.rs script that uses cbindgen to generate C headers for a Rust library -->

### Example: tquic

[tquic](https://github.com/Tencent/tquic) is a QUIC implementation that demonstrates how to expose
Rust code to C:

- Uses cbindgen to generate C headers
- Shows memory management patterns across the FFI boundary
- Designs an API that feels natural to C users

<!-- code example: A simple example of defining a Rust function with proper FFI attributes that is exposed to C -->

[cbindgen]: https://github.com/mozilla/cbindgen

## Notable C Binding Libraries

Several well-established Rust libraries demonstrate effective C interoperability:

- **rusqlite**: Bindings to SQLite
- **openssl-rs**: Rust interface to OpenSSL
- **sdl2-rs**: SDL2 graphics/audio library bindings
- **gtk-rs**: GTK and other GLib-based libraries
- **libc**: Low-level bindings to platform C libraries
- **winapi**: Windows API bindings

## cargo-c

[cargo-c][] is a cargo subcommand that makes building C bindings easier. It provides a simple way to
generate C headers and static libraries from Rust code. This tool automates the process of:

- Generating headers with cbindgen
- Building static and dynamic libraries
- Creating pkg-config files
- Installing the libraries and headers in the right location

It's particularly useful for distributing Rust libraries that need to be consumed by C/C++ projects
or other languages through their C FFI.

[cargo-c]: https://github.com/lu-zero/cargo-c

## Reading

```reading
style: article
title: How to create a C binding to a Rust library
url: https://developers.redhat.com/articles/2022/09/05/how-create-c-binding-rust-library#memory_leak_test_for_the_c_binding
author: Gris Ge
---
```

```reading
style: article
title: "Rust FFI and bindgen: Integrating embedded C code in Rust"
url: https://blog.theembeddedrustacean.com/rust-ffi-and-bindgen-integrating-embedded-c-code-in-rust
author: Omar Hiari
---
```

```reading
style: book
title: The bindgen User Guide
url: https://rust-lang.github.io/rust-bindgen/
author: Rust Project
---
The bindgen User Guide shows how to set up bindgen, and how to use it in a Rust
project.
```

```reading
style: article
title: cbindgen User Guide
url: https://github.com/mozilla/cbindgen/blob/master/docs.md
author: Mozilla
---
The cbindgen User Guide shows how to set up cbindgen, and how to use it in a Rust
project.
```

```reading
style: book
title: Foreign Function Interface
url: https://doc.rust-lang.org/nomicon/ffi.html
author: Rust Project
---
In this chapter of the Rust Nomicon, Foreign Function Interfaces are explained.
The chapter outlines how Rust can bind with other languages, such as C, and
gives some examples.
```

```reading
style: article
title: Rust to C - FFI Guide
url: https://github.com/Quin-Darcy/rust-c-ffi-guide
author: Quin Darcy
---
Quin shows how to call C code from Rust in this example repository. He has set up
an example whereby some C library is called from Rust, and walks through how it
works.
```
