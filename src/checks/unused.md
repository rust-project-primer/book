# Unused Dependencies

An issue you might run into is having unused dependencies in your Rust crates.
When this is the case, Cargo will fetch and build those dependencies every time
your crate is built, and it will not warn if they are not used. This leads to
unneccessarily long compile times.

> If you need some dependencies only conditionally, consider using [crate
> features](../organization/features.md) or [conditional
> dependencies](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html#platform-specific-dependencies)
> to prevent them from being depended on unneccessarily. These let you
> express that dependencies are only needed if a certain feature is activated
> or on a specific platform.

This is something that can easily happen, especially as code is refactored.
Since there is no warning, it happens silently and can go by undetected.  For
that reason, it is advisable to have some kind of check (whether automated or
manual) to detect this situation and remedy it proactively.

There are two tools which can help in this situation: [cargo-udeps][] and
[cargo-machete][]. These both achieve the same but have different tradeoffs.

### `cargo-machete`

The [`cargo-machete`][cargo-machete] tool should be your go-to to solve this.
It aims to be very fast, at the expense of some precision. For that reason, it
is very suitable for running in CI for every merge request.

It can be installed like this:

```
cargo install cargo-machete
```

Once installed, it can be invoked in a project like this:

```
cargo machete
```


### `cargo-udeps`

The [`cargo-udeps`][cargo-udeps] tool is more accurate, however it is also
slower as it relies on compiling the crate.

It can be installed like this:

```
cargo install cargo-udeps
```

Once installed, it can be invoked in a project like this:

```
cargo udeps
```

## Examples

> Having unused dependencies only impacts compile time but not the correctness
> of the project. For that reason, this check could be performed on a schedule
> (nightly or weekly, for example) rather than on every merge request, or
> manually by maintainers.

*TODO*

## Reading

- [cargo machete, find unused dependencies quickly](https://blog.benj.me/2022/04/27/cargo-machete/)
- [Item 25: Manage your dependency graph](https://www.lurklurk.org/effective-rust/dep-graph.html) (Effective Rust)
- [Finding unused dependencies with `cargo-udeps`](https://fasterthanli.me/series/updating-fasterthanli-me-for-2022/part-1#finding-unused-dependencies-with-cargo-udeps) by [@fasterthanlime](https://fasterthanli.me)

[cargo-udeps]: https://github.com/est31/cargo-udeps
[cargo-machete]: https://github.com/bnjbvr/cargo-machete
