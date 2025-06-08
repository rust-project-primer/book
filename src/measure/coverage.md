# Test Coverage

The only way to ensure that you are doing a good job of testing all the paths in
the code is by measuring them. 

In general, I believe that library crates should aim to get as close to 100% of
test coverage as possible. Binary crates may not be able to achieve that, also
because they might use libraries that make it difficult to test them at all.
This is another good reason for splitting up project into small crates: it allows
you to have and enforce a good test coverage for all of the library crates that can
be tested, while allowing certain binary crates to not be well-tested.

## Cargo LLVM-Cov

In Rust, you can use [`cargo-llvm-cov`][cargo-llvm-cov] to determine code coverage. It can output
in different formats, including HTML, JSON and text.

```
$ cargo llvm-cov
```

## Cargo Tarpaulin

Another option is to use [`cargo-tarpaulin`][tarpaulin].

## GRCOV

Finally, there is also [grcov][].

## Reading

~~~reading
style: book
title: Instrumentation-based Code Coverage
url: https://doc.rust-lang.org/rustc/instrument-coverage.html
author: The rustc Book
---
This chapter in the rustc book explains low-level features of rustc that
enable adding instrumentation to binaries for measuring execution coverage, and
how to use the raw output to generate coverage reports.
~~~

~~~reading
style: article
title: How to do code coverage in Rust
url: https://blog.rng0.io/how-to-do-code-coverage-in-rust/#source-based-coverage
author: Dotan J. Nahum 
archived: rng0-code-coverage-rust.pdf
---
Dotan explains how to measure test coverage in Rust using both Tarpaulin and
grcov. He shows how to set it up for a project, with working GitHub Actions
workflows.
~~~

[cargo-llvm-cov]: https://github.com/taiki-e/cargo-llvm-cov
[tarpaulin]: https://github.com/xd009642/tarpaulin
[grcov]: https://github.com/mozilla/grcov

