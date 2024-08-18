# Introduction

## Why this book?

### Why Rust?

As a programming language nerd, I have had the privilege to be able to explore
a number of programming languages before, many of which are niche. Every
programming language I have encountered has some amount of wisdom embedded in
it; I feel that I learn something new from every language I encounter and
familiarize myself with.

The three most significant wisdoms that the Rust language has taught me are:

1. Abstractions are not always a trade-off. You can design useful abstractions
   that have no cost (*zero-cost abstractions*).
2. Safety, especially memory safety, is non-negotiable. It is fundamental to
   building robust software and it cannot be an afterthought.
3. Having good tooling makes working with a language delightful.

To me, having a language that both has useful abstractions, prioritizes and has
tooling that is a joy to use sounds like a really good time. Combine that with
an excellent software ecosystem, a package manager that works well and
reliably, and a user base that is helpful makes it my favorite language.

### Why robust software?

One of the things I have learned from working at various companies is that bugs
are very expensive, and they grow quadratically. That means that when you have
a small codebase, it is quite easy to make sure the code you write is correct.
But as codebases grow, it becomes harder and harder to ensure that. Systems
become large and interact in complex ways, which makes it easy to introduce
unintended bugs and difficult to track them down.

I think the reason why this happens is that every programmer has a constant
rate of bugs that they produce. As software grows in complexity, it accumulates
systems, and it accumulates system interactions. As systems interact with more
other systems, directly or transitively, the chance of introducing bugs and the
difficulty of tracking them down gets higher.

As a programming language, Rust allows you to write code that is free from a
lot of classes of bugs: it makes it *impossible*, or at the least *very
difficult* to write code that is memory-unsafe or multithreading-unsafe.
However, there can still be logic bugs in the application. The majority of
this guide is focussed on giving you the tools you need to make sure you
structure projects in a way that minimizes the number of bugs.

### Why free?

I'm putting this guide out there, permissively licensed under the *CC BY-NC-SA
4.0* license. There are no advertisements or subscriptions, nor do I charge a
fee for reading this book.

In some ways, writing this guide is my way of giving back to the Rust community,
which has given me the tools to write code joyfully. If you want to give something
back, I suggest you get involved in the community, for example:

- Helping with the [Rust compiler development][rustc], [RFC process][rfc] or
  joining a [workgroup][governance],
- Helping the Rust [crate ecosystem][crates], by participating in building
  features or fixing bugs,
- Sharing your knowledge through blog posts, guides or tutorials.

[rustc]: https://github.com/rust-lang/rust
[rfc]: https://github.com/rust-lang/rfcs
[governance]: https://www.rust-lang.org/governance
[crates]: https://crates.io

If you are new to the Rust programming language, I recommend you to spend some
time writing documentation for Rust crates that need it. It is a good way to be
exposed to some Rust code and make an impact, and documentation is usually
appreciated and uncontroversial.

### How to read this book

This book is structured like a recipe book: you can read it cover-to-cover,
if you like. But you can also use it as a tool to look up recipes for how
to solve issues you might run across.

## Reading

This guide is aimed at developers and project managers already comfortable with
the Rust programming language. It does not cover any fundamentals of the
language itself, only how to structure projects.

If you feel that you need to refresh your Rust basics, I recommend you to read
or work through some of these resources. You do not need to work through all of
them, the guide merely assumes some foundational knowledge.

### Books

Below is a list of essential readings for grasping the intricacies of Rust.
Throughout this guide, where relevant, I will provide links to specific
chapters from these books. This will enable you to explore topics more
thoroughly and deepen your understanding as needed.


