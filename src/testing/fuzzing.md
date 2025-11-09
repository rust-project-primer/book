# Fuzzing

```
v-----------feedback from instrumentation----v
randomized input > perform action > test for invalid behaviour
failing > reduce input > output error
                       > record failure
```

Fuzzing is an approach to testing code that generates random inputs for your code and uses
instrumentation to monitor which branches are being triggered, with the goal of triggering all
branches inside the code. In doing so, it can test your code very thoroughly and often times
discover edge cases.

Fuzzing is usually an effective technique for testing parsers. Fuzzing implementations are usually
able to use valid, working inputs as a starting point and randomly mutate them to try to find inputs
that either crash the program, or lead to some kind of invalid behaviour.

```admonish note
Fuzzing is a popular technique for testing parsers written in memory-unsafe
languages. It focusses on trying to reach all branches and testing for invalid
behaviour (stack overflows, read or write out of bounds). For this reason, it
is often combined with sanitizers. There is even some infrastructure for
continuously running fuzzing against popular open-source libraries, done by
Google Project Zero.

Because Rust is a memory-safe language, fuzzing is generally less important.
Some places where you might want to use it are:

- If your code makes a lot of use of `unsafe` and raw pointer access,
- If you are trying to test the soundness of a program that interacts with
  memory-unsafe languages (for example, bindings for a C or C++ library).

Otherwise, it might make more sense for you to look into doing [Property
testing](./property.md), which focusses more on testing individual components,
and is more focussed on *correctness* rather than *memory safety*.
```

Fuzzing is a very good strategy when your code parses untrusted data. It allows you to have
confidence that for any possible input, your program does not misbehave. The downside of fuzzing is
that usually, it can only detect crashes. When possible, it is better to test individual pieces of
code using property testing.

## afl.rs

afl, or _american fuzzy lop_, is one of the original fuzzing tools. It is designed to work very
easily: it continuously launches your program, feeds random bytes into it and monitors which
branches are taken.

When your program does anything invalid, it will record the input it used as a failing test case,
later you can attempt to reproduce the crash and try to figure out what caused it.

In general, what is considered _invalid_ is anything that causes your program to abort unsually, for
example a NULL pointer dereference. If you compile your program with sanitizers, you can detect more
invalid behaviours (these sanitizers will abort your program if it does anything they can detect):

- Undefined behaviour: reading uninitialized memory
- Memory: reading out-of-bounds, writing out-of-bounds

## cargo-fuzz

## Examples

_TODO_

## Reading

```reading
style: book
title: Rust-Fuzz Book
url: https://rust-fuzz.github.io/book/introduction.html
author: Rust Fuzz Book
---
This book explains what fuzz testing is, and how it can be implemented in Rust
using `afl.rs` and `cargo-fuzz`.
```

```reading
style: article
title: How to fuzz Rust code continuously
url: https://about.gitlab.com/blog/2020/12/03/how-to-fuzz-rust-code/
author: Yevgeny Pats
---
Yevgeny explains why you should fuzz your Rust code, and shows you how
to do it in GitLab. GitLab has some features that make running fuzzing
inside GitLab CI quite convenient.
```

```reading
style: article
title: Fuzzing Solana
url: https://secret.club/2022/05/11/fuzzing-solana.html
author: Addison Crump
---
Addison shows how Rust can be used to fuzz the Solana eBPF JIT compiler, and
outlines the security vulnerabilities found within uses this approach.
```

```reading
style: article
title: "Earn $200K by fuzzing for a weekend: Part 1"
url: https://secret.club/2022/05/11/fuzzing-solana.html
author: Addisn Crump
---
```

https://tweedegolf.nl/en/blog/154/what-is-my-fuzzer-doing

https://www.fuzzingbook.org/html/Grammars.html

https://antithesis.com/blog/2025/gradius/

https://googleprojectzero.blogspot.com/2024/10/effective-fuzzing-dav1d-case-study.html?m=1

https://carstein.github.io/fuzzing/2020/04/18/writing-simple-fuzzer-1.html

https://blog.includesecurity.com/2024/04/coverage-guided-fuzzing-extending-instrumentation/

https://nullprogram.com/blog/2025/02/05/
