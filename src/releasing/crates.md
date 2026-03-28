# Crate Registries

The standard way to distribute Rust crates is through a registry.
[Crates.io][crates.io] is the public registry used by the Rust community. It is
free, integrates with [docs.rs][] for automatic documentation hosting, and is
where the vast majority of open-source Rust libraries are published. Publishing
a library there makes it available to anyone via `cargo add`, and binary crates
can be installed with `cargo install`.

## Publishing to crates.io

To publish, you need a GitHub account to log in to crates.io and generate an API
token. Authenticate with Cargo and publish:

```bash
cargo login <api-token>
cargo publish
```

Your crate must include certain metadata in `Cargo.toml` (name, version,
license, description) before it can be published. See [Publishing on
crates.io][publishing] for the full requirements.

If you publish a version by mistake, you can yank it. Yanking prevents new
projects from depending on that version, but does not delete it — existing
projects that already depend on it continue to work. This avoids the kind of
breakage seen in the
[left-pad incident](https://www.theregister.com/2016/03/23/npm_left_pad_chaos/),
where deleting a package from NPM broke a large part of the JavaScript
ecosystem.

```bash
cargo yank --version 1.2.3
```

## Private Registries

In a commercial setting, you may have internal crates that you want to share
within your organization but not publish publicly. While Cargo supports git
dependencies, a private registry is preferable because it enables semantic
versioning and version resolution — features that do not work with git
dependencies. [RFC 2141][rfc2141] specifies how alternative registries work with
Cargo.

Several private registry options exist:

- [**Shipyard**](https://shipyard.rs/) is a hosted private registry service. It
  replicates the crates.io experience for private crates, with authentication
  and access control.
- [**Kellnr**](https://kellnr.io/) is a self-hosted private registry that you
  can run on your own infrastructure.
- [**JFrog Artifactory**](https://jfrog.com/) supports Cargo registries as part
  of its broader artifact management platform, alongside npm, Maven, Docker, and
  other package formats.

To configure Cargo to use an alternative registry, add it to your
`.cargo/config.toml`:

```toml
[registries.my-registry]
index = "sparse+https://my-registry.example.com/index/"
```

Then publish to it or depend on crates from it:

```bash
cargo publish --registry my-registry
```

```toml
[dependencies]
my-internal-crate = { version = "1.0", registry = "my-registry" }
```

## Reading

```reading
style: book
title: "Chapter 14.2: Publishing to Crates.io"
url: https://doc.rust-lang.org/book/ch14-02-publishing-to-crates-io.html
author: The Rust Book
---
Walks through the full process of publishing a crate: adding metadata to
`Cargo.toml`, writing documentation, choosing a license, and running
`cargo publish`. Also covers managing crate owners and yanking versions.
```

```reading
style: article
title: Using the Shipyard private crate registry with Docker
url: https://fasterthanli.me/series/building-a-rust-service-with-nix/part-7
author: Amos Wenger
archived: fasterthanlime-build-a-rust-service-shipyard.pdf
---
Practical walkthrough of setting up a private crate registry with Shipyard,
including configuring Cargo authentication, publishing crates from both local
development and CI, and using the registry inside Docker builds where
credential handling requires extra care.
```

```reading
style: book
title: "Registries"
url: https://doc.rust-lang.org/cargo/reference/registries.html
author: The Cargo Book
---
Reference for how Cargo interacts with registries: the registry protocol,
authentication, configuring alternative registries, and publishing. Covers
both the older git-based index protocol and the newer sparse protocol that
crates.io uses by default since Rust 1.70.
```

[crates.io]: https://crates.io
[docs.rs]: https://docs.rs
[publishing]: https://doc.rust-lang.org/cargo/reference/publishing.html
[rfc2141]: https://rust-lang.github.io/rfcs/2141-alternative-registries.html
