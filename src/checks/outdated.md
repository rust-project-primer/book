# Outdated Dependencies

Besides fixing bugs, new versions of dependencies also usually come with new
features and sometimes better performance. For that reason, it is usually
advisable to not fall behind too far in terms of which version is being used.

There is some tooling in the Rust ecosystem which can check for outdated
dependencies automatically. This can be used as a maintenance task or a periodic
CI job.

```admonish note
If you are working on an open source project, you can also rely on the
[deps.rs][] service to tell you if your dependencies are outdated. It provides
a badge you can add to your README that shows whether your dependencies are
up to date.
```

## `cargo-upgrades`

[`cargo-upgrades`][cargo-upgrades] is a Cargo subcommand to check if any of the
direct dependencies have newer versions available. It has a simpler
implementation than `cargo-outdated` and is typically a bit faster, because it
does not rely on using Cargo's dependency resolution.

You can install it using `cargo` and run it against your project:

```bash
cargo install cargo-upgrades
cargo upgrades
```

You can add a periodic CI job that checks for outdated dependencies using
`cargo-upgrades`. This example runs weekly and opens an issue if any
dependencies have newer versions available:

```yaml framed title=".github/workflows/outdated.yml"
name: Check outdated dependencies
on:
  schedule:
    - cron: '0 9 * * 1'  # Every Monday at 9:00 UTC
  workflow_dispatch:

jobs:
  outdated:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - run: cargo install cargo-upgrades
      - run: cargo upgrades
```

## `cargo-outdated`

[`cargo-outdated`][cargo-outdated] is a Cargo subcommand for displaying when
Rust dependencies are out of date. It works by creating a temporary Cargo
workspace and running `cargo-update`, and finally comparing the resolved crate
versions against the ones in the original crate. This makes it slower than
`cargo-upgrades`, but it can also detect transitive dependency updates.

You can install it using `cargo`, and run it against your project:

```bash
cargo install cargo-outdated
cargo outdated
```

Similar to the `cargo-upgrades` example, but using `cargo-outdated` to also
check transitive dependencies. The `--exit-code 1` flag causes the job to fail
if any outdated dependencies are found.

```yaml framed title=".github/workflows/outdated.yml"
name: Check outdated dependencies
on:
  schedule:
    - cron: '0 9 * * 1'
  workflow_dispatch:

jobs:
  outdated:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - run: cargo install cargo-outdated
      - run: cargo outdated --exit-code 1
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

[deps.rs]: https://deps.rs
[cargo-upgrades]: https://gitlab.com/kornelski/cargo-upgrades
[cargo-outdated]: https://github.com/kbknapp/cargo-outdated
