# Error handling

Error handling is essential to writing robust software. Rust has chosen a model
for error handling that emphasizes correctness.

Many programming languages use *exceptions* to communicate errors. In some way,
exceptions are a kind of hidden return value: a function can either return the
value it declares it will return, or it can *throw* an exception.

Rust deliberately chose not to do this, and rather uses return types. This
ensures that it is always clearly communicated which failure modes a function
has, and failure handling does not use a different channel. It also forces
programmers to handle errors, at least to some degree: a fallible function
returns a `Result<T, E>`, and you have to either handle the error (with a
`match` statement), or propagate it up with a `?`.

Part of the reason that doing this is ergonomic in Rust is because Rust has
great syntactical support for pattern matching. This is not the case for many
other languages, which is partially why *exceptions* were created and remain in
use.

### Communicating Failures in Rust

Rust has three principal methods of communicating failures. In the order
of utility, this is what they are:

- **Missing data**: Rust has the `Option<T>` type, which can communicate if
  something is missing. Generally, this is not an error. For example, when you
  look up a value in a map, it will return either `None` or `Some(T)`.
- **Recoverable errors**: Rust has the `Result<T, E>` type, which can
  either contain your data as `Ok(T)`, or contain an error as `Err(E)`.
- **Unrecorverable errors**: Panics are the Rust way to express an error that
  cannot be recovered from. This is perhaps the closest thing Rust has to
  exceptions. These are generated when invariants are invalid, or when memory
  cannot be allocated. When they are encountered, a backtrace is printed and
  your program is aborted, although there are some ways to catch them if
  need be.

Rust also has ways to convert between these types of errors. For example, if
a missing key in a map is to be treated as an error, you can write:

```rust
// get user name, or else return a user missing error
let value = map.get("user").ok_or(Error::UserMissing)?;
```

If an error is unrecoverable (or perhaps, you are prototyping some code and
chose not to properly handle errors yet), then you can turn a `Err(T)` into
a panic using `unwrap()` or `expect()`.

```rust
let file = std::fs::read_to_string("file.txt").unwrap();
```

### Panics in Rust

We can't talk about error handling in Rust without mentioning *panicing*. They
are a way to signify failures that cannot reasonable be recovered from. Panics
are not a general way to communicate errors, they are a method of last resort.

There are different ways to trigger panics in Rust. Commonly, using panics is
used when writing prototype code, because you want to focus on the code and
defer implementing error handling when the code works.

For example, when you write some code which traverses a data structure, you
might defer implementing the functionality for all edge cases. You can do this
by using the `todo!()` macro, which will trigger a panic if called.

```rust
fn test_value(value: &Value) -> bool {
    match value {
        Value::String(string) => string.len() > 0,
        Value::Number(number) => number > 0,
        Value::Map(map) => todo!(),
        Value::Array(array) => todo!(),
    }
}
```

Using [catch_unwind()](https://doc.rust-lang.org/std/panic/fn.catch_unwind.html),
you can catch panics. This might be useful if you use libraries that might panic.

```rust
std::panic::catch_unwind(|| {
    panic!("oops!");
});
```

### The `Error` trait

### Libraries for custom error types

Rust comes with some libraries, which can help you integrate into the Rust error
system. On a high level, these libraries fall into one of two categories:

- **Custom error types**: these libraries allow you to define custom error types,
  by implementing the `Error` trait and any neccessary other traits. A common
  pattern is to define an error type, which is an enumeration of all possible
  errors your application (or this particular function) may produce. These libraries
  often also help you by implementing a `From<T>` implementation for errors that
  your error type wraps.
- **Dealing with arbitrary errors**: In some cases, you want to be able to handle
  arbitrary errors. If you are writing a crate which is to be used by others,
  this is generally a bad idea, because you want to expose the raw errors to
  consumers of your library. But if you are writing an application, and all you
  want to do is to render the error at some point, it is usually beneficial to
  use some library which has the equivalent of `Box<dyn Error>` and lets you
  not worry about defining custom error types. Often times, these libraries also
  contain functionality for pretty-printing error chains and stack traces.

In general, if you write a crate that is to be used as a library by other
crates, you should be using a library which allows you to define custom error
types. You want the users of your crate to be able to handle the different
failure modes, and if the failure modes change (your error enums change), you
want to force them to adjust their code. This makes the failure modes
explicity.

If you write an application (such as a command-line application, a web
application, or any other code where you are not exposing the errors in any
kind of API), then using the latter kind of error-handling library is
appropriate. In this case, all you care about is reporting errors and metadata
(where they occured) to an end-user.

## Thiserror

The [thiserror](https://docs.rs/thiserror/latest/thiserror/) crate is a popular
crate for defining custom errors. It helps you to implement the `Error` trait
for your custom error types. For example:

```rust
#[derive(thiserror::Error, Debug)]
pub enum MyError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("user {0:} not found")]
    UserNotFound(String),
}
```

## Anyhow

The [anyhow](https://docs.rs/anyhow/latest/anyhow/) crate gives you the
ability to work with dynamic and ad-hoc errors. It exports the `anyhow::Error`
type, which can capture any Rust error.

```rust
use anyhow::Error;

fn main() -> Result<(), Error> {
    let data = std::fs::read_to_string("file.txt")?;
    Ok(())
}
```

## Eyre



## Miette



## Reading

[The definitive guide to Rust error handling](https://www.howtocodeit.com/articles/the-definitive-guide-to-rust-error-handling)

[Error Handling](https://doc.rust-lang.org/book/ch09-00-error-handling.html) in The Rust Programming Language

[Error handling in Rust: a comprehensive tutorial](https://blog.logrocket.com/error-handling-rust/) by Eze Sunday

[Rust Error Handling: thiserror, anyhow, and When to Use Each](https://momori.dev/posts/rust-error-handling-thiserror-anyhow/) by Momori Nakano

[Error Handling in Rust: A Deep Dive](https://www.lpalmieri.com/posts/error-handling-rust/) by Luca Palmieri

[Error Handling in a Correctness-Critical Rust Project](https://sled.rs/errors.html) by Tyler Neely

[Three kinds of Unwrap](https://zkrising.com/writing/three-unwraps/)

[Designing Error Types in Rust
Libraries](https://d34dl0ck.me/rust-bites-designing-error-types-in-rust-libraries/index.html)
by Sven Kanoldt

[Why Use Structured Errors in Rust Applications?](https://home.expurple.me/posts/why-use-structured-errors-in-rust-applications/) by Dmitrii Aleksandrov

[Error Handling in Rust](https://burntsushi.net/rust-error-handling/) by Andrew Gallant

https://mmapped.blog/posts/12-rust-error-handling
