# Outdated Dependencies

_Your Rust project is going smoothly, however one of your developers complains
that you are using an old version of Tokio, and that a feature he would like to
use is not available yet. You tell him to upgrade the version of Tokio, and is
happy. However, that makes you wonder: would it be possible to find out
automatically if you have any outdated dependencies in any of your crates?_

Besides fixing bugs, new versions of dependencies also usually come with new
features and sometimes better performance. For that reason, it is usually
advisable to not fall behind too far in terms of which version is being used.

There is some tooling in the Rust world which can check for outdated
dependencies automatically. This can be used as a maintenance task or a periodic
CI job.

```admonish
If you are working on an open source project, you can also rely on the [deps.rs][]
service to tell you if your dependencies are outdated.
```

## `cargo-upgrades`

[`cargo-upgrades`][cargo-upgrades] is a Cargo subcommand to check if any of the
direct dependencies have newer versions available. It has a simpler
implementation than `cargo-outdated` and is typically a bit faster, because it
does not rely on using Cargo's dependency resolution.

You can install it using `cargo` and run it against your project:

```
cargo install cargo-upgrades
cargo upgrades
```

```admonish example title="Using <cargo-upgrades> to check for outdated dependencies in CI"

```

## `cargo-outdated`

[`cargo-outdated`][cargo-outdated] is a Cargo subcommand for displaying when
Rust dependencies are out of date. It works by creating a temporary Cargo
workspace and running `cargo-update`, and finally comparing the resolved crate
versions against the ones in the original crate.

You can install it using `cargo`, and run it against your project:

```
cargo install cargo-outdated
cargo outdated
```

```admonish example title="Using <cargo-outdated> to check for outdated dependencies in CI"

```

## Reading

```reading
style: article
title: Cleaning up and upgrading third-party crates
url: https://fasterthanli.me/series/updating-fasterthanli-me-for-2022/part-1
author: Amos Wenger
---
In this article, Amos shows how to clean up and upgrade crate dependencies. He
uses `cargo-outdated` to do this, but he mentions that it has an issue with
path dependencies in Cargo workspaces.
```

[cargo-upgrades]: https://gitlab.com/kornelski/cargo-upgrades
[cargo-outdated]: https://github.com/kbknapp/cargo-outdated
