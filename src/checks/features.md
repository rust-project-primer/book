# Crate Features

Crate features let you gate functionality behind compile-time flags, reducing
build times and dependency footprint for users who don't need everything. But
features introduce a combinatorial testing problem: code that compiles with all
features enabled can break when only a subset is active. These bugs are easy to
introduce (a refactored `#[cfg]` block, a missing feature gate on a new
function) and hard to catch without testing each combination.

## The Problem

Consider a crate that provides multiple parsers behind feature flags. Each
parser is gated with `#[cfg(feature = "...")]`, and there is a convenience
function that dispatches to the right parser based on the input:

```rust
{{#include ../../examples/crate_features/src/lib.rs:cfg_bug}}
```

When both `json` and `yaml` are enabled, this works fine. But when only `json`
is enabled, `parse_auto` still compiles (because of the `any(...)` gate), yet
calling it with non-JSON input will panic because the `yaml` fallback branch is
compiled out. The test that covers `parse_auto` is gated behind
`#[cfg(all(feature = "json", feature = "yaml"))]`, so it never runs with
individual features:

```rust
{{#include ../../examples/crate_features/src/lib.rs:test_parse_auto}}
```

This is a common pattern: tests are written against the "all features enabled"
configuration, and bugs in individual feature combinations go unnoticed until a
user hits them. Similar to using `#ifdef` statements in C and C++, using
`#[cfg]` blocks is inherently brittle. Using a crate such as [`cfg_if`][cfg_if]
can help make it more manageable, but it does not address the root issue: you
really need to test your code for all feature combinations.

## `cargo-hack`

[`cargo-hack`][cargo-hack] is a Cargo subcommand that lets you run a command
(such as `cargo check` or `cargo test`) for every possible feature or every
possible combination of features. This catches `#[cfg]`-related compilation
failures and test gaps that only appear with specific feature sets.

### Installation

```bash
cargo install cargo-hack
```

### Feature Sets

You need to tell `cargo-hack` which sets of features to test. The two main
options are `--each-feature` and `--feature-powerset`. To illustrate the
difference, consider a crate with features `a`, `b`, and `c`:

| Flag                 | Feature Sets                                        |
| -------------------- | --------------------------------------------------- |
| `--each-feature`     | (none); `a`; `b`; `c`                               |
| `--feature-powerset` | (none); `a`; `b`; `c`; `a,b`; `a,c`; `b,c`; `a,b,c` |

The `--each-feature` flag tests each feature in isolation (plus no features at
all). This is fast and catches the most common issues: code that compiles with
all features but breaks when a single feature is enabled on its own.

The `--feature-powerset` flag tests every possible combination. This is thorough
but grows exponentially with the number of features. For a crate with `n`
features, it produces 2^n combinations. For crates with many features, you can
limit the depth with `--depth`:

```bash
# Test all combinations of up to 2 features at a time
cargo hack check --feature-powerset --depth 2
```

### Commands

You also need to tell `cargo-hack` what command to run:

| Command | Description                                              |
| ------- | -------------------------------------------------------- |
| `check` | Runs `cargo check` for each of the selected feature sets |
| `test`  | Runs `cargo test` for each of the selected feature sets  |

Using `check` verifies that every feature combination compiles. Using `test`
goes further and runs your test suite for each combination, catching runtime
issues that only manifest with specific feature sets. Checking is much faster
than testing, so a common strategy is to use `check` with `--feature-powerset`
and `test` with `--each-feature`.

### Examples

Checking that all individual features compile:

```bash
cargo hack check --each-feature
```

Running tests for every feature combination:

```bash
cargo hack test --feature-powerset
```

For workspace projects, you can run cargo-hack across all members:

```bash
cargo hack check --each-feature --workspace
```

```admonish tip
A practical CI configuration is to run `cargo hack check --feature-powerset
--depth 2` to catch compilation issues across combinations, combined with
`cargo hack test --each-feature` to verify tests pass for each feature in
isolation. This balances thoroughness with CI runtime.
```

## `cargo-features-manager`

[`cargo-features-manager`][features-manager] is a terminal UI tool that helps
you manage the features of your dependencies. It shows which features each of
your dependencies has and lets you toggle them interactively. This is useful for
auditing your dependency tree and disabling features you don't need, which
reduces compile times and binary size.

## CI Examples

```admonish example title="Feature checking in GitHub Actions"
~~~yaml
name: Features
on: [pull_request]

jobs:
  feature-check:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - uses: taiki-e/install-action@v2
        with:
          tool: cargo-hack
      - run: cargo hack check --feature-powerset --depth 2
      - run: cargo hack test --each-feature
~~~
```

```admonish example title="Feature checking in GitLab CI"
~~~yaml
features:
  image: rust:latest
  script:
    - cargo install cargo-hack
    - cargo hack check --feature-powerset --depth 2
    - cargo hack test --each-feature
~~~
```

[cfg_if]: https://docs.rs/cfg-if/latest/cfg_if/
[cargo-hack]: https://github.com/taiki-e/cargo-hack
[features-manager]: https://github.com/ToBinio/cargo-features-manager

## Reading

```reading
style: article
title: Tips for faster Rust compile times
url: https://corrode.dev/blog/tips-for-faster-rust-compile-times/#disable-unused-features-of-crate-dependencies
author: Corrode
---
This article covers many strategies for reducing Rust compile times, including
a section on disabling unused features of your crate dependencies. The
`cargo-features-manager` tool is highlighted as a way to audit and trim
unnecessary features.
```
