# Features

_A Rust library that you are working on implements a web server. Your webserver comes with support
for doing TLS, however not all of your users require it. The support for TLS requires several
dependencies. Your users complain that they like to use your webserver, however they are unhappy
with it making their builds slow because it pulls in a lot of dependencies. What can you do to
ensure that your webserver does not take too much time to build?_

In this situation, you have two options: you can move the TLS-related functionality into a separate
crates (as discussed in the previous section), or you can turn it into an optional feature of your
crate such that it can be disabled.

```admonish
When you have optional features that can easily be pulled out into a separate
crate, prefer doing so.
```

## Crate Features

You may run into a situation where you are working on a crate which has support for features (and
thus dependencies) that not all of the users need. Using crate features allows you to express this
and give your users the ability to turn those features (and thereby dependencies) off if they do not
need them.

```admonish
Where possible, optional functionality in crates should be exposed using
features.
```

As crates get large, this makes it possible to build and run tests on just the feature that you are
currently working on, reducing compile times. It also makes it possible for consumers of the crate
to specify which features they need.

Finally, it allows for building zero-cost abstractions: you can build extra features into your code,
but the consumer only pays the cost (in terms of compile time and binary size) when they are turned
on.

Declaring optional features in a Cargo manifest is quite simple, simply add a `[features]` section
to the `Cargo.toml` with the list of features. You can also add a `default` key which holds the
features which are enabled by default.

```toml
[features]
default = ["a"]
a = []
b = []
```

Often times, enabling a feature requires a dependency. In that case, you can tell Cargo that the
dependency is only needed when the feature is enabled like this:

```toml
[dependencies]
# mark dependency as optional
extra-dependency = { version = "0.1.2", optional = true }

[features]
# require extra-dependency if my-feature is enabled
my-feature = ["dep:extra-dependency"]
```

## Examples

````admonish example
```files
path = "crate_features"
git_ignore = true
```
````

````admonish example
A library containing data structures, which has an optional `serde` feature to enable
serialization and deserialization with serde. Using `#[cfg(feature = "serde")]`, lines
of the code can turned on conditionally. Using `#[cfg_attr(feature = "serde", ...)]`,
derives can be made conditionally on the feature being enabled.

```files
path = "optional-serde"
files = ["!Cargo.lock"]
git_ignore = true
hidden = true
```
````

## Reading

- The Cargo Book: [Features](https://doc.rust-lang.org/cargo/reference/features.html)
- Effective Rust: [Be weary of feature creep](https://www.lurklurk.org/effective-rust/features.html)
