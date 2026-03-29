# Unused Dependencies

Unused dependencies cost compile time and expand the dependency graph without
providing any value. In large projects with many crates, they tend to accumulate
as code is refactored and dependencies that were once needed become orphaned.
Removing them reduces compile times, shrinks the set of dependencies that need
auditing, and lowers the maintenance burden.

```admonish tip
If you have dependencies that are only needed conditionally, use [optional
features](../organization/features.md) or [platform-specific
dependencies][pdeps] to avoid pulling them in
unnecessarily.

[pdeps]: https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html#platform-specific-dependencies
```

Two tools in the Rust ecosystem detect unused dependencies: `cargo-machete` and
`cargo-udeps`. They use fundamentally different detection strategies, which
affects both their speed and accuracy.

A related problem is unused _features_ on dependencies. Enabling optional
features can pull in additional transitive dependencies and compile code paths
you never use. There is currently no automated tooling to detect features you
enable but don't need, so this requires manual review. A common case is
depending on `tokio` with the `full` feature when only a few like `fs`, `net`,
and `macros` are actually required.

## `cargo-machete`

[`cargo-machete`][cargo-machete] detects unused dependencies by searching your
source files for references to each dependency's crate name using simple text
matching. Because it never compiles your code, it is very fast — fast enough to
run on every pull request, even in large workspaces.

```
cargo install cargo-machete
cargo machete
```

The tradeoff is precision. Since `cargo-machete` works at the text level, it
cannot detect dependencies that are used only through procedural macros or build
scripts, because the generated code is not visible to a text search. These show
up as false positives. You can suppress them by adding the crate names to an
ignore list in `Cargo.toml` under `[package.metadata.cargo-machete]`.

## `cargo-udeps`

[`cargo-udeps`][cargo-udeps] takes the opposite approach: it compiles the crate
and analyzes the compiler's output to determine which dependencies were actually
used during compilation. This makes it more accurate than `cargo-machete`, but
also significantly slower and it requires a nightly toolchain.

```
cargo +nightly udeps
```

One limitation is that `cargo-udeps` cannot detect usage from doc-tests, which
may produce false positives for dependencies only referenced in documentation
examples. These can be suppressed in `Cargo.toml` under
`[package.metadata.cargo-udeps.ignore]`.

## CI Examples

Unused dependencies affect compile time but not correctness, so these checks do
not necessarily need to run on every pull request. Running them on a schedule
(weekly, for example) or as a periodic maintenance task is a reasonable
alternative.

```yaml framed title=".github/workflows/machete.yml"
name: Unused dependencies
on:
  schedule:
    - cron: "0 9 * * 1" # Every Monday at 9:00 UTC
  workflow_dispatch:

jobs:
  machete:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: taiki-e/install-action@v2
        with:
          tool: cargo-machete
      - run: cargo machete
```

```yaml framed title=".gitlab-ci.yml"
machete:
  image: rust:latest
  rules:
    - if: $CI_PIPELINE_SOURCE == "schedule"
  script:
    - cargo install cargo-machete
    - cargo machete
```

## Reading

```reading
style: article
title: "cargo machete: find unused dependencies quickly"
url: https://bouvier.cc/2022/04/27/cargo-machete/
author: Benjamin Bouvier
---
Explains the design of `cargo-machete` and why it uses text search rather than
compiler analysis. Benchmarks it against the Rust compiler repository (1.08
seconds) and discusses the false positive tradeoff: dependencies used through
macros or build scripts are invisible to text search, so they need to be listed
as known exceptions. Also compares the "transitively-used dependency" problem
in `cargo-udeps`, where workspace-level dependency sharing can mask truly unused
crates.
```

```reading
style: article
title: "Finding unused dependencies with cargo-udeps"
url: https://fasterthanli.me/series/updating-fasterthanli-me-for-2022/part-1#finding-unused-dependencies-with-cargo-udeps
author: Amos Wenger
---
Walkthrough of using `cargo-udeps` on a real project, showing how it detected
an unused `ulid` crate. Covers the limitation that it cannot detect usage from
doc-tests and how to suppress false positives using
`package.metadata.cargo-udeps.ignore` in `Cargo.toml`.
```

```reading
style: book
title: "Item 25: Manage your dependency graph"
url: https://www.lurklurk.org/effective-rust/dep-graph.html
author: Effective Rust
---
Broader advice on managing Rust dependencies: how Cargo resolves
semver-incompatible versions, when to use version ranges versus pinning, and
using `cargo tree --duplicates` to spot redundancy. Also discusses the supply
chain risk of build scripts and procedural macros executing arbitrary code at
compile time.
```

[cargo-udeps]: https://github.com/est31/cargo-udeps
[cargo-machete]: https://github.com/bnjbvr/cargo-machete
