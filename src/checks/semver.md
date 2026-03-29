# Semantic Versioning

Rust's dependency ecosystem relies on [Semantic Versioning][semver] (SemVer).
When you publish a crate, your version number is a promise to downstream users:
patch releases contain only bugfixes, minor releases add functionality without
breaking existing code, and only major releases may introduce breaking changes.
Cargo's dependency resolver depends on these promises being accurate — it will
happily upgrade to a new minor release without asking, trusting that nothing
will break.

Getting this right manually is harder than it looks. Some breaking changes are
obvious (removing a public function), but others are subtle: adding a new
variant to a non-exhaustive enum, changing a type's auto-trait implementations,
or even adding a new public item can break downstream code that uses glob
imports. An analysis of the 1,000 most-downloaded crates found that roughly 1 in
6 crates violated semver at least once, affecting about 1 in 31 releases. This
is not a failure of discipline — it is a failure of tooling.

## `cargo-semver-checks`

[`cargo-semver-checks`][cargo-semver-checks] automates semver verification by
comparing your current crate against its latest published version and
determining whether the changes constitute a patch, minor, or major update. If
the version number in your `Cargo.toml` does not match the detected change
level, it reports the violations with detailed explanations.

```bash
cargo install cargo-semver-checks
cargo semver-checks
```

Since it compares against the published version from a registry, it is primarily
useful for crates that are published to [crates.io][crates-io] or a private
registry. Running it in CI prevents accidental semver violations from being
published.

## CI Examples

```yaml framed title=".github/workflows/semver.yml"
name: Semver
on: [pull_request]

jobs:
  semver:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - uses: taiki-e/install-action@v2
        with:
          tool: cargo-semver-checks
      - run: cargo semver-checks
```

```yaml framed title=".gitlab-ci.yml"
semver:
  image: rust:latest
  script:
    - cargo install cargo-semver-checks
    - cargo semver-checks
```

## Reading

```reading
style: article
title: Semantic Versioning 2.0.0
url: https://semver.org/
author: Tom Preston-Werner
---
The specification that defines the rules of semantic versioning. Short and
worth reading in full — the FAQ section addresses common edge cases like what to
do before 1.0, how to handle deprecations, and how version precedence works.
```

```reading
style: book
title: "Chapter 3.15: SemVer Compatibility"
url: https://doc.rust-lang.org/cargo/reference/semver.html
author: The Cargo Book
---
Rust-specific reference for what counts as a breaking change. Categorizes
changes as major, minor, or patch across API items, types, traits, generics,
and functions. Contains several surprising cases: adding a public item can
break code using glob imports, adding `repr(align)` prevents use in
`repr(packed)` types, and making an `unsafe` function safe is only a minor
change. Essential reading for library authors.
```

```reading
style: article
title: Semver violations are common, better tooling is the answer
url: https://predr.ag/blog/semver-violations-are-common-better-tooling-is-the-answer/
author: Predrag Gruevski and Tomasz Nowak
---
Analyzes over 14,000 releases across the 1,000 most-downloaded Rust crates and
finds 3,062 verified semver violations — roughly 1 in 31 releases and more than
1 in 6 crates affected. Categorizes the violations (missing methods, added enum
variants, removed auto traits) and argues this rate reflects tooling gaps
rather than maintainer negligence, motivating `cargo-semver-checks`.
```

```reading
style: article
title: Checking semver in the presence of doc(hidden) items
url: https://predr.ag/blog/checking-semver-for-doc-hidden-items/
author: Predrag Gruevski
---
Deep dive into how `cargo-semver-checks` handles `#[doc(hidden)]` items — code
that is publicly accessible but not part of the intended API. These accounted
for over 60% of false positives before being addressed. Explains the solution:
two synthetic properties (`public_api_eligible` and `public_api`) that
distinguish between items that are truly public and those that are merely
reachable. Also covers the edge case where re-exports can make hidden items
part of the public API.
```

[semver]: https://semver.org/
[cargo-semver-checks]: https://github.com/obi1kenobi/cargo-semver-checks
[crates-io]: https://crates.io/
