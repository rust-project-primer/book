# Interfacing

There are various reasons why you may want to interoperate with another
language in a Rust project. Often times, code is not written in a vacuum,
but rather the code you write needs to interact with an existing system.
Or you need to make use of a library writte nin another language.
The reverse could also be true: maybe you did write something useful
in Rust, and you want to make it usable for people in another language.

### Reasons for interop

Sometimes, you want to be able to use a Rust library in other languages.
This could be because you've written something that is performance-sensitive
in Rust, and the application you want to embed it into is written in a higher-level,
but simpler language. Or the Rust library you have written focusses on correctness.
Some examples are:

- [`rustls` is used by Curl](https://github.com/curl/curl/blob/master/docs/RUSTLS.md)
- [`polars`][polars] is a popular data frame library that can be easily consumed in Python

Other times, you want to use some existing (typically native) library in your
Rust project. You may want to do this because there is no Rust library to do
what it does, or because it is faster/more complete than native Rust alternatives.

- SQLite is commonly used in Rust as `rusqlite`
- several compression libraries have Rust wrappers

Another reason you may need bindings is because your Rust code runs
embedded within some runtime that uses a different native language.

- Rust can run in the browser as a WebAssembly program, and needs to interact
  with JavaScript to access browser APIs.
- You can build an Android application with Rust, but you need to bind to the JVM
  to access native Android APIs.

Whatever your reason is for interoperating with a different language in Rust,
this chapter will give you the context you need to safely interact with foreign
languages, and show you the tools you need.

[polars]: https://pola.rs/

### Basics of language interop

In order for native code to call other native code, it generally needs two
pieces of information: the address of the function to call, and how to pass
arguments and receive return values. This knowledge is called the Application
Binary Interface (ABI).

Rust does not have a stable ABI. But for interfacing with other language, this
does not matter, it only matters if Rust interfaces with (other) Rust libraries
through dynamic linking. When talking to other native code, generally the
lowest common demoninator is the C ABI.

Implementing interop with other native languages therefore typically involves
*squeezing* types and function calls through some kind of C ABI. It means you
need wrapper functions that use the C ABI, and you somehow need to tell the
other language what they are called and how to find them. Similarly, to access
code from another language, Rust needs to be told what types there are, what
functions there are and where to find them. A lot of the tools in this section
help automate this process so you don't have to write and maintain these
bindings by hand.

Another case is when you use languages that don't run natively, but use an
interpreter. In this case, your Rust types and functions need to be registered
with the interpreter, so that it can call into them.

### Interopability of Async code

If you want to call asynchronous functions across a language boundary, there is
some more work to do. Asynchronous functions return futures, and in the case of
Rust these need to be polled and need to have an active asynchronous runtime. 

Sometimes, there are ways to glue a different language's asynchronous runtime
with Rust, and you can easily exchange futures and poll them across the
language boundary. Other times, you may need to write wrapper types, or convert
your async methods into synchronous ones, that spawn the work off into a
background thread. Again, some glue frameworks can handle this for you, but it
depends on the language that you are interacting with.  Some languages just
don't have support for asynchronous programming, or their models are too
different from Rust's to be compatible.

### Dangers of interoperating with other languages

Mixing Rust and other languages is often dangerous territory. The Rust
language (and by that, the Rust compiler) can keep guarantees about your
code, such as ensuring that you do not keep references around for longer
than they are alive (through the lifetime system). When you send values
across to another language, you need to take good care that the invariants
that the Rust compiler enforces are also upheld on the other side.

You need to think about:

- **Ownership**: when you pass types through the language boundary, does the
  other language take ownership?
- **Thread-safety**: are you able to use types from multiple threads?  Will the
  other language allow using your Rust types from multiple threads?
- **Copying and Cloning**: how can you clone or copy types from the other
  language? How will the other language copy or clone your Rust types?
- **Error handing**: what facilities does the other language have for
  expressing errors? For example, if the other language uses exceptions, how
  will you catch those and represent them as Rust errors when you call into its
  code?
- **Memory management**: does the other language do manual memory management?
  Does it have a garbage collector? How can you make sure that references to
  types that you receive are cleaned up?
- **Mapping types**: how can you map the other language's types into native
  Rust types? For example, how can you map strings from the other language into
  Rust strings? Does the other language enforce that strings are UTF-8 encoded?

In some cases, the tool can handle a lot of this for you automatically, or even
enforce that Rust constraints are properly expressed in the other language.
But other times, you need to handle these yourself.

You also have to think about the execution model of the target language.  Some
languages run single-threaded, and you have to make sure that you don't call
into the language from a different thread. 

### Patterns for Rust interop

- the `-sys` pattern: allowing other crates to access the raw C api
- using features to expose the api
- using a `build.rs` script

### How this chapter is structured

In this chapter, we will walk through several tools that can be used to
interoperate with different languages and Rust. Sometimes, this
interoperability is two-sided: you can use it both to call from Rust into the
other language, and to call from the other language into Rust.

First, we will walk through some tools that help with interfacing with multiple
languages. In the sections of this chapter, we will talk though approaches for
specific languages.

## UniFFI

[UniFFI](https://mozilla.github.io/uniffi-rs/latest/) is a Mozilla project that
aims to make it easy to interface with other languages from Rust. It supports
generating bindings for Kotlin, Swift, Python and WebAssembly. There is
third-party support for generating bindings to JavaScript, Kotlin
Multiplatform, Go, C#, Dart and Java, but these are not officially supported.
It also has support for generating bindings for asynchronous code for languages
that support it, for example Python and Kotlin.

It is used in production by Mozilla, making it an interesting project to use,
becuase it means it comes with some amount of stability.

## Interoptopus

[Interoptopus](https://github.com/ralfbiedert/interoptopus) is another tool for
generating cross-language bindings. It supports C#, C and Python, but promises
that it is easy to add support for more languages.

## Diplomat

[Diplomat](https://rust-diplomat.github.io/diplomat/) is a tool for generating
bindings for Rust for other languages. It supports C, C++, Dart,
JavaScript/TypeScript, Kotlin and Python.

## Language-Specific Interop

The other sections in this chapter discuss specific tools which can be used
to interoperate with other languages. Depending on your use-case, these might
be a better fit, because they are more tailor-made to the language that they
are covering.

## Reading

~~~reading
style: article
title: Rust Language Interop
url: https://www.hobofan.com/rust-interop/
author: Maximilian Goisser
---
Maximilian gives an overview of tools that allow you to interface
Rust with various languages.
~~~

~~~reading
style: book
title: Nomicon
url: https://doc.rust-lang.org/stable/nomicon/
author: Rust Language
---
~~~

~~~reading
style: article
title: Linking Rust crates
url: https://blog.pnkfx.org/blog/2022/05/12/linking-rust-crates/
author: Felix S. Klock II
archived: pnkfx-linking-rust-crates.pdf
---
~~~

~~~reading
style: article
title: Binding Rust to other languages safely and productively
url: https://stepfunc.io/blog/bindings/
author: Émile Grégoire
archived: stepfunc-bindings.pdf
---
~~~

https://viruta.org/rust-stable-abi.html

https://blaz.is/blog/post/we-dont-need-a-stable-abi/

https://doc.rust-lang.org/reference/abi.html

