# Dynamic Analysis

The Rust programming language does not prevent you from writing invalid code, it
just makes it a lot harder. The default state is that code is subject to the
borrow checker, which ensures memory safety. However, sometimes you need to
write code that bypasses these safety guarantees and places the burden of
ensuring correctness on you: `unsafe` code.

A typical Rust program contains minimal `unsafe` code. Most crates avoid it, and
when they do use it, it tends to be in small, contained sections. Rust doesn't
eliminate the ability to shoot yourself in the foot; it just forces you to be
intentional about it. In languages like C or C++, effectively all code is
implicitly `unsafe`, without the clear boundaries Rust provides.

Sometimes, you would like to check if the `unsafe` code you have written is in
fact valid. This can be challenging because what you're trying to catch is
_undefined behavior_. For example, reading one byte past an array's bounds
wouldn't necessarily cause your program to crash; you might simply read garbage
data.

One solution is to use _dynamic analysis_, where your program runs in a special
environment (instrumented or emulated) and a higher-level tool validates every
action your program takes. If your program triggers any _undefined behavior_,
you receive an error and a description of what went wrong:

- Read uninitialized memory
- Read past memory allocation/stack
- Write past memory allocation/stack
- Free memory that is already freed (double free)
- Forget to free memory (memory leak)

These tools can be enabled when running unit tests to monitor your code's
behavior and provide diagnostic errors when it performs invalid operations.
Triggering undefined behavior is dangerous because your program may break when
switching compilers or when running on different hardware. For example, x86 CPUs
permit unaligned memory reads, but other platforms might not, so code that
relies on this behavior will fail on those platforms.

Due to Rust's built-in safety guarantees, most Rust code doesn't contain
significant amounts of undefined behavior, making these tools less frequently
needed than in languages like C or C++.

There is one tool particularly well-suited for detecting invalid operations in
Rust code: [Miri][miri].

## Miri

[Miri][miri] is a tool that lets you find undefined behaviour in Rust programs.
It works as an interpreter for Rust's _mid-level intermediate representation_
(MIR), which the compiler uses internally. Similar to Valgrind, Miri works by
interpreting code rather than executing it directly. The advantage of Miri over
Valgrind is that MIR retains rich semantic information, resulting in more
precise diagnostic messages. However, like Valgrind, it significantly slows down
your program's execution.

You can install and use Miri with the following commands:

```bash
rustup +nightly component add miri
cargo +nightly miri test
```

Miri can detect numerous issues such as:

- Invalid memory accesses
- Use of uninitialized memory
- Data races
- Violations of Rust's stacked borrows model
- Leaking of memory marked as `MayLeak`

Miri is particularly valuable for testing `unsafe` code, as it can catch subtle
issues that might not manifest in normal testing environments. It is also useful
for testing code that interfaces with external libraries through FFI, as this is
a common source of unsafety.

Miri is a bit limited, there are certain functionality that it does not support.
For eaxmple, Miri does not support multithreading or SIMD instructions.

## Cargo Careful

