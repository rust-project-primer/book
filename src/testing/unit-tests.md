# Unit Tests

Unit tests are intended to test one small unit at a time. It might be a feature,
it might be a specific input to an algorithm. Rust has native support for them
with the built-in testing harness support.

In Rust, you can annotate any function with `#[test]` and it will be a (unit or
integration) test. Here is how a simple test case might look like:

```rust
#[test]
fn can_add() {
    assert_eq!(1 + 1, 2);
}
```

Running `cargo test` will run all of the tests present in a project.

## Where to put unit tests

Usually, when you write unit tests in Rust you put them at the end of every
module, and you declare a `tests` module inline.

Here's an example of what this might look like:

```rust
fn function_one() -> &'static str {
    "hello"
}

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

## Testing async code

If you chose to use async code in your project, you migth run into a situation
where you need to write unit tests for asynchronous code. Usually, most of the
unit tests don't require it, because you will follow the *blocking core, async shell*
paradigm.

If you do need to write async unit tests, then the Tokio library has some
functionality you can use for that. They have a `#[tokio::test]` macro that
you can use to annotate any unit test to turn it into an asynchronous unit test.

```rust
#[tokio::test]
async fn async_unit_test() {
    assert_eq!(test_something().await, 42);
}
```

## Reading

[Unit testing](https://doc.rust-lang.org/rust-by-example/testing/unit_testing.html) in Rust By Example


[Unit Testing](https://abseil.io/resources/swe-book/html/ch12.html) in Software Engineering at Google
