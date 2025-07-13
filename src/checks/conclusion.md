# Conclusion

The Rust ecosystem has a lot of great tooling that can be used to verify
properties of software projects. Every check has a certain cost associated with
it: if it runs in the CI system, it means the CI system needs more resources. If
the checks are to be run locally, it takes resources to ensure everyone is using
the right checks and in the right way.

In this section, I will summarize all of the available checks, what I consider
their value and costs to be and give a recommendation of whether and where to
run them.

| Goal                  | Tool                     | Value           | Cost   | Use            |
| --------------------- | ------------------------ | --------------- | ------ | -------------- |
| Formatting            | `rustfmt`                | High            | Low    | Yes            |
| Linting               | `clippy`                 | High            | Low    | Yes            |
| Spelling              | `typos`                  | Medium          | Low    | Yes            |
| SemVer                | `cargo-semver-checks`    | High[^lib]      | Medium | Yes            |
| Minimum Versions      | `cargo-minimal-versions` | High[^lib]      | Medium | Yes            |
| Unused Dependencies   | `cargo-machete`          | High            | Low    | Yes            |
| Auditing Dependencies | `cargo-deny`             | High            | Medium | Yes            |
| Auditing Dependencies | `cargo-vet`              | High            | High   | Maybe          |
| Outdated Dependencies | `cargo-upgrades`         | Medium          | Low    | Yes            |
| Crate Features        | `cargo-hack`             | High[^features] | Medium | Yes[^features] |

[^features]: If your crate makes use of Cargo features.

[^lib]: If you are working on a library which you intend for others to consume.
