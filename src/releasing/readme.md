# Releasing

Releasing means publishing artifacts that others can use: source code to a
package registry, compiled binaries as downloadable assets, system packages for
distribution through a package manager, or container images for deployment. A
release also communicates what changed, through version numbers and a changelog,
so that users and downstream maintainers can decide when and how to upgrade.

## Communication

Every release needs to answer two questions: what version is this, and what
changed? [Versioning](versioning.md) covers semantic versioning, how Cargo
interprets version ranges, and conventions around pre-1.0 crates and
prereleases. [Changelog](changelog.md) covers the _Keep A Changelog_ format and
tools like `git-cliff` that generate changelogs from commit history. Together,
version numbers and changelogs give downstream users enough information to
decide whether an upgrade is safe and worth the effort.

## Distribution

What you publish depends on what you're building. A library publishes to a
[crate registry](crates.md), either crates.io or a private registry for internal
code. An application has more options: [container images](containers.md) for
server deployments, [system packages](packaging.md) like `.deb` files for
end-user installation, or compiled binaries attached to a GitHub or GitLab
release. Most projects use one or two of these; few need all of them.

## Automation

The Rust ecosystem has two main tools for automating the full release workflow.
[`cargo-release`](changelog.md#cargo-release) runs locally: it bumps the version
in `Cargo.toml`, generates changelog entries, creates a git tag, and publishes
to crates.io in a single command. It handles workspace releases where multiple
crates need coordinated version bumps. [`release-plz`](changelog.md#release-plz)
takes a CI-first approach: it runs in your pipeline and opens a release PR
containing the version bump and updated changelog. Merging the PR triggers
publication automatically. Both tools integrate with
[`git-cliff`](changelog.md#git-cliff) for changelog generation and
[`cargo-semver-checks`](../checks/semver.md) for verifying that version bumps
match the actual API changes. The [CI chapter](../ci/readme.md) covers how to
wire these into your pipeline.
