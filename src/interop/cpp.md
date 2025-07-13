# C++

https://google.github.io/autocxx/

https://cxx.rs/

https://github.com/pcwalton/cxx-async

## Reading

```reading
style: article
title: How to Rewrite a C++ Codebase Successfully
author: Philippe Gaultier
url: https://gaultier.github.io/blog/how_to_rewrite_a_cpp_codebase_successfully.html
archived: "gaultier-rewrite-cpp-codebase.pdf"
---
Phillipe explains how he got to inherit a legacy C++ codebase that used
in production, but is not in a good state. He explains his thinking process
and what lead him to make the decision to do an incremental rewrite in Rust.
The project in question is a library that is used in a lot of places, from
mobile (Android and iOS) to embedded (ARM microcontrollers) to the backend.

In the process, he made use of some useful techniques such as fuzzing, and used
various Rust FFI tooling to make the rewrite easier, as he was incrementally
porting functionality from the legacy codebase to Rust.

He also explains how he got cross-compilation working for the different targets,
resorting to using the Zig compiler to cross-compile for iOS.
```
