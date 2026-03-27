# Mutation Testing

[Mutation testing](https://en.wikipedia.org/wiki/Mutation_testing) is an
approach to randomized testing code that uses a different approach to property
testing and fuzzing. Instead of randomly generating inputs to the program or
functions, it works by randomly mutating the code and running the existing
tests. The goal is to find mutations that do not break the tests: this usually
means that that section of code is not covered by tests, or that the tests are
not sufficient to explore all possible paths through the code. On a high level,
mutation testing frameworks try to inject bugs into your code and see if your
existing tests will catch them.

```admonish note
In some ways, you could say that mutation tests are *testing your tests*. If
you have good tests, then changing anything about your code should result in
at least one failing test. If that is not the case, then your tests do not cover
all properties (or branches, or edge cases) of your code.
```

## cargo-mutants

[cargo-mutants](https://github.com/sourcefrog/cargo-mutants)

## Reading

```reading
style: book
title: cargo-mutants book
url: https://mutants.rs/
author: cargo-mutants authors
---
This book explains how cargo-mutants works, and how it can be deployed in Rust
projects to find areas where bugs might be lurking.
```

```reading
style: article
title: Mutation Testing in Rust
url: https://blog.frankel.ch/mutation-testing-rust/
author: Nicolas Fränkel
---
Nicolas explains how to use `cargo-mutants` in Rust. He does this by setting up
an example Rust project, and running it against it. In the process, he
discovers a missed mutation test in `cargo-mutants` and creates a pull request
for it.  He shows how to fix his (intentionally buggy) code with the discovered
mutation.
```
