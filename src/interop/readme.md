# Interfacing

There are various reasons why you may want to interoperate with another
language in a Rust project. Often times, code is not written in a vacuum,
but rather the code you write needs to interact with an existing system.
Or you need to make use of a library writte nin another language.
The reverse could also be true: maybe you did write something useful
in Rust, and you want to make it usable for people in another language.

Mixing Rust and other languages is often dangerous territory. The Rust
language (and by that, the Rust compiler) can keep guarantees about your
code, such as ensuring that you do not keep references around for longer
than they are alive (through the lifetime system). When you send values
across to another language, you need to take good care that the invariants
that the Rust compiler enforces are also upheld on the other side.

https://www.hobofan.com/rust-interop/

https://blog.pnkfx.org/blog/2022/05/12/linking-rust-crates/

https://github.com/mozilla/uniffi-rs

https://github.com/rust-diplomat/diplomat/


## C and C++

https://github.com/mozilla/cbindgen

https://cxx.rs/

https://github.com/rust-lang/rust-bindgen

## Java

https://github.com/jni-rs/jni-rs

## Dart and Flutter

https://pub.dev/packages/flutter_rust_bridge

## Erlang

https://github.com/rusterlium/rustler

## Python

https://pyo3.rs/v0.25.1/

https://github.com/PyO3/maturin

## JavaScript

https://github.com/rustwasm/wasm-bindgen

https://docs.rs/js-sys/latest/js_sys/

https://docs.rs/web-sys/latest/web_sys/


## Reading

~~~reading
style: book
title: Nomicon
url: https://doc.rust-lang.org/stable/nomicon/
author: Rust Language
---
~~~
