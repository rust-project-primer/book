# Expand Macros

Rust makes extensive use of metaprogramming through its support for both
declarative macros and procedural macros. Both of these allow you to avoid
repeating similar code, but through different mechanisms. Declarative macros
work like a pattern matching and replacement mechanisms, allowing you to
define your own expansions. Procedural macros, on the other hand, are more
complex: they work by you writing a little Rust program, which is called
with the input (as parsed tokens), and are able to perform any kinds of
substitutions on it.

These mechanisms are very powerful, allowing you to achieve very complex
things. For example, often times Prodecural macros are used to derive trait
implementations for your structs, as is the case with `serde`.

However, this has the downside that the code you write is not the code
that gets compiled: first, it is expanded by invoking the macro. When you
are trying to debug macro-heavy code, this can be an issue, because you
cannot see what the compiler actually gets as input.

## Cargo Expand

For this reason, [cargo expand](https://github.com/dtolnay/cargo-expand)
exists, which is a Cargo plugin that allows you to view your code after macro
expansion. It does some other nice things, such as formatting and
syntax-highlighting your code for easier consumption.

## Examples


## Reading

[Chapter 19.5: Macros](https://doc.rust-lang.org/book/ch19-06-macros.html) in The Rust Book

*Section in The Rust Book introducing and explaining macros.*

[Rust Macros and inspection with cargo expand](https://medium.com/@adamszpilewicz/rust-macros-and-inspection-with-cargo-expand-9236b6ccff17) by Adam Szpilewicz

*Adam explains Rust macros and how they can be inspected with `cargo expand`.*
