# GitHub Actions

GitHub Actions is the CI/CD platform built into GitHub. It launched in 2019 and
has since become a popular CI system for open-source Rust projects, largely
because it is free for public repositories and deeply integrated with pull
requests and issue tracking. Workflows are defined as YAML files in the
`.github/workflows/` directory of your repository, and they run in response to
events like pushes, pull request updates, or cron schedules.

This chapter focuses on the practical side: how GitHub Actions works, how to set
up effective CI for a Rust project, and how to avoid common pitfalls. Because
this is the point in the book where all the individual tools come together, this
chapter cross-references the checks, testing, building, and releasing chapters
extensively.

## Mental Model

A **workflow** is a YAML file in `.github/workflows/`. Each workflow is
triggered by one or more **events** and contains one or more **jobs**. Each job
runs on a fresh virtual machine (called a **runner**), and GitHub provides
hosted runners for Linux (Ubuntu), macOS, and Windows. Jobs run in parallel by
default, but you can create dependencies between them using the `needs:` keyword
so that a job is skipped if an earlier job fails.

Each job consists of a sequence of **steps**. A step is either a shell command
(`run:`) or an invocation of an **action** (`uses:`). Actions are reusable units
of CI logic published as GitHub repositories. For example, `actions/checkout@v4`
checks out your repository, and `dtolnay/rust-toolchain@stable` installs a Rust
toolchain.

Because each job starts from a clean VM, nothing persists between jobs unless
you explicitly pass data through artifacts or caches. The upside is full
isolation; the downside is that you pay the cost of setup (toolchain
installation, dependency download, compilation) on every run unless you
configure caching.