*Rust Programming Language, 2nd Edition* by Steve Klabnik and Carol Nichols
(available from [No Starch
Press](https://nostarch.com/rust-programming-language-2nd-edition) and
[online][rust-book]) is the official guide to Rust.

*Rust for Rustaceans* by Jon Gjengset (available from [No Starch
Press](https://nostarch.com/rust-rustaceans)) is a deep dive into the Rust
programming language.

*Rust Design Patterns* (available
[online](https://rust-unofficial.github.io/patterns/)) is a catalogue of Rust
design patterns, anti-patterns and idioms.

*Software Engineering at Google* (available [online](https://abseil.io/resources/swe-book/html/toc.html) and from [O'Reilly](https://www.oreilly.com/library/view/software-engineering-at/9781492082781/))

### Interactive

Some people, including myself, enjoy learning new things through interactive
exploration. These resources teach Rust concepts primarily in such a way.

*Zero to Production* by *Luca Palmieri* (available
[online](https://www.zero2prod.com/)) an introduction to backend development in
Rust.

*Rust Adventure* by *Chris Biscardi* (available
[online](https://www.rustadventure.dev/)) is collection of interactive courses
that teaches you how to build things in Rust through a set of workshops. |

*CodeCrafters* Available [online](https://codecrafters.io/).  While not specific to Rust, CodeCrafters has a growing number of courses that are all built around the idea of reimplementing popular software yourself. Some of the courses they have are *Build your own Git*, *Build your own Redis*, and *Build your own SQLite*, to name but a few. What makes the courses fun is that they are broken down into small steps and come with unit tests that allow you to test your implementation as your progress.

### Articles

Some people in the Rust community have written articles and guides with a
similar scope as this book. While some of the takes may be different from those
presented in this book, it can be valuable to review these to see which
conclusions others in the Rust community have arrived at.

[Writing Software that's reliable enough for
production](https://www.sciagraph.com/docs/understanding/reliable/) by
[Sciagraph](https://www.sciagraph.com/)

*Sciagraph is a profiler for Python data processing pipelines. In this blog
post, they explain how they approach writing software that is reliable, with
some very similar approaches as this guide recommends.*

[One Hundred Thousand Lines of
Rust](https://matklad.github.io/2021/09/05/Rust100k.html) by [Alex
Kladov](https://matkld.github.io)

*Alex Kladov is the driving force behind several high-profile projects in the
Rust community, such as
[rust-analyzer](https://github.com/rust-lang/rust-analyzer). In this article,
he explains lessons he has learned from maintaining several medium-sized Rust
projects.*

[Basic Things](https://matklad.github.io/2024/03/22/basic-things.html) by Alex Kladov

*Alex argues for some basic properties of software projects. He discusses how
getting these right can be a force-multiplier as projects grow in scope,
developers and users.*

[My Ideal Rust
Workflow](https://fasterthanli.me/articles/my-ideal-rust-workflow#building-checking-testing-linting)
by [fasterthanlime](https://fasterthanli.me/)

[Chapter 5: Continuous Deployment for Rust
Applications](https://www.lpalmieri.com/posts/2020-11-01-zero-to-production-5-how-to-deploy-a-rust-application/)
in [Zero to Production](https://zero2prod.com)

[Good Practises for Writing Rust
Libraries](https://pascalhertleif.de/artikel/good-practices-for-writing-rust-libraries/)
by [pascalhertleif](https://pascalhertleif.de/) (*published in 2015*)

[Issue #5656: Expand "CI Best Practises" section in the
guide](https://github.com/rust-lang/cargo/issues/5656) in
[rustlang/cargo](https://github.com/rust-lang/cargo)

### Videos

[Setting up CI and property testing for a Rust
crate](https://www.youtube.com/watch?v=xUH-4y92jPg) by Jon Gjengset

*In this video, Jon shows how to set up a CI pipeline and property testing for
a crate he has authored. This primer explains a lot of the things he does here
and why he does them. This stream is worth watching if you are interested in
watching the process of getting useful testing setup for a project.*

[rust is not about memory safety](https://o-santi.github.io/blog/rust-is-not-about-memory-safety/)


[zero-to-production]: https://www.zero2prod.com/
[rust-design-patterns]: https://rust-unofficial.github.io/patterns/
[effective-rust]: https://www.lurklurk.org/effective-rust/
[rustonimicon]: https://doc.rust-lang.org/nightly/nomicon/
[rust-book]: https://doc.rust-lang.org/book/
[rust-book-nostarch]: https://nostarch.com/rust-programming-language-2nd-edition
[rust-book-image]: https://nostarch.com/sites/default/files/styles/uc_product/public/RustProgramming2ndEd_comp.png
[rust-for-rustaceans]: https://nostarch.com/rust-rustaceans
[rust-for-rustaceans-image]: https://nostarch.com/sites/default/files/styles/uc_product/public/RustforRustaceans_cover.png
