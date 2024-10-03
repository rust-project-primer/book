# Lints

*You have noticed that a lot of trivial mistakes exist in the project. You
would like to mechanically prevent these from passing code reviews. How can you
mechanically detect them?*

In programming, *linting* refers to the process of performing static code
analysis on a software project to flag programming errors, bugs, stylistic
errors and suspicious constructs. The term originates from an old UNIX tool
named `lint`, which was used to check C programs for common mistakes.

The Rust community has an excellent linting tool named Clippy.

## Clippy

The linter that is typically used in Rust is [Clippy][clippy]. It is commonly
used in Rust projects to enforce good practises, recognize unsafe or slow code.
It is also [configurable][clippy-lints].

```admonish info
When enforcing coding style, it goes beyond mere aesthetic concerns. Coding style
linters can do a lot more:

- Detect patterns that have cleaner alternatives
- Detect code that is correct, but slow
- Disallow writing unsafe code
```

It usually comes preinstalled when installing Rust through Rustup, or it can be
added later by running

```
rustup component add clippy
```

What makes Clippy interesting is that it is quite configurable. Lints can be
enabled either individually, or in groups. Some default lint groups are enabled
by default, but the list[^clippy-lints] can be examined to pick out ones that
are relevant for the project.

### Example: Overriding Lints in Code

Instead of making sure during code reviews that no unsafe code is written, a Clippy
annotation can be added to the crates that should not have unsafe code in them.

```rust
#![deny(clippy::unsafe_code)]
```

### Example: Overriding Lints in Cargo.toml



## Reading

[Static Analysis](https://abseil.io/resources/swe-book/html/ch20.html) in Software Engineering at Google

[Rust Lints you may not know](https://www.possiblerust.com/pattern/rust-lints-you-may-not-know) by Andrew Lilley Brinker


[clippy]: <https://github.com/rust-lang/rust-clippy>
[clippy-lints]: <https://rust-lang.github.io/rust-clippy/>
