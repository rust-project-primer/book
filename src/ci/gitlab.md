# GitLab CI

GitLab is an [open-source][source] software development platform with a built-in
CI/CD system called GitLab CI. Unlike GitHub Actions, which is configured
through a directory of workflow files, GitLab CI uses a single `.gitlab-ci.yml`
file at the repository root. The other major difference is that GitLab CI is
built around Docker: by default, every job runs inside a Docker container, which
means your CI environment is defined by the Docker image you choose rather than
by actions that install tools onto a VM.

This chapter covers the Rust-specific aspects of GitLab CI. For general GitLab
CI features, the [GitLab CI documentation](https://docs.gitlab.com/ee/ci/) is
comprehensive.

[source]: https://gitlab.com/gitlab-org/gitlab

## Mental Model

A **pipeline** is a set of jobs triggered by an event (a push, a merge request,
a schedule, a manual trigger, or an API call). Pipelines are organized into
**stages** that run sequentially. Within a stage, all jobs run in parallel. A
typical pipeline might have stages like `format`, `check`, `test`, and `deploy`,
where all jobs in `check` must pass before any job in `test` starts.

Each **job** runs in a fresh Docker container specified by the `image:` keyword.
A job executes a list of shell commands defined in `script:`, and can produce
**artifacts** that downstream jobs consume or that users can download from the
GitLab UI. Jobs can also start background **services** (like a PostgreSQL
database) by specifying additional Docker images.

<center>

![GitLab CI Job](gitlab-ci-job.svg)

</center>

For pipelines where the strict stage ordering is too rigid, GitLab supports
**DAG pipelines** using the `needs:` keyword, which allows a job to run as soon
as its specific dependencies finish, regardless of which stage it belongs to.
This is similar to GitHub Actions' `needs:` keyword and is useful for running
independent jobs as early as possible.

GitLab CI has a few other features worth knowing about. The `rules:` keyword
(which replaces the older `only:`/`except:`) controls when a job runs based on
branch names, file changes, variables, or other conditions. The `include:`
keyword lets you split configuration across multiple files or import shared
configuration from other repositories, similar to GitHub's reusable workflows.

**Runners** are the machines that execute jobs. GitLab.com provides shared
runners, but self-hosted runners are common in GitLab setups, especially for
projects that need persistent caches, specialized hardware, or network access to
internal services. Runner executors determine how jobs are isolated: Docker (the
most common), Kubernetes, shell, or virtual machines via QEMU (useful for
testing on platforms like FreeBSD or Windows).

## Patterns

The following patterns are specific to using GitLab CI with Rust projects. For
which checks to run and how to organize them into tiers, see the
[What to Run](readme.md#what-to-run) section in the CI overview.

### Docker Images

In GitLab CI, the Docker image you choose for a job determines your Rust
toolchain. The official Rust images on Docker Hub are the standard choice:

```yaml
test:
  image: rust:1.82.0
  script:
    - cargo test
```

Pin the image to a specific Rust version rather than using `rust:latest`, which
can change at any time. For jobs that need nightly (such as formatting with
unstable rustfmt options), use `rustlang/rust:nightly` or a dated nightly image.

The official images come in several variants. `rust:1.82.0-slim` omits
development tools and documentation for a smaller image, and
`rust:1.82.0-alpine` uses Alpine Linux for an even smaller footprint (though
Alpine's musl libc can cause issues with crates that assume glibc).

For projects that need additional tools beyond what the official images provide,
you can build a custom Docker image with your Rust toolchain and tools
pre-installed, push it to GitLab's built-in container registry, and use it as
the base image for your jobs. This avoids spending time on `cargo install` or
`rustup component add` in every pipeline run. The downside is maintenance: you
need to rebuild the image when Rust updates, tag versions properly, and keep it
in sync with your project's requirements. For small projects, installing tools
in the `before_script` is simpler even if it is slower.

### Caching

GitLab CI has built-in caching that works well for Rust projects. The key
directories to cache are `target/` (build artifacts) and the Cargo home
directories (`~/.cargo/registry` and `~/.cargo/git`):

```yaml
variables:
  CARGO_HOME: ${CI_PROJECT_DIR}/.cargo

test:
  image: rust:1.82.0
  cache:
    key: ${CI_COMMIT_REF_SLUG}
    paths:
      - .cargo/registry
      - .cargo/git
      - target/
  script:
    - cargo test
```

Setting `CARGO_HOME` to a directory inside the project is necessary because
GitLab CI can only cache paths relative to the project directory. The cache key
determines when the cache is shared or invalidated. Using `$CI_COMMIT_REF_SLUG`
means each branch gets its own cache, which prevents branches from polluting
each other's build artifacts. For more aggressive caching, you can use a hash of
`Cargo.lock` as the key so that the cache is invalidated whenever dependencies
change.

For pipelines with many jobs, cache policies help avoid contention. A job with
`policy: pull` only reads from the cache and never writes to it, while
`policy: push` only writes. This is useful when you have one job that builds
everything and writes the cache, and several downstream jobs that only need to
read it.

As with any build cache, stale artifacts can cause mysterious failures. If a CI
run fails in a way that does not reproduce locally, clearing the cache is a good
first debugging step.

For larger projects, [sccache](../building/cache.md) can provide compilation
caching at the object-file level, which is more fine-grained than caching the
entire `target/` directory.

### Services

GitLab CI has first-class support for running background services alongside your
jobs. This is particularly useful for
[integration tests](../testing/external-services.md) that need a database or
other external service:

```yaml
test:integration:
  image: rust:1.82.0
  services:
    - postgres:16
  variables:
    POSTGRES_DB: test
    POSTGRES_USER: runner
    POSTGRES_PASSWORD: password
    DATABASE_URL: "postgres://runner:password@postgres/test"
  script:
    - cargo test --features integration
```

The service is accessible by its image name as a hostname (`postgres` in this
case). This is simpler than the equivalent setup in GitHub Actions, where you
would need Docker Compose or testcontainers to achieve the same thing.

### Environment Variables

The same Rust-specific environment variables that are useful in GitHub Actions
apply here, set via the `variables:` keyword:

```yaml
variables:
  CARGO_INCREMENTAL: "0"
  RUSTFLAGS: "-D warnings"
  CARGO_TERM_COLOR: always
```

`CARGO_INCREMENTAL=0` disables incremental compilation (wasteful in CI),
`RUSTFLAGS="-D warnings"` promotes warnings to errors, and
`CARGO_TERM_COLOR=always` enables colored compiler output. For jobs that run
`cargo doc`, set `RUSTDOCFLAGS: "-D warnings"` separately, since `RUSTFLAGS`
does not affect rustdoc. See the
[GitHub Actions](github.md#rust-specific-environment-variables) chapter for a
detailed explanation of each variable.

### Unit Test Integration

GitLab can display test results directly in merge requests, showing which tests
passed, failed, or were newly added without needing to dig through CI logs. To
enable this, configure your test job to produce a JUnit XML report and upload it
as an artifact. `cargo-nextest` can produce JUnit output directly, and for
standard `cargo test` you can use `cargo2junit` to convert the output:

```yaml
test:
  image: rust:1.82.0
  script:
    - cargo nextest run --profile ci
  artifacts:
    reports:
      junit: target/nextest/ci/junit.xml
```

GitLab will then show the test results in the merge request's test report tab.

### Coverage Integration

GitLab can also display line-by-line coverage diffs directly in merge requests,
so developers can see exactly which new lines are covered and which are not.
`cargo-llvm-cov` can output Cobertura XML, which is the format GitLab expects:

```yaml
coverage:
  image: rust:1.82.0
  script:
    - cargo llvm-cov --cobertura --output-path cobertura.xml
  artifacts:
    reports:
      coverage_report:
        coverage_format: cobertura
        path: cobertura.xml
```

### Release Pipelines

Release pipelines are typically triggered by Git tags. For publishing to
crates.io, store your `CARGO_REGISTRY_TOKEN` as a protected CI/CD variable
(restricted to protected tags) so that only tag pipelines can access it. Note
that crates.io's trusted publishing (OIDC) is currently GitHub-only, so GitLab
requires the traditional API token approach.

```yaml
publish:
  image: rust:1.82.0
  rules:
    - if: $CI_COMMIT_TAG =~ /^v/
  script:
    - cargo publish
```

For creating GitLab releases with downloadable binaries, you can use the
`release:` keyword in combination with a build job that cross-compiles for
multiple platforms using [`cross`](../building/cross.md). The
[Changelog](../releasing/changelog.md) chapter covers how to automate changelog
generation as part of this process.

### GitLab Pages

GitLab Pages is a straightforward way to host static content generated by your
CI pipeline. Any job named `pages` that produces an artifact in a `public/`
directory will be deployed to your project's Pages URL automatically. This is
useful for hosting [API documentation](../documentation/rustdoc.md),
[coverage reports](../measure/coverage.md),
[book documentation](../documentation/book.md), and nightly binaries.

By default, the artifacts published by GitLab Pages will be on a GitLab provided
domain. For example, if your repository is at `gitlab.com/yourname/reponame`,
then they would be published to `yourname.gitlab.io/reponame/`. However, you can
add custom domains in Settings -> Deploy -> Pages, so that you can point for
example `docs.reponame.com` to it (or whatever domain or subdomain you want).

Here's an example that builds both rustdoc code documentation and a
mdbook-powered documentation:

```yaml
stages:
  - build
  - deploy

# build code documentation with rustdoc
docs:
  stage: build
  image: rust:1.82.0
  script:
    - cargo doc --no-deps --all-features
  artifacts:
    paths:
      - target/doc
    expire_in: 1 week

# build documentation book with mdbook
book:
  stage: build
  image: alpine:latest
  before_script:
    - apk install mdbook
  script:
    - mdbook build
  artifacts:
    paths:
      - book
    expire_in: 1 week

# deploy to pages (replace your_crate_name with the name of the crate you
# want to show docs for by default)
pages:
  stage: deploy
  image: alpine:latest
  rules:
    - if: $CI_COMMIT_BRANCH == $CI_DEFAULT_BRANCH
  cache: []
  script:
    - mv book public
    - mv target/doc public/code
    - echo '<meta http-equiv="refresh" content="0;url=code/your_crate_name/">' >
      public/index.html
  artifacts:
    paths:
      - public
```

The build jobs run in parallel and produce artifacts that the `pages` job
collects. You can add more things to publish (coverage reports, nightly
binaries) by adding more build jobs and extracting their artifacts into
`public/`.

## Reproducibility

The [Reproducibility](readme.md#reproducibility) section in the CI overview
covers the platform-agnostic techniques: pinning the Rust toolchain with
`rust-toolchain.toml`, pinning dependencies with `--locked`, pinning tool
versions, and using Nix. This section covers the GitLab-specific concerns.

### Pinning Docker Images

In GitLab CI, the Docker image is the primary input to control. Always use a
specific version tag (`rust:1.82.0`) rather than `rust:latest`. This applies
both to the `image:` keyword in your jobs and to any custom base images you
build. If you use a custom image from your GitLab container registry, tag it
with a version or commit hash so you can trace exactly which image a pipeline
used.

### Pinning Included Configuration

If you use `include:` to import configuration from other repositories, pin it to
a specific ref rather than a branch name:

```yaml
include:
  - project: "my-group/shared-ci"
    ref: "v1.2.0"
    file: "/rust.yml"
```

Without a pinned ref, an update to the shared configuration can change your
pipeline behavior without any change to your own repository.

### Nix

For projects that use [Nix](../build-system/nix.md), you can use the `nixos/nix`
Docker image and run commands inside `nix develop`:

```yaml
test:
  image: nixos/nix:latest
  variables:
    # Flakes are not enabled by default in the nixos/nix image.
    NIX_CONFIG: "experimental-features = nix-command flakes"
  script:
    - nix develop --command cargo test
```

This pins the toolchain and all tools via the Nix flake lockfile.

The main challenge with Nix in GitLab CI is caching. GitLab can only cache paths
relative to the project directory, but the Nix store lives at `/nix/store`.
There are several ways to deal with this. The simplest is to use a Nix binary
cache like [Cachix](https://www.cachix.org/) or the self-hosted
[Attic](https://github.com/zhaofengli/attic) so that derivations are fetched
from the cache rather than rebuilt from source. For self-hosted runners, a more
effective approach is to mount the host's Nix store into the container, so all
jobs share the same store and never rebuild what another job already built. You
can also build custom Docker images with your project's dependencies
pre-populated using Nix's `dockerTools`, then push them to the GitLab container
registry with `skopeo`.

## Security

GitLab CI has a different security model than GitHub Actions. There is no
third-party actions ecosystem, so supply-chain risk from actions is not a
concern. The main threats come from Docker images, secrets management, and
runner configuration.

**Protected variables** ensure that sensitive values (like
`CARGO_REGISTRY_TOKEN`) are only available in pipelines running on protected
branches or protected tags. This prevents a contributor from accessing your
publishing token by submitting a merge request that prints environment
variables.

**Runner security** is important for projects that accept external
contributions. If your self-hosted runner is shared across projects, a malicious
merge request could access the runner's local filesystem, network, or cached
data from other projects. GitLab allows you to restrict which projects can use a
runner and to disable pipeline execution for merge requests from forks.

**`CI_JOB_TOKEN`** is an automatically generated token that provides scoped
access to the GitLab API from within a CI job. It can be used to access the
container registry, pull from other projects, or trigger downstream pipelines
without storing additional secrets.

## Example

The following `.gitlab-ci.yml` puts together the patterns from this chapter.
Inline comments explain the choices made.

```yaml
# Set these globally so every job inherits them.
variables:
  CARGO_HOME: ${CI_PROJECT_DIR}/.cargo
  CARGO_INCREMENTAL: "0"
  RUSTFLAGS: "-D warnings"
  CARGO_TERM_COLOR: always

# Cache Cargo artifacts. Jobs inherit this by default.
# The cache key is per-branch so branches don't pollute each other.
default:
  cache:
    key: ${CI_COMMIT_REF_SLUG}
    paths:
      - .cargo/registry
      - .cargo/git
      - target/

stages:
  - format
  - check
  - test
  - deploy

# Formatting is the cheapest check and runs first. Uses nightly because
# some rustfmt options (like imports_granularity) require it.
format:
  stage: format
  image: rustlang/rust:nightly
  # Formatting doesn't need the build cache.
  cache: []
  script:
    - rustup component add rustfmt
    - cargo fmt --check

lint:
  stage: check
  image: rust:1.82.0
  script:
    - rustup component add clippy
    - cargo clippy --all-targets -- -D warnings

test:
  stage: test
  image: rust:1.82.0
  script:
    - cargo test --all-features --locked

# Build documentation and fail on warnings. The --no-deps flag skips
# building docs for dependencies, which can be very large and are
# already available on docs.rs.
doc:
  stage: check
  image: rust:1.82.0
  variables:
    RUSTDOCFLAGS: "-D warnings"
  script:
    - cargo doc --no-deps --all-features --locked
  artifacts:
    paths:
      - target/doc
    expire_in: 1 week

# Audit runs only on the default branch or on a schedule. Advisory
# databases change independently of your code, so merge request
# pipelines should not fail for reasons outside the contributor's
# control. To set up a weekly schedule, go to Build > Pipeline
# schedules in the GitLab UI.
audit:
  stage: check
  image: rust:1.82.0
  rules:
    - if: $CI_PIPELINE_SOURCE == "schedule"
    - if: $CI_COMMIT_BRANCH == $CI_DEFAULT_BRANCH
  script:
    - cargo install cargo-audit
    - cargo audit

# Feature powerset check is expensive (combinatorial) and only runs
# on a schedule. Catches feature flag combinations that fail to compile.
features:
  stage: check
  image: rust:1.82.0
  rules:
    - if: $CI_PIPELINE_SOURCE == "schedule"
  script:
    - cargo install cargo-hack
    - cargo hack check --feature-powerset

# Generate an HTML coverage report.
coverage:
  stage: test
  image: rust:1.82.0
  before_script:
    - rustup component add llvm-tools-preview
    - cargo install cargo-llvm-cov
  script:
    - cargo llvm-cov --html
  artifacts:
    paths:
      - target/llvm-cov/html
    expire_in: 1 week

# Assemble outputs from other jobs and deploy to GitLab Pages.
# Uses a minimal Alpine image since this job only copies files.
pages:
  stage: deploy
  image: alpine:latest
  needs: [doc, coverage]
  rules:
    - if: $CI_COMMIT_BRANCH == $CI_DEFAULT_BRANCH
  # No compilation happens here, so skip the Cargo cache.
  cache: []
  script:
    - mv target/doc public
    - echo '<meta http-equiv="refresh" content="0;url=your_crate_name/">' >
      public/index.html
    - mv target/llvm-cov/html public/coverage
  artifacts:
    paths:
      - public
```

The `$CI_PIPELINE_SOURCE == "schedule"` rule ensures that these jobs only run
when triggered by a pipeline schedule, which you configure in the GitLab UI
under Build > Pipeline schedules. A weekly schedule is typical for auditing and
feature powerset checks.

## Reading

```reading
style: article
url: https://docs.gitlab.com/ee/ci/
title: Get started with GitLab CI/CD
author: GitLab
---
The official GitLab CI documentation, covering pipeline configuration, runners,
variables, caching, artifacts, and all other features in detail.
```

```reading
style: article
url: https://www.fpcomplete.com/blog/deploying-rust-with-docker-and-kubernetes/
title: Deploying Rust with Docker and Kubernetes
author: FP Complete
---
A walkthrough of deploying a Rust application with Docker and Kubernetes using
GitLab CI, covering multi-stage Docker builds and CI pipeline configuration.
```

```reading
style: article
url: https://www.bassi.io/articles/2019/04/13/adventures-in-ci/
title: (New) Adventures in CI
author: Emmanuele Bassi
---
A blog post about how the GNOME project uses GitLab CI to generate coverage
reports for every commit, with practical examples of integrating coverage
tooling into a GitLab pipeline.
```

```reading
style: article
url: https://cobalt.rocks/posts/nix-gitlab/
title: Nix and GitLab CI
author: Cobalt
---
Covers three approaches to integrating Nix with GitLab CI runners: mounting the
host Nix store into containers for shared caching, using a plain Docker executor
with no shared state, and S3-backed caching. Addresses the core problem that
GitLab cannot cache `/nix/store` directly, with solutions including custom
container images built with `dockerTools` and pushed via `skopeo`, and
self-hosted binary caches using Attic. Also covers packaging CI scripts as Nix
derivations with `writeShellApplication` for local reproducibility.
```
