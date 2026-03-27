# Caching

Caching build artifacts is a large quality of life improvement: typically,
dependencies do not change too much, and not all of the crates in your project
change all the time either. With a good build cache, compiling the project after
small changes can become very fast.

Rust already has a local build cache in the `target` folder, but this is only
useful for local development. In CI, the project is usually built from a clean
checkout every time. A shared cache allows your team and CI to reuse compilation
results across builds and machines.

```admonish info
If you are using a [Build System](../build-system/readme.md), you may get this
for free: Bazel, Buck2 and Nix all support caching compilations.
```

## `sccache`

[`sccache`][sccache] is a compiler caching tool developed by Mozilla (originally
for Firefox builds). It wraps the compiler and stores the output of compilation
in a shared cache. When the same source file is compiled again with the same
flags, sccache returns the cached result instead of recompiling.

It supports multiple storage backends:

- Local disk
- Cloud storage (S3, GCS, Azure Blob Storage)
- Redis
- Memcached

You install it and tell Cargo to use it as a wrapper around rustc:

```bash
cargo install sccache
export RUSTC_WRAPPER=sccache
cargo build
```

You can also set this permanently in your `.cargo/config.toml`:

```toml
[build]
rustc-wrapper = "sccache"
```

After a build, you can check the cache statistics with `sccache --show-stats`
to see hit rates and how much time was saved.

```admonish example title="Using sccache in GitHub Actions"
This example uses sccache with a cloud storage bucket in GitHub Actions. The
[Mozilla sccache action][sccache-action] handles setup and teardown:

~~~yaml
name: Build
on: [push]

jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - uses: mozilla-actions/sccache-action@v0.0.7
      - run: cargo build --release
        env:
          RUSTC_WRAPPER: sccache
          SCCACHE_GHA_ENABLED: "true"
~~~

The `SCCACHE_GHA_ENABLED` flag tells sccache to use GitHub Actions' built-in
cache as the storage backend, which requires no additional infrastructure.
```

For local development, sccache is most useful when you frequently switch between
branches or work on multiple projects that share dependencies. The local disk
backend requires no setup beyond installing sccache and setting the wrapper.

## Reading

```reading
style: article
title: "sccache"
url: https://github.com/mozilla/sccache
author: Mozilla
---
sccache is a ccache-like tool that provides shared compilation caching with
various storage backends including cloud storage buckets, Redis, and memcached.
Originally developed by Mozilla for Firefox builds, it supports Rust, C, and
C++ compilation.
```

[sccache]: https://github.com/mozilla/sccache
[sccache-action]: https://github.com/mozilla-actions/sccache-action
