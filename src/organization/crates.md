# Small Crates

It is very easy to get started with a Rust project simply by creating a new
crate, adding some dependencies to it and calling it a day.

```
cargo init my-project
```

However, for more complex projects, this might not be the optimal way to
organize your code.  There are two reasons for this, one having to do with
compilation times and the other having to do with a philosophy of how projects
should be structured.

### Compilation time

Languages such as C, C++ and Java have files as their smallest compilation
unit. This means that when you edit some files, only those files that you have
edited need to be rebuilt.  Rust has a different approach: in Rust, the
smallest compilation unit is a crate. This means that whenever you change any
file, the entire crate needs to be rebuilt.

There are some advantages to this. It means that Rust can do some good
optimization of your code because it can see the entire crate at compile time,
rather than just one file at a time.  It also means that you can have import
loops in your modules.

However, the main downside of this approach is the implication it has on
compile times. If you change any part of your crate, the entire crate needs to
be recompiled and linked again.  This means that you end up having slower
iteration loops.

In general, good advice is thus to try to break down complex projects into
smaller pieces, such that the compile times stay low. When you do that,
only the small crate you are working on needs rebuilding while the other
crates of the project can be cached.

### Philosophy of structure

Another reason to prefer having small crates is a philosophical one, perhaps it
is more of a personal opinion. As projects grow large, complex and tightly
coupled, they become much harder to change and test. 

Having a project where it is not only encouraged but also easy to take
working functionality and split it out into a crate leads to code that is
much more testable, and therefore a lot easier to change.

Breaking functionality down into crates and proactively pulling them out
encourages good software design. It allows you to build useful abstractions,
and it avoids ending up with a giant, complex monolithic application that is
very tightly coupled. This achieves [Loose
coupling](https://en.wikipedia.org/wiki/Loose_coupling).

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

- [Chapter 3.3: Workspaces](https://doc.rust-lang.org/cargo/reference/workspaces.html) in *The Cargo Book*
- [Chapter 14.3: Cargo Workspaces](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html) in *The Rust Programming Language*
- [Chapter 7: Managing Growing Projects with Packages, Crates and Modules](https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html) in *The Rust Programming Language*
- [Prefer small crates](https://rust-unofficial.github.io/patterns/patterns/structural/small-crates.html) in *Rust Design Patterns*
- [An Opinionated Guide To Structuring Rust Projects](https://www.justanotherdot.com/posts/an-opinionated-guide-to-structuring-rust-projects.html) by Ryan James Spencer

