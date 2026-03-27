# Workspace

As your project grows, you may feel the need to split it up into multiple
crates. Maybe the compilation times are becoming a problem, and having multiple
smaller crates means that most of the application does not need to be rebuilt
when you make a change in one file. Or maybe you want to enforce more loose
coupling between the application, and split the responsibility of various parts
to separate teams.

Rust is designed to cope well with projects that contain a lot of crates. It
even has a feature catered to exactly this use-case: the _workspace_. When you
use a workspace, you tell Cargo that group of crates are related and should
share the same build cache, and optionally some metadata.

- statistics on how many Rust projects use workspaces

## Creating a Workspace

You can create a Cargo workspace by adding a `[workspace]` section in your
`Cargo.toml`:

```toml
[workspace]
resolver = "2"
members = ["crates/crate-a", "crates/crate-b"]
```

The main reasons why you would want to use workspaces rather than simply putting
several crates into a repository is twofold:

- When you use a `workspace`, then your entire project uses a single `target`
  folder, meaning that every dependency is built exactly once. This speeds up
  the build time.
- When you run operations, such as tests, then you can tell `cargo` to run them
  for all crates in the workspace.

Workspaces have some other interesting properties. When you run `cargo test` in
a workspace, it defaults to running all tests for all crates. Some of the Rust
tooling has `--workspace` or `--all` flags which tell the tools to act on the
entire workspace instead of only the crate you are currently located in.

### Examples

````admonish example
Here is an example of what a cargo workspace project looks like. You can see
how the root `Cargo.toml` only contains the workspace definition, and there
are several crates contained in it.

```files
path = "cargo-workspace"
git_ignore = true
default_file = "Cargo.toml"
```
````

## Dependencies

When you work in a large workspace, often times you have a set of dependencies
that all of the crates in the workspace use. In that case, typically you want to
ensure that they all use the same version of the dependency.

For that use-case, Cargo Workspaces allows you to declare dependencies on a
workspace level, and reference them in the daughter crates. This makes it easier
to keep versions of dependencies in sync when they are used by a lot of crates.

To use this feature, you can simply set the `workspaces.dependencies` in the
same way that you would set `dependencies` in a regular crate.

```toml
[workspace.dependencies]
anyhow = "1"
```

In the child crates, you can then reference them like this:

```toml
[dependencies]
anyhow = { workspace = true }
```

It's still possible to override it, for example to turn on additional features.

```toml
[dependencies]
anyhow = { workspace = true, features = ["abc"] }
```

## Metadata

Another commonly used feature of Cargo Workspaces is the ability to set shared
metadata. For example, you can use it to set a license for all crates, or keep
the version of the crates in sync. To do this, you set metadata in the
`workspace.package` in the workspace config, like this:

```toml
[workspace.package]
license = "MIT"
authors = ["John Doe <john.doe@example.com"]
```

To use this, you have to then reference it in the child crates.

```toml
[package]
name = "crate-a"
license.workspace = true
authors.workspace = true
```

Doing this makes sense if you want all child crates to share some amount of
metadata, as is often the case with licenses or authors.

## When to split crates

When is the right time to split crates? This is a question that is not so easy
to answer. Splitting crates has a cost: it means you need to define the
interface well. But if you do it well, it also has advantages. Maybe the code
can be reused for future projects, because it is generic enough. Splitting
crates out prematurely is probably not a good idea, but doing it too late risks
that your code will depend on and use private interfaces that you don't want it
to use.

## Reading

```reading
style: book
title: "Chapter 7: Managing Growing Projects with Packages, Crates and Modules"
url: https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html
author: The Rust Programming Language
---
This chapter in the Rust book explains the different organizational structures
that Rust has, and how they can be used. It mentions the use of workspaces for
managing related crates in a project.
```

```reading
style: book
title: "Chapter 14.3: Cargo Workspaces"
url: https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html
author: The Rust Programming Language
---
This section in the Rust book introduces the concept of the workspace, and
gives some examples for how it can be used in a project.
```

```reading
style: book
title: "Chapter 3.3: Workspaces"
url: https://doc.rust-lang.org/cargo/reference/workspaces.html
author: The Cargo Book
---
This section in the Cargo book explains the workspace feature, and all of the
configuration options that are available for it in the Crate manifest.
```

```reading
style: article
title: An Opinionated Guide To Structuring Rust Projects
url: https://www.justanotherdot.com/posts/an-opinionated-guide-to-structuring-rust-projects.html
author: Ryan James Spencer
---
```

```reading
style: article
title: Prefer small crates
url: https://rust-unofficial.github.io/patterns/patterns/structural/small-crates.html
author: Rust Design Patterns
---
This article argues that Rust makes it easy to add dependencies, so there is
no downside to having more of them. Additionally, smaller crates are easier to
understand and lead to more modular code, therefore small crate sizes should be
encouraged.
```

```reading
style: article
title: "Brainstorm request: How to get benefits of small and large crates"
url: https://internals.rust-lang.org/t/brainstorm-request-how-to-get-benefits-of-small-and-large-crates/10585/2
author: abc
---
In this discussion, the upsides and downsides of having small crates is
discussed.
```

```reading
style: article
title: Cargo Workspace and the Feature Unification Pitfall
url: https://nickb.dev/blog/cargo-workspace-and-the-feature-unification-pitfall/
author: Nick B
---
```

```reading
style: book
title: Resolver
url: https://doc.rust-lang.org/cargo/reference/resolver.html
author: Cargo Project
---
This chapter in the Cargo Book explains how Cargo resolves crate features
in workspaces.
```

[rfc: collapse Tokio sub crates into single tokio crate](https://github.com/tokio-rs/tokio/issues/1318)

_The Tokio project did the reverse: they used to be composed of many small
crates, and merged them all into one crate. This discussion contains important
context for why this decision was made, and has some arguments against having
many small crates._

[Why is my Rust build so slow: splitting into more crates](https://fasterthanli.me/articles/why-is-my-rust-build-so-slow#splitting-into-more-crates)

https://corrode.dev/blog/tips-for-faster-rust-compile-times/#split-big-crates-into-smaller-ones-using-workspaces

_TODO_
