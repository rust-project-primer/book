# Code Documentation

Code-level documentation in Rust is almost always done using Rustdoc, which is
an incredible piece of software that makes writing usable documentation several
orders of magnitude easier than other documentation tools that I have worked
with.

## Rustdoc

Rustdoc works by parsing documentation comments that are left in the code and
turning them into a pretty, searchable HTML output. It understands Markdown for
simple formatting and is able to link things. There is also a service that
builds and renders documentation for all published Rust crates, which is
[docs.rs][docs.rs].

```rust
/// This is a documentation comment.
///
/// In here, it is also possible to link to other types, such as [`Vec`].
pub fn my_function() {
    todo!()
}
```

What is important about the Rustdoc documentation is that it is only useful if
it is published somewhere. For that reason, I suggest publishing it in the CI
on every merge to `master` (or whatever the unstable branch name is that is
used) to some location where it can be viewed by the team.

### Example

*TODO*

### Usage

*TODO*

### Enforcing

You can enforce that all public API memebers have rustdoc annotations
using the `missing_docs` lint. For example this annotation will turn all
places where documentation is missing into compile-time warnings:

```rust
#![warn(missing_docs)]
```

This is recommended for libraries, as documentation is quite important
for downstream users.

## Reading

[Rustdoc Book](https://doc.rust-lang.org/rustdoc/what-is-rustdoc.html)

[docs.rs]: https://docs.rs
