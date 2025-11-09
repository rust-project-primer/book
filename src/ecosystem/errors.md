# Error handling

Error handling is essential to writing robust software. Rust has chosen a model for error handling
that emphasizes correctness.

Many programming languages use _exceptions_ to communicate errors. In some way, exceptions are a
kind of hidden return value: a function can either return the value it declares it will return, or
it can _throw_ an exception.

Rust deliberately chose not to do this, and rather uses return types. This ensures that it is always
clearly communicated which failure modes a function has, and failure handling does not use a
different channel. It also forces programmers to handle errors, at least to some degree: a fallible
function returns a `Result<T, E>`, and you have to either handle the error (with a `match`
statement), or propagate it up with a `?`.

```admonish note
In some ways, this is only partially true. Rust does have a kind of *exception*,
through the `panic!()` and `.unwrap()` mechanism. However, the difference is
that these are generally only used for *unrecoverable* errors.
```

Part of the reason that doing this is ergonomic in Rust is because Rust has great syntactical
support for pattern matching. This is not the case for many other languages, which is partially why
_exceptions_ were created and remain in use.

### Communicating Failures in Rust

Rust has three principal methods of communicating failures. In the order of utility, this is what
they are:

- **Missing data**: Rust has the `Option<T>` type, which can communicate if something is missing.
  Generally, this is not an error. For example, when you look up a value in a map, it will return
  either `None` or `Some(T)`.
- **Recoverable errors**: Rust has the `Result<T, E>` type, which can either contain your data as
  `Ok(T)`, or contain an error as `Err(E)`.
- **Unrecoverable errors**: Panics are the Rust way to express an error that cannot be recovered
  from. This is perhaps the closest thing Rust has to exceptions. These are generated when
  invariants are invalid, or when memory cannot be allocated. When they are encountered, a backtrace
  is printed and your program is aborted, although there are some ways to catch them if need be.

Rust also has ways to convert between these types of errors. For example, if a missing key in a map
is to be treated as an error, you can write:

```rust
// get user name, or else return a user missing error
let value = map.get("user").ok_or(Error::UserMissing)?;
```

If an error is unrecoverable (or perhaps, you are prototyping some code and chose not to properly
handle errors yet), then you can turn a `Err(T)` into a panic using `unwrap()` or `expect()`.

```rust
let file = std::fs::read_to_string("file.txt").unwrap();
```

### Panics in Rust

We can't talk about error handling in Rust without mentioning _panicking_. Panics are a way to
signify failures that cannot reasonably be recovered from. Panics are not a general way to
communicate errors, they are a method of last resort. Usually, when a panic is encountered, it means
that something went wrong that the programmer did not anticipate or handle and the program should
abort.

There are different ways to trigger panics in Rust. Commonly, panics are used when writing prototype
code, because you want to focus on the code and defer implementing error handling when the code
works.

For example, when you write some code which traverses a data structure, you might defer implementing
the functionality for all edge cases. You can do this by using the `todo!()` macro, which will
trigger a panic if called.

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

