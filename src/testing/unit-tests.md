# Unit Tests

Unit tests are intended to test one small unit at a time. It might be a feature,
it might be a specific input to an algorithm. Rust has native support for them
with the built-in testing harness support.

```admonish info
Unit tests are similar to integration tests. In fact, they both look the same:
a function annotated with `#[test]`. But there is an important difference in
how they run. Unit tests are written inside your code base. Depending on where
they are placed, they have visibility into non-`pub` methods and functions,
allowing them to test internal state.

Integration tests on the other hand are compiled as if they were an external
crate that happens to depend on your crate. They can only test what is
publicly visible, not internal state of your structs.
```

In Rust, you can annotate any function with `#[test]` and it will be a (unit or
integration) test. Here is what a simple test case looks like:

```rust
#[test]
fn can_add() {
    assert_eq!(1 + 1, 2);
}
```

Running `cargo test` will run all of the tests present in a project.

### Where to put unit tests

Usually, when you write unit tests in Rust you put them at the end of every
module, and you declare a `tests` module inline.

Here's an example of what this might look like:

```rust
fn function_one() -> &'static str {
    "hello"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_function_one() {
        assert_eq!(function_one(), "hello");
    }
}
```

This is, however, a question of style. It's also perfectly okay to just
intersperse tests with the code. Keeping the tests close to the code is
important, because it means that they will have visibility into non-public
methods and fields.

### Enabling unit-test-only code

Sometimes you may want to enable additional code only when building and running
unit tests. When Cargo builds your unit tests, it enables the `test` cfg, which
you can use inside your code. For example, you can use it to enable additional
logging when building unit tests:

```rust
#[cfg(test)]
debug!("extra debug log");
```

But you can do more than this: you can add members to your structs that only
exist during unit testing. For example, if you want visibility into internal
states, this allows you to enable extra member methods.

### Testing Panics

Sometimes you want to verify that code panics under certain conditions — for
example, that an out-of-bounds index triggers a panic rather than silently
returning garbage. The `#[should_panic]` attribute marks a test that is expected
to panic:

```rust
#[test]
#[should_panic(expected = "index out of bounds")]
fn out_of_bounds() {
    let v = vec![1, 2, 3];
    let _ = v[5];
}
```

The `expected` parameter is optional but recommended: it matches against the
panic message, so the test fails if the code panics for a different reason than
you intended.

### Ignoring Tests

The `#[ignore]` attribute marks a test that should be skipped during normal
`cargo test` runs. This is useful for tests that are slow, require special
setup, or depend on external services:

```rust
#[test]
#[ignore]
fn slow_integration_test() {
    // takes minutes to run
}
```

Ignored tests can be run explicitly with `cargo test -- --ignored`, or you can
run all tests including ignored ones with `cargo test -- --include-ignored`.

### Parameterized Tests with rstest

The [`rstest`][rstest] crate lets you write parameterized tests — running the
same test logic with multiple inputs without duplicating the test function:

```rust
use rstest::rstest;

#[rstest]
#[case(0, 0)]
#[case(1, 1)]
#[case(2, 1)]
#[case(3, 2)]
#[case(4, 3)]
fn fibonacci(#[case] input: u32, #[case] expected: u32) {
    assert_eq!(fib(input), expected);
}
```

Each `#[case]` generates a separate test, so failures point you directly to
which input combination failed. `rstest` also supports fixtures for shared setup
logic across tests.

### Pretty Assertions

The [`pretty-assertions`][pretty-assertions-repo] crate
([docs][pretty-assertions-docs]) helps you understand test failures by showing a
colored diff when two values don't match, rather than just printing both values.

## Testing async code

If you chose to use async code in your project, you might run into a situation
where you need to write unit tests for asynchronous code. Usually, most of the
unit tests don't require it, because you will follow the _blocking core, async
shell_ paradigm.

If you do need to write async unit tests, then the Tokio library has some
functionality you can use for that. They have a `#[tokio::test]` macro that you
can use to annotate any unit test to turn it into an asynchronous unit test.

```rust
#[tokio::test]
async fn async_unit_test() {
    assert_eq!(test_something().await, 42);
}
```

## Reading

```reading
style: book
url: https://doc.rust-lang.org/rust-by-example/testing/unit_testing.html
title: Unit testing
author: Rust By Example
---
This chapter outlines features of Rust's built-in support for unit tests. It
shows advanced features, such as unit-testing panics, marking tests as ignored
and running specific tests from the command-line.
```

```reading
style: book
url: https://abseil.io/resources/swe-book/html/ch12.html
title: Unit Testing
author: Software Engineering at Google
---
This chapter discusses how Google approaches unit testing. It argues for testing
via public APIs rather than implementation details, testing state rather than
interactions, and structuring tests around behaviors rather than methods. It
also advocates for DAMP (Descriptive And Meaningful Phrases) over DRY in test
code, accepting some duplication in exchange for clarity.
```

[pretty-assertions-repo]:
  https://github.com/rust-pretty-assertions/rust-pretty-assertions
[pretty-assertions-docs]:
  https://docs.rs/pretty_assertions/latest/pretty_assertions/
[rstest]: https://docs.rs/rstest/latest/rstest/
