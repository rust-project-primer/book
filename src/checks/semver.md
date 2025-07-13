# Semantic Versioning

_The crate you are working on is used by others, either in your own company or
externally. You often release new versions of it with added functionality.
However, sometimes you get complaints from downstream users that a newly
released version introduces breaking changes. Usually, you try to correctly
release a new major version when that happens, but sometimes you don't notice
it. How can you make sure that version numbers are correctly managed?_

Rust is built heavily on [Semantic Versioning][semver] to make it easy to
compose software and update dependencies without needing to worry about
everything breaking when dependencies are updated.

```admonish title="Semantic Versioning"
Semantic versioning, often abbreviated as SemVer, is a versioning scheme for
software that aims to convey meaning about the underlying changes in each
release. It uses a three-part version number format, *major*.*minor*.*match*
(e.g., `2.0.1`), where Major versions introduce breaking changes, Minor
versions add new features without breaking backward compatibility, and Patch
versions include bug fixes that don't affect the API.

This system helps developers and users understand the impact of updating to a
new version, ensuring more predictable and manageable software upgrades.
```

This also puts some responsibilities on you as a crate author: you have to
ensure that when you release new versions of your crates, you do not violate
semantic versioning by accidentally publishing versions with breaking changes
but not marking them as such by incrementing the major version.

Doing this manually is possible, but difficult and it does not always scale
well. The more widely used your crate is, the more frustration it causes when
you get it wrong. Thankfully, there exists some tooling that can help here by
automating the process of determining if you are correctly versioning your
crate.

## `cargo-semver-checks`

[`cargo-semver-checks`][cargo-semver-checks] is an amazing tool designed to
detect invalid semantic versioning in crates automatically, by parsing both your
crate and the current latest version and determining if the changes between them
can be considered a _patch_, a _minor_ change or a _major_ change.

As it relies of pulling the latest version of your crate from a registry, it is
only really useful for crates which are published.

You can use it by installing it with `cargo`, and running it:

```
cargo install cargo-semver-checks
cargo semver-checks
```

It will check if the version you have currently specified in the crate is
aligned with what it should be. This is a good check to run in a CI system.

```admonish example
TODO: example using cargo-semver-checks
```

## Reading

[Semantic Versioning 2.0.0][semver]

_Semantiv Versioning specification which explains the rules of how to apply it._

[Chapter 3.15: SemVer Compatibility](https://doc.rust-lang.org/cargo/reference/semver.html)
in _The Cargo Book_

_Provides details on what is conventionally considered a compatible or breaking
SemVer change for new releases of a Cargo package._

[Semver violations are common, better tooling is the answer](https://predr.ag/blog/semver-violations-are-common-better-tooling-is-the-answer/)
by Predrag Gruevski and Tomasz Nowak

_Article which analyzes how common SemVer violations are in the Rust ecosystem,
and what can be done to address this. Concludes that violations are fairly
common, and that better tooling can improve the state of versioning._

[Checking semver in the presence of doc(hidden) items](https://predr.ag/blog/checking-semver-for-doc-hidden-items/)
by Predrag Gruevski

_Explains how difficult it is to check SemVer compatibility in the presence of
hidden code in Rust, and how this was addressed in `cargo-semver-checks`.
Contains a deep dive into the implementation details and how they help to make
these checks reliable and maintainable._

[semver]: https://semver.org/
[cargo-semver-checks]: https://github.com/obi1kenobi/cargo-semver-checks
[crates.io]: https://crates.io/
