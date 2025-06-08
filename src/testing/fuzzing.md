# Fuzzing

Fuzzing is an approach to testing code that generates random inputs for your
code and uses instrumentation to monitor which branches are being triggered,
with the goal of triggering all branches inside the code. In doing so, it can
test your code very thoroughly and often times discover edge cases.

Fuzzing is a very good strategy when your code parses untrusted data. It allows
you to have confidence that for any possible input, your program does not
misbehave.  The downside of fuzzing is that usually, it can only detect
crashes. When possible, it is better to test individual pieces of code
using property testing.

## Examples

*TODO*

## Reading

~~~reading
style: book
title: Rust-Fuzz Book
url: https://rust-fuzz.github.io/book/introduction.html
author: Rust Fuzz Book
---
This book explains what fuzz testing is, and how it can be implemented in Rust
using `afl.rs` and `cargo-fuzz`.
~~~

~~~reading
style: article
title: How to fuzz Rust code continuously
url: https://about.gitlab.com/blog/2020/12/03/how-to-fuzz-rust-code/
author: Yevgeny Pats
---
Yevgeny explains why you should fuzz your Rust code, and shows you how
to do it in GitLab. GitLab has some features that make running fuzzing
inside GitLab CI quite convenient.
~~~

~~~reading
style: article
title: Fuzzing Solana
url: https://secret.club/2022/05/11/fuzzing-solana.html
author: Addison Crump
---
Addison shows how Rust can be used to fuzz the Solana eBPF JIT compiler, and
outlines the security vulnerabilities found within uses this approach.
~~~

