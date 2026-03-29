# Dependency Auditing

As a project's dependency graph grows, so does its exposure to security
vulnerabilities and licensing conflicts. Vulnerabilities in transitive
dependencies are particularly dangerous because they are easy to miss — your
project may not directly depend on the affected crate, yet still ship the buggy
code. Licensing is a separate but related concern: Cargo makes it trivially easy
to add dependencies, and each one brings its own license terms that may conflict
with your project's requirements.

The Rust ecosystem has strong tooling for automating these checks. The
[RustSec][rustsec] project maintains a database of security advisories against
Rust crates, and several tools build on it. This chapter covers three of them,
each operating at a different level of rigor.

## `cargo-audit`

[`cargo-audit`][cargo-audit] is part of the RustSec project. It checks your
crate's dependencies against the RustSec advisory database by scanning the
[Cargo lockfile][lockfile], covering both direct and transitive dependencies.

```bash
cargo install cargo-audit
cargo audit
```

If any dependency has a known vulnerability, `cargo-audit` prints a detailed
advisory with the affected versions and suggested remediation (usually upgrading
to a patched version). It exits with a nonzero status, making it straightforward
to use as a CI gate.

## `cargo-deny`

[`cargo-deny`][cargo-deny] goes further than `cargo-audit` by acting as a
general-purpose linter for your dependency graph. It checks four categories,
each configurable independently:

- **Advisories** — the same RustSec database check that `cargo-audit` provides.
- **Licenses** — enforces an allowlist or denylist of acceptable licenses across
  all dependencies.
- **Bans** — prevents specific crates from appearing in your dependency tree, or
  flags duplicate versions of the same crate.
- **Sources** — restricts which registries or Git repositories dependencies may
  come from.

Configuration lives in a `deny.toml` file. Running `cargo deny init` generates a
starter configuration with comments explaining each section.

```bash
cargo install cargo-deny
cargo deny init
cargo deny check
```

Each violation can be configured as an error or a warning, so you can
incrementally adopt stricter policies without blocking all CI immediately.

## `cargo-vet`

[`cargo-vet`][cargo-vet] takes a fundamentally different approach from the
advisory-based tools. Rather than checking against a database of known
vulnerabilities, it enforces that _every_ dependency has been explicitly audited
and certified to meet specific criteria (such as "safe-to-deploy" or
"safe-to-run"). Unaudited dependencies cause a build failure until someone
reviews them.

The key insight is that audits are shareable. Organizations can publish their
audit records, and other teams can import them. Both [Google][google-audits] and
[Mozilla][mozilla-audits] publish their Rust crate audits, so you can bootstrap
your audit set by importing theirs and only need to manually review crates they
haven't covered.

This is a higher-effort approach than `cargo-audit` or `cargo-deny`, but it
provides stronger guarantees: rather than reacting to known vulnerabilities, it
ensures that human eyes have reviewed every piece of third-party code before it
enters your project.

### Setting Up

To start using `cargo-vet`, initialize it in your project:

```bash
cargo install cargo-vet
cargo vet init
```

This creates a `supply-chain/` directory containing an `audits.toml` (where your
audit certifications live) and an `imports.lock` (for audits imported from other
organizations). Running `cargo vet` immediately after init will likely fail,
because your existing dependencies have not been audited yet. You have a few
options to get to a clean state:

- **Import audits** from organizations like Mozilla or Google using
  `cargo vet import`. This covers many popular crates without you having to
  review them yourself.
- **Certify dependencies** you have reviewed with `cargo vet certify`, which
  records your audit in `audits.toml`.
- **Exempt dependencies** you trust but have not reviewed with
  `cargo vet suggest`, which adds them to an exemptions list. This lets you
  adopt `cargo-vet` incrementally rather than auditing everything upfront.

Once `cargo vet` passes locally, commit the `supply-chain/` directory. From that
point on, `cargo vet` in CI will fail only when a new or updated dependency
lacks an audit, prompting someone to review it before it can be merged.

## CI Examples

`cargo-audit` and `cargo-deny` are well suited for running on every pull
request. `cargo-vet` is typically run the same way but requires more initial
setup to establish the audit baseline.

```yaml framed title=".github/workflows/deny.yml"
name: Audit
on: [pull_request]

jobs:
  deny:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: taiki-e/install-action@v2
        with:
          tool: cargo-deny
      - run: cargo deny check
```

Since `cargo-deny` includes advisory checking, it subsumes `cargo-audit`. If
you only need advisory checks without license or ban policies, you can use
`cargo-audit` directly instead:

