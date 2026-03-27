# Dependency Minimum Versions

When you specify a dependency like `serde = "1.0"` in `Cargo.toml`, you are
declaring that any version from `1.0.0` up to (but not including) `2.0.0` should
work. In practice, Cargo always resolves to the _latest_ version within that
range. This means your CI and local builds always test against the newest
compatible release, never the lower bound.

The problem is subtle: over time, your code may start relying on a function,
trait implementation, or bugfix that was only introduced in `1.0.44`, but your
version bound still claims `1.0.0` is sufficient. A downstream user who happens
to resolve to an older version within your declared range will hit a compilation
error that you never saw. This is primarily a concern for library crates, where
you do not control which version of your dependencies your users end up with.

## `cargo-minimal-versions`

[`cargo-minimal-versions`][cargo-minimal-versions] automates testing against the
lowest versions your `Cargo.toml` allows. Under the hood, it uses Cargo's
unstable `-Z minimal-versions` flag, but wraps the multi-step process (updating
the lockfile with minimal versions, then running the check) into a single
command. It also handles workspace complications that make the raw flag
difficult to use correctly.

It requires a nightly toolchain and [`cargo-hack`][cargo-hack] (for proper
workspace handling):

```bash
cargo install cargo-minimal-versions
cargo minimal-versions check --workspace
```

For workspaces, the `--ignore-private` flag skips binaries and private crates
that are not published and therefore don't need to worry about downstream
version resolution:

```bash
cargo minimal-versions check --workspace --ignore-private
```

If some transitive dependencies have incorrect lower bounds (a common problem in
the ecosystem), the `--direct` flag resolves only your direct dependencies to
their minimum versions while letting indirect dependencies resolve normally:

```bash
cargo minimal-versions check --workspace --direct
```

## CI Examples

```admonish example title="Minimum version check in GitHub Actions"
~~~yaml
name: Minimum versions
on: [pull_request]

jobs:
  minimal-versions:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@nightly
      - uses: taiki-e/install-action@v2
        with:
          tool: cargo-hack,cargo-minimal-versions
      - run: cargo minimal-versions check --workspace --ignore-private --direct
~~~
```

```admonish example title="Minimum version check in GitLab CI"
~~~yaml
minimal-versions:
  image: rust:latest
  script:
    - rustup toolchain install nightly
    - cargo install cargo-hack cargo-minimal-versions
    - cargo minimal-versions check --workspace --ignore-private --direct
~~~
```

## Reading

```reading
style: book
title: "Chapter 3.1: Specifying Dependencies"
url: https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html
author: The Cargo Book
---
Reference for dependency version syntax in `Cargo.toml`. Explains the
shorthand (`"1.2"` means `>=1.2.0, <2.0.0`), caret and tilde requirements,
wildcard versions, and how Cargo interprets version bounds. Necessary
background for understanding why minimum version testing matters.
```

```reading
style: book
title: "Chapter 3.14: Dependency Resolution"
url: https://doc.rust-lang.org/cargo/reference/resolver.html#semver-compatibility
author: The Cargo Book
---
Explains how Cargo's resolver picks versions given the constraints from your
`Cargo.toml` and your dependencies' constraints. Covers the default behavior
of resolving to the maximum compatible version, which is the root cause of the
minimum version problem this chapter addresses.
```

```reading
style: book
title: "Chapter 3.18: Unstable Features — minimal-versions"
url: https://doc.rust-lang.org/cargo/reference/unstable.html#minimal-versions
author: The Cargo Book
---
Documents the unstable `-Z minimal-versions` and `-Z direct-minimal-versions`
flags. The former resolves all dependencies (including transitive) to their
minimum versions; the latter only resolves direct dependencies minimally while
letting transitive ones resolve normally. Both require a nightly toolchain.
`cargo-minimal-versions` wraps these flags into a more practical workflow.
```

```reading
style: article
title: "Rust minimum versions: SemVer is a lie!"
url: https://blog.illicitonion.com/rust-minimum-versions-semver-is-a-lie/
author: Daniel Wagner-Hall
---
Tests a 50,000-line project with 134 transitive dependencies against
`-Z minimal-versions` and finds widespread breakage: ancient crate versions
like `log 0.1.0` no longer compile with modern Rust, yet many popular libraries
still declare them as acceptable lower bounds. Argues that the ecosystem needs
either enforcement at publish time or a different approach to version bounds.
The article is from 2019 but the underlying problem persists.
```

[cargo-minimal-versions]: https://github.com/taiki-e/cargo-minimal-versions
[cargo-hack]: https://github.com/taiki-e/cargo-hack
