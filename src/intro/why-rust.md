# Why Rust?


In his seminal paper, [Go To Statement Considered
Harmful](https://homepages.cwi.nl/~storm/teaching/reader/Dijkstra68.pdf) the
Dutch computer scientist Edgar Dijkstra postulated something novel: restricting
computer scientists in what they can do can lead to better software projects.
The paper explains that, while it is possible to write programs that use `goto`
statements to jump around, better code quality and maintainability can be
achieved through the use of *structured programming*, concept such as *for* and
*while* loops, and functions. This abstraction lets programmers write code that
is easy to follow, expand and maintain.

In some ways, the Rust programming language is a manifestation of an extension
of this idea: just like the flow through a program needs structure, the ownership
of memory needs ownership.

## Software Complexity is Growing

- software complexity
- difficulty to write multithreaded applications
- difficulty to scale software
- increase in vulnerabilities

![XKCD 2375](https://imgs.xkcd.com/comics/dependency.png)

## Rust as a Programming Language

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

https://medium.com/@penberg/why-i-am-not-yet-ready-to-switch-to-zig-from-rust-3660e02f0060

https://matklad.github.io/2024/10/06/ousterhouts-dichotomy.html

https://www.infoq.com/presentations/Null-References-The-Billion-Dollar-Mistake-Tony-Hoare/
