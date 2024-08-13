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

## [cargo-mutants](https://github.com/sourcefrog/cargo-mutants)

Cargo mutants

## Reading

[cargo-mutants book](https://mutants.rs/)

*This book explains how cargo-mutants works, and how it can be deployed in
Rust projects to find 
