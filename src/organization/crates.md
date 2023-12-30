# Small Crates

Languages such as C, C++ and Java have files as their smallest compilation unit. This means
that when you edit some files, only those files that you have edited need to be rebuilt.

Rust has a different approach: in rust, the smallest compilation unit is a crate. This means
that Rust can do more powerful optimizations, because the compiler can always see the entire
crate. However, it also means that it drives compilation times higher, because any single
change you make means that your entire crate needs to be built again.

One piece of good advice is to try to break functionality down into smaller crates. You
want to end up in a project where it is easy and encouraged to pull functionality out into
separate crates. This is useful because it encourages good software design, and it has the
added bonus that it allows for faster compilation and thereby development.

## Workspace

Breaking functionality down into crates and proactively pulling them out encourages good
software design. It allows you to build useful abstractions, and it avoids ending up with
a giant, complex monolithic application that is very tightly coupled. This achieves [Loose coupling](https://en.wikipedia.org/wiki/Loose_coupling).


### Examples

Here is an example of what a cargo workspace project looks like.

```files
path = "cargo-workspace"
git_ignore = true
default_file = "Cargo.toml"
```


## Reading

- [Chapter 7: Managing Growing Projects with Packages, Crates and Modules](https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html) in *The Rust Programming Language*
- [Prefer small crates](https://rust-unofficial.github.io/patterns/patterns/structural/small-crates.html) in *Rust Design Patterns*
- [An Opinionated Guide To Structuring Rust Projects](https://www.justanotherdot.com/posts/an-opinionated-guide-to-structuring-rust-projects.html) by Ryan James Spencer