[Cargo Careful](https://github.com/RalfJung/cargo-careful) is a lightweight tool
that adds additional checks to your Rust code without the overhead of a full
interpreter like Miri. It works by adding runtime checks for undefined behavior
to your code through compiler flags and environment variables.

Cargo Careful is particularly useful for:

- Detecting issues in `unsafe` code during regular testing
- Finding integer overflow in debug builds (which would panic in debug mode but
  could cause undefined behavior in release mode)
- Validating alignment requirements for memory accesses
- Detecting uninitialized memory usage

To use Cargo Careful, install it and run your tests with it:

```bash
cargo install cargo-careful
cargo careful test
```

Cargo Careful offers a significant speed advantage over Miri, making it suitable
for integration into regular test workflows. However, it detects fewer types of
issues than Miri's more comprehensive analysis.

## Valgrind

[Valgrind](https://valgrind.org/) lets you run your program in an emulated way,
where all memory access is monitored. It has a relatively faithful emulation of
the x86 architecture, it even incorporates features such as a model of how CPU
caches work so you can check how good the memory locality of your program is.

Due to the emulation, there is some overhead. It can also report how many
instructions your program took to run, which is more useful for microbenchmarks
than time, because it is stable between machines (but not architectures).

There is a [cargo-valgrind][] tool that you can use to run your Rust unit tests
with valgrind. It will parse the output of valgrind and output them in a
human-readable format.

[cargo-valgrind]: https://github.com/jfrimmel/cargo-valgrind

## LLVM Sanitizers

LLVM sanitizers ([AddressSanitizer][], [ThreadSanitizer][],
[UndefinedBehaviorSanitizer][], [LeakSanitizer][]) must be enabled at compile
time. They instrument your binary with additional checks on memory accesses or
operations, depending on the sanitizer type. This instrumentation introduces
performance overhead that varies by sanitizer type. These tools can detect
certain issues beyond Valgrind's capabilities. issues these can detect that go
beyond what Valgrind can detect, because of the instrumentation and metadata
that they have.

[AddressSanitizer]: https://clang.llvm.org/docs/AddressSanitizer.html
[ThreadSanitizer]: https://clang.llvm.org/docs/ThreadSanitizer.html
[UndefinedBehaviorSanitizer]:
  https://clang.llvm.org/docs/UndefinedBehaviorSanitizer.html
[LeakSanitizer]: https://clang.llvm.org/docs/LeakSanitizer.html

### Address Sanitizer (ASan)

AddressSanitizer is designed to detect memory errors such as:

- Use-after-free
- Heap/stack/global buffer overflow
- Stack-use-after-return
- Double-free, invalid free

You can use ASan with Rust by setting the following environment variables:

```bash
RUSTFLAGS="-Z sanitizer=address" cargo test
```

ASan typically introduces a 2-3x runtime overhead but runs significantly faster
than Valgrind while providing comparable detection capabilities.

### Memory Sanitizer (MSan)

MemorySanitizer detects uses of uninitialized memory, which can cause subtle
bugs that are hard to track down. Unlike ASan, MSan focuses specifically on
detecting reads from uninitialized memory.

```bash
RUSTFLAGS="-Z sanitizer=memory" cargo test
```

MSan is particularly valuable for code that manually manages memory or
interfaces with C libraries where memory initialization might be incomplete.

### Undefined Behaviour Sanitizer (UBSan)

UndefinedBehaviorSanitizer detects various types of undefined behavior at
runtime, including:

- Integer overflow
- Invalid bit shifts
- Misaligned pointers
- Null pointer dereferences
- Unreachable code execution

```bash
RUSTFLAGS="-Z sanitizer=undefined" cargo test
```

UBSan has relatively low performance overhead (typically 20-50%) and can detect
issues that other sanitizers might miss.

### Thread Sanitizer (TSan)

ThreadSanitizer detects data races in multithreaded code. This is particularly
valuable in Rust when using `unsafe` to implement concurrent data structures or
when interfacing with external threading libraries.

```bash
RUSTFLAGS="-Z sanitizer=thread" cargo test
```

TSan has higher overhead (5-15x) but excels at identifying race conditions that
might occur only sporadically during normal testing.

## Reading

```reading
style: article
title: Data-driven performance optimization with Rust and Miri
url: https://medium.com/source-and-buggy/data-driven-performance-optimization-with-rust-and-miri-70cb6dde0d35
author: Keaton Brandt
archived: keaton-data-driven-performance-optimization-rust-miri.pdf
---
Keaton shows you how you can use Miri to get detailed profiling information
from Rust programs, visualize them in Chrome developer tools and use this
information to optimize your program's execution time.
```

```reading
style: article
title: Unsafe Rust and Miri
url: https://www.youtube.com/watch?v=svR0p6fSUYY
author: Ralf Jung
---
In this talk, Ralf explains key concepts around writing unsafe code, such as
what "undefined behaviour" and "unsoundness" mean, and explains how to write
unsafe code in a systematic way that reduces the chance of getting it wrong.
```

```reading
style: article
title: C++ Safety, in context
url: https://herbsutter.com/2024/03/11/safety-in-context/
author: Herb Sutter
archived: herbsutter-safety-in-context.pdf
---
In this article, Herb Sutter discusses the safety issues C++ has. While this is
not directly relevant to Rust, he does make a good point about the fact that
there is good tooling to catch a lot of issues (sanitiziers, for example) and
that they should be more widely used, even by projects that use languages that
are safer by design, such as Rust. While some consider C++ to be
[defective](https://yosefk.com/c++fqa/defective.html), with the right tooling a
majority of issues can be caught.
```

```reading
style: article
title: The Soundness Pledge
url: https://raphlinus.github.io/rust/2020/01/18/soundness-pledge.html
author: Ralph Levien
archived: raphlinus-soundness-pledge.pdf
---
Ralph talks about the use of `unsafe` in Rust. Many developers consider using
it to be bad style, but he argues that it is not `unsafe` that is a problem, it
is unsound code that is a problem. As a community, we should strive to
eliminate unsound code. This includes using tools like Miri to ensure
soundness.
```

```reading
style: article
title: Rust and Valgrind
url: https://nnethercote.github.io/2022/01/05/rust-and-valgrind.html
author: Nicholas Nethercote
---
Nicolas explains why your should use Valgrind with Rust, and what kinds
of issues it can detect.
```

[miri]: https://github.com/rust-lang/miri
