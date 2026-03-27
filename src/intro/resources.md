# Resources

This guide is aimed at developers and project managers already comfortable with
the Rust programming language. It does not cover any fundamentals of the
language itself, only how to structure projects.

You don't necessarily need to be good at Rust for this book to be useful to you,
for example if you are reading it from the perspective of an engineering manager
or software architect who just wants to understand what Rust is all about or
what tools it comes with. But if you do want to write effective Rust, then these
resources should be helpful to you to get started.

I have categorized these resources into two sections: _foundational_ contains
resources that explain concepts and strategies, whereas the _practical_
resources contain hands-on projects for you to follow. None of the links here
earn me any commission. I am recommending them because I think they are useful,
and not because I earn any money from doing so.

## Foundational

Below is a list of books that I've personally found useful resources for
understanding the Rust programming language, and some of the more complex
features it has (for example, how async works under the hood, or how atomics
work). You should have read at least one of these before you embark on your Rust
project.

```reading
style: book
title: Rust Programming Language, 2nd Edition
url: https://doc.rust-lang.org/book/
author: Steve Klabnik and Carol Nichols
---
This book is the official book of the Rust programming language. It covers the
language and toolchain, giving you a thorough starting point for writing
real-world Rust code and understanding other people's code. It also includes
some example projects for you to follow to see how to use it in practice. Also
available in
[print](https://nostarch.com/rust-programming-language-2nd-edition).
```

```reading
style: book
title: Rust for Rustaceans
author: Jon Gjengset
url: https://nostarch.com/rust-rustaceans
---
This book is a deep dive into the Rust programming language. It gives you a
structured understanding how to apply Rust, covering many parts of Rust
projects, from designing interfaces to writing effective tests. In my opinion
it is one of the best explanations of how async works.
```

```reading
style: book
title: Rust Atomics and Locks
author: Mara Bos
url: https://marabos.nl/atomics/
---
This is a book that gives you a deep understanding of atomics. Some of the
core assumptions that you have as a programmer (such as, if your code writes to
variables in a specific order, that the CPU writes to them in that order) break
down the moment you use multi-threading. Rust makes it easy for you to write
heavy multithreaded applications, and typically you will use safe abstractions
to do so. But there are times, for example when you want to implement custom
data-structures, that you need to know how to do so safely. This book gives you
that background information.
```

```reading
style: book
title: Rust Design Patterns
author: Unknown
url: https://rust-unofficial.github.io/patterns/
archived: rust-design-patterns.pdf
---
This is a catalogue of Rust design patterns, anti-patterns and idioms. Going
through these will help you understand common patterns, and avoid
anti-patterns. It also gives rationale for why to avoid certain patterns.
```

```reading
style: book
title: Software Engineering at Google
author: Various Authors
url: https://abseil.io/resources/swe-book/html/toc.html
---
This is not a Rust-specific book. Rather it is a generic book about software
engineering. The reason I am linking it here is that Google is undoubtedly a
company that has originated many of the philosophies of modern software
engineering, and many of those philosophies have ended up being codified in the
Rust programming language and developer tooling. Understanding this book gives
you some of the *whys* behind why the Rust developer tooling is the way it is,
and why it is so effective. Also available as
[print](https://www.oreilly.com/library/view/software-engineering-at/9781492082781/).
```

```reading
style: book
title: Rust Under the Hood
author: Sandeep Ahluwalia and Deepa Ahluwalia
url: https://www.eventhelix.com/rust/rust-under-the-hood/
---
This is a deep-dive into Rust internals and generated assembly. It shows you
how Rust concepts map to machine code, how Rust represents various types
in-memory, how it uses compiler optimizations (such as loop optimizations and
SIMD auto-vectorization). This book is useful if you care about low-level
details, even if you know little about x86 assembly.
```

