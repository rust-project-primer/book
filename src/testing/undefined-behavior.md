# Undefined behaviour

The Rust programming language does not prevent you from writing invalid code,
it just makes it a lot harder. The default state is that code is subject to the
borrow checker, which ensures that it is memory-correct. However, sometimes you
do want to write some code that opens up a little hatch and lets you take on
the burden of validating that it is correct yourself: `unsafe` code.

A typical Rust program does not use a lot of `unsafe` code. It is more the
exception that crates use it, and if they do it tends to be in small,
contained spaces. Rust does not eliminate the ability to shoot yourself
into the foot, it just forces you to be intentional about it. In languages
like C or C++, any piece of code is an `unsafe` block, it's like the wild
west.

Sometimes, you would like to check if the `unsafe` code you have written
is in fact valid. This can be a bit tricky, because the thing you are trying
to catch is *undefined behaviour*. For example, reading one byte past an array
would not necessarily cause your program to crash, instead you would just read
garbage.

One solution here is to use *dynamic analysis*, where your program is run in a
special way (instrumented or emulated) and a higher-level tool validates every
action your program takes. If your program triggers any of these *undefined
behaviours*, then you get an error and a description of what it did wrong:

- Read uninitialized memory
- Read past memory allocation/stack
- Write past memory allocation/stack
- Free memory that is already freed (double free)
- Forget to free memory (memory leak)

There are some tools from the C/C++ world (where this kind of thing is
quite common) that we can use:

- [Valgrind](https://valgrind.org/) lets you run your program in a kind of
  virtual machine, where all memory access is monitored. It is quite powerful,
  it even incorporates features such as a model of how CPU caches work so you can
  check how good the memory locality of your program is.  Due to the
  virtualisation, there is some overhead.  It can also report how many
  instructions your program took to run, which is more useful for microbenchmarks
  than time, because it is stable between machines (but not architectures).
- LLVM sanitizers ([AddressSanitizer][], [ThreadSanitizer][],
  [UndefinedBehaviorSanitizer][], [LeakSanitizer][]): these need to be enabled
  at compile time and instrument your binary with extra code on every memory
  access or operation (depending on the kind of sanitizer). The added code adds
  an overhead, depending on the kind of sanitizer this can be a lot. There
  are some things these can detect that go beyond what Valgrind can detect.

[AddressSanitizer]: https://clang.llvm.org/docs/AddressSanitizer.html
[ThreadSanitizer]: https://clang.llvm.org/docs/ThreadSanitizer.html
[UndefinedBehaviorSanitizer]: https://clang.llvm.org/docs/UndefinedBehaviorSanitizer.html
[LeakSanitizer]: https://clang.llvm.org/docs/LeakSanitizer.html

The idea with these tools is that you can enable them when running unit tests,
and they will monitor what your code does and give you a diagnostic error when
it does anything invalid. Triggering undefined behavior is quite dangerous,
it means that your program can break when you switch compilers or it might
work because your CPU happens to support certain things (for example, x86 CPUs
will let you perform unaligned reads, but other platforms might not, so if your
code performs those it will break on other platforms).

For Rust, due to the protections the language offers, we usually don't have large
amounts of undefined behaviour in the first place, so these tools are not usually
needed.

There is one tool that is particularily suited to helping detect invalid
operations in Rust code, and that is [Miri][miri].

## Miri

Miri is a tool that lets you find undefined behaviour in Rust programs. It
works by acting as an interpreter for Rust's *mid-level intermediate
representation*, which is used by the compiler internally. In some ways, it is
similar to Valgrind, because it works by interpreting this representation. The
advantage of using Miri over Valgrind is that this representation retains a lot
of semantic information, which means you get much better diagnostic messages.
It has the same downside as Valgrind, in that it makes your program's execution
very slow.

## Reading

[Data-driven performance optimization with Rust and
Miri](https://medium.com/source-and-buggy/data-driven-performance-optimization-with-rust-and-miri-70cb6dde0d35)
by Keaton Brandt

*Keaton shows you how you can use Miri to get detailed profiling information
from Rust programs, visualize them in Chrome developer tools and use this
information to optimize your program's execution time.*

[Unsafe Rust and Miri](https://www.youtube.com/watch?v=svR0p6fSUYY) by Ralf Jung

*In this talk, Ralf explains key concepts around writing unsafe code, such as
what "undefined behaviour" and "unsoundness" mean, and explains how to write
unsafe code in a systematic way that reduces the chance of getting it wrong.*

[C++ Safety, in context](https://herbsutter.com/2024/03/11/safety-in-context/) by Herb Sutter

*In this article, Herb Sutter discusses the safety issues C++ has. While this
is not directly relevant to Rust, he does make a good point about the fact that
there is good tooling to catch a lot of issues (sanitiziers, for example) and
that they should be more widely used, even by projects that use languages that
are safer by design, such as Rust. While some consider C++ to be [defective](https://yosefk.com/c++fqa/defective.html), with the right tooling a majority of issues
can be caught.*

[The Soundness Pledge](https://raphlinus.github.io/rust/2020/01/18/soundness-pledge.html) by Ralph Levien

*Ralph talks about the use of `unsafe` in Rust. Many developers consider using
it to be bad style, but he argues that it is not `unsafe` that is a problem, it
is unsound code that is a problem. As a community, we should strive to
eliminate unsound code. This includes using tools like Miri to ensure
soundness.*

[miri]: https://github.com/rust-lang/miri
