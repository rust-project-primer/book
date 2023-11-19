# Style

When enforcing coding style, it goes beyond mere aesthetic concerns. Coding style
linters can do a lot more:

- Detect patterns that have cleaner alternatives
- Detect code that is correct, but slow
- Disallow writing unsafe code

The linter that is typically used in Rust is [Clippy][rust-clippy]. It usually comes
preinstalled when installing Rust through Rustup, or it can be added later by running

```
rustup component add clippy
```

What makes Clippy interesting is that it is quite configurable. Lints can be
enabled either individually, or in groups. Some default lint groups are enabled
by default, but the [list][clippy-lints] can be examined to pick out ones that
are relevant for the project.

## Examples

### Denying unsafe code

Instead of making sure during code reviews that no unsafe code is written, a Clippy
annotation can be added to the crates that should not have unsafe code in them.

```rust
#![deny(clippy::unsafe_code)]
```

## Usage

### GitHub CI

*TODO*

### GitLab CI

*TODO*

## Reading

- [Clippy Lints][clippy-lints]
- GitHub: [rust-clippy][]

[rust-clippy]: https://github.com/rust-lang/rust-clippy
[clippy-lints]: https://rust-lang.github.io/rust-clippy/
