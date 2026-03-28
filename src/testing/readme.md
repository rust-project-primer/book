# Testing

Testing is the process of verifying that code is correct. It can be done
manually, but automated testing is cheaper over the long run because the same
checks run on every change without human effort. Some development paradigms,
like [Test-Driven Development][tdd], go further and use tests as the primary
artifacts that drive design.

## Why Tests are Needed

Thorough tests give you three things: confidence that features work correctly
for both expected and unexpected inputs, protection against regressions when
code changes, and, when documentation is lacking, a form of executable
specification that shows how code is intended to be used.

This matters for development speed. With good test coverage, developers can
implement new features or refactor code without worrying about silently breaking
existing functionality. Without it, bugs surface only in production.

The most robust software tends to have the most extensive tests. SQLite, the
most widely deployed database, is a good example: the source code is free and
open-source, but the developers charge for access to their test suite. This
reflects a practical insight — for a database that must guarantee data integrity
across billions of deployments, the value lies not in the code itself but in the
tests that make it safe to change. SQLite has 100% branch coverage and millions
of test cases.

## How Tests are Written

Tests are typically divided into unit tests and integration tests. Unit tests
exercise small pieces of code in isolation, often with access to private
internals, and each test verifies a single behavior. Integration tests exercise
the code from the outside, without access to internals, and verify that
components work together correctly. The aim is to have many fast unit tests for
individual behaviors and a smaller set of integration tests that tie the system
together. Writing tests early influences the system design toward code that is
easy to test.

Rust adds a third category: documentation tests. Code examples in doc comments
are compiled and executed by `cargo test`, which ensures that documentation
stays in sync with the code. If an interface changes in a way that breaks a doc
example, the test suite catches it.

## What this Chapter Covers

This chapter covers the testing approaches available in the Rust ecosystem, from
built-in facilities to third-party tools. Each approach has different strengths
and costs, and they complement each other rather than competing.

| Approach          | What it catches                       | Speed  | Run in CI    | Run locally         |
| ----------------- | ------------------------------------- | ------ | ------------ | ------------------- |
| Unit tests        | Logic errors, regressions             | Fast   | Every commit | Every change        |
| Integration tests | Interface mismatches, system behavior | Medium | Every commit | Before push         |
| Doc tests         | Outdated documentation examples       | Fast   | Every commit | Every change        |
| Snapshot tests    | Unintended output changes             | Fast   | Every commit | Every change        |
| Property tests    | Edge cases, invariant violations      | Fast   | Every commit | Every change        |
| Fuzzing           | Crashes, panics on untrusted input    | Slow   | Scheduled    | Occasionally        |
| Mutation testing  | Gaps in test coverage                 | Slow   | Scheduled/PR | Occasionally        |
| Dynamic analysis  | Undefined behavior, memory errors     | Slow   | Every commit | When writing unsafe |

A practical starting point for most projects is: unit tests and integration
tests on every commit, property tests alongside unit tests for code that handles
varied inputs, and snapshot tests for anything with complex output. Fuzzing and
mutation testing are valuable but slow, so they work best as scheduled CI jobs
or as targeted checks on changed files.

The common thread is that testing should be fast enough that developers actually
run it. If your test suite takes too long, people will skip it locally and push
untested code. Splitting tests into fast tests (unit, snapshot, property) that
run on every change and slow tests (fuzzing, mutation, dynamic analysis) that
run on a schedule is a good way to get broad coverage without slowing down
development.

## Reading

```reading
style: book
title: "Item 30: Write more than unit tests"
url: https://effective-rust.com/testing.html
author: Effective Rust
---
This chapter advocates for a comprehensive testing strategy beyond unit tests,
covering integration tests, doc tests, examples, benchmarks, and fuzz testing.
It emphasizes that different test types serve distinct purposes: unit tests
verify internals, while integration tests and examples validate the public
contract.
```

```reading
style: article
title: How to organize Rust tests
url: https://blog.logrocket.com/how-to-organize-rust-tests/
author: Andre Bogus
---
In this article, Andre discusses how tests are best organized in a Rust project.
He goes over the various facilities that Rust has for writing tests, from
testing that code in the documentation compiles (doctests), to simple unit
tests, to integration tests, and explains concepts such as snapshot-testing and
fuzzing.
```

```reading
style: article
title: Writing software that's reliable enough for production
url: https://www.sciagraph.com/docs/understanding/reliable/
author: Sciagraph
---
Describes the testing strategy for Sciagraph, a Python memory profiler built
with Rust. Covers coverage marks (verifying specific code paths are hit),
property-based testing with proptest, end-to-end tests in both debug and
release modes, and panic injection testing. Also discusses choosing Rust for
memory safety, wrapping unsafe APIs in safe interfaces, and environmental
assertions at startup to catch configuration mismatches.
```

```reading
style: book
title: Testing Overview
url: https://abseil.io/resources/swe-book/html/ch11.html
author: Software Engineering at Google
---
Adam discusses the philosophy behind writing software tests. He explains that
well-written tests are crucial to allow software to change. For tests to scale,
they must be automated. Features that other components or teams rely on should
have tests to ensure they work correctly. Testing is as much a cultural problem
as it is a technical one, and changing the culture in an organization takes
time.
```

```reading
style: book
title: "Chapter 11: Writing automated tests"
url: https://doc.rust-lang.org/book/ch11-00-testing.html
author: The Rust Book
---
This chapter of the Rust book explains Rust's facilities for writing unit
tests, and how they can be organized in a project.
```

```reading
style: article
title: How SQLite is tested
url: https://www.sqlite.org/testing.html
author: SQLite
---
SQLite is the world's most deployed database. It is implemented as a C library
that can be embedded into applications directly, and it powers anything from
iPhones to web servers. This article outlines the approach that the SQLite team
uses to ensure that it stays correct over time, with 100% branch test coverage
and millions of test cases. The SQLite team considers testing so valuable that
while the source code itself is free and open-source, the tests are only
available commercially.
```

```reading
style: article
title: How to Test
url: https://matklad.github.io/2021/05/31/how-to-test.html
author: Alex Kladov
---
This article outlines Alex' philosophy when it comes to testing software. He
explains some goals and strategies to make tests easier to maintain, to make it
easier to add tests (reduce friction), make tests fast, using snapshot/expect
style tests for ease of maintenance, and other strategies that make testing
more effective and more pleasant.
```

```reading
style: article
title: Unit and Integration tests
url: https://matklad.github.io//2022/07/04/unit-and-integration-tests.html
author: Alex Kladov
---
In this article, Alex compares unit-testing and integration-testing, and
concludes that their main difference is the amount of purity (I/O) and the
extent of the code that they are testing. He argues that it makes sense to try
to get tests to be as pure as possible.
```

```reading
style: article
title: Everything you need to know about testing in Rust
url: https://www.shuttle.rs/blog/2024/03/21/testing-in-rust
author: Joshua Mo
---
This article gives an overview of Cargo features for testing and libraries in
the Rust ecosystem that can help in writing useful tests for software. It goes
through multiple concepts, such as property testing, fuzzing and snapshot
testing and gives examples.
```

```reading
style: book
title: Advanced Rust testing
url: https://rust-exercises.com/advanced-testing/00_intro/00_welcome.html
author: rust-exercises.com
---
Hands-on course that goes beyond basic testing into testing interactions with
external systems like APIs and databases. Progresses through small lessons with
exercises, building up to a comprehensive testing strategy for complex
applications. Aimed at intermediate Rust developers who already know the
basics of `#[test]` and want to expand their toolkit.
```

[tdd]: https://en.wikipedia.org/wiki/Test-driven_development
