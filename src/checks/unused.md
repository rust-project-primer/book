# Unused Dependencies

Another issue that the Rust compiler will not catch, but can have quite an impact
is when you have unused dependencies. This can happen when you are using a crate for
something, but later switch to a different one, and forget to remove the previous
dependency.

This is an issue because every crate needs to be downloaded and compiled, regardless
of whether it is used or not. As project complexity grows, this is also not something
we humans are good at tracking. For this reason, it makes sense to use some tooling
to detect this and enforce that all dependencies must be used.

The `cargo-udeps` tool can do this for us. It can be installed like this:

```
cargo install cargo-udeps
```

Once installed, it can be invoked in a project like this:

```
cargo udeps
```

## Examples

*TODO*

## Reading

- GitHub: [est31/cargo-udeps](https://github.com/est31/cargo-udeps)
