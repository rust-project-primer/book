# Organization

In Rust, code organization is facilitated through a range of structures:
*files*, *modules*, *crates*, and *workspaces*. This chapter aims to provide
guidance on how to best utilize these elements to structure your Rust projects
effectively. The emphasis will be on achieving two key objectives: *enhancing
development speed* and *promoting loose coupling* for better code
maintainability.

<figure>

![Crate workspace example](../images/workspace.svg)

<caption>

*Example of a Rust project's organization, with a single workspace containing multiple
crates.*

</caption>
</figure>

## Development Speed

Rust emphasizes a feature known as *zero-cost abstractions*. These are
programming abstractions that are beneficial for developers, offering utility
without incurring any runtime cost. This focus sets Rust apart from many other
programming languages, which offer similar abstractions but with a runtime
penalty. However, these zero-cost abstractions in Rust are not without their
own trade-off: they often lead to longer compile times[^proc].

This trade-off means Rust code is typically optimized for fast execution at the
expense of compile speed. Yet, faster compile times hold their own importance.
They are crucial in maintaining a tight iteration loop, allowing developers to
quickly make code changes, compile, and test. This rapid feedback loop is
essential for efficient feature development and debugging.

In this chapter, we'll delve into various choices that can be made while
setting up a Rust project to optimize compile times. We'll explore these
options and their implications, aiming to balance efficient development cycles
with optimal runtime performance.

## Loose Coupling

To ensure a system remains maintainable, testable, and easily adaptable,
employing a strategy of loose coupling[^coupling] is often useful. Working
with a large, monolithic application that's tightly coupled can be challenging
and complex, making changes difficult. The ideal scenario involves creating
code composed of smaller, independent units that can be tested on an individual
basis. In this chapter, we'll explore how to achieve this level of modularity
and loose coupling in Rust, laying out strategies to build systems that are
both robust and flexible.

## Reading

- [Chapter 7: Managing Growing Projects with Packages, Crates, and Modules](https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html) in *The Rust Programming Language*
- [Chapter 2.5: Project Layout](https://doc.rust-lang.org/cargo/guide/project-layout.html) in *The Cargo Book*
- [Rust at scale: packages, crates, modules](https://mmapped.blog/posts/03-rust-packages-crates-modules) by Roman Kashitsyn
- [Rust compile times](https://endler.dev/2020/rust-compile-times/) by Matthias Endler
- [The Dark side of inlining and monomorphization](https://nickb.dev/blog/the-dark-side-of-inlining-and-monomorphization/) by Nick Babcock
- [Delete Cargo Integration Tests](https://matklad.github.io/2021/02/27/delete-cargo-integration-tests.html) by Alex Kladov

---

[^proc]: For example, procedural macros allow for eliminating a lot of repeated
code, for example by automatically deriving traits on structures. However, they
need to be built and executed and thus add to the compilation time.
[^coupling]: See [Loose Coupling](https://en.wikipedia.org/wiki/Loose_coupling) (Wikipedia).
