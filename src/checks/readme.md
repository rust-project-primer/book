# Checks

The Rust compiler catches a lot through its type system and borrow checker, but
there are properties of a project that the compiler does not verify: formatting
consistency, semver compliance, dependency security, spelling, and more. The
Rust ecosystem has tooling to check each of these automatically, and this
chapter covers the most useful ones.

Not all of these checks will be relevant to every project. For each one, you
need to decide whether it runs in CI on every pull request, on a schedule, or
only locally. Some checks (formatting, linting) are fast and cheap enough to
gate every merge. Others (dependency auditing, feature powerset testing) are
more expensive and may be better suited to scheduled runs. The summary table
below gives recommendations for each tool.

Note that several of these checks go beyond Rust source code — they cover your
dependency graph, your `Cargo.toml` manifests, and your documentation.

## Summary

Which checks matter depends on the project: a published library needs semver and
minimum version checks that a binary never will, and `cargo-vet` is overkill for
a personal project but essential for security-sensitive work. The table below
summarizes each tool and suggests when to run it, ordered by lifecycle stage.

| Goal                  | Tool                     | Cost   | When     |
| --------------------- | ------------------------ | ------ | -------- |
| Formatting            | `rustfmt`                | Low    | Commit   |
| TOML Formatting       | `taplo`                  | Low    | Commit   |
| Spelling              | `typos`                  | Low    | Commit   |
| Linting               | `clippy`                 | Medium | Merge    |
| Unused Dependencies   | `cargo-machete`          | Low    | Periodic |
| Auditing Dependencies | `cargo-deny`             | Medium | Merge    |
| Auditing Dependencies | `cargo-vet`              | High   | Merge    |
| Outdated Dependencies | `cargo-upgrades`         | Low    | Periodic |
| Crate Features        | `cargo-hack`             | High   | Merge    |
| SemVer                | `cargo-semver-checks`    | Medium | Release  |
| Minimum Versions      | `cargo-minimal-versions` | Medium | Release  |
| MSRV                  | `cargo-msrv`             | Medium | Release  |

**Commit** checks are fast enough to run locally as pre-commit hooks or
format-on-save. **Merge** checks gate pull requests in CI. **Release** checks
only matter when publishing a new version. **Periodic** checks run on a schedule
(weekly, for example) to flag maintenance work without blocking day-to-day
development.

## Reading
