# Formatting

From the point of view of the Rust compiler, whitespace is insignificant. The
Rust compiler does not care how many spaces or tabs you use, how far you indent
lines, or how long the lines in your source files are.

However, code is not only read by the compiler. It is also read by humans, who
collaborate on it, read it together, and discuss it. If you have inconsistent
formatting, it can create friction when people contribute to your project.

Ensuring consistent formatting across a project helps reduce friction. It allows
Code Reviews to focus on the content, and not the formatting of the code.

To ensure that code is consistently formatted, you can use a code formatter.
This is a tool that parses your code and applies a set of rules to format it.
These rules are designed to ensure that the code is easy to read and understand.
It means that Rust code is globally consistent, no matter if it is an
open-source project, or which company you work for.

Rust comes with a code formatter called `rustfmt`. It is a core piece of Rust
tooling and used by the whole Rust community.

## Rustfmt

Rustfmt is part of the Rust toolchain and is used to format Rust code according
to a set of rules. It works by parsing your code, applying formatting rules, and
writing the formatted code back to your files.

There are a few ways you can use `rustfmt`:

- You can run it manually on your code using the `cargo fmt` command. This will
  format all code files inside your package.
- You can configure your editor to automatically format your code when you save
  it.
- You can integrate `rustfmt` into your CI system to ensure that all code is
  properly formatted before it is merged into the main branch.
- You can use `rustfmt` as a pre-commit hook to ensure that all code is properly
  formatted before it is committed to the repository.

While `rustfmt` comes with sane default configuration, it is possible to
override the rules that it uses. In general, you don't need to do this, it is
recommended to use the default configuration. However, if you do want to
override the rules, you can do so by creating a `.rustfmt.toml` file in the root
of your project.

### Installation

Usually, `rustfmt` comes preinstalled when you install Rust. However, if you do
not have it, you can install it using:

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

If the code is not properly formatted, this will return a nonzero exit code and
cause the CI check to fail.

### Configuration

Rustfmt can also optionally take some [configuration][rustfmt-conf] in a
`rustfmt.toml` file. This allows you to override specific behaviour, for example
to set how it will group imports.

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

### Format on save

If you don't want to worry about formatting, you can configure your editor to
automatically run the formatter when you save a file. Doing this ensures that
you cannot forget to run `rustfmt`, and find out that your code isn't formatted
properly due to a CI failure or during code review.

If you use the Zed editor, then you can configure it to format your code on
save. Add the following to your `settings.json` file:

```json
{
  "format_on_save": "on"
}
```

If you use VS Code, you can install the Rust extension and configure it to
format your code on save. Add the following to your `settings.json` file:

```json
{
  "rust.formatOnSave": true
}
```

Other editors have similar features. Check your editor's documentation for
instructions on how to configure it to format your code on save.

### Format before commit

### Format with Nix

## Formatting TOML Configuration

In your Rust projects, you also have some configuration files in the TOML format
that need to be formatted properly. You can use
[Taplo](https://taplo.tamasfe.dev/) to achieve this.

## Reading

```reading
style: article
title: Configuring Rustfmt
url: https://rust-lang.github.io/rustfmt/
author: Rustfmt Project
---
Overview of all of the configuration options of Rustfmt. In general, you
should not need to tweak these: the defaults that it comes with out-of-the-box
are sane and used by the majority of Rust projects. However, if you have a good
reason, you can look around here and configure Rustfmt. Keep in mind that using
a non-standard Rustfmt configuration might alienate some developers.
```

```reading
style: book
title: The Rust Style Guide
url: https://doc.rust-lang.org/stable/style-guide/index.html
author: The Rust Foundation
---
Style guide issued by the Rust foundation. This is a concise document that
outlines good style recommendations for Rust code. Usually, reading these is
not as important because Rustfmt will enforce these automatically, but it can
be useful to read.
```

[rustfmt-conf]: https://rust-lang.github.io/rustfmt/
[rustfmt]: https://github.com/rust-lang/rustfmt
