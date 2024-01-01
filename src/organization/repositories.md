# Repositories

*Following the advice of this book, you have split your project into a set of
smaller crates which has allowed you to speed up development again, because it
makes it faster to build and run tests. However, often times developers need to
run tests of several crates, and they have noticed that Rust seems to compile
the same dependencies once for every crate. It make you wonder if there is some
mechanism by which a set of crates can share a single build cache and not need
that.*

Rust is designed to cope with projects with a lot of crates. In fact, this is
a pattern that is quite common amongst Rust projects.

```admonish
While you can put every crate into its own repository, I recommend that you
start new Rust projects with a monorepo approach at first, keeping all crates
that the project consists of in a single repository. This gives you greater
iteration speed and allows you to make use of this workspace feature.

Once the code has reached some amount of stability, you can consider moving
some individual crates out into their own repository. Only do this if and when
you think it is worth doing so and paying the cost of doing proper versioning.
```

If you have a single repository which contains multiple crates, you can use the
Cargo workspace feature, which exists for exactly this use-case.

## Workspace

Rust is designed to cope well with projects that have a lot of small crates.
It even has a feature catered to exactly this use-case: the *workspace*. When you
use a workspace, you tell Cargo that group of crates are related and should share
the same build cache, and optionally some metadata.

```admonish
Any time you have a project that contains more than one crate, you should
create a Cargo Workspace to make sure that they can all share a build cache.
```

You can crate a Cargo workspace by adding a `[workspace]` section in you `Cargo.toml`:

```toml
[workspace]
resolver = "2"
members = ["crates/crate-a", "crates/crate-b"]

[workspace.package]
license = "MIT"
authors = ["John Doe <john.doe@example.com"]
```

Workspaces have some other interesting properties. When you run `cargo test` in
a workspace, it defaults to running all tests for all crates. Some of the Rust
tooling has `--workspace` or `--all` flags which tell the tools to act on the
entire workspace instead of only the crate you are currently located in.

### Examples

~~~admonish example
Here is an example of what a cargo workspace project looks like. You can see
how the root `Cargo.toml` only contains the workspace definition, and there
are several crates contained in it.

```files
path = "cargo-workspace"
git_ignore = true
default_file = "Cargo.toml"
```
~~~


## Reading

[Chapter 7: Managing Growing Projects with Packages, Crates and Modules](https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html) in *The Rust Programming Language*

*This chapter in the Rust book explains the different organizational structures
that Rust has, and how they can be used. It mentions the use of workspaces for
managing related crates in a project.*

[Chapter 14.3: Cargo Workspaces](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html) in *The Rust Programming Language*

*This section in the Rust book introduces the concept of the workspace, and
gives some examples for how it can be used in a project.*

[Chapter 3.3: Workspaces](https://doc.rust-lang.org/cargo/reference/workspaces.html) in *The Cargo Book*

*This section in the Cargo book explains the workspace feature, and all of the
configuration options that are available for it in the Crate manifest.*

[An Opinionated Guide To Structuring Rust Projects](https://www.justanotherdot.com/posts/an-opinionated-guide-to-structuring-rust-projects.html) by Ryan James Spencer

