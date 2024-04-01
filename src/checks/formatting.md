# Formatting

*You have noticed that in your project, a lot of time is wasted in code reviews
on debating over style. You have also noticed that there seems to be an
inconsistency, whereby different teams produce different-looking code. This
creates friction for developers switching between teams and when teams try to
collaborate. How can you ensure that the style issue is settled mechanically?*

Ensuring consistent formatting across a project is quite important. It allows
Code Reviews to focus on the content, and not the formatting of the code. One
thing the Rust community does very well is have a consistent formatting style,
which eliminates the surprises you will encounter when reading other people's
code. The canonical tool used for formatting Rust code is `rustfmt`.

## Rustfmt

Generally, Rust code can be formatted using [rustfmt][], which is a core piece
of Rust tooling and in general, the whole Rust community uses it.

By incorporating `rustfmt` checks into the CI system, you can make sure that
issues with formatting are caught before code review.

### Installation

Usually, `rustfmt` comes preinstalled when you install Rust. However, if you
do not have it, you can install it using:

    rustup component add rustfmt

You can run `rustfmt` against a crate like this:

```
cargo fmt
```

In a CI system, you can check if the code is properly formatted using the
`--check` command-line flag.

```
cargo fmt --check
```

If the code is not properly formatted, this will return a nonzero exit code
and cause the CI check to fail.

### Configuration

Rustfmt can also optionally take some [configuration][rustfmt-conf] in a
`rustfmt.toml` file.  This allows you to override specific behaviour, for
example to set how it will group imports.

Some configuration options are unstable at the moment and therefore require an
unstable build of Rustfmt. When using it you have to call `rustfmt` like this:

```
cargo +nightly fmt
```

### Examples

~~~admonish example
Here is one example of a project which has a `rustfmt.toml` to configure
rustfmt, and some CI steps which enforce the formatting in CI.

```files
path = "check-formatting"
git_ignore = true
```
~~~

[rustfmt-conf]: https://rust-lang.github.io/rustfmt/
[rustfmt]: https://github.com/rust-lang/rustfmt