```yaml
      - uses: taiki-e/install-action@v2
        with:
          tool: cargo-audit
      - run: cargo audit
```

```yaml framed title=".gitlab-ci.yml"
deny:
  image: rust:latest
  script:
    - cargo install cargo-deny
    - cargo deny check
```

```yaml framed title=".github/workflows/vet.yml"
name: Vet
on: [pull_request]

jobs:
  vet:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - uses: taiki-e/install-action@v2
        with:
          tool: cargo-vet
      - run: cargo vet
```

## Reading

```reading
style: article
title: Comparing Rust Supply Chain Safety Tools
url: https://blog.logrocket.com/comparing-rust-supply-chain-safety-tools/
author: Andre Bogus
---
Surveys five tools that address different layers of supply chain security:
`cargo-audit` (vulnerability scanning), `cargo-deny` (license and source
checking), `cargo-outdated` (version staleness), `cargo-geiger` (unsafe code
detection in dependencies), and `cargo-crev` (cryptographically-signed peer
reviews). The article emphasizes that these tools are complementary rather than
competing, and shows actual command output to illustrate what each tool reveals
in practice. Good starting point if you want to understand which tools to
combine.
```

```reading
style: book
title: "Item 25: Manage your dependency graph"
url: https://www.lurklurk.org/effective-rust/dep-graph.html
author: Effective Rust
---
Covers the practical side of managing Rust dependencies: how Cargo resolves
semver-incompatible versions (and the problems this causes with FFI), when to
use version ranges versus pinning, and when to commit `Cargo.lock` (applications
yes, libraries no). Introduces `cargo tree` for visualizing dependency graphs
with flags like `--duplicates` and `--invert`. Also discusses the supply chain
risk that build scripts and procedural macros can execute arbitrary code at
compile time, which motivates the auditing tools in this chapter.
```

```reading
style: book
title: Cargo Deny Book
url: https://embarkstudios.github.io/cargo-deny/
author: Cargo Deny Project
---
Full reference for `cargo-deny`. Goes beyond running the tool into detailed
configuration for each check category: advisory database sources and severity
thresholds, license allowlists with SPDX expression matching, banning specific
crates or flagging duplicate versions, and restricting which registries or Git
sources dependencies may come from. Also covers diagnostic output
interpretation and CI integration via GitHub Actions.
```

```reading
style: book
title: Securing the Software Supply Chain
url: https://media.defense.gov/2022/Sep/01/2003068942/-1/-1/0/ESF_SECURING_THE_SOFTWARE_SUPPLY_CHAIN_DEVELOPERS.PDF
author: US-American Department of Defense
archived: securing-the-software-supply-chain.pdf
---
A guidance document from the US Department of Defense aimed at software
developers, covering the threat model of supply chain attacks and recommended
mitigations: secure development practices, dependency management, build
integrity, and artifact verification. Not Rust-specific, but provides the
broader security framework that motivates tools like `cargo-vet` and
`cargo-deny`.
```

```reading
style: book
title: Cargo Vet Book
url: https://mozilla.github.io/cargo-vet/
author: Mozilla
---
Comprehensive guide to the `cargo-vet` workflow. Explains audit criteria
(safe-to-deploy, safe-to-run), how to perform and record audits, how to
conduct relative audits between similar versions to reduce review effort, how
to import audits from other organizations, and how trusted publishers work.
Also covers the algorithm `cargo-vet` uses to determine whether dependencies
meet your project's requirements.
```

```reading
style: article
title: Vetting the Cargo
url: https://lwn.net/Articles/897435/
author: Jonathan Corbet
---
LWN article explaining why Mozilla built `cargo-vet`: Firefox grew to depend on
nearly 400 third-party crates, and the ease of pulling in dependencies from
crates.io increased the attack surface significantly. Covers how the shared
audit model works (projects import audits from organizations they trust rather
than duplicating review effort) and its current limitations, including the lack
of reputation systems to verify audit authenticity.
```

[cargo-vet]: https://github.com/mozilla/cargo-vet
[cargo-deny]: https://github.com/EmbarkStudios/cargo-deny
[lockfile]: https://doc.rust-lang.org/cargo/guide/cargo-toml-vs-cargo-lock.html
[cargo-audit]: https://github.com/RustSec/rustsec/tree/main/cargo-audit
[rustsec]: https://rustsec.org/
[google-audits]:
  https://opensource.googleblog.com/2023/05/open-sourcing-our-rust-crate-audits.html
[mozilla-audits]: https://github.com/mozilla/supply-chain
