# Crate Features

When writing code in Rust, it is often necessary to make certain parts of the crate optional. Rust achieves this via features. Inside the code, `cfg` blocks can be used to query which features are enabled and enable or disable parts of the code:

```rust
#[cfg(feature = "abc")]
pub fn my_function() {
}
```

The downside of this approach is that it makes it harder to properly test code, because certain feature configurations can now lead to broken code. An approach that the Rust community tends to support is making use of tooling to `cargo check` or `cargo test` every feature, or all feature powersets.

When running this tool in a CI environment, you can guarantee that every feature cleanly builds. However, the downside is that it can add some overhead due to needing to parse the code multiple times (for every feature set). These are some possible ways to use this tool:

```
# Run `cargo check` for every feature:
cargo hack --no-default-features --every-feature check

# Run `cargo check` for every possible combination of features:
cargo hack --no-default-features --feature-powerset check

# Run `cargo test` for every 
cargo hack --no-default-features --every-feature test

# Run `cargo test` for every possible combination of features:
cargo hack --no-default-features --feature-powerset test
```

We would like to use this tool as an optional feature in kraken for building Rust code.
