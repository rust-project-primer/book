# Lints

In programming, _linting_ refers to the process of performing static code
analysis on a software project to flag programming errors, bugs, stylistic
errors, and suspicious constructs. The term originates from an old UNIX tool
named `lint`, which was used to check C programs for common mistakes. Linting
goes beyond what the compiler catches: it can detect patterns that have cleaner
alternatives, flag code that is correct but slow, and enforce project-specific
rules like forbidding `unsafe` code.

## Clippy

The standard linter for Rust is [Clippy][clippy]. It ships with the Rust
toolchain and is used across the ecosystem to enforce good practices, catch
common bugs, and flag performance issues. It usually comes preinstalled through
Rustup, or can be added with `rustup component add clippy`.

Clippy organizes its lints into groups such as `correctness`, `style`,
`complexity`, `perf`, and `pedantic`. The default groups are enabled out of the
box, but the full [lint list][clippy-lints] can be examined to pick out
additional lints relevant to your project. Lints can be enabled or disabled
individually or by group, either in source code or in `Cargo.toml`.

### Overriding Lints in Code

Source-level attributes let you override lint severity for a specific crate,
module, or item. For example, to forbid `unsafe` code in a crate:

```rust
#![deny(unsafe_code)]
```

Or to enable the `pedantic` lint group as warnings:

```rust
#![warn(clippy::pedantic)]
```

These attributes are useful when different crates in a workspace need different
lint policies — a low-level crate might allow `unsafe`, while application crates
forbid it.

### Overriding Lints in Cargo.toml

Since Rust 1.74, lints can also be configured in `Cargo.toml` using the
`[lints]` table. This keeps lint configuration next to the rest of the crate
metadata rather than scattered across source files. In a workspace, you can
define shared lint policy in `[workspace.lints]` and inherit it per-crate with
`lints.workspace = true`.

```toml
[lints.clippy]
pedantic = "warn"
unwrap_used = "deny"
```

```toml
# In a workspace root Cargo.toml:
[workspace.lints.clippy]
pedantic = "warn"
unwrap_used = "deny"

# In a member's Cargo.toml:
[lints]
workspace = true
```

One thing to be aware of is that `[lints]` applies uniformly to all code in the
crate: library code, tests, benchmarks, and examples. There is currently no way
to scope lint configuration to only library code or only tests through
`Cargo.toml`. If you find that a lint like `unwrap_used` is useful in library
code but too noisy in tests (where panicking on failure is normal), you can use
`#[allow(...)]` attributes on the test modules or individual test functions to
relax it selectively.

## Typos

Spelling errors in code, documentation, and error messages tend to slip through
code review because reviewers are focused on semantics, not spelling. These
mistakes accumulate into follow-up pull requests and sometimes make it into
released documentation. A spell checker designed for code can catch them
automatically.

[`typos-cli`][typos-cli] is a spell checker built specifically for source code.
It understands programming conventions (camelCase, snake_case, abbreviations)
and has a low false positive rate, making it practical to run on every pull
request even in large monorepos.

```
cargo install typos-cli
typos
```

If `typos` detects a spelling error, it outputs a nonzero exit code and a
diagnostic message explaining the error and suggesting a fix. False positives
can be suppressed by adding exceptions to a `_typos.toml` or `typos.toml`
configuration file in the project root.

## SARIF

The [Static Analysis Results Interchange Format][sarif] (SARIF) is a standard
JSON format for representing the output of static analysis tools. It is
supported by GitHub: when you upload SARIF results as part of a GitHub Actions
workflow, GitHub renders the diagnostics as annotations directly in pull request
code review, inline with the relevant lines of code.

The [`sarif-rs`][sarif-rs] project provides command-line converters that
transform the output of tools like Clippy and `cargo-audit` into SARIF. For
example, `clippy-sarif` pipes Clippy's JSON output into a SARIF file that can be
uploaded with GitHub's `upload-sarif` action. This is useful when you want lint
results to appear as code annotations rather than buried in CI logs.

## CI Examples

Both Clippy and typos are fast enough to run on every pull request. Below are
examples for GitHub Actions and GitLab CI.

```yaml framed title=".github/workflows/lints.yml"
name: Lints
on: [pull_request]

jobs:
  clippy:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
        with:
          components: clippy
      - run: cargo clippy --all-targets --all-features -- -D warnings

  typos:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: crate-ci/typos@master
```

```yaml framed title=".gitlab-ci.yml"
clippy:
  image: rust:latest
  script:
    - rustup component add clippy
    - cargo clippy --all-targets --all-features -- -D warnings

typos:
  image: rust:latest
  script:
    - cargo install typos-cli
    - typos
```

## Reading

```reading
style: book
title: "Chapter 20: Static Analysis"
url: https://abseil.io/resources/swe-book/html/ch20.html
author: Software Engineering at Google
---
Chapter on static analysis from Google's software engineering book. Covers the
philosophy of static analysis at scale, focusing on keeping false positive rates
low enough that developers trust and act on the results.
```

```reading
style: article
title: Rust Lints you may not know
url: https://www.possiblerust.com/pattern/rust-lints-you-may-not-know
author: Andrew Lilley Brinker
---
Walks through lesser-known Rust lints that can catch subtle issues. Good for
discovering lints beyond the defaults that might be relevant to your project.
```

[clippy]: https://github.com/rust-lang/rust-clippy
[clippy-lints]: https://rust-lang.github.io/rust-clippy/
[typos-cli]: https://github.com/crate-ci/typos
[sarif]: https://sarifweb.azurewebsites.net/
[sarif-rs]: https://github.com/psastras/sarif-rs
