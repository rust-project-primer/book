# Dependency Auditing

*In a recent software audit that your company performed, it was discovered that
your Rust project uses several dependencies with known security issues that
need to be replaced with safer alternatives. It was also discovered that your
project has some dependencies which are unlicensed, which means that your
company is not allowed to use it. As this security audit was very expensive,
you now wonder if there is a way to automate some of these checks to catch
them sooner.*

Supply chain attacks are currently discussed a lot. As the complexity of the
average software project grows, so does it's dependency graph. At the same
time, bugs are being found, sometimes in high-profile software, leading to
attacks. The worst situation is when bugs are found in popular libraries, but
mitigations are not applied due to companies not recognizing they (indirectly)
depend on the buggy versions.

```admonish
Talking about software supply chain attacks in the context of Rust cannot be
done without mentioning the [RustSec][rustsec] project, which collects security
advisories against Rust crates.
```

Another common issue has to do with licensing: Cargo (and other good tooling)
makes it very easy to add dependencies, but sometimes these come with incompatible
licenses.

Fortunately, the Rust ecosystem has come up with some exceptional tooling that
helps address this with very little friction. These tools can do a lot, more
so than I can cover in this chapter, but I will attempt to give an overview
of what each does.

## `cargo-audit`

A part of the RustSec project, [`cargo-audit`][cargo-audit] is a tool that will
validate your crate's dependencies against the RustSec advisory database and
let you know if your project has any dependencies, direct or transitive, by
looking at the [Cargo Lockfile][lockfile].

To use `cargo-audit`, you can install it using Cargo and run it:

```bash
cargo install cargo-audit
cargo audit
```

If your crate contains any dependencies which it deems are insecure, it
will produce a useful warning about it.

```admonish example title="Using <code>cargo-audit</code> in CI"
*TODO*
```

## `cargo-deny`

[`cargo-deny`][cargo-deny] is very similar to `cargo-audit`, however it goes
several steps further. Instead of only checking for security advisories, it acts
as a linter of your dependency graph. It allows you to put constraints on the
licenses of dependencies.

You can use it by installing `cargo-deny`:

```
cargo install cargo-deny
```

Initializing a new configuration:

```
cargo deny init
```

And finally checking if the current project satisfies the constraints set in
the `deny.toml` file:

```
cargo deny check
```

It will produce a report containing the violations it has found. It has the ability
to treat violations as either errors or warnings.

```admonish example title="Using <code>cargo-deny</code> in CI"
*TODO*
```

## `cargo-vet`

[`cargo-vet`][cargo-vet] is another very interesting tool from the Rust ecosystem.
The way it works is quite different from `cargo-audit` and `cargo-edit`. Instead of
relying on advisories, it enforces that *every* dependency is audited so that certain
properties can be asserted. These audits can be published, and audits from other
organizations can be imported.

```admonish
Google announced that it is [Open sourcing Rust crate audits](https://opensource.googleblog.com/2023/05/open-sourcing-our-rust-crate-audits.html), similar as [Mozilla is already doing](https://github.com/mozilla/supply-chain).
```

It is a good tool to add to your CI pipeline.

```admonish example title="Using <code>cargo-vet</code> in your CI pipeline"
*TODO*
```

## Reading

~~~reading
style: article
title: Comparing Rust Supply Chain Safety Tools
url: https://blog.logrocket.com/comparing-rust-supply-chain-safety-tools/
author: Andre Bogus
---
Blog post summarizing several Rust supply chain safety tools, including the
ones discussed in this chapter.
~~~

~~~reading
style: book
title: "Item 25: Manage your dependency graph"
url: https://www.lurklurk.org/effective-rust/dep-graph.html)
author: Effective Rust
---
Chapter about how to manage your crate's dependency graph. Mentions some of the
tools from this chapter.
~~~

~~~reading
style: book
title: Cargo Deny Book
url: https://embarkstudios.github.io/cargo-deny/
author: Cargo Deny Project
---
Cargo Deny documentation, explains what it is and how it works. Has a very good
overview of all of the capabilities it has and how to configure them.
~~~

~~~reading
style: book
title: Securing the Software Supply Chain
url: https://media.defense.gov/2022/Sep/01/2003068942/-1/-1/0/ESF_SECURING_THE_SOFTWARE_SUPPLY_CHAIN_DEVELOPERS.PDF
author: US-American Department of Defense
archived: securing-the-software-supply-chain.pdf
---
Document released by the US-American Department of Defense, outlining how
software supply chains should be secured.
~~~

~~~reading
style: book
title: Cargo Vet Book
url: https://mozilla.github.io/cargo-vet/
author: Mozilla
---
Official documentation book of `cargo-vet`. Explains in detail what it does and
how to set it up for your project.
~~~

~~~reading
style: article
title: Vetting the Cargo
url: https://lwn.net/Articles/897435/
author: Jonathan Corbet
---
Article explaining what `cargo-vet` does and how it works.
~~~

[cargo-vet]: https://github.com/mozilla/supply-chain
[cargo-deny]: https://github.com/EmbarkStudios/cargo-deny
[lockfile]: https://doc.rust-lang.org/cargo/guide/cargo-toml-vs-cargo-lock.html
[cargo-audit]: https://github.com/RustSec/rustsec/tree/main/cargo-audit
[rustsec]: https://rustsec.org/

