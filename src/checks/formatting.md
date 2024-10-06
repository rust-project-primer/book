# Formatting



Ensuring consistent formatting across a project helps reduce friction. It
allows Code Reviews to focus on the content, and not the formatting of the
code. 

One thing the Rust community does very well is have a consistent
formatting style, which eliminates the surprises you will encounter when
reading other people's code. The canonical tool used for formatting Rust code
is `rustfmt`.

## Rustfmt

Rust code can be formatted using [rustfmt][], which is a core piece of Rust
tooling and used by the whole Rust community.

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

Here is one example of a project which has a `rustfmt.toml` to configure
rustfmt, and some CI steps which enforce the formatting in CI.

```files
path = "check-formatting"
git_ignore = true
```

## Reading

[How to configure rustfmt](https://rust-lang.github.io/rustfmt/)

*Overview of all of the configuration options of Rustfmt. In general, you
should not need to tweak these: the defaults that it comes with out-of-the-box
are sane and used by the majority of Rust projects. However, if you have a good
reason, you can look around here and configure Rustfmt. Keep in mind that using
a non-standard Rustfmt configuration might alienate some developers.*

[The Rust Style Guide](https://doc.rust-lang.org/stable/style-guide/index.html) by The Rust Foundation

*Style guide issued by the Rust foundation. This is a concise document that
outlines good style recommendations for Rust code. Usually, reading these is
not as important because Rustfmt will enforce these automatically, but it can
be useful to read.*

[rustfmt-conf]: https://rust-lang.github.io/rustfmt/
[rustfmt]: https://github.com/rust-lang/rustfmt
