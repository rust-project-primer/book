# Changelog

When you release a new version, users and downstream developers need to know
what changed. Semantic versioning tells them the _kind_ of change (breaking,
feature, or fix), but a changelog documents _what specifically changed_ and why
it matters. For libraries, this means new API surface, deprecations, and
migration instructions. For applications, this means user-visible features,
bugfixes, and behavioral changes.

Changelogs typically live in a `CHANGELOG.md` file in the repository root,
updated during development or just before release. Some projects also use
GitHub/GitLab releases to write notes when tagging a version, or generate one
from the other automatically.

## Format

A common format is specified by [Keep A Changelog](https://keepachangelog.com/),
which organizes changes by version and category (Added, Changed, Deprecated,
Removed, Fixed, Security):

```markdown framed title="CHANGELOG.md"
## [1.2.0] - 2024-01-15

### Added

- New `parse_config` function for reading configuration files

### Fixed

- Fixed panic when handling empty input in `process_data`
```

The version header often links to a diff or tag. For breaking changes, be
explicit about what changed and how to migrate. Many Rust projects also note
MSRV changes in their changelogs, since bumping the minimum supported Rust
version affects when users can upgrade.

For real-world examples, see the changelogs of [rand][rand-changelog],
[hashbrown][hashbrown-changelog], and [bitflags][bitflags-changelog].

[rand-changelog]: https://github.com/rust-random/rand/blob/master/CHANGELOG.md
[hashbrown-changelog]:
  https://github.com/rust-lang/hashbrown/blob/master/CHANGELOG.md
[bitflags-changelog]:
  https://github.com/bitflags/bitflags/blob/master/CHANGELOG.md

## `git-cliff`

[`git-cliff`][git-cliff] is a changelog generator that creates structured
changelogs from your Git commit history. It works best when your project follows
[Conventional Commits](https://www.conventionalcommits.org/) (commit messages
like `feat: add config parser` or `fix: handle empty input`), but its
regex-powered parser can be configured to work with other commit message styles.

```bash
cargo install git-cliff
git-cliff --init  # generate a cliff.toml configuration
git-cliff         # generate changelog from commit history
```

`git-cliff` outputs in the Keep A Changelog format by default and can be
configured to group commits by type, filter out certain categories, and link to
issues or pull requests. It is used internally by both `cargo-release` and
`release-plz`.

## `cargo-release`

[`cargo-release`][cargo-release] automates the full release workflow for Rust
crates: bumping the version in `Cargo.toml`, updating the changelog (using
`git-cliff`), creating a git tag, and publishing to crates.io. It handles
workspace releases where multiple crates need coordinated version bumps.

```bash
cargo install cargo-release
cargo release patch  # bump patch version, update changelog, tag, publish
```

See the [changelog FAQ][maintain-changelog] for details on how it manages
changelog entries.

## `release-plz`

[`release-plz`](https://release-plz.dev/) takes a CI-first approach to
releasing. Rather than running release commands locally, it runs in your CI
pipeline and opens a _release PR_ that contains the version bump, updated
changelog, and any other metadata changes. When you merge the PR, it
automatically creates git tags, publishes to crates.io, and creates
GitHub/GitLab releases.

`release-plz` uses `git-cliff` for changelog generation and
[`cargo-semver-checks`](../checks/semver.md) to detect whether a change is
breaking, ensuring the version bump matches the actual API change. This makes it
a good fit for projects that want fully automated releases gated by code review.

[git-cliff]: https://git-cliff.org/
[cargo-release]: https://github.com/crate-ci/cargo-release
[maintain-changelog]:
  https://github.com/crate-ci/cargo-release/blob/master/docs/faq.md#maintaining-changelog

## Reading

```reading
style: article
title: Keep A Changelog
url: https://keepachangelog.com/en/1.1.0/
author: Olivier Lacan
archived: keep-a-changelog.pdf
---
The specification that defines how changelogs should be structured: one
section per version, categorized by type of change (Added, Changed, Fixed,
etc.), with the most recent version first. Short and worth reading — the FAQ
section addresses common questions like whether changelogs should be
auto-generated (the author argues no, but the Rust ecosystem tooling makes
auto-generation practical).
```

```reading
style: article
title: Conventional Commits
url: https://www.conventionalcommits.org/
author: Conventional Commits Project
---
A specification for writing commit messages that tools like `git-cliff` and
`release-plz` can parse automatically. Commits are prefixed with a type
(`feat:`, `fix:`, `chore:`) and optionally a scope. Breaking changes are
marked with `!` or a `BREAKING CHANGE:` footer. Adopting this convention is
not required for changelog generation, but it makes the output much better.
```
