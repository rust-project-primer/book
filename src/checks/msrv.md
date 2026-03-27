# Minimum Supported Rust Version

In [Build System: Cargo](../build-system/cargo.md), we've explained that when
you build library crates, you can specify a MSRV. This specifies the minimum
version of the Rust toolchain you need to use your library. Setting this
communicates to the _users_ of your library what version of Rust they should be
using at least.

If you set this, you might end up in a situation where this is no longer true:
you've inadvertently started using Rust features that are not available in the
MSRV version. Specifying a MSRV that is incorrect is arguably worse than not
specifying one at all.

So, how can you use tooling to ensure that the MSRV that you specify matches the
reality of what your crate needs? Here is another Cargo plugin that comes to the
rescue: `cargo-msrv` allows us to determine our crate's true MSRV.

## Cargo MSRV to determine MSRV

```
cargo-msrv
cargo-hack --rust-version
```

## Cargo Hack to test MSRV

## Reading

TODO
