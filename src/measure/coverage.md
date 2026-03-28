# Test Coverage

Test coverage measures which parts of your code are executed during tests. A
coverage report highlights lines, branches, or functions that no test exercises,
pointing you toward gaps in your test suite. Coverage is not a guarantee of
correctness: a line can be covered without its edge cases being tested. However,
low coverage is a reliable signal that something is undertested.

Focusing on coverage early helps guide architecture toward code that is easy to
test in isolation. Library crates with well-defined APIs should aim for high
coverage, ideally approaching 100%. This is one of the reasons why splitting a
project into smaller, focused library crates is valuable: a pure library with no
I/O is straightforward to test thoroughly, while a binary crate that wires
everything together will inevitably have harder-to-reach code paths.

```admonish note title="Mutation Testing"
[Mutation testing](../testing/mutations.md) complements coverage by checking
whether your tests actually detect changes to the code. It makes small
modifications (flipping operators, replacing return values) and verifies that
at least one test fails for each mutation.
```

## `cargo-llvm-cov`

[`cargo-llvm-cov`][cargo-llvm-cov] is the recommended tool for measuring code
coverage in Rust. It uses LLVM's source-based instrumentation, which tracks
coverage at the region level (not just lines), giving more accurate results than
approaches based on debug info or binary instrumentation.

```bash
cargo install cargo-llvm-cov
cargo llvm-cov
```

To generate an HTML report you can browse locally:

```bash
cargo llvm-cov --html --open
```

### Output Formats

`cargo-llvm-cov` supports several output formats, which matters for CI
integration. Different services and platforms expect different formats:

```bash
# LCOV — used by Codecov, Coveralls, and VS Code Coverage Gutters
cargo llvm-cov --lcov --output-path lcov.info

# Cobertura XML — natively supported by GitLab CI for inline MR annotations
cargo llvm-cov --cobertura --output-path coverage.xml

# Codecov's custom format
cargo llvm-cov --codecov --output-path codecov.json
```

