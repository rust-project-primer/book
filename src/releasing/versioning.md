# Versioning

Rust comes with built-in support for [Semantic Versioning][semver], and you
should use it unless you have a strong reason not to.

Semantic versioning encodes information into the version string. A version looks
like `1.2.3`, where the three numbers are called _major_, _minor_, and _patch_:

- **Patch** (1.2.3 → 1.2.4): bugfixes only, no interface changes. Always safe to
  apply.
- **Minor** (1.2.3 → 1.3.0): new functionality that does not break existing
  users.
- **Major** (1.2.3 → 2.0.0): backwards-incompatible changes.

## Pre-1.0 Versions

Crates with a `0.x.y` version are treated differently by Cargo. Before 1.0, the
semver rules are shifted: a minor bump (`0.1.0` → `0.2.0`) is treated as a
breaking change, and a patch bump (`0.1.0` → `0.1.1`) can include new features.
This means `"0.2"` in a `Cargo.toml` dependency is interpreted as
`>=0.2.0, <0.3.0`, not `>=0.2.0, <1.0.0`.

This convention exists because pre-1.0 crates are expected to have unstable
APIs. Many crates in the Rust ecosystem stay at 0.x for a long time, so this is
worth understanding.

## How Cargo Interprets Versions

Cargo uses version requirements, not exact versions, when specifying
dependencies. The shorthand `"1.2"` is syntactic sugar for `>=1.2.0, <2.0.0`.
This means Cargo will always resolve to the latest compatible version within
that range.

```toml
[dependencies]
serde = "1.0"       # >=1.0.0, <2.0.0
uuid = "0.8"        # >=0.8.0, <0.9.0
rand = "=0.8.5"     # exactly 0.8.5
```

The caret (`^`), tilde (`~`), and wildcard (`*`) operators provide finer
control. The Cargo Book's [Specifying Dependencies][specifying-deps] chapter
covers all of these in detail.

## Prereleases

If you want to make a prerelease of an upcoming version — for example to let
users test it before the final release — you can add a hyphen suffix. For
example, `1.3.0-rc.1` is a release candidate for version 1.3.0. Cargo will not
resolve to prereleases unless explicitly requested, so existing users are not
affected.

## Build Metadata

Semver also supports a `+` suffix for build metadata: `1.3.0+build.42`. The
metadata after the `+` is ignored for version precedence — `1.3.0+build.42` and
`1.3.0+build.43` are considered the same version. This is used to attach
information about the build environment (commit hash, build date, CI job ID)
without affecting version resolution.

Cargo currently ignores build metadata entirely, so it has no effect on
dependency resolution. It can still be useful for tracking which exact build
produced a binary, for example by embedding `1.3.0+abc1234` where `abc1234` is
the git commit hash.

This metadata is commonly used for two purposes:

- If your crate is a Rust interface to an existing library (for example, a C
  library), you can use the metadata to denote which version of the library it
  wraps. For example, if your crate wraps `libxyz` version 1.5, you could
  release it as `1.16.0+libxyz1.5`. The crate is versioned independently
  (because you might update it to improve bindings or abstractions), but you
  still communicate the version of the underlying library.
- If your crate implements a specification, for example XML version 1.2, you can
  release it as `2.15.2+xml1.2` to communicate which version of the spec your
  crate implements.

## Enforcing Correct Versioning

Getting semver right manually is difficult, especially for subtle breaking
changes (see the [Semantic Versioning](../checks/semver.md) chapter for
examples). The [`cargo-semver-checks`](../checks/semver.md) tool can automate
this by comparing your crate against the published version and detecting whether
the changes are patch, minor, or major.

[semver]: https://semver.org/
[specifying-deps]:
  https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html
