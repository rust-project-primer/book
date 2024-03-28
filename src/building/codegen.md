# Codegen

Rust is built on top of the LLVM compiler, which means it can make use of the
optimizations and the wide range of backends. This is a good thing, Rust would
not be able to support as many targets as it does without the LLVM
underpinnings of its code generation.

However, LLVM is a rather heavy dependency. It is designed to produce fast
binaries, not to produce binaries fast. This is a good property when building
release binaries, however when you are actively developing on a project, you
tend to care about a fast iteration loop more than getting the best performance
out of your binaries (which, in this case, will mostly be unit tests).

Therefore, there is an advantage in switching to a different codegen backend
during development (non-release builds), if that leads to faster iterations.

## Cranelift

[Cranelift](https://cranelift.dev/) started out as a library to help in
implementing the JIT-based WebAssembly runtime
[wasmtime](https://wasmtime.dev/). However, due to the way in which it was
built, it can be used in more than just this application. The Rust compiler
team has adopted it as an alternative codegen backend, in addition to LLVM.
Since it is relatively young, it is not as mature as LLVM. It does have some
properties that make it very appealing in certain scenarios: because it
focusses on generating binaries quickly, it tends to be faster than LLVM.
This makes it useful for building in development code, where a fast iteration
time is more important than good runtime performance.

In my testing, I was able to get around a 30% speedup by using the cranelift
backend instead of the default LLVM backend, but your mileage may vary.

### Example


## Reading

[Cranelift code generation comes to Rust](https://lwn.net/Articles/964735/)
