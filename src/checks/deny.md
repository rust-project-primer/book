# Dependency Linter

When building on software, there are situations where you have some constraints
on what external dependencies you are allowed to depend. The `cargo-deny` tool
allows you to do just that. It has the ability to, amongst other things:

- Put constraints on the licenses that your dependencies have
- Disallow dependencies for which vulnerabilities are known
- Disallow dependencies with not explicit version set (`*`)
- Detect when multiple versions of the same crate are used

You can use it by installing `cargo-deny`:

```
cargo install cargo-deny
```

Initializing a new configuration:

```
cargo deny init
```

And finally checking if the current project satisfies the constraints set in
the `deny.toml` file:

```
cargo deny check
```

## Usage

### GitHub

*TODO*

### GitLab

*TODO*

## Reading

- [Cargo Deny Book](https://embarkstudios.github.io/cargo-deny/)
- GitHub: [EmbardStudios/cargo-deny][cargo-deny]

[cargo-deny]: https://github.com/EmbarkStudios/cargo-deny
