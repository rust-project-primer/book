# Mutation Testing

[Mutation testing](https://en.wikipedia.org/wiki/Mutation_testing) is an
approach to testing that works differently from property testing and fuzzing.
Instead of randomly generating inputs, it works by randomly mutating your _code_
and running the existing tests against each mutation. The goal is to find
mutations that do not break any tests: this usually means that a section of code
is not covered by tests, or that the tests are not specific enough to catch the
change.

On a high level, mutation testing frameworks try to inject bugs into your code
and see if your existing tests catch them.

```admonish note
In some ways, you could say that mutation tests are *testing your tests*. If
you have good tests, then changing anything about your code should result in
at least one failing test. If that is not the case, then your tests do not cover
all properties (or branches, or edge cases) of your code.
```

## cargo-mutants

[`cargo-mutants`][cargo-mutants] is the main mutation testing tool for Rust. It
works by applying mutations to your source code, running `cargo test` (or
`cargo nextest`) for each mutation, and reporting which mutations were caught by
your tests and which were not.

### Installation

```bash
cargo install cargo-mutants
```

### Running

Run it in your project directory:

```bash
cargo mutants
```

cargo-mutants will automatically find functions in your code, apply mutations to
them one at a time, and run your tests after each mutation. The output looks
something like this:

```
Found 38 mutants to test
ok       Unmutated baseline in 1.2s build + 0.3s test
14 mutants tested in 0:08: 2 missed, 9 caught, 3 unviable
```

### Interpreting Results

Each mutation falls into one of four categories:

- **Caught**: A test failed after the mutation was applied. This is good: it
  means your tests are specific enough to detect this kind of change.
- **Missed**: All tests still passed after the mutation. This suggests that the
  mutated code is either untested or that the tests don't check for the behavior
  that changed.
- **Unviable**: The mutation caused a compilation error. This is neutral: it
  means the type system already prevents this kind of bug, which is one of
  Rust's strengths.
- **Timeout**: The mutation caused the test suite to hang (for example, by
  turning a loop condition into `true`). These are treated as caught, since a
  hang is detectable.

The missed mutations are the interesting ones. They point to places where your
tests could be stronger. cargo-mutants writes detailed results to `mutants.out/`
in your project directory, including the exact mutation applied and which file
and function it was in.

### Types of Mutations

cargo-mutants applies several kinds of mutations:

**Return value replacement** is the most common: it replaces function bodies
with default values that match the return type. For example, a function
returning `bool` will be replaced with one that always returns `true` (and then
`false`), a function returning `i32` will return `0` and `1`, and a function
returning `String` will return an empty string. This tests whether your code
actually checks return values.

**Binary operator replacement** swaps operators in expressions: `==` becomes
`!=`, `&&` becomes `||`, `+` becomes `-`, and so on. This tests whether your
conditional logic and arithmetic are actually verified by tests.

**Unary operator deletion** removes negation (`-x` becomes `x`, `!b` becomes
`b`), testing whether sign and boolean inversion matter to your tests.

cargo-mutants also supports **match arm deletion** and **struct field
deletion**, though these are applied less frequently.

### Skipping Functions

Some functions are not worth mutating: logging helpers, debug formatting, or
code that is intentionally untested. You can skip them with an attribute:

```rust
#[mutants::skip]
fn debug_log(msg: &str) {
    eprintln!("[DEBUG] {msg}");
}
```

This attribute has no effect on normal compilation and is only recognized by
cargo-mutants.

### Using in CI

Mutation testing is slow compared to running your test suite once, because it
runs the full suite for every mutation. For large projects, running it on every
commit is impractical. A common approach is to run it on a schedule (for
example, weekly) or only on changed files using the `--in-diff` flag:

```bash
git diff main | cargo mutants --in-diff -
```

This limits mutation testing to functions that were modified in the current
branch, which is fast enough for PR checks.

## Reading

```reading
style: book
title: cargo-mutants book
url: https://mutants.rs/
author: cargo-mutants authors
---
The official documentation for cargo-mutants, covering installation, usage,
configuration, mutation types, CI integration, and strategies for dealing with
missed mutations.
```

```reading
style: article
title: Mutation Testing in Rust
url: https://blog.frankel.ch/mutation-testing-rust/
author: Nicolas Fränkel
---
Nicolas explains how to use cargo-mutants by setting up an example project and
running it. In the process, he discovers a missed mutation, creates a pull
request to fix it, and shows how mutation testing can reveal gaps in test
coverage that other approaches miss.
```

[cargo-mutants]: https://github.com/sourcefrog/cargo-mutants
