# External Services

_You notice that in a lot of pull requests, authors need to push several fix commits to get the CI
pipelines to run correctly. A lot of the time, some simple unit tests need to be fixed. When asked,
developers note that they cannot run the test suite locally because it depends on services that run
in the cloud. This makes you wonder, if there is a way to increase iteration speed by making sure
that tests can run locally._

Having a fast iteration loop is key to fast software development. To make that possible, it is
generally advantageous if test suites have no external dependencies, making it easy for developers
to launch them locally and test projects end-to-end.

Whenever possible, try to make it such that you can run all tests locally, and that you can do so
relatively easily.

```admonish
When interfacing with external systems, you need to make sure that every test
is isolated. Tests in Rust are designed to be able to be run in parallel. This
means that every test needs, ideally, a fresh, empty environment to run
against.
```

In general, there are three strategies that I have used, and I will outline them here. If you can
make use of one of these strategies, then it might be a worthwhile investment. In some cases,
however, it is not possible.

## Use Service as Dependency

If you are writing tests for a component which talks to some API, and the API is also written in
Rust, then you might be able to simply add a development dependency to the API and launch it for the
unit tests.

For example, if you have a project which consists of two crates: `api` and `client`, then in the
`client` crate you could add the `api` crate as a test dependency in the Cargo manifest:

```toml
[dev-dependencies]
api = { path = "../api" }
```

And then you could write your unit tests in such a way that you launch a fresh instance of the API
for every test. You may have to pick a random free port or use some feature to bypass the network
and inject requests directly.

```rust
#[test]
fn test_some_call() {
    let server = api::Server::launch();

    // make request
    assert_eq!(make_request(), Response {});
}
```

```admonish example
*TODO*
```

## Docker Compose

In many cases, you do not need to run a separate copy of your dependencies for every unit test. Many
services, such as databases, allow you to create a fresh, empty database for every unit test. In
that case, using docker compose is a good strategy. A docker-compose file can be written which
defines all the prerequisite services, which can be launched manually before running the tests.

```admonish example title="Example project using a docker-compose file"
*TODO*
```

## Testcontainers

[Testcontainers](https://testcontainers.com/) is a project that aims to make it simple to use Docker
containers in unit tests. They maintain the
[testcontainers](https://github.com/testcontainers/testcontainers-rs) crate, which is the Rust
implementation of this project.

This makes it easy to run a fresh copy of whichever service your unit tests need when you execute
them.

```admonish example
*TODO*
```

## Mock Service

If you can easily mock the service, that is a good approach as well.

For example, the [mockall](https://docs.rs/mockall/latest/mockall/) crate lets you easily mock
external services.

Some external systems might have a built-in ability to create an environment. For example, when
talking to a storage system, every test might get it's own bucket with a randomized name. When
talking to Postgres, every test might get it's own database.

Some systems do not have that built-in, in this case one can use something like the Testcontainers
crate, which is designed to launch a fresh container for every invocation of a test.

## Reading

```reading
style: article
title: Increase Test Fidelity By Avoiding Mocks
url: https://testing.googleblog.com/2024/02/increase-test-fidelity-by-avoiding-mocks.html)
author: Google Testing Blog
---
In this post from Google's Testing on the Toilet series, the topic of how to
interact with external services is discussed. The preference to use real
instances is mentioned.
```

```reading
style: article
title: Rust Mock Shootout!
url: https://asomers.github.io/mock_shootout/
author: Alan Somers
---
In this post, Alan discusses various mocking crates in Rust.
```

```reading
style: article
title: Rust Development with Testcontainers
url: https://blog.ediri.io/rust-development-with-testcontainers
author: Engin Diri
---
*In this blog post, Engin discussed how
[testcontainers](https://docs.rs/testcontainers/latest/testcontainers/) can be
used to make sure external dependencies are spawned in Docker containers for
each unit test.*
```