GitLab CI can read the cobertura format and show test coverage changes inline in
diffs, an example for this is in the [Examples](#ci-examples) section.

### Enforcing a Minimum

In CI, you can fail the build if coverage drops below a threshold:

```bash
cargo llvm-cov --fail-under-lines 80
```

If you have a project that has low test coverage, you can measure the coverage
you have, and add a CI job that ensures that the coverage does not decrease, and
adjust it any time the coverage increases. That way, you encourage new code to
come with tests.

There are other flags, for example `--fail-uncovered-lines` that let you set an
absolute amount of uncovered lines rather than a percentage.

### Excluding Code

Some code is not worth measuring: test helpers, generated code, or functions
that are impossible to test without mocking the operating system. You can
exclude individual functions using the `#[coverage(off)]` attribute. Since this
attribute is currently unstable, `cargo-llvm-cov` provides a cfg flag that lets
you write it conditionally:

```rust
#[cfg_attr(coverage_nightly, coverage(off))]
fn not_worth_covering() {
    // ...
}
```

You can also exclude entire files by pattern:

```bash
cargo llvm-cov --ignore-filename-regex "tests/|generated/"
```

### Combining Multiple Runs

If you need to collect coverage across different feature sets or test suites,
you can run tests separately and merge the results:

```bash
cargo llvm-cov clean --workspace
cargo llvm-cov --no-report --features a
cargo llvm-cov --no-report --features b
cargo llvm-cov report --lcov --output-path lcov.info
```

If you need to run the binaries manually, you can do it like this:

```bash
# setup environment
eval "$(cargo llvm-cov show-env --sh)"
cargo llvm-cov clean --workspace

# run regular cargo commands and run your binaries (will be instrumented
# due to the environment values set by show-env)
cargo build
./target/debug/your-binary --some --flags

# write coverage report (you can run this multiple times if you need it
# in different formats)
cargo llvm-cov report
```

This latter approach is sometimes necessary if some of your tests require
specific setup or `root` privileges.

## `cargo-tarpaulin`

[`cargo-tarpaulin`][tarpaulin] is an older coverage tool designed specifically
for Rust. It uses a different instrumentation approach based on ptrace, which
means it works without LLVM's instrumentation flags but only supports Linux
x86_64 (no macOS or Windows). It can generate reports in HTML, XML, JSON, and
LCOV formats.

```bash
cargo install cargo-tarpaulin
cargo tarpaulin --out html
```

Tarpaulin was the standard coverage tool before `cargo-llvm-cov` existed and is
still widely used, but for new projects `cargo-llvm-cov` is generally the better
choice due to broader platform support and more accurate source mapping.

## `grcov`

[`grcov`][grcov] is a coverage report generator developed by Mozilla. Rather
than running tests itself, it processes raw coverage data that you collect
separately. This makes it useful for aggregating coverage from multiple test
runs or environments into a single report.

A typical workflow involves setting environment variables to enable LLVM's
instrumentation, running tests, and then processing the resulting `.profraw`
files:

```bash
# Run tests with coverage instrumentation
CARGO_INCREMENTAL=0 \
RUSTFLAGS='-Cinstrument-coverage' \
LLVM_PROFILE_FILE='cargo-test-%p-%m.profraw' \
cargo test

# Generate an HTML report from the profiling data
grcov . \
  --binary-path ./target/debug/ \
  -s . \
  -t html \
  --branch \
  --ignore-not-existing \
  -o ./target/debug/coverage/
```

`CARGO_INCREMENTAL=0` disables incremental compilation (which can produce
inconsistent coverage data), `RUSTFLAGS='-Cinstrument-coverage'` enables LLVM's
instrumentation, and `LLVM_PROFILE_FILE` controls where the raw profiling data
is written. The `grcov` command then reads the `.profraw` files and cross-
references them with the debug info in the compiled binaries to produce a
report.

For most projects, `cargo-llvm-cov` is simpler because it handles all of this
internally. `grcov` is mainly useful when you need to aggregate coverage from
multiple separate test invocations or when you need more control over the
profiling pipeline.

## CI Examples

```admonish example title="Coverage in GitHub Actions"
This workflow generates test coverage and uploads it to
[Codecov](https://codecov.io/), a service that tracks coverage over time,
shows coverage diffs on pull requests, and can enforce minimum coverage
thresholds. Codecov is free for open-source projects and integrates with
GitHub, GitLab, and Bitbucket.

~~~yaml
name: Coverage
on: [pull_request]

jobs:
  coverage:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - uses: taiki-e/install-action@v2
        with:
          tool: cargo-llvm-cov
      - run: cargo llvm-cov --lcov --output-path lcov.info
      - uses: codecov/codecov-action@v4
        with:
          files: lcov.info
~~~
```

```admonish example title="Coverage in GitLab CI"
GitLab can display coverage annotations inline in merge requests if you upload
a Cobertura XML report. This allows GitLab to display changes in test coverage
inline in merge requests, which is useful feedback for developers and during
code review.

~~~yaml
coverage:
  image: rust:latest
  script:
    - cargo install cargo-llvm-cov
    - cargo llvm-cov --cobertura --output-path coverage.xml
  artifacts:
    reports:
      coverage_report:
        coverage_format: cobertura
        path: coverage.xml
~~~
```

## Reading

```reading
style: book
title: Instrumentation-based Code Coverage
url: https://doc.rust-lang.org/rustc/instrument-coverage.html
author: The rustc Book
---
Low-level reference for rustc's `-Cinstrument-coverage` flag. Explains how
LLVM's source-based coverage works, how profiling data is collected into
`.profraw` files, and how to use `llvm-profdata` and `llvm-cov` to generate
reports. This is what `cargo-llvm-cov` wraps — read this if you need to
understand or customize the underlying pipeline.
```

```reading
style: article
title: How to do code coverage in Rust
url: https://blog.rng0.io/how-to-do-code-coverage-in-rust/#source-based-coverage
author: Dotan J. Nahum
archived: rng0-code-coverage-rust.pdf
---
Practical guide to setting up a "coverage trinity": local HTML reports, IDE
integration using VS Code's Coverage Gutters extension pointed at an LCOV file,
and CI automation with GitHub Actions uploading to Codecov. Covers both the
`grcov` workflow and modern source-based coverage, with working CI configs.
```

[cargo-llvm-cov]: https://github.com/taiki-e/cargo-llvm-cov
[tarpaulin]: https://github.com/xd009642/tarpaulin
[grcov]: https://github.com/mozilla/grcov
