# Binary Crates

*You have written some tooling in Rust. It is implemented in a single, binary
crate. You have published this on on [crates.io]. Some of your users like to
use parts of your crate or embed it in their own tooling, but they are
complaining that they cannot add it as a dependency. How could you structure
your crate to make this possible easily?*

One recommendation I would make is to keep most of the code for *binary crates*
(that is, crates which primarily are a binary tool) in the *library* section of
them. This approach I call *library-first development*. In doing so, you make
sure that the code you write remains reusable and embeddable.

## Library-First

In Rust, a single crate can house both a library and a binary. By default,
the entry point for the library portion of the crate is `lib.rs` and the entry
point for the binary is `main.rs`, although a single crate can define multiple
binaries (but not multiple libraries).

When you are writing a binary, the first instinct is to simply create a new
crate and put all of the code into `main.rs` and call it a day.

However, my recommendation is to keep the `main.rs` as small as possible by
implementing everything in `lib.rs`.

```admonish
When building binary crates, the crate should be designed from the ground up to
be a library exposing the necessary functionality, and a thin wrapper around it
that is a binary which excises the functionality.
```

```admonish example title="Library-first binary crate"

```


## Reading

- [Chapter 2.5: Package Layout](https://doc.rust-lang.org/cargo/guide/project-layout.html) in *The Cargo Book*

[crates.io]: https://crates.io/
