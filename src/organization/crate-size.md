# Small Crates

*When you started your Rust project, you just created a new carte using `cargo
new` and started developing. Initially, everything seemed to go well, however
over time the complexity and the number of features grew. More and more you are
feeling held back by very long compile times and a very long time needed to run
all of the unit tests. You feel that there should be a better way to do this.*

It is very easy to get started with a Rust project simply by creating a new
crate, adding some dependencies to it and calling it a day.

```
cargo init my-project
```

However, for more complex projects, this might not be the optimal way to
organize your code. It general, it is better to have multiple, smaller crates
which follow the UNIX philosophy of *do one thing, do it well*.  There are two
reasons for this, one having to do with compilation times and the other having
to do with a philosophy of how projects should be structured.

### Compilation time

Languages such as C, C++ and Java have files as their smallest compilation
unit. This means that when you edit some files, only those files that you have
edited need to be rebuilt.  Rust has a different approach: in Rust, the
smallest compilation unit is a crate. This means that whenever you change any
file, the entire crate needs to be rebuilt.

There are some advantages to this. It means that Rust can do some good
optimization of your code because it can see the entire crate at compile time,
rather than just one file at a time. It also means that you can have import
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

### Examples

If you look at a lot of larger, complex Rust project, you will find that
many of the are composed of smaller, purposeful crates which are pulled
in by one project crate.

```admonish example title="Project composed of small crates"
*TODO*
```

https://two-wrongs.com/decompose-into-larger-modules

## Reading

[Prefer small crates](https://rust-unofficial.github.io/patterns/patterns/structural/small-crates.html) in *Rust Design Patterns*

*This article argues that Rust makes it easy to add dependencies, so there is
no downside to having more of them. Additionally, smaller crates are easier to
understand and lead to more modular code, therefore small crate sizes should be
encouraged.*

[Brainstorm request: How to get benefits of small and large crates](https://internals.rust-lang.org/t/brainstorm-request-how-to-get-benefits-of-small-and-large-crates/10585/2)

*In this discussion, the upsides and downsides of having small crates is
discussed.*

[rfc: collapse Tokio sub crates into single tokio crate](https://github.com/tokio-rs/tokio/issues/1318)

*The Tokio project did the reverse: they used to be composed of many small
crates, and merged them all into one crate. This discussion contains important
context for why this decision was made, and has some arguments against having
many small crates.*

[Why is my Rust build so slow: splitting into more crates](https://fasterthanli.me/articles/why-is-my-rust-build-so-slow#splitting-into-more-crates)

*TODO*
