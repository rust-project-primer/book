# Formatting

Ensuring consistent formatting across a project is quite important. It allows
Code Reviews to focus on the content, and not the formatting of the code. One
thing the Rust community does very well is have a consistent formatting style,
which eliminates the surprises you will encounter when reading other people's
code.

## Rustfmt

Generally, Rust code can be formatted using `rustfmt`, which you can call like
this:

```
cargo fmt
```

In a CI system, you can check if the code is properly formatted using the
`--check` command-line flag.

```
cargo fmt --check
```

### Configuration

Rustfmt can also optionally take some [configuration][rustfmt-conf] in a
`rustfmt.toml` file. 

This allows you to override specific behaviour, for example to set how it
will group imports.

Unfortunately, this is only supported by the nightly
version of Rustfmt at the moment, so when using it you have to call `rustfmt`
like this:

```
cargo +nightly fmt
```

### Examples

Here is one example of a project which has a `rustfmt.toml` to configure
rustfmt, and some CI steps which enforce the formatting in CI.

```files
path = "check-formatting"
git_ignore = true
```

## Reading

- [Configuring Rustfmt][rustfmt-conf]
- GitHub: [rustfmt](https://github.com/rust-lang/rustfmt)

[rustfmt-conf]: https://rust-lang.github.io/rustfmt/
