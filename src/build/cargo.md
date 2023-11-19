# Cargo

I probably do not need to convince you of this, but a large part of Rust's popularity
is perhaps not due to the language features, but to the quality of the tooling around
it. Cargo is the build system that the Rust language uses, and it is excellent.

If your project only consists of Rusty things, then you likely just need to use it.
If you have a few more complex steps that you need to do when building your
code, you can always use a build script.

The other sections of this chapter are only relevant to you if your project
consists of a mixture of languages, and building it is sufficiently complex
that it cannot trivially be expressed or implemented in a `build.rs` file (such
as: it needs external dependencies).

## Reading

- [The Cargo Book](https://doc.rust-lang.org/cargo)
- [The Cargo Book: Build Scripts](https://doc.rust-lang.org/cargo/reference/build-scripts.html)

