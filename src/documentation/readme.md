# Documentation

Writing software is as much communicating to *other humans* as it is
communicating with the machine we expect it to run in.

In part, documentation solves the \\( O(n^2) \\) communication complexity
issue: if you have three developers which each own some part of the project,
then you can afford to have them communicate with each other to understand how
things work and skip the work of documenting it propertly. However, this does
not scale to large teams: if you have 100 developers that each own some
components, and they all need to talk to each other to understand each other's
work (and no documentation), then your developers will spend more time asking
how things work than getting things done (or, worse, reimplement things because
it is easier than figuring out how the original thing was supposed to work).

In other words, in a commercial project, having great documentation saves you a
lot of cost in the long run. It makes the difference whether you need a
year-long onboarding programme for new hires until they hit their productivity
peak, because they don't know how things work and there is not central place to
find out, or whether they can hit the ground running and achieve baseline
productivity within weeks or a month.

In the context of an open-source project, documentation saves you cost as well,
but in a different way. Projects that have good documentation tend to be more
discoverable, and the developers need to spend less time giving users support
or explaining how to do things. That is the power of words: you write them just
once, but they can be read many (millions, thousands) of times.

```admonish info
In some way, we can look at the Rust project for an example of exceptional
documentation.  The Rust community has put a lot of effort into making sure
that there is ample documentation, which helps people get started, get things
done and it even makes it easier for people to contribute to the project. The
Rust project has many kinds of documentation:

- [The Rust Book](https://doc.rust-lang.org/stable/book/) documents the
  language itself, helping people get up to speed.
- [Standard library documentation](https://std.rs) document the standard
  library
- [docs.rs](https://docs.rs) hosts documentation for all crates which are
  published on [crates.io](https://crates.io)
- Books for various parts of the Rust toolchain
  ([rustc](https://doc.rust-lang.org/rustc/index.html),
  [cargo](https://doc.rust-lang.org/cargo/),
  [clippy](https://doc.rust-lang.org/stable/clippy/development/infrastructure/book.html))
- Books for various use-cases
  ([embedded](https://doc.rust-lang.org/stable/embedded-book/),
  [webassembly](https://rustwasm.github.io/docs/book/), [command-line
  applications](https://rust-cli.github.io/book/index.html))
- Books from some popular framework crates
  ([Criterion](https://bheisler.github.io/criterion.rs/book/index.html),
  [Tokio](https://tokio.rs/tokio/tutorial), [Serde](https://serde.rs/))
```

With this breadth of documentation, people new to the Rust language can quickly
get to high-quality explanations for whatever it is they are trying to do.
Having a service publish documentation for crates also has another effect: it
forces crate authors to put good documentation into their crates, because a
lack of such is immediately visible.  This alone has a strong positive effect
on the crate ecosystem.

When you write documentation, the most important question you have to ask is:
who are you writing documentation for? What are they trying to do? If you know
who you are writing documentation for, it tells you what style you have to
write it in, what knowledge you can expect, and into what depth you can go.

Generally, you will have two target audiences:

- **End-users**: they want to evaluate if your project is fit to solve the
  problem they are trying to solve, and find out how they can use it.
- **Developers**: they are trying to understand how your project works, because
  they want to contribute, or maybe they want to fix an issue with it.

The definition of what your end users are depends on what kind of project you
are working on. If you are writing a library, then your end-user will be other
developers who consume this library. If you are writing an application, then
your end-users will be people who install and use the application.

### End-user Documentation

End-users are less interested in the internals (*how things work*) and more
interested in how they can use your project to solve a particular problem.
They want to be able to quickly find out if your project is useful to them, and
how they can use it. Once they have decided to use your project, they will want
an easy way to find out what changed between releases (in terms of features or
APIs).

End-user documentation should contain:

- Explanation of what problems your project solves
- Instruction of how to install your application (or compile your library)
- Instruction of how to configure your application (or library)
- Examples or guides on how to use it for specific use-cases
- Changelog of additions, deprecations or removals of features or APIs between releases
- Code-level documentation (if it is a library)

Often times, this documentation exists in a Readme file and/or a web book
that is hosted by the project.

### Developer documentation

Developers are programmers that want to understand how your project works.
Typically, this is because they are working on it, they want to implement a
feature, they want to improve it, or they want to fix a bug with it.  They need
to be able to easily clone and compile it locally, run unit tests to see if
they changes broke anything, run benchmarks to check if they changes introduced
a regression. They need to be able to submit a patch (merge request) with their
changes. Some developers (maintainers) also need to be able to release new
versions of the code.

Developer documentation should contain:

- Instructions on how to fetch the code (git clone)
- Architecture of the project (diagram)
- High-level explanation of how the code works
- Instructions on how to compile the library
- Instructions on how to run unit tests and benchmarks
- Style guide for code, commits, documentation
- Documentation of processes (how to submit a patch, how to cut a release)
- Code-level documentation (APIs, data structures, invariants)

### Tools to write and publish documentation

In the sections of this chapter, I will go though some of the functionality
Rust has built-in for generating documentation for software projects, and some
tools that are useful for writing the kinds of end-user and developer
documentation outlined here.

## Reading

[Documentation](https://abseil.io/resources/swe-book/html/ch10.html) in Software Engineering at Google

*Tom explains why documentation is needed for software projects to scale,
because they communicate important information about how things work and why
they work the way they do. They save valuable engineering time by giving
engineers access to the information they need quickly, without needing to look
into the code.  He explains what good documentation looks like, and what Google
does to keep it accurate and of high quality.*