There may be more useful foundational Rust resources that I have not listed
here, because I might not be aware of them. There are some sites that maintain
collections of useful Rust books, for example
[The Little Book of Rust Books](https://lborb.github.io/book/title-page.html),
[The Rust Bookshelf](https://bookshelf.rs/).

[effective-rust]: https://www.lurklurk.org/effective-rust/
[rustonimicon]: https://doc.rust-lang.org/nightly/nomicon/
[rust-book]: https://doc.rust-lang.org/book/
[rust-book-nostarch]: https://nostarch.com/rust-programming-language-2nd-edition

## Practical

Some people, including myself, enjoy learning new things through interactive
exploration. These resources teach Rust concepts primarily in such a way.

**Effective Rust** by David Drysdale is a book that lists hands-on
recommendations for writing effective Rust code. It focusses on idioms, giving
practical advice on implementing types, traits, Rust concepts, dependencies, and
tooling. I would consider it a must-read for anyone new to Rust. Available
[online](https://effective-rust.com/), in
[print](https://www.oreilly.com/library/view/effective-rust/9781098151393/),
[archived](/archived/2024-11-09-effective-rust.pdf).

**Zero to Production** by Luca Palmieri is a practical guide for building
production-ready Rust web applications. This is a great book to get started on
understanding how to build real-world Rust application, including handling
migrations, logging, error reporting, metrics. Available
[online][zero-to-production].

**Comprehensive Rust** is a Rust training course developed by Google, aimed at
getting people new to Rust up to speed on development quickly. Available
[online](https://google.github.io/comprehensive-rust/).

**CodeCrafters** is a learning platform with support for Rust. While not
specific to Rust, CodeCrafters has a growing number of courses that are all
built around the idea of reimplementing popular software yourself. Some of the
courses they have are _Build your own Git_, _Build your own Redis_, and _Build
your own SQLite_, to name but a few. What makes the courses fun is that they are
broken down into small steps and come with unit tests that allow you to test
your implementation as your progress. Available
[online](https://codecrafters.io/).

**Rust Adventure** by Chris Biscardi is a collection of interactive courses that
teaches you how to build things in Rust through a set of workshops. Available
[online](https://www.rustadventure.dev/).

[zero-to-production]: https://www.zero2prod.com/

## Articles

Some people in the Rust community have written articles and guides with a
similar scope as this book. While some of the takes may be different from those
presented in this book, it can be valuable to review these to see which
conclusions others in the Rust community have arrived at.

```reading
style: article
title: One Hundred Thousand Lines of Rust
url: https://matklad.github.io/2021/09/05/Rust100k.html
author: Alex Kladov
---
This is a series of articles that summarize what Alex has learned in
maintaining several mid-sized Rust projects. He has some advice on
documentation, writing effective tests and improving build times.  Alex Kladov
is the driving force behind several high-profile projects in the Rust
community, such as [rust-analyzer](https://github.com/rust-lang/rust-analyzer).
```

[Writing Software that's reliable enough for production](https://www.sciagraph.com/docs/understanding/reliable/)
by [Sciagraph](https://www.sciagraph.com/): Sciagraph is a profiler for Python
data processing pipelines. In this blog post, they explain how they approach
writing software that is reliable, with some very similar approaches as this
guide recommends.

[Basic Things](https://matklad.github.io/2024/03/22/basic-things.html) by Alex
Kladov

_Alex argues for some basic properties of software projects. He discusses how
getting these right can be a force-multiplier as projects grow in scope,
developers and users._

[My Ideal Rust Workflow](https://fasterthanli.me/articles/my-ideal-rust-workflow#building-checking-testing-linting)
by [fasterthanlime](https://fasterthanli.me/)

[Chapter 5: Continuous Deployment for Rust Applications](https://www.lpalmieri.com/posts/2020-11-01-zero-to-production-5-how-to-deploy-a-rust-application/)
in [Zero to Production](https://zero2prod.com)

[Good Practices for Writing Rust Libraries](https://pascalhertleif.de/artikel/good-practices-for-writing-rust-libraries/)
by [pascalhertleif](https://pascalhertleif.de/) (_published in 2015_)

## Videos

[Setting up CI and property testing for a Rust crate](https://www.youtube.com/watch?v=xUH-4y92jPg)
by Jon Gjengset

_In this video, Jon shows how to set up a CI pipeline and property testing for a
crate he has authored. This primer explains a lot of the things he does here and
why he does them. This stream is worth watching if you are interested in
watching the process of getting useful testing setup for a project._

https://guidebook.theopensourceway.org/license
