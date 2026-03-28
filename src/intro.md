# Introduction

Once you are comfortable with the Rust language, the next set of questions is
about the ecosystem and the practices around it. How do you structure a project
that will grow over time? Which libraries do you reach for when you need
logging, serialization, or error handling? How do you set up CI so that
formatting, linting, testing, and auditing happen automatically? How do you
release your work — to crates.io, as a container image, as a system package?

This book is organized around those questions. Each chapter covers a different
aspect of running a Rust project:

- [**Development Environment**](development-environment/index.md) covers editor
  setup and toolchain configuration.
- [**Build System**](build-system/readme.md) covers Cargo, Nix, Bazel, and other
  build tools, and how Rust code can fit into projects written in other
  languages.
- [**Organization**](organization/readme.md) explains how to structure a
  codebase as it grows: when to split into multiple crates, how to use
  workspaces, and how to lay out a repository.
- [**Ecosystem**](ecosystem.md) surveys popular Rust libraries for common
  problems: logging, errors, serialization, concurrency, and more. It explains
  the tradeoffs between competing options so you can pick the right one.
- [**Interop**](interop/readme.md) covers calling C, C++, Python, and other
  languages from Rust, including the FFI frameworks available and common hazards
  to watch for.
- [**Checks**](checks/readme.md) explains how to automatically verify properties
  of your code: formatting, linting, dependency auditing, semver correctness,
  and more. These are the tools that catch problems before they reach code
  review.
- [**Testing**](testing/readme.md) covers strategies for verifying correctness,
  from unit tests and property testing to fuzzing, mutation testing, and dynamic
  analysis with tools like Miri.
- [**Measure**](measure/readme.md) explains how to collect metrics about your
  codebase: test coverage, benchmarking, and memory profiling.
- [**Building**](building/readme.md) covers what happens during `cargo build`:
  reducing binary size, tuning compiler output for performance, cross-compiling
  for other platforms, and caching builds.
- [**Documentation**](documentation/readme.md) covers how to write and publish
  documentation, from API-level rustdoc to standalone books with mdBook and
  architecture decision records.
- [**Releasing**](releasing/readme.md) explains the process of shipping your
  work to users: versioning, changelogs, publishing to crate registries,
  building container images, and creating system packages.
- [**Continuous Integration**](ci/readme.md) ties the preceding chapters
  together by showing how to run checks, tests, and builds automatically on
  every commit, with examples for GitHub Actions and GitLab CI.
- [**Tools**](tools/readme.md) covers general-purpose development tools that are
  useful across workflows: code search, task runners, macro expansion, and
  debuggers.

Not every chapter will be relevant to every project, and you do not need to
adopt everything at once. Chapters are self-contained, so you can read the book
cover to cover or use it as a reference: jump to whichever chapter addresses the
problem you are facing now, and come back for others as the need arises. The
[Resources](resources.md) chapter lists books and courses for learning the Rust
language itself.
