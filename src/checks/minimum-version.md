# Dependency Minimum Versions

In Rust projects, usually [Semantic Versioning][semver] is used. This allows us to specify
dependencies not by their exact versions, but by their version bounds. For example, when you
have a dependency bound such as this in your project:

```toml
name = "0.2"
```

You are expressing that you need at least version `0.2.0`, but lower than `0.3.0`. The current
crate version might be at `0.2.77` and it might compile just fine.

However, there is one potential issue here. Because Cargo always tries to use the maximum
possible version, it usually ends up using a later version than the minimum bound you have
specified. 

It is possible that your crate depends on some feature or API that was not
present in `0.2.0`, however because you never actually build against that
version (because version resolution always tries to give you the latest version
possible), you do not find out doing normal builds. That is, until your crate is
used by someone who depends on the same crate, but in version `=0.2.0`.

To detect this issue automatically, Cargo has a feature that allows you to
override the version resolution strategy to always use the minimum possible
version. You can enable this feature using `-Z minimum-version`.

However, an easier approach is to install `cargo-minimal-version` and running
it to check if your code will compile:

```
cargo install cargo-minimal-version
cargo minimal-version check
```


## Reading

- [Semantic Versioning][semver]
- GitHub: [taiki-e/cargo-minimal-versions][cargo-minimal-versions]
- Article: [Rust minimum versions: SemVer is a lie!](https://blog.illicitonion.com/rust-minimum-versions-semver-is-a-lie/)

[semver]: https://semver.org/
[cargo-minimal-versions]: https://github.com/taiki-e/cargo-minimal-versions
