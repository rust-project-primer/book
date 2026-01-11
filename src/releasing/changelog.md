# Changelog

As you release new versions of your software, you often times need to communicate important
changes of your software to users and developers. If your software is user-facing, these
changes have to do with the user experience, such as new features, bug fixes, and performance 
improvements. If your software is a library, these changes have to do with the API, such as new
functions, bug fixes, and performance improvements and targets developers using your library.

Semantic versioning tells you the *type* of change—whether it's breaking, adds features, or fixes 
bugs—but a changelog documents *what specifically changed*. While Rust's strong type system and 
testing culture mean breaking changes are often caught at compile time, a well-maintained 
changelog remains valuable for understanding the evolution of an API and making informed 
decisions about upgrades.

Changelogs typically live in one of two places. Many projects maintain a `CHANGELOG.md` file in the
repository root, updated during development or just before release. Others use GitHub/GitLab 
releases, writing detailed notes when tagging a version. Some projects do both, auto-generating 
`CHANGELOG.md` from release notes or vice versa.

## Changelog Example

A common format is specified in [Keep A Changelog](https://keepachangelog.com/en/1.1.0/), which 
organizes changes by version and category—Added, Changed, Deprecated, Removed, Fixed, and Security.
Here's what that looks like:

```markdown
## [1.2.0] - 2024-01-15
### Added
- New `parse_config` function for reading configuration files

### Fixed
- Fixed panic when handling empty input in `process_data`
```

The version header often links to a diff or tag. For breaking changes, be explicit about what changed and how to
migrate. Many Rust projects also note the minimum supported Rust version (MSRV) changes in their changelogs, since
this affects when users can upgrade.

You can see some example crates for the [rand][rand-changelog], [hashbrown][hashbrown-changelog], 
[bitflags][bitflags-changelog] crates.

[rand-changelog]: https://github.com/rust-random/rand/blob/master/CHANGELOG.md
[hashbrown-changelog]: https://github.com/rust-lang/hashbrown/blob/master/CHANGELOG.md
[bitflags-changelog]: https://github.com/bitflags/bitflags/blob/master/CHANGELOG.md

## Cargo-Release

[`cargo-release`][] automates the release process for Rust crates, including changelog management. It can generate
changelog entries from Git history, particularly if you use conventional commit messages. This approach trades manual
curation for consistency and reduced effort. See [FAQ: Maintaining Changelog][maintain-changelog] for details on
how it handles changelogs. Internally, it uses [git-cliff][].

There are some alternatives to this tool, for example [release-plz](https://release-plz.dev/).

[git-cliff]: https://git-cliff.org/
[`cargo-release`]: https://github.com/crate-ci/cargo-release
[maintain-changelog]: https://github.com/crate-ci/cargo-release/blob/master/docs/faq.md#maintaining-changelog

## Reading

```reading
style: article
title: Keep A Changelog
url: https://keepachangelog.com/en/1.1.0/
author: Olivier Lacan
archived: keep-a-changelog.pdf
---
Keep A Changelog is a specification for how to structure changelogs. It
attempts to standardize their structure and make them useful, and explains why
they are useful.
```
