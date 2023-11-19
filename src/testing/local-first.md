# Local-First

Whenever possible, try to make it such that you can run all tests locally, and
that you can do so relatively easily. 

Typically:

- Unit tests, which have a `#[test]` annotation and live in the crate source code, do not
  talk to external systems.
- Integration tests, which typically live in the `tests/` folder in a crate, may talk to
  external systems.

When interfacing with external systems, you need to make sure that every test is
isolated. Tests in Rust are designed to be able to be run in parallel. This means that
every test needs, ideally, a fresh, empty environment to run against.

Some external systems might have a built-in ability to create an environment. For example,
when talking to a storage system, every test might get it's own bucket with a randomized
name. When talking to Postgres, every test might get it's own database.

Some systems do not have that built-in, in this case one can use something like
the Testcontainers crate, which is designed to launch a fresh container for
every invocation of a test.

## Reading

- Rustdoc: [testcontainers](https://docs.rs/testcontainers/latest/testcontainers/)
- [Rust Development with Testcontainers](https://blog.ediri.io/rust-development-with-testcontainers)
