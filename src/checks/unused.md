# Unused Dependencies

*After you have been working on your Rust project for a while, you notice
complaints that the compilation times are slow. You look into it, and discover
that in multiple crates of the project, several dependencies exist which are
unused, driving up compilation times. You remove them manually for now,
but you wonder if there is a way to ensure that your crates do not accumulate
unused dependencies in the future.*

Having unused dependencies in your crates means the compiler needs to do
extra work to fetch and build them even though they will not be used.
Ensuring that you don't have any is therefore an important part in making
sure your compile times are low. 

```admonish
If you have some some dependencies that you need only conditionally, consider
using crate features[^feature] or conditional
dependencies[^conditional-dependencies]
to prevent them from being depended on unneccessarily. These let you express
that dependencies are only needed if a certain feature is activated or on a
specific platform.
```

There are two tools in the Rust ecosystem that you can use to detect
unused dependencies: `cargo-udeps` and `cargo-machete`. These differ in the way
they attempt to detect unused dependencies, and thereby the time they take
to make the determination.

## `cargo-machete`

The [`cargo-machete`][cargo-machete] tool should be your go-to to solve this[^cargo-machete-quickly].
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


## `cargo-udeps`

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

```admonish
Having unused dependencies only impacts compile time but not the correctness of
the project. Running these checks for every merge request might be too
expensive for your project. For that reason, this check could be performed on a
schedule (nightly or weekly, for example) rather than on every merge request,
or manually by maintainers.
```

*TODO*

## Reading

[cargo machete, find unused dependencies quickly](https://blog.benj.me/2022/04/27/cargo-machete/) by Benjamin Bouvier

*Benjamin introduces the `cargo-machete` tool, which lets you quickly find
unused dependencies. He explains how it works, and how it works differently to
`cargo-udeps` to be much faster, at the expense of some accuracy.*

[Item 25: Manage your dependency graph](https://www.lurklurk.org/effective-rust/dep-graph.html) in Effective Rust


[Finding unused dependencies with `cargo-udeps`](https://fasterthanli.me/series/updating-fasterthanli-me-for-2022/part-1#finding-unused-dependencies-with-cargo-udeps) by Amos Wenger

[cargo-udeps]: https://github.com/est31/cargo-udeps
[cargo-machete]: https://github.com/bnjbvr/cargo-machete

[^feature]: See chapter [Optional Features](../organization/features.md).

[^conditional-dependencies]: A feature of Cargo to let you specify dependencies depending on the circumstance, for example the platform. See <https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html#platform-specific-dependencies> for more information.

[^cargo-machete-quickly]: cargo-machete, find unused dependencies quickly: <https://blog.benj.me/2022/04/27/cargo-machete/>