Using [catch_unwind()](https://doc.rust-lang.org/std/panic/fn.catch_unwind.html), you can catch
panics. This might be useful if you use libraries that might panic.

```rust
std::panic::catch_unwind(|| {
    panic!("oops!");
});
```

For example, the `axum` framework uses `catch_unwind` to catch panics in request handlers, which
makes it convenient to use it during development, because the server will not crash when it
encounters a panic. However, they warn that this is not recommended for production usage because it
has performance implications.

```admonish warning
While it is possible to catch panics with
[`catch_unwind()`](https://doc.rust-lang.org/std/panic/fn.catch_unwind.html),
this is not recommended, has a performance penalty and does not work across
an FFI boundary. Panics are considered *unrecoverable* errors, and catching them
only works on a best-effort basis, on supported platforms.

Catching panics can be useful for development. For example, when you implement
a backend with an API, it can be useful to use `todo!()` statements in the code
and catch panics in your request handler, so that your backend does not
terminate when you hit something that isn't implemented yet.

Production applications should generally never panic, and if they do it should
result in the default behaviour, which is the application aborting.
```

### The `Result` type

In general, fallible functions in Rust use the `Result` return type to signify this. It is an
enumeration that represents either success with an expected result value `Ok(value)` or failure with
an error `Err(error)`.

If you have a common error type that you use in your application, then it is possible to make an
alias of the `Result` type that defaults to your error type, but allows you to override it with a
different error type if needed:

```rust
type Result<T, E = MyError> = Result<T, E>;
```

When you do this, `Result<String>` will resolve to `Result<String, MyError>`. However, you can still
write `Result<String, OtherError>` to use a specific error type. Your custom error type is only used
as the default when you don't specify any other type.

### The `Error` trait

In general, all error types in Rust implement the [`Error`][error] trait. This trait allows for
getting a simple textual description of an error and information about the source of the error.

If you create custom error types, you should implement this trait on them. There are some common
libraries that help with doing this.

[error]: https://doc.rust-lang.org/stable/std/error/trait.Error.html

### Libraries for custom error types

Rust comes with some libraries, which can help you integrate into the Rust error system. On a high
level, these libraries fall into one of two categories:

- **Custom error types**: these libraries allow you to define custom error types, by implementing
  the `Error` trait and any necessary other traits. A common pattern is to define an error type,
  which is an enumeration of all possible errors your application (or this particular function) may
  produce. These libraries often also help you by implementing a `From<T>` implementation for errors
  that your error type wraps.
- **Dealing with arbitrary errors**: In some cases, you want to be able to handle arbitrary errors.
  If you are writing a crate which is to be used by others, this is generally a bad idea, because
  you want to expose the raw errors to consumers of your library. But if you are writing an
  application, and all you want to do is to render the error at some point, it is usually beneficial
  to use some library which has the equivalent of `Box<dyn Error>` and lets you not worry about
  defining custom error types. Often times, these libraries also contain functionality for
  pretty-printing error chains and stack traces.
- **Error reporting**: Some libraries focus specifically on presenting errors to users in a readable
  way, often with rich formatting, source code snippets, and helpful hints. These libraries are
  particularly useful for developer tools, compilers, and applications that need to provide detailed
  error information.

In general, if you write a crate that is to be used as a library by other crates, you should be
using a library which allows you to define custom error types. You want the users of your crate to
be able to handle the different failure modes, and if the failure modes change (your error enums
change), you want to force them to adjust their code. This makes the failure modes explicit.

If you write an application (such as a command-line application, a web application, or any other
code where you are not exposing the errors in any kind of API), then using the latter kind of
error-handling library is appropriate. In this case, all you care about is reporting errors and
metadata (where they occurred) to an end-user.

```admonish warning
When using error handling libraries, keep in mind the trade-offs:

- Libraries should generally avoid using `anyhow`, `eyre`, or similar "opaque
  error" libraries in their public API, as this hides error details from consumers.
- Adding too much context to errors can bloat binary size due to string literals.
- For applications with complex domain logic, consider custom error types even if
  you're the only consumer.
- Be cautious about adding backtraces to all errors, as this can impact performance.
- If you re-export other crates' error types in your custom error enum, then that
  crate version becomes part of your public API. This has implications for versioning,
  if you update the version of the dependency, this may be a breaking change requiring
  a major version bump.
```

If you're writing a library, you should use a structured error library like [thiserror](#thiserror)
to define custom error types, with useful metadata and context. This will allow downstream consumers
to work with and handle the errors. If you write an application, you may want to consider using a
more dynamic library like [anyhow](#anyhow), which allows you to not worry about specific error
types and simply propagate them. If you need a library that focusses on good error reporting,
consider using [miette](#miette) or [eyre](#eyre).

## Thiserror

The [thiserror](https://docs.rs/thiserror/latest/thiserror/) crate is a popular crate for defining
custom structured errors. It helps you to implement the `Error` trait for your custom error types.

Imagine you have an application that uses an SQLite database to store data and properties. Every
query to the database returns some custom error type of the database library. However, you want
consumers of your crate to be able to differentiate between different error cases.

For example:

```rust
#[derive(thiserror::Error, Debug)]
pub enum MyError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("user {0:} not found")]
    UserNotFound(String),
}
```

The crate is specifically useful for implementing your own structured error types, or for composing
multiple existing error types into a wrapper enum.

By writing wrapper enums, you are also able to _refine_ errors, for example classifying errors you
receive from an underlying database.

## Anyhow

The [anyhow](https://docs.rs/anyhow/latest/anyhow/) crate gives you the ability to work with dynamic
and ad-hoc errors. It exports the `anyhow::Error` type, which can capture any Rust error.

```rust
use anyhow::Error;

fn main() -> Result<(), Error> {
    let data = std::fs::read_to_string("file.txt")?;
    Ok(())
}
```

The anyhow crate also has a `Result` alias, which defaults to using its Error type.

This library is very useful for when you are writing an application that uses multiple libraries,
and you don't want to inspect or handle the errors explicitly. Rather, you can use anyhow's Error
type to pass them around and render them to the user.

## Eyre

[Eyre](https://docs.rs/eyre/latest/eyre/) is similar to anyhow but focuses more on customizable
error reporting. It provides a context-aware error type that can capture information about where and
why an error occurred.

```rust
use eyre::{eyre, Result};

fn main() -> Result<()> {
    let file = std::fs::read_to_string("config.toml")
        .wrap_err("failed to read configuration file")?;
    Ok(())
}
```

Eyre is particularly useful when you want to add additional context to errors as they propagate
through your application.

The [color-eyre](https://docs.rs/color-eyre/latest/color_eyre/) crate extends Eyre with colorful,
pretty error reports and even better panic messages with backtraces.

## Miette

[Miette](https://docs.rs/miette/latest/miette/) is an error reporting library that focuses on
providing detailed, human-readable diagnostic information. It excels at displaying code snippets
with error spans and fancy formatting.

```rust
use miette::{Diagnostic, Result};
use thiserror::Error;

#[derive(Error, Diagnostic, Debug)]
#[error("invalid configuration")]
#[diagnostic(
    code(app::invalid_config),
    help("check the syntax in your config file")
)]
struct ConfigError {
    #[source_code]
    src: String,

    #[label("this part is invalid")]
    span: (usize, usize),
}
```

Miette is ideal for applications that need to provide detailed, contextual error information to
users, such as compilers, linters, or configuration validators.

## Other Error Libraries

### Error-Stack

[Error-stack](https://docs.rs/error-stack/latest/error_stack/) is a more recent error handling
library that provides an extended approach to error creation and propagation. It allows for
attaching arbitrary context to errors as they bubble up through your program, creating a detailed
"stack" of information.

```rust
use error_stack::{Report, ResultExt};

fn read_config(path: &str) -> error_stack::Result<String, ConfigError> {
    std::fs::read_to_string(path)
        .change_context(ConfigError::FileIO)
        .attach_printable(format!("while reading config file: {}", path))
}
```

Error-stack excels at creating rich error contexts without the overhead of capturing full
backtraces.

### SNAFU

[SNAFU](https://docs.rs/snafu/latest/snafu/) (Situation Normal: All Fouled Up) is another library
for defining error types and context information. It uses a different approach than thiserror,
relying on context selectors rather than derive macros.

```rust
use snafu::{prelude::*, Whatever};

#[derive(Debug, Snafu)]
enum Error {
    #[snafu(display("Could not open config file: {}", source))]
    OpenConfig { source: std::io::Error },
}

fn open_config() -> Result<(), Error> {
    std::fs::File::open("config.toml").context(OpenConfigSnafu)?;
    Ok(())
}
```

SNAFU is particularly useful for situations where you need fine-grained control over how context is
attached to errors.

### Ariadne

[Ariadne](https://docs.rs/ariadne/latest/ariadne/) is an alternative to miette that focuses on
displaying source code diagnostics. It's designed for parsers, compilers, and interpreters that need
to report syntax errors or other issues in source code.

## Conclusion

In this section, we've explored several error handling libraries in Rust, including thiserror,
snafu, anyhow, and miette. We've seen how each library provides a different approach to error
handling, and how they can be used in different situations.

When you write libraries, you may want to use a library like `thiserror` or `snafu` to provide
structured errors to users of your library where the focus is on providing context and allowing
users to handle errors. When you write applications, you may want to use a library like `anyhow` or
`miette` that provide convenient, generic containers for arbitrary errors where the focus is on
communicating errors to end-users.

## Reading

```reading
style: article
title: The definitive guide to Rust error handling
url: https://www.howtocodeit.com/articles/the-definitive-guide-to-rust-error-handling
author: Angus Morrison
archived: howtocodeit-definitive-guide-to-error-handling.pdf
---
Angus walks through the basics of error handling in Rust. He explains the
`Error` trait, and when to use boxed versions of it to pass error around. He
shows how it can be downcast into concrete error types, and how anyhow's `Error`
type can be used instead. He explains structured error handling by implementing
custom types. The article provides excellent coverage of `thiserror` and `anyhow`,
with real-world examples from popular crates like Actix Web and wgpu. Special
attention is given to `std::io::Error` as a complex example and Hyrum's Law's
impact on error design.
```

```reading
style: book
title: "Chapter 9: Error Handling"
url: https://doc.rust-lang.org/book/ch09-00-error-handling.html
author: The Rust Programming Language
---
The official Rust Book's chapter on error handling covers the fundamental concepts of recoverable and unrecoverable errors. It introduces `Result<T, E>` and the `panic!` macro, explaining when to use each approach. The chapter provides the foundational understanding needed before diving into more advanced error handling patterns and libraries.
```

```reading
style: article
title: "Error handling in Rust: a comprehensive tutorial"
url: https://blog.logrocket.com/error-handling-rust/
author: Eze Sunday
---
A practical tutorial covering recoverable vs unrecoverable errors, with hands-on examples of various error handling methods like `.unwrap_or()`, `.expect()`, and the `?` operator. Sunday provides a helpful comparison table of `thiserror`, `anyhow`, and `color-eyre` libraries, along with best practices for debugging and logging. The article emphasizes practical application over theory.
```

```reading
style: article
title: "Rust Error Handling: thiserror, anyhow, and When to Use Each"
url: https://momori.dev/posts/rust-error-handling-thiserror-anyhow/
author: Momori Nakano
archived: momori-rust-error-handling-thiserror-anyhow.pdf
---
A focused comparison of `thiserror` and `anyhow` with practical examples. Nakano demonstrates how to build custom error enums, implement required traits manually, then shows how `thiserror` simplifies the process. The article clearly explains when to use structured errors (`thiserror`) vs opaque errors (`anyhow`), with the rule of thumb that libraries should provide detailed error information while applications can hide internal details.
```

```reading
style: article
title: "Error Handling in Rust: A Deep Dive"
url: https://www.lpalmieri.com/posts/error-handling-rust/
author: Luca Palmieri
---
An in-depth exploration of error handling patterns from a backend development perspective. Palmieri covers the dual purposes of errors (control flow and reporting), layering strategies, and avoiding "ball of mud" error enums. The article includes extensive examples from a newsletter application, showing how to implement proper error chains and logging. Particularly valuable for understanding error handling architecture in larger applications.
```

```reading
style: article
title: Error Handling in a Correctness-Critical Rust Project
url: https://sled.rs/errors.html
author: Tyler Neely
---
A battle-tested perspective on error handling from the author of the sled database. Neely argues against global error enums based on real-world experience with catastrophic failures. The article demonstrates how nested `Result` types (`Result<Result<T, LocalError>, FatalError>`) can prevent improper error propagation. Includes practical advice on error injection testing using tools like PingCAP's `fail` crate to catch bugs in error handling logic.
```

```reading
style: article
title: Three kinds of Unwrap
url: https://zkrising.com/writing/three-unwraps/
author: zk
---
An analysis of the semantic differences between various uses of `.unwrap()` in Rust applications. The author identifies three distinct categories: unwrap as `panic!()` (intentional termination), unwrap as `unreachable!()` (impossible error states), and unwrap as `todo!()` (temporary placeholder). The article proposes new standard library methods like `.todo()` and `.unreachable()` to better express intent and enable better tooling support.
```

```reading
style: article
title: Designing Error Types in Rust Libraries
url: https://d34dl0ck.me/rust-bites-designing-error-types-in-rust-libraries/index.html
author: Sven Kanoldt
archived: kanoldt-designing-error-types-in-rust-libraries.pdf
---
A library author's guide to designing error types that provide useful information to consumers. Kanoldt covers the trade-offs between different error type designs, including when to use enums vs structs, how to provide context without breaking encapsulation, and techniques for making errors actionable. The article emphasizes designing errors from the consumer's perspective rather than the implementation's convenience.
```

```reading
style: article
title: Why Use Structured Errors in Rust Applications?
url: https://home.expurple.me/posts/why-use-structured-errors-in-rust-applications/
author: Dmitrii Aleksandrov
---
Aleksandrov challenges the conventional wisdom that applications should use opaque errors like `anyhow::Error`. He argues that even applications can benefit from structured error types for better testing, debugging, and maintainability. The article provides practical examples of how structured errors can improve application robustness and developer experience, even when errors aren't exposed in public APIs.
```

```reading
style: article
title: Error Handling in Rust
url: https://burntsushi.net/rust-error-handling/
author: Andrew Gallant
archived: burntsushi-rust-error-handling.pdf
---
A foundational article on Rust error handling from the author of ripgrep and many other popular Rust tools. Gallant provides a comprehensive overview of error handling patterns, from basic `Result` usage to advanced composition techniques. The article includes detailed examples of building custom error types and discusses the philosophy behind Rust's approach to error handling compared to exceptions in other languages.
```

```reading
style: article
title: Designing error types in Rust
url: https://mmapped.blog/posts/12-rust-error-handling
author: Roman Kashitsyn
archived: mmapped-rust-error-handling.pdf
---
Kashitsyn provides practical guidance on designing effective error types in Rust, with a focus on balancing expressiveness with usability. The article covers error composition patterns, the trade-offs between different error representations, and how to design errors that scale with your application's complexity. Includes real-world examples and performance considerations for different error handling approaches.
```
