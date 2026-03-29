# Test Runners

Cargo ships with a built-in test runner invoked through `cargo test`. It
discovers functions annotated with `#[test]` (see [Unit Tests](unit-tests.md)),
runs integration tests from the `tests/` directory, and executes code examples
in documentation (see [Code Documentation](../documentation/rustdoc.md)). For
workspaces, `cargo test --workspace` runs all tests across all crates.

## `cargo test`

The built-in runner covers the basics well. A few flags are worth knowing:

```bash
# enable all features for tests
cargo test --all-features

# don't capture stdout (useful to see standard output of tests)
cargo test -- --nocapture

# run tests sequentially
cargo test -- --test-threads=1

# skip all tests with names matching filter (use it to skip slow tests)
cargo test -- --skip 'slow_'
```

Note that some flags are directly for `cargo-test`, like `--all-features`,
whereas some flags are passed to the test binaries themselves (such as
`--nocapture`), which is why there is that stray double hyphen.

## `cargo-nextest`

[`cargo-nextest`][nextest] is a drop-in replacement for `cargo test` that uses a
process-per-test execution model: each test runs in its own process rather than
sharing a process with other tests from the same binary. This provides better
isolation (a panic or segfault in one test can't take down others) and enables
nextest to be [up to 3x faster][benchmarks] by scheduling test processes more
efficiently.

The actual speedup depends on your workload. For projects where tests are
bottlenecked by external services, the difference may be modest. For large
workspaces with many fast unit tests, the improvement can be significant.

### Configuration

Nextest is configured through `.config/nextest.toml` at the workspace root.
Configuration is organized into profiles — named sets of options that you can
switch between. Every setting falls back to the `default` profile if not
specified.

A typical configuration covers several areas:

```toml framed title=".config/nextest.toml"
[profile.default]
# Stop running tests after the first failure.
fail-fast = true
# Retry failed tests up to 2 times (useful for flaky tests).
retries = 2
# Mark tests as slow if they take longer than 60 seconds.
slow-timeout = { period = "60s" }

[profile.ci]
# In CI, run all tests even if some fail.
fail-fast = false
# Produce JUnit XML for CI test reporting.
[profile.ci.junit]
path = "results.xml"
```

The `retries` option is particularly useful for dealing with flaky tests:
nextest re-runs failed tests and only reports them as failures if they fail on
every attempt. The `slow-timeout` option prints a warning when a test exceeds
the threshold, and can optionally terminate it if it exceeds a multiple of the
period.

To run with a specific profile:

```bash
cargo nextest run --profile ci
```

### Filtering

Nextest has an expression language for selecting which tests to run, going
beyond `cargo test`'s name-based filtering. You can filter by test name, binary
name, package, or platform:

```bash
# Run only tests in the "core" package
cargo nextest run -E 'package(core)'

# Run tests whose name contains "parse" in any package
cargo nextest run -E 'test(parse)'

# Combine filters
cargo nextest run -E 'package(core) & test(parse)'
```

### JUnit XML Output

Both GitHub Actions and GitLab CI can parse JUnit XML to display test results
directly in pull request or merge request UIs. When you configure a `junit`
section in a nextest profile (as shown above), nextest writes the report to the
specified path after each run.

### Test Partitioning

For large test suites, nextest can split tests across multiple CI jobs. Each job
runs a different slice:

```bash
# In a CI matrix with 3 jobs:
cargo nextest run --partition count:1/3  # job 1
cargo nextest run --partition count:2/3  # job 2
cargo nextest run --partition count:3/3  # job 3
```

This is useful when your test suite is slow enough that parallelizing across
machines provides a meaningful speedup.

## Serial Tests

By default, `cargo test` runs tests in parallel within each test binary. This is
usually what you want, but some tests cannot run concurrently — for example,
tests that share a database, bind to a fixed port, or modify global state.

To force all tests to run sequentially, limit the thread count:

```bash
cargo test -- --test-threads=1
```

If only some tests need serialization, the [`serial_test`][serial-test] crate
lets you mark individual tests with `#[serial]` while allowing the rest to run
in parallel:

```rust
use serial_test::serial;

#[test]
#[serial]
fn test_that_uses_shared_database() {
    // this test will never run concurrently with other #[serial] tests
}
```

## CI Examples

This workflow installs nextest, runs all tests with the `ci` profile, and
uploads the JUnit XML report. The `dorny/test-reporter` action parses the
report and displays individual test results as check annotations on the pull
request, so you can see which tests failed without opening the CI logs.

```yaml framed title=".github/workflows/test.yml"
name: Test
on: [pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - uses: taiki-e/install-action@v2
        with:
          tool: cargo-nextest
      - run: cargo nextest run --profile ci
      - uses: dorny/test-reporter@v1
        if: always()
        with:
          name: Tests
          path: results.xml
          reporter: java-junit
```

GitLab natively understands JUnit XML reports. When you declare the report as
an artifact, GitLab displays test results in the merge request's test tab,
showing which tests were added, removed, or started failing.

```yaml framed title=".gitlab-ci.yml"
test:
  image: rust:latest
  script:
    - cargo install cargo-nextest
    - cargo nextest run --profile ci
  artifacts:
    when: always
    reports:
      junit: results.xml
```

[serial-test]: https://docs.rs/serial_test/latest/serial_test/

## Reading

```reading
style: article
url: https://sunshowers.io/posts/nextest-and-tokio/
title: How (and why) nextest uses Tokio
author: Siddharth Agarwal
archived: sunshowers-nextest-and-tokio.pdf
---
Explains why nextest uses Tokio internally despite not doing any networking.
The async model turns out to map well to scheduling and managing test
processes: waiting for tests to finish, handling timeouts, and reacting to
signals are all naturally expressed as futures. A good look at the internals
of how nextest achieves its speed.
```

```reading
style: book
url: https://nexte.st/index.html
title: cargo-nextest book
author: cargo-nextest
---
Full reference for nextest: installation, configuration, filtering which
tests to run, retry policies, JUnit XML output for CI, and partitioning
tests across multiple CI jobs for parallelism.
```

[nextest]: https://nexte.st/index.html
[benchmarks]: https://nexte.st/book/benchmarks.html
