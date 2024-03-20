# Introduction

```admonish info
This book is still under development. You may find some sections which are
missing, or not fully written out. If you find sections which are incorrect,
feel free to send a correction in the form of an issue or pull request.
```

I wrote this book as a guide for how to bootstrap, structure and maintain Rust
projects out of a passion for the Rust programming language. Having accumulated
some opinions on what allows projects to succeed, I feel that it is valuable to
share what I have learned.

You should not view this as a definitive guide, but rather as a collection of
nuggets of advice that come with rationale and examples. Not every piece of
advice may apply to your project, and you might have different opinions than I
have.  Whether you agree with the advice I give in this book or not, I hope
that you can draw some benefit from it and find useful patterns.

The Rust programming language is a lot younger than some of the programming
languages you have worked with in the past. It also has a development velocity
that is faster than you are used to. You may find that some pieces of advice
become outdated over time. I do my best to try to update this book when I can,
but you should feel free to do your own research.

If you find something in this book that is ourageously wrong, feel free to
create a [merge request][repo] with your suggestion, I am happy to take a look
at it.

[repo]: https://gitlab.com/rust-project-primer/book

This book is a living document, not a finished product. It will be periodically
updated with new insights and revised examples, reflecting ongoing learnings in
the field of Rust programming.

## Why Rust?

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

## Why robust software?

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

## Why free?

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
