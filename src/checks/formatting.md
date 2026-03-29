# Formatting

Consistent formatting removes an entire category of friction from collaboration.
Code reviews focus on substance rather than style, and contributors don't need
to guess where to put braces or how far to indent. Rust has a standard
formatter, `rustfmt`, that ships with the toolchain and is used across nearly
all Rust projects. Because the community has converged on a single style, Rust
code looks the same whether it is an open-source library or an internal
codebase.

## Rustfmt

Rustfmt parses your Rust source files, applies formatting rules, and writes the
result back. It usually comes preinstalled with Rust, or can be added with
`rustup component add rustfmt`. To format all code in a package:

```
cargo fmt
```

To check whether code is formatted without modifying it (useful in CI), pass
`--check`. This returns a nonzero exit code if any file needs formatting:

```
cargo fmt --check
```

### Configuration

Rustfmt's defaults are intentionally opinionated and used by the vast majority
of Rust projects. If you need to override specific rules (for example, to change
how imports are grouped), you can create a `rustfmt.toml` or `.rustfmt.toml`
file in the project root. See the [configuration reference][rustfmt-conf] for
available options.

Some options are unstable and require a nightly toolchain:

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

### Format on Save

Most editors can be configured to run `rustfmt` automatically when you save a
file, so formatting never falls out of sync. In Zed, add the following to your
`settings.json`:

```json
{
  "format_on_save": "on"
}
```

In VS Code with the rust-analyzer extension, enable format on save in
`settings.json`:

```json
{
  "editor.formatOnSave": true
}
```

### Format before Commit

If you want to ensure formatting even when someone forgets to enable
format-on-save, you can add a Git pre-commit hook that runs `cargo fmt --check`
and rejects the commit if any file is not formatted. Tools like
[lefthook](https://github.com/evilmartians/lefthook) or
[pre-commit](https://pre-commit.com/) make it straightforward to manage these
hooks across a team. This is a lighter-weight alternative to catching formatting
issues in CI, since it provides feedback before the code is even pushed.

### Format with Nix

If your project uses Nix, you can define a formatter app in your flake that runs
`rustfmt` (and any other formatters you need) with pinned versions. This lets
contributors run `nix run .#fmt` to format everything without installing tools
manually, and ensures that everyone uses the exact same formatter version
regardless of what is installed on their system.

## Formatting TOML

Rust projects also contain TOML configuration files (`Cargo.toml`,
`rustfmt.toml`, `deny.toml`, etc.) that benefit from consistent formatting.
[Taplo][taplo] is a TOML formatter and validator that can sort keys, normalize
whitespace, and check for syntax errors. Like `rustfmt`, it supports a `--check`
flag for CI usage.

## CI Examples

```yaml framed title=".github/workflows/fmt.yml"
name: Format
on: [pull_request]

jobs:
  fmt:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@nightly
        with:
          components: rustfmt
      - run: cargo +nightly fmt --check
```

Using nightly `rustfmt` in CI ensures that unstable configuration options are
applied. If you only use stable options, `dtolnay/rust-toolchain@stable` is
sufficient.

```yaml framed title=".gitlab-ci.yml"
fmt:
  image: rust:latest
  script:
    - rustup component add rustfmt
    - cargo fmt --check
```

## Reading

```reading
style: article
title: Configuring Rustfmt
url: https://rust-lang.github.io/rustfmt/
author: Rustfmt Project
---
Full reference of all `rustfmt` configuration options: import grouping, brace
style, line width, comment formatting, and more. Most projects never need to
change the defaults, but this is where to look if you have a specific rule you
want to override. Keep in mind that non-standard configuration can surprise
contributors who expect the community defaults.
```

```reading
style: book
title: The Rust Style Guide
url: https://doc.rust-lang.org/stable/style-guide/index.html
author: The Rust Foundation
---
The official style guide that `rustfmt` implements. Covers indentation (4
spaces), line width (100 characters), trailing commas, blank lines, and
formatting rules for items, expressions, types, and attributes. Since `rustfmt`
enforces these rules automatically, reading the guide is mainly useful for
understanding the reasoning behind specific formatting decisions.
```

[rustfmt-conf]: https://rust-lang.github.io/rustfmt/
[rustfmt]: https://github.com/rust-lang/rustfmt
[taplo]: https://taplo.tamasfe.dev/
