# Test Coverage

Test coverage measures which parts of your code are executed during tests. It's
an important metric to ensure that your test suite adequately exercises all
paths in your codebase.

Library crates should generally aim for high test coverage, ideally approaching
100%. Binary crates may be more challenging to fully test, particularly if they
use external libraries with limited testability. This reinforces the value of
splitting projects into smaller crates: you can maintain and enforce good test
coverage for library crates that are easily testable, while accepting more
limited coverage for certain binary crates.

## Cargo LLVM-Cov

[`cargo-llvm-cov`][cargo-llvm-cov] is the officially recommended tool for
measuring code coverage in Rust. It uses LLVM's source-based code coverage to
generate accurate reports.

You can install it with:

```
cargo install cargo-llvm-cov
```

Basic usage:

```
cargo llvm-cov
```

For generating a HTML report:

```
cargo llvm-cov --html
```

Cargo LLVM-Cov supports various output formats including HTML, JSON, and text,
making it suitable for both human inspection and CI integration.

## Cargo Tarpaulin

[`cargo-tarpaulin`][tarpaulin] is a code coverage tool specifically designed for
Rust. It works by instrumenting your tests and tracking which lines of code are
executed.

Tarpaulin is particularly useful on Linux systems (it does not support Windows
and has limited macOS support). Install it with:

```
cargo install cargo-tarpaulin
```

Basic usage:

```
cargo tarpaulin
```

Tarpaulin can generate reports in multiple formats including HTML, XML, JSON,
and LCOV, making it easy to integrate with tools like Codecov or Coveralls.

## Grcov

[grcov][] is a coverage reporting tool developed by Mozilla that processes
coverage data from various sources. It's particularly useful for complex
projects that need to aggregate coverage data from multiple runs or
environments.

Grcov can process coverage information collected from LLVM's instrumentation or
gcov, making it versatile across different environments. Install it with:

```
cargo install grcov
```

A typical workflow with grcov involves collecting coverage data first, then
processing it:

```
CARGO_INCREMENTAL=0 RUSTFLAGS='-Cinstrument-coverage' LLVM_PROFILE_FILE='cargo-test-%p-%m.profraw' cargo test
grcov . --binary-path ./target/debug/ -s . -t html --branch --ignore-not-existing -o ./target/debug/coverage/
```

Grcov is especially valuable for CI/CD pipelines where coverage information
needs to be collected from different test runs and combined into a single
report.

## Choosing a Coverage Tool

When selecting a coverage tool for your Rust project, consider:

- **Platform compatibility**: cargo-tarpaulin works best on Linux, while
  cargo-llvm-cov and grcov have broader platform support
- **Integration needs**: Consider which CI systems and coverage services you
  need to work with
- **Project complexity**: For large projects with multiple test suites, grcov
  may offer better aggregation capabilities
- **Accuracy requirements**: LLVM-based tools generally provide more accurate
  source-mapping than other approaches

## Reading

```reading
style: book
title: Instrumentation-based Code Coverage
url: https://doc.rust-lang.org/rustc/instrument-coverage.html
author: The rustc Book
---
This chapter in the rustc book explains low-level features of rustc that
enable adding instrumentation to binaries for measuring execution coverage, and
how to use the raw output to generate coverage reports.
```

```reading
style: article
title: How to do code coverage in Rust
url: https://blog.rng0.io/how-to-do-code-coverage-in-rust/#source-based-coverage
author: Dotan J. Nahum
archived: rng0-code-coverage-rust.pdf
---
Dotan explains how to measure test coverage in Rust using both Tarpaulin and
grcov. He shows how to set it up for a project, with working GitHub Actions
workflows.
```

[cargo-llvm-cov]: https://github.com/taiki-e/cargo-llvm-cov
[tarpaulin]: https://github.com/xd009642/tarpaulin
[grcov]: https://github.com/mozilla/grcov
