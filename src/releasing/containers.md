# Containers

Deploying Rust services as containers is common, given the tooling around
container orchestration, monitoring, and scaling. The two main container
runtimes are [Docker](https://www.docker.com/) and [Podman](https://podman.io/).
Podman is a daemonless, rootless alternative to Docker that uses the same image
format and can run the same Dockerfiles (called Containerfiles in Podman's
terminology). Everything in this chapter applies to both.

The challenge with containerized builds is that they are hermetic by default:
each build starts from scratch without access to Cargo's `target` directory or
any previous build cache. For a Rust project with hundreds of dependencies, this
means rebuilding everything from source on every change, which can be very slow.

There are several approaches to making container builds faster for Rust
projects.

## Layer Caching

Container builds work in layers, and layers are cached based on whether their
inputs have changed. The key insight for Rust is that your dependencies change
far less often than your source code. If you can build dependencies in a
separate layer from your application code, that layer gets cached and reused on
most builds.

A common technique is to copy only `Cargo.toml` and `Cargo.lock` first, build
dependencies, and then copy your source code and build the final binary:

```dockerfile framed title="Dockerfile"
FROM rust AS builder

# copy manifests and build dependencies only
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

# now copy real source and rebuild (only your code recompiles)
COPY src ./src
RUN cargo build --release

FROM debian:bookworm-slim
COPY --from=builder /app/target/release/my-app /usr/local/bin/
CMD ["my-app"]
```

This works but is fragile: the dummy `main.rs` trick can break if you have
multiple binaries, build scripts, or workspace members. `cargo-chef` (below)
automates this pattern more reliably.

## `cargo-chef`

[`cargo-chef`][cargo-chef] is a Cargo subcommand designed to make Docker layer
caching work well with Rust. It analyzes your project and generates a "recipe"
file that captures your dependency graph without your source code. The build is
then split into three stages:

1. **Prepare**: generate the recipe from your source tree.
2. **Cook**: build all dependencies using only the recipe (this layer is
   cached).
3. **Build**: copy your source code and build the final binary.

```dockerfile framed title="Dockerfile"
FROM rust AS chef
RUN cargo install cargo-chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
COPY --from=builder /app/target/release/my-app /usr/local/bin/
CMD ["my-app"]
```

The cook step only re-runs when your dependencies change (the recipe changes).
Source code changes skip straight to the final build step, which only recompiles
your code. This works correctly with workspaces, build scripts, and complex
project layouts.

## Podman

[Podman](https://podman.io/) can build the same Dockerfiles/Containerfiles and
produces OCI-compatible images. If you use Podman, the commands are the same —
just replace `docker` with `podman`:

```bash
podman build -t my-app .
podman run my-app
```

Podman runs without a daemon and supports rootless containers by default, which
makes it a good fit for CI environments where running a Docker daemon requires
elevated privileges. On systems where Docker is not available (some enterprise
Linux distributions ship Podman instead), the same Containerfiles work without
modification.

## Multi-Stage Builds

The examples above already use multi-stage builds (multiple `FROM` statements),
which is the standard approach for producing small container images from Rust.
The builder stage compiles your code with the full Rust toolchain, and the final
stage copies only the compiled binary into a minimal base image. This keeps the
final image small: a Rust binary on `debian:bookworm-slim` or `alpine` is
typically under 50 MB.

For even smaller images, you can build a statically linked binary using the
`x86_64-unknown-linux-musl` target and use `scratch` or `distroless` as the
base:

```dockerfile framed title="Dockerfile"
FROM rust AS builder
RUN rustup target add x86_64-unknown-linux-musl
WORKDIR /app
COPY . .
RUN cargo build --release --target x86_64-unknown-linux-musl

FROM scratch
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/my-app /
CMD ["/my-app"]
```

<!-- TODO: add section on BuildKit cache mounts as an alternative to cargo-chef,
     and discuss CI-specific caching strategies (registry caching, GitHub Actions
     cache). -->

## Reading

```reading
style: article
title: "Shipping Rust in Docker"
url: https://www.lpalmieri.com/posts/fast-rust-docker-builds/
author: Luca Palmieri
---
Luca, the author of `cargo-chef`, explains the problem with Docker layer
caching for Rust projects and walks through the solution. Covers the dummy
build trick, why it breaks for complex projects, and how `cargo-chef` solves
it with the prepare/cook/build workflow.
```

[cargo-chef]: https://github.com/LukeMathWalker/cargo-chef
