# External Services

Most non-trivial applications depend on external services: databases, message
queues, caches, APIs. Testing code that interacts with these services can be
challenging. If your test suite requires a running PostgreSQL instance or a
connection to a cloud API, developers can't easily run it locally, which slows
down their iteration loop and pushes bug discovery to CI.

Whenever possible, try to make it so that the full test suite can run locally
without any manual setup. This chapter outlines several strategies for achieving
that, roughly ordered from lightest to heaviest.

```admonish note
When interfacing with external systems in tests, you need to make sure that
every test is isolated. Rust's test harness runs tests in parallel by default,
so every test needs its own clean environment. For databases, this typically
means creating a fresh database or schema per test. For services, it means
launching a separate instance or using non-overlapping namespaces.
```

## Mocking

The simplest approach is to replace the external service with a mock that
implements the same interface. This works well when the behavior you need from
the service is straightforward and you are primarily testing your own logic, not
the interaction with the service.

In Rust, this is typically done by defining a trait for the service interaction
and providing both a real implementation and a mock:

```rust
trait UserStore {
    fn get_user(&self, id: u64) -> Result<User, Error>;
    fn create_user(&self, name: &str) -> Result<User, Error>;
}

struct PostgresUserStore { /* ... */ }
impl UserStore for PostgresUserStore { /* ... */ }

struct MockUserStore {
    users: std::sync::Mutex<Vec<User>>,
}
impl UserStore for MockUserStore { /* ... */ }
```

The [mockall](https://docs.rs/mockall/latest/mockall/) crate can generate mock
implementations automatically using a procedural macro, which saves you from
writing boilerplate. For HTTP services specifically,
[wiremock](https://docs.rs/wiremock/latest/wiremock/) lets you set up a local
HTTP server that returns canned responses.

The downside of mocking is that your tests only verify that your code interacts
with the mock correctly, not that it works with the real service. Schema
changes, subtle behavioral differences, and integration bugs will slip through.
For this reason, mocks are best used for unit tests of business logic, not as a
replacement for integration testing against real services.

## Service as Dependency

If the service you depend on is also written in Rust and lives in the same
workspace (or is available as a crate), you can add it as a dev-dependency and
launch it directly in your tests. This gives you a real instance without any
Docker or external infrastructure.

For example, if your project has an `api` crate and a `client` crate, the client
can depend on the API in its test configuration:

```toml
[dev-dependencies]
api = { path = "../api" }
```

Then each test can spin up a fresh server instance:

```rust
#[tokio::test]
async fn test_create_user() {
    // Launch the API on a random available port.
    let server = api::Server::start("127.0.0.1:0").await;
    let addr = server.local_addr();

    let client = Client::new(&format!("http://{addr}"));
    let user = client.create_user("alice").await.unwrap();
    assert_eq!(user.name, "alice");
}
```

This approach works well for microservice architectures where the services are
all Rust crates in a single workspace. It doesn't require Docker and tests start
fast. The limitation is that it only works when you control the dependency and
it can be embedded as a library.

## Docker Compose

When your tests depend on services that can't be embedded as a Rust dependency
(PostgreSQL, Redis, Kafka, etc.), Docker Compose is a straightforward way to
provide them. You write a `docker-compose.yml` that defines the services, and
developers run `docker compose up -d` before running the test suite.

This also works with [Podman][podman], which is a daemonless container engine
that can serve as a drop-in replacement for Docker. Podman supports both
`docker-compose` (through its Docker-compatible socket) and its own
`podman-compose` tool. If your team prefers rootless containers or wants to
avoid the Docker daemon, Podman is worth considering.

```yaml
services:
  postgres:
    image: postgres:17
    environment:
      POSTGRES_PASSWORD: test
      POSTGRES_DB: test
    ports:
      - "5432:5432"
  redis:
    image: redis:7
    ports:
      - "6379:6379"
```

Your tests then connect to these services on localhost. To ensure isolation,
each test should create its own database or use a unique key prefix:

```rust
async fn create_test_db(pool: &PgPool) -> String {
    let db_name = format!("test_{}", uuid::Uuid::new_v4().simple());
    sqlx::query(&format!("CREATE DATABASE \"{db_name}\""))
        .execute(pool)
        .await
        .unwrap();
    db_name
}
```

The advantage of Docker Compose is its simplicity: the file is declarative,
developers understand it, and it works with any service that has a Docker image.
The downside is that it's a manual step (developers need to remember to start
the containers), and services are shared across all tests rather than being
isolated per-test.

## Testcontainers

[Testcontainers][testcontainers] combines the real-service advantage of Docker
Compose with per-test isolation. Instead of requiring developers to manually
start containers, the testcontainers library launches them programmatically from
within your tests. Each test (or test group) gets a fresh container that is
automatically cleaned up when the test finishes.

The Rust implementation is the
[testcontainers](https://docs.rs/testcontainers/latest/testcontainers/) crate.
It provides built-in support for common services through companion crates like
`testcontainers-modules`:

```rust
use testcontainers::runners::AsyncRunner;
use testcontainers_modules::postgres::Postgres;

#[tokio::test]
async fn test_with_postgres() {
    let container = Postgres::default().start().await.unwrap();
    let port = container.get_host_port_ipv4(5432).await.unwrap();

    let connection_string =
        format!("postgres://postgres:postgres@127.0.0.1:{port}/postgres");

    // Use the connection string to set up your database pool
    // and run your tests against a real PostgreSQL instance.
}
```

Every test gets its own PostgreSQL instance running in a dedicated container.
There is no shared state between tests, and no manual setup step for developers.
The tradeoff is startup time: launching a container takes a few hundred
milliseconds to a few seconds, which adds up if you have many tests. For this
reason, testcontainers is best suited for integration tests rather than fast
unit tests.

## Choosing a Strategy

These strategies are not mutually exclusive. A common pattern is to use mocks
for unit tests that exercise business logic, and testcontainers or Docker
Compose for integration tests that verify the actual service interaction. The
service-as-dependency approach is ideal when you control both sides and they're
in the same workspace.

The general principle is: use the lightest approach that gives you confidence in
the behavior you're testing. Mocks are fast but low-fidelity. Real services are
high-fidelity but slower. Pick the right tool for each layer of your test suite.

## Reading

```reading
style: article
title: Increase Test Fidelity By Avoiding Mocks
url: https://testing.googleblog.com/2024/02/increase-test-fidelity-by-avoiding-mocks.html
author: Google Testing Blog
---
In this post from Google's Testing on the Toilet series, the preference to use
real service instances over mocks is discussed, and the tradeoffs between test
fidelity and test speed are outlined.
```

```reading
style: article
title: Rust Mock Shootout!
url: https://asomers.github.io/mock_shootout/
author: Alan Somers
---
A comparison of various mocking crates in Rust, covering their strengths,
limitations, and the kinds of mocking patterns each one supports.
```

```reading
style: article
title: Rust Development with Testcontainers
url: https://blog.ediri.io/rust-development-with-testcontainers
author: Engin Diri
---
Engin discusses how the testcontainers crate can be used to spawn external
dependencies in Docker containers for each unit test, with practical examples
using PostgreSQL.
```

[testcontainers]: https://testcontainers.com/
[podman]: https://podman.io/
