# Dependency Minimum Versions

_One component of the Rust project you are working on is a library. Everything
has been going smooth, until someone complains that your library does not work.
You are able to reproduce it locally, it seems that there is an issue with an
early version of a dependency. Your project depends on it by a range, but in
fact the early versions of the range do not work. However, in your CI setup you
are always only testing against the latest possible version. How can you make
sure these kinds of issues are caught in CI?_

To understand this issue, a bit of an explainer for how dependency versioning is
performed in Rust is required.

### Crate Dependency Versions

In Rust projects, usually [Semantic Versioning][semver] is used for versioning
crates. The _semantic_ part of that name means that versions are not just
arbitrary tuples of numbers, but they have a meaning that comes with some
stability guarantees that are necessary for writing software that does not
implode when you update dependencies.

```admonish title="Semantic Versioning"
Semantic versioning, often abbreviated as SemVer, is a versioning scheme for
software that aims to convey meaning about the underlying changes in each
release. It uses a three-part version number format, *major*.*minor*.*patch*
(e.g., `2.0.1`), where Major versions introduce breaking changes, Minor
versions add new features without breaking backward compatibility, and Patch
versions include bug fixes that don't affect the API.

This system helps developers and users understand the impact of updating to a
new version, ensuring more predictable and manageable software upgrades.
```

This allows us to specify dependencies not by their exact versions, but by their
version bounds. For example, when you have a dependency bound such as this in
your project:

```toml
name = "1.2"
```

This is in fact syntactic sugar for `>=1.2.0,<1.3.0`. You are expressing that
you need at least version `1.2.0`, but lower than `1.3.0`. The current crate
version might be at `1.2.77` and it might compile just fine. Since semantic
versioning guarantees that the API remains stable between patch releases, you
can trust that when the dependency receives an update, that the newly released
version should still work with your code.

However, there is one potential issue here: Cargo always tries to use the
maximum possible version. This means that even though version `1.2.0` is within
the range you have specified, Cargo will only ever test it with whatever is the
_latest_ version within those version bounds.

It is possible that your crate depends on some feature or fix that was not
present in `1.2.0`, but only added in `1.2.44`, but since Cargo always tests
against the latest, you will never know.

To detect this issue automatically, Cargo has a feature that allows you to
override the version resolution strategy to always use the minimum possible
version. You can enable this feature using `-Z minimum-version`.

## `cargo-minimial-versions`

The [`cargo-minimal-versions`][cargo-minimal-versions] tool helps to validate
whether your crate works with the minimal versions it advertises.

However, an easier approach is to install `cargo-minimal-version` and running it
to check if your code will compile:

```
cargo install cargo-minimal-version
cargo minimal-version check
```

## Reading

```reading
style: book
title: Semantic Versioning 2.0.0
url: https://semver.org
author: Tom Preston-Werner
---
Semantic Versioning specification which explains the rules of how to apply it.
```

```reading
style: book
title: "Chapter 3.1: Specifying Dependencies"
url: https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html
author: The Cargo Book
---
Explains how Cargo crate dependencies are specified in terms of syntax and
semantics.
```

```reading
style: book
title: "Chapter 3.14: Dependency Resolution"
url: https://doc.rust-lang.org/cargo/reference/resolver.html#semver-compatibility
author: The Cargo Book
---
Explains how Cargo resolves crate dependency versions given the version
constraints set by the dependencies section of your crate.
```

```reading
style: book
title: "Chapter 3.18: Unstable Features"
url: https://doc.rust-lang.org/cargo/reference/unstable.html#minimal-versions
author: The Cargo Book
---
Explains the Cargo features `minimal-versions` and `direct-minimal-versions`
which force Cargo to resolve (direct) dependencies to their minimal versions
instead of the latest versions.
```

```reading
style: article
title: "Rust minimum versions: SemVer is a lie!"
url: https://blog.illicitonion.com/rust-minimum-versions-semver-is-a-lie/
author: Daniel Wagner-Hall
---
Article which argues that a lot of crates are broken, because they do not
compile with the versions they specify in their manifests. Note that this
article is rather old.
```

[semver]: https://semver.org/
[cargo-minimal-versions]: https://github.com/taiki-e/cargo-minimal-versions
