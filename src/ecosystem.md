# Ecosystem

Before you start your project, you may need to put some throughts towards what
kind of project you want to build, and choose the right ecosystem. 

Rust has a vibrant community of all kinds of projects, usually over time
certain crates become more popular and establish themselves as the go-to. You
should certainly make use of the ecosystem and the ease with which Cargo lets
you add and manage dependencies.

![bubble graph of popular crates](graphics/crate-popularity.svg)

Most of the time, it is relatively easy to switch between different crates.
However, in some cases the crates you decide to use have an influence over the
architecture of your project. For example, it is not always so easy to convert
a blocking, threaded application into an async one, or to switch from one web
framework to another.

It is usually better to put some thought into this before you start developing,
because it might be difficult to switch once you've already invested in
building your project with one ecosystem. This sections aims at showing you
the Rust ecosystem for some common tasks, wherever the choices you make have
a large impact on the architecture of your project.

## Reading

[On Dependency Usage in Rust](https://landaire.net/on-dependency-usage-in-rust/)

*The C programming language is often critizied for not bringing a lot of
foundational data structures out-of-the-box, leading many developers to
reinvent the wheel. Adding and managing dependencies in C/C++ is difficult,
because there is no standardized build system. On the other hand, in JavaScript
it is so easy to add dependencies, that many small projects end up with
gigabytes worth of trivial (transitive dependencies), which is criticized as a
security risk. This article explains how dependencies work in Rust, and why
it's okay to use them.*
