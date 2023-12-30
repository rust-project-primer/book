# Library-First

*You have written some tooling in Rust. It is implemented
in a single, binary crate. You have published this on on [crates.io]. Some of your users
like to use parts of your crate or embed it in their own tooling, and they are
complaining that they cannot add it as a dependency.
How could you make this available easily?*

## Library-First

When writing code that is primarily designed to be a binary, my approach is to
always structure it as a library which implements all of the hard work, with a
small binary wrapper. This design makes it easy to compose, extend or embed
your tool into other tools.

```admonish
In Rust, a single crate can house both a library and a binary.
```

In Rust, the entry point for binaries is typically `main.rs`, while the entry point
for libraries is `lib.rs`. My recommendation is to keep the `main.rs` as small
as possible by implementing everything in `lib.rs`.

```admonish
When building binary crates, the crate should be designed from the ground up to
be a library exposing the necessary functionality, and a thin wrapper around it
that is a binary which excises the functionality.
```

### Examples


## Reading

- [Chapter 2.5: Package Layout](https://doc.rust-lang.org/cargo/guide/project-layout.html) in *The Cargo Book*

[crates.io]: https://crates.io/
