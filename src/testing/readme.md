# Testing

Testing is the process of ensuring that code is correct. It can be done
manually, but typically it is done in an automated way because it is cheaper
over the long run. There are even some development paradigms that use tests are
the primary artifacts of development, such as [Test-Drivent Development][tdd].

## Why Tests are needed

Having rigid tests ensures three things:

1. Features are implemented and work correctly for all inputs, expected or unexpected.
2. Features that are correct now don't break in the future
3. In the case of missing documentation, tests are often the only thing close
   to documenting how code is intended to be used.

Having a thorough tests makes development more pleasant (and speedy), because
it allows developers to implement new features or refactor code, without having
to worry about accidentally breaking existing functionality and only finding
out when the code is deployed. But this is only possible when there is a
reasonable test coverage.

If you look at some of the most widely deployed and robust software, it tends
to have one thing in common: there is an extensive set of tests for it. It goes
so far that some projects charge a fee to get the tests. For example, SQLite, the
most widely deployed database, is open-source. But the developers charge for access
to the private test suite.

## How tests are written

Tests are often divided into different categories:

1. Unit tests test small units of code. They often have access to the private internal
   state of the code. Every unit test tests exactly one feature.
2. Integration tests test the code from an outside perspective, they don't have access to
   the internal state of the code. They often don't test individual features, but
   functionality as a whole.

Typically, the aim is to have many unit tests, to make sure the features work, and have
some integration tests that tie the system together as a whole. Ideally, running unit tests
does not require external dependencies, but integration tests might.

- testing pyramid graphic

Another advantage of writing tests early is that it influences the system
design to be in such a way that it is easy to test.

Rust has another category of tests: documentation tests. In Rust, documentation
can include code examples, and Cargo will test these as well. This ensures that
documentation does not inadvertently get out of date, for example by changing
interfaces.

## What this chapter covers

- culture
- testability engineering


This chapter discusses various features of Rust and the ecosystem and strategies
for using to to ensure correctness of the code.

## Reading

[How to organize Rust tests](https://blog.logrocket.com/how-to-organize-rust-tests/) by Andre Bogus

*In this article, Andre discusses how tests are best organized in Rust project.*

[Writing software that's reliable enough for production](https://www.sciagraph.com/docs/understanding/reliable/) by Scigraph

[Testing Overview](https://abseil.io/resources/swe-book/html/ch11.html) in Software Engineering at Google

*Adam discusses the philosophy behind writing software tests. He explains that
well-written tests are crucial to allow software to change. For tests to scale,
they must be automated. Features that other components or teams rely on should
have tests to ensure they work correctly. Testing is as much a cultural problem
as it is a technical one, and changing the culture in an organization takes
time.*



[Chapter 11: Writing automated tests](https://doc.rust-lang.org/book/ch11-00-testing.html) in *The Rust Book*

https://doc.rust-lang.org/book/ch11-03-test-organization.html

[How SQLite is tested](https://www.sqlite.org/testing.html)

[cargo-nextest book](https://nexte.st/index.html)

[How to Test](https://matklad.github.io/2021/05/31/how-to-test.html) by Alex Kladov

[Unit and Integration tests](https://matklad.github.io//2022/07/04/unit-and-integration-tests.html)


[Everything you need to know about testing in Rust](https://www.shuttle.rs/blog/2024/03/21/testing-in-rust) by Joshua Mo

*This article gives an overview of Cargo features for testing and libraries
in the Rust ecosystem that can help in writing useful tests for software.*

[Advanced Rust testing](https://rust-exercises.com/advanced-testing/00_intro/00_welcome.html) by rust-exercises.com


[tdd]: https://en.wikipedia.org/wiki/Test-driven_development