The most common triggers are `push` and `pull_request` (for checking code on
every change), `schedule` (for running expensive checks periodically using cron
syntax), and `workflow_dispatch` (which adds a manual "Run workflow" button in
the GitHub UI). You can scope triggers to specific branches or file paths, and
use `if:` conditions to skip individual jobs or steps based on context. Larger
projects often split workflows across multiple YAML files (one for CI checks,
one for scheduled audits, one for releases), and organizations with many Rust
repositories can share workflow logic across repos using reusable workflows via
the `workflow_call` trigger. The
[GitHub Actions documentation](https://docs.github.com/en/actions) covers all of
these features in detail.

```yaml
name: CI
on: [push, pull_request]
jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - run: cargo test
```

This minimal workflow checks out the code, installs Rust, and runs `cargo test`
on every push and pull request.

## Patterns

The following is a collection of patterns commonly used when writing GitHub
Actions workflows for Rust projects. Most of what GitHub Actions offers is not
Rust-specific and is well-documented elsewhere, so this section focuses on the
parts where Rust's compile times, toolchain ecosystem, or cargo conventions
require special attention.

### Toolchain Installation

The standard approach is `dtolnay/rust-toolchain`, which installs a specific
Rust toolchain and is faster and more cacheable than calling `rustup` directly:

```yaml
- uses: dtolnay/rust-toolchain@stable
```

You can also pin to a specific version, install nightly, or add components:

```yaml
- uses: dtolnay/rust-toolchain@stable
  with:
    toolchain: 1.78.0
    components: clippy, rustfmt
```

For projects that need to test across multiple Rust versions (stable, beta,
nightly, and their MSRV), this pairs well with the matrix strategy covered
below.

To install additional Cargo subcommands like `cargo-nextest`, `cargo-hack`, or
`cargo-audit`, the `taiki-e/install-action` provides pre-built binaries for many
common tools, which is significantly faster than building them from source with
`cargo install`:

```yaml
- uses: taiki-e/install-action@v2
  with:
    tool: cargo-nextest,cargo-hack
```

### Caching

Rust projects are notorious for slow CI builds because the compilation step
dominates. A fresh `cargo build` of a moderately-sized project can take 10-20
minutes. Caching the build artifacts between runs is essential.

The `Swatinem/rust-cache` action is the standard solution. It caches
`~/.cargo/registry`, `~/.cargo/git`, and the `target/` directory, with automatic
cache key generation based on your `Cargo.lock`, toolchain version, and job
name:

```yaml
- uses: Swatinem/rust-cache@v2
```

This single line typically cuts subsequent build times by 50-80%. You can also
configure it to cache additional directories or share caches between jobs.

For larger projects, consider [sccache](../building/cache.md) as a compilation
cache that operates at the object-file level. The
`mozilla-actions/sccache-action` makes this easy to set up and can share cached
artifacts across different workflow runs and even across different CI jobs.

One thing to watch out for is stale caches. When dependencies change or the Rust
compiler is updated, cached build artifacts can become invalid and cause
mysterious compilation failures that do not reproduce locally. If a CI run fails
in a way you cannot explain, try deleting the cache via the GitHub Actions UI or
API before spending time debugging.

### Concurrency Control

Because Rust CI runs are dominated by compilation, they tend to be long. When
you force-push to a PR branch, GitHub starts a new workflow run while the
previous one is still compiling. For a language with fast builds this is barely
noticeable, but for Rust you can easily end up with 20+ minutes of runner time
wasted on a run whose results you no longer care about. The `concurrency:` key
solves this by automatically cancelling in-progress runs:

```yaml
concurrency:
  group: ${{ github.workflow }}-${{ github.ref }}
  cancel-in-progress: ${{ github.ref != 'refs/heads/main' }}
```

This cancels duplicate runs on PR branches but never cancels runs on `main`
(where you want every push to complete).

### Matrix Strategy

A matrix lets you run the same job across multiple configurations. This is how
you test across Rust versions, operating systems, or feature flag combinations:

```yaml
jobs:
  test:
    strategy:
      matrix:
        os: [ubuntu-latest, macos-latest, windows-latest]
        rust: [stable, beta, nightly]
    runs-on: ${{ matrix.os }}
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@master
        with:
          toolchain: ${{ matrix.rust }}
      - uses: Swatinem/rust-cache@v2
      - run: cargo test
```

This generates 9 jobs (3 operating systems times 3 toolchains). You can use
`include:` to add specific combinations (like testing your
[MSRV](../checks/msrv.md) on Ubuntu only) and `exclude:` to skip combinations
that are not relevant.

For projects that claim cross-platform support, testing on Windows is important.
Windows does some unusual things regarding path separators, line endings,
filesystem case sensitivity, and symlink behavior. These issues do not surface
on Linux-only CI. For more on this topic, see the
[Cross-Compiling](../building/cross.md) chapter.

### Rust-Specific Environment Variables

A few environment variables are worth setting globally for CI jobs:

```yaml
env:
  CARGO_INCREMENTAL: 0
  RUSTFLAGS: "-D warnings"
  CARGO_TERM_COLOR: always
```

**`CARGO_INCREMENTAL=0`** disables incremental compilation. Incremental
compilation speeds up rebuilds on a developer's machine by caching intermediate
artifacts, but in CI every build starts from a clean state (or a cache that may
be stale), so incremental compilation just wastes disk space and can
occasionally cause spurious failures.

**`RUSTFLAGS="-D warnings"`** promotes all warnings to errors. This ensures that
CI fails on warnings without requiring developers to set `#![deny(warnings)]` in
their code, which would also affect downstream users of the crate. Setting it as
an environment variable keeps the strictness scoped to CI.

**`CARGO_TERM_COLOR=always`** forces colored output. GitHub Actions renders ANSI
colors in its log viewer, and colored compiler output is significantly easier to
read.

Note that `RUSTFLAGS` only affects `rustc`. For `cargo doc`, you need to set
`RUSTDOCFLAGS="-D warnings"` separately to turn documentation warnings (broken
intra-doc links, missing code examples, etc.) into errors. This is typically set
on the doc job rather than globally, since not every job runs rustdoc.

### What to Run

The [What to Run](readme.md#what-to-run) section in the CI overview chapter
covers which checks to run and how to organize them into a fast tier (every pull
request) and a thorough tier (on merge or on schedule). The example workflow at
the end of this chapter demonstrates how to implement both tiers in GitHub
Actions.

### Release Workflows

CI is not just about checks. Many Rust projects use GitHub Actions to build
release binaries and publish crates. A common pattern is a workflow triggered by
Git tags:

```yaml
on:
  push:
    tags: ["v*"]
```

This workflow can use a matrix with `cross` (see
[Cross-Compiling](../building/cross.md)) to build binaries for multiple
platforms, upload them as GitHub release assets, and optionally publish the
crate to [crates.io](../releasing/crates.md). The
[Changelog](../releasing/changelog.md) chapter covers how to automate changelog
generation as part of this process.

For publishing to crates.io, the traditional approach is to store a
`CARGO_REGISTRY_TOKEN` as a repository secret. A newer and more secure
alternative is
[trusted publishing](https://rust-lang.github.io/rfcs/3691-trusted-publishing-cratesio.html),
which uses GitHub's OpenID Connect (OIDC) tokens to authenticate directly with
crates.io without any stored secrets. You configure crates.io to trust publishes
from a specific repository and workflow, and GitHub provides a short-lived token
at runtime. This eliminates the risk of a leaked or stale API token and removes
the need to rotate secrets.

### GitHub Pages

GitHub Pages can host static content generated by your CI workflows, similar to
[GitLab Pages](gitlab.md#gitlab-pages). This is useful for publishing
[API documentation](../documentation/rustdoc.md),
[coverage reports](../measure/coverage.md), and
[book documentation](../documentation/book.md).

By default, GitHub Pages publishes to a domain based on your username and
repository name. For example, if your repository is at
`github.com/yourname/reponame`, the pages will be at
`yourname.github.io/reponame/`. You can configure a custom domain in Settings > Pages.

GitHub Pages requires a dedicated deployment workflow using the
`actions/upload-pages-artifact` and `actions/deploy-pages` actions. You also
need to configure the repository to deploy from GitHub Actions (Settings >
Pages > Source > GitHub Actions).

```yaml
name: Pages

on:
  push:
    branches: [main]

permissions:
  pages: write
  id-token: write

jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - uses: taiki-e/install-action@v2
        with:
          tool: mdbook
      # Build rustdoc and mdBook, then assemble into a single directory.
      - run: cargo doc --no-deps --all-features
      - run: mdbook build
      - run: mkdir site && mv book site/book && mv target/doc site/code
      # Replace your_crate_name with your crate name (hyphens become
      # underscores in rustdoc output).
      - run:
          echo '<meta http-equiv="refresh" content="0;url=book/">' >
          site/index.html
      - uses: actions/upload-pages-artifact@v3
        with:
          path: site

  deploy:
    needs: build
    runs-on: ubuntu-latest
    environment:
      name: github-pages
      url: ${{ steps.deployment.outputs.page_url }}
    steps:
      - id: deployment
        uses: actions/deploy-pages@v4
```

You can add more things to publish (coverage reports, nightly binaries) by
adding them to the build job.

### Common Actions

Several community-maintained actions are commonly used in Rust CI workflows:

| Action                                                                                  | Description                                                                                                             |
| --------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------- |
| [`dtolnay/rust-toolchain`](https://github.com/dtolnay/rust-toolchain)                   | Installs and configures Rust toolchains. Replaces the unmaintained `actions-rs/toolchain`.                              |
| [`Swatinem/rust-cache`](https://github.com/Swatinem/rust-cache)                         | Caches Cargo registry, git checkouts, and build artifacts.                                                              |
| [`taiki-e/install-action`](https://github.com/taiki-e/install-action)                   | Installs pre-built binaries of common Cargo subcommands (nextest, hack, audit, and more) without compiling from source. |
| [`mozilla-actions/sccache-action`](https://github.com/mozilla-actions/sccache-action)   | Sets up [sccache](../building/cache.md) for shared compilation caching.                                                 |
| [`EmbarkStudios/cargo-deny-action`](https://github.com/EmbarkStudios/cargo-deny-action) | Runs [cargo-deny](../checks/audit.md) to check licenses, advisories, and banned dependencies.                           |
| [`actions-rust-lang/audit`](https://github.com/actions-rust-lang/audit)                 | Runs `cargo audit` to check for known vulnerabilities. Replaces the unmaintained `actions-rs/audit-check`.              |
| [`bencherdev/bencher`](https://github.com/bencherdev/bencher)                           | Tracks benchmark results over time, useful for detecting [performance](../measure/performance.md) regressions.          |
| [`crate-ci/typos`](https://github.com/crate-ci/typos)                                   | Checks for spelling mistakes in source code, comments, and documentation.                                               |
| [`crate-ci/committed`](https://github.com/crate-ci/committed)                           | Checks that commit messages follow conventional commit formatting.                                                      |

Note that the `actions-rs` family of actions (toolchain, cargo, audit-check,
clippy-check) is unmaintained. If you encounter them in existing workflows,
consider migrating to the alternatives listed above.

## Reproducibility

The [Reproducibility](readme.md#reproducibility) section in the CI overview
covers the platform-agnostic techniques: pinning the Rust toolchain with
`rust-toolchain.toml`, pinning dependencies with `--locked`, pinning tool
versions, and using Nix. This section covers the GitHub-specific concerns.

### Pinning Actions

GitHub Actions references like `actions/checkout@v4` point to a mutable Git tag.
A maintainer can push new code under an existing tag at any time, which is both
a security risk (covered in the Security section below) and a reproducibility
problem: an action update can change CI behavior without any change to your
code. Pinning to a full commit SHA eliminates this:

```yaml
- uses: actions/checkout@b4ffde65f46336ab88eb53be808477a3936bae11 # v4.1.1
```

Tools like Dependabot and Renovate can keep SHA pins up to date automatically,
giving you both pinning and freshness.

### Pinning Tool Versions

When using `taiki-e/install-action`, you can pin tool versions explicitly:

```yaml
- uses: taiki-e/install-action@v2
  with:
    tool: cargo-nextest@0.9.81,cargo-hack@0.6.33
```

Without version pins, each CI run installs whatever the latest release is, which
can introduce new warnings or behavior changes unrelated to your code.

### Pinning Runner Images

GitHub's `ubuntu-latest` label is convenient, but it periodically moves to a
newer Ubuntu release. When it does, system libraries, default compiler versions,
and other host dependencies change. For most Rust projects this is harmless, but
if your build depends on system packages (OpenSSL, SQLite, protoc), the version
jump can break things. Pinning to a specific image avoids this:

```yaml
runs-on: ubuntu-24.04 # instead of ubuntu-latest
```

The same applies to Docker base images. Using `rust:latest` in a Dockerfile
means the Rust version can change at any time. Pin to a specific version
instead: `rust:1.82.0`.

### Nix as a Reproducibility Layer

For projects that already use [Nix](../build-system/nix.md), running CI inside a
Nix development shell achieves all of the above in one step. The Nix flake
lockfile pins the Rust toolchain, all system dependencies, and all auxiliary
tools to exact versions. A workflow using Nix looks like this:

```yaml
jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: DeterminateSystems/nix-installer-action@main
      - uses: DeterminateSystems/magic-nix-cache-action@main
      - run: nix develop --command cargo test
```

The `DeterminateSystems/nix-installer-action` installs Nix on the runner, and
`magic-nix-cache-action` transparently caches Nix store paths using GitHub
Actions' built-in cache, so Nix does not rebuild everything from source on each
run. No external accounts or secrets are needed. For projects that need to share
a binary cache across multiple repositories or CI systems,
[Cachix](https://www.cachix.org/) is a hosted Nix binary cache service that
integrates with GitHub Actions via `cachix/cachix-action`. With either approach,
the only mutable input is the runner image itself, and even that has minimal
impact because Nix provides its own toolchain and libraries.

The tradeoff is adoption cost. Nix has a steep learning curve, and adding it to
a project solely for CI reproducibility is rarely worth it. But for projects
that already have a `flake.nix`, using it in CI is a natural extension that
eliminates most of the pinning concerns described above.

The advantage of using Nix in CI is that it extends the reproducibility to local
development environments. You can run the CI checks locally and be confident
that the outcome is the same as in the CI environment.

## Security

CI workflows often have access to secrets: registry tokens for publishing
crates, deployment credentials, API keys. This makes them an attractive attack
surface.

The most common vector is **action supply-chain attacks**. Because action
references like `actions/checkout@v4` resolve to a mutable Git tag, a
compromised or malicious action maintainer can push new code under an existing
tag. Every workflow using that tag will then execute the attacker's code on its
next run, with access to whatever secrets the job has. This has happened in
practice. The mitigation is SHA pinning, as described in the Reproducibility
section above. For workflows that handle secrets, SHA pinning is not optional.

The second vector is **`pull_request_target`**. Unlike `pull_request`, this
event runs in the context of the base branch, which means it has access to
repository secrets. If the workflow checks out and executes the PR's code, an
attacker can submit a malicious pull request that exfiltrates those secrets. The
safe pattern is to use `pull_request_target` only for steps that do not run
untrusted code (like labeling or commenting), and never to check out
`github.event.pull_request.head.ref` in a workflow that has access to secrets.

A third concern is **overly broad secret scopes**. GitHub allows scoping secrets
to specific environments and requiring approval for deployments. Use these
features to limit which jobs can access which secrets, rather than making all
secrets available to all workflows.

## Example

The following workflow puts together the patterns from this chapter into a
realistic CI setup. It demonstrates the fast-tier checks on every PR, a
thorough-tier audit on merge to `main`, cross-platform testing, and several
practical patterns explained in inline comments.

```yaml
name: CI

on:
  push:
    branches: [main]
  pull_request:
    branches: [main]

# Cancel in-progress runs on the same branch. Never cancel runs on main,
# where every push should complete.
concurrency:
  group: ${{ github.workflow }}-${{ github.ref }}
  cancel-in-progress: ${{ github.ref != 'refs/heads/main' }}

env:
  CARGO_INCREMENTAL: 0
  RUSTFLAGS: "-D warnings"
  CARGO_TERM_COLOR: always

jobs:
  # Formatting is the cheapest check. Other jobs depend on it via `needs:`
  # so that if formatting fails, everything else is skipped immediately.
  format:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      # Nightly is used because some rustfmt options (like
      # imports_granularity) are only available on nightly.
      - uses: dtolnay/rust-toolchain@nightly
        with:
          components: rustfmt
      - run: cargo fmt --check

  lint:
    runs-on: ubuntu-latest
    needs: format
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
        with:
          components: clippy
      - uses: Swatinem/rust-cache@v2
      - run: cargo clippy --all-targets -- -D warnings

  test:
    needs: format
    strategy:
      matrix:
        os: [ubuntu-latest, macos-latest, windows-latest]
    runs-on: ${{ matrix.os }}
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - uses: Swatinem/rust-cache@v2
      # --locked ensures CI uses exactly the dependency versions in
      # Cargo.lock, catching forgotten lock file updates.
      - run: cargo test --all-features --locked

  doc:
    runs-on: ubuntu-latest
    needs: format
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - uses: Swatinem/rust-cache@v2
      - run: cargo doc --no-deps --all-features --locked
        env:
          RUSTDOCFLAGS: "-D warnings"

  # The audit job only runs on pushes to main, not on PRs. Advisory
  # databases change independently of your code, so you don't want PR
  # builds failing for reasons outside the contributor's control.
  audit:
    runs-on: ubuntu-latest
    if: github.event_name == 'push' && github.ref == 'refs/heads/main'
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - uses: taiki-e/install-action@v2
        with:
          tool: cargo-audit
      - run: cargo audit
```

## Reading

```reading
style: article
url: https://docs.github.com/en/actions/quickstart
title: GitHub Actions QuickStart
author: GitHub
---
Shows you how to get started with GitHub Actions.
```

```reading
style: article
url: https://www.youtube.com/watch?v=9qljpi5jiMQ
title: GitHub Actions Feels Bad
author: Amos Wenger
---
The history and design of GitHub Actions, and why they are perhaps not
designed in an ideal way.
```

```reading
style: article
url: https://doc.rust-lang.org/cargo/guide/continuous-integration.html
title: Continuous Integration
author: The Cargo Book
---
The official Cargo documentation on setting up CI, with examples for both
GitHub Actions and GitLab CI.
```

```reading
style: article
url: https://blog.urth.org/2023/03/05/cross-compiling-rust-projects-in-github-actions/
title: Cross-Compiling Rust Projects in GitHub Actions
author: Dave Rolsky
---
A practical walkthrough of setting up cross-compilation in GitHub Actions,
covering toolchain setup, target installation, and common pitfalls.
```
