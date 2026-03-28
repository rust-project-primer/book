# Resources

The rest of this book assumes you are comfortable with Rust as a language. If
you are still learning, or want to deepen your understanding of specific areas
like async or atomics, the resources below are a good place to start.

## Books

```reading
style: book
title: The Rust Programming Language, 2nd Edition
url: https://doc.rust-lang.org/book/
author: Steve Klabnik and Carol Nichols
---
The official book of the Rust programming language. Covers the language and
toolchain from the ground up, with example projects that show how concepts
fit together in practice. The starting point for most Rust developers. Also
available in
[print](https://nostarch.com/rust-programming-language-2nd-edition).
```

```reading
style: book
title: Effective Rust
url: https://effective-rust.com/
author: David Drysdale
---
Hands-on recommendations for writing idiomatic Rust code, organized as a
series of actionable items covering types, traits, error handling,
dependencies, and tooling. Particularly strong on the "why" behind Rust
idioms. Also available in
[print](https://www.oreilly.com/library/view/effective-rust/9781098151393/).
```

```reading
style: book
title: Rust for Rustaceans
url: https://nostarch.com/rust-rustaceans
author: Jon Gjengset
---
A deep dive for developers who already know the basics. Covers designing
interfaces, writing effective tests, unsafe code, async internals, and
performance. Contains one of the clearest explanations of how async works
under the hood.
```

```reading
style: book
title: Rust Atomics and Locks
url: https://marabos.nl/atomics/
author: Mara Bos
---
Covers low-level concurrency: atomics, memory ordering, and lock
implementations. Essential reading if you need to implement custom
synchronization primitives or understand why certain concurrent patterns
are safe in Rust and others are not.
```

```reading
style: book
title: Rust Design Patterns
url: https://rust-unofficial.github.io/patterns/
author: Rust Community
archived: rust-design-patterns.pdf
---
A community-maintained catalogue of design patterns, anti-patterns, and
idioms specific to Rust. Each entry includes rationale explaining why a
pattern works well or why an anti-pattern should be avoided.
```

```reading
style: book
title: The Rustonomicon
url: https://doc.rust-lang.org/nomicon/
author: The Rust Project
---
The official guide to unsafe Rust. Covers raw pointers, transmutes,
uninitialized memory, the Drop Check, and the exact rules for what
constitutes undefined behavior. Essential reading if you work with FFI
(see the [Interop](interop/readme.md) chapter) or need to implement
data structures that require unsafe code.
```

```reading
style: book
title: Rust by Example
url: https://doc.rust-lang.org/rust-by-example/
author: The Rust Community
---
A companion to The Rust Programming Language that teaches through
annotated, runnable examples rather than long explanations. Each concept
is demonstrated with code you can modify and run in the browser. A good
option if you prefer learning by doing.
```

```reading
style: book
title: The Cargo Book
url: https://doc.rust-lang.org/cargo/
author: The Rust Project
---
The official reference for Cargo: dependency management, workspace
configuration, build scripts, feature flags, publishing, and custom
profiles. Since nearly every chapter in this book involves Cargo in
some way, this is a useful reference to keep at hand.
```

For more Rust books, see
[The Little Book of Rust Books](https://lborb.github.io/book/title-page.html)
and [The Rust Bookshelf](https://bookshelf.rs/).

## Courses

```reading
style: book
title: Comprehensive Rust
url: https://google.github.io/comprehensive-rust/
author: Google
---
A multi-day Rust training course developed by Google's Android team. Covers
the language from basics through advanced topics like async and unsafe,
with exercises throughout. A good option if you prefer structured,
classroom-style learning.
```

```reading
style: book
title: Zero to Production in Rust
url: https://www.zero2prod.com/
author: Luca Palmieri
---
A practical guide that walks through building a production-ready web
application in Rust, covering project setup, database migrations, logging,
error reporting, and deployment. Good for seeing how the tools and
practices discussed in this book come together in a real project.
```

## Articles

These articles cover similar ground to this book, approaching Rust project
practices from different angles. Reading them alongside this book gives you a
broader perspective on where the Rust community has converged and where opinions
still differ.

```reading
style: article
title: One Hundred Thousand Lines of Rust
url: https://matklad.github.io/2021/09/05/Rust100k.html
author: Alex Kladov
---
Lessons from maintaining several mid-sized Rust projects, including
rust-analyzer. Covers documentation, testing strategies, build times,
and project organization. Many of the recommendations align with what
this book covers, but from the perspective of someone maintaining
widely-used developer tools.
```

```reading
style: article
title: Basic Things
url: https://matklad.github.io/2024/03/22/basic-things.html
author: Alex Kladov
---
Argues that foundational infrastructure (documentation, code review,
testing, reproducible builds, metrics) compounds over time and becomes a
major multiplier as projects grow. A good companion to the Checks and
Testing chapters of this book.
```

```reading
style: article
title: My Ideal Rust Workflow
url: https://fasterthanli.me/articles/my-ideal-rust-workflow
author: Amos Wenger
---
A detailed walkthrough of one developer's professional Rust setup,
covering editor configuration, automated checks with Clippy and
cargo-hack, CI pipelines, and private infrastructure. Useful for seeing
how the individual tools discussed in this book fit together in a
cohesive workflow.
```

```reading
style: article
title: Good Practices for Writing Rust Libraries
url: https://pascalhertleif.de/artikel/good-practices-for-writing-rust-libraries/
author: Pascal Hertleif
---
A practical checklist for publishing Rust libraries: code quality tools
(rustfmt, Clippy, lints), project metadata, README conventions, CI setup,
and documentation deployment. Written in 2015 but most of the advice
remains relevant.
```

```reading
style: article
title: Writing Software that's Reliable Enough for Production
url: https://www.sciagraph.com/docs/understanding/reliable/
author: Sciagraph
---
Uses a production memory profiler as a case study to demonstrate
reliability strategies: language choice, comprehensive testing (unit,
end-to-end, property-based, panic injection), startup validation, and
careful dependency management. Shows how many of the practices in this
book work together in a real system.
```

## Videos

```reading
style: video
title: Setting up CI and Property Testing for a Rust Crate
url: https://www.youtube.com/watch?v=xUH-4y92jPg
author: Jon Gjengset
---
Jon walks through setting up a CI pipeline and property testing for one
of his crates, explaining his reasoning at each step. A good complement
to the Testing and CI chapters of this book, as it shows the process of
making these decisions in real time.
```
