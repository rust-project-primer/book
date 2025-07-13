# Lints

_You have noticed that a lot of trivial mistakes exist in the project. You would
like to mechanically prevent these from passing code reviews. How can you
mechanically detect them?_

In programming, _linting_ refers to the process of performing static code
analysis on a software project to flag programming errors, bugs, stylistic
errors and suspicious constructs. The term originates from an old UNIX tool
named `lint`, which was used to check C programs for common mistakes.

The Rust community has an excellent linting tool named Clippy.

## Clippy

The linter that is typically used in Rust is [Clippy][clippy]. It is commonly
used in Rust projects to enforce good practises, recognize unsafe or slow code.
It is also [configurable][clippy-lints].

```admonish info
When enforcing coding style, it goes beyond mere aesthetic concerns. Coding style
linters can do a lot more:

- Detect patterns that have cleaner alternatives
- Detect code that is correct, but slow
- Disallow writing unsafe code
```

It usually comes preinstalled when installing Rust through Rustup, or it can be
added later by running

```
rustup component add clippy
```

What makes Clippy interesting is that it is quite configurable. Lints can be
enabled either individually, or in groups. Some default lint groups are enabled
by default, but the list[^clippy-lints] can be examined to pick out ones that
are relevant for the project.

### Example: Overriding Lints in Code

Instead of making sure during code reviews that no unsafe code is written, a
Clippy annotation can be added to the crates that should not have unsafe code in
them.

```rust
#![deny(clippy::unsafe_code)]
```

### Example: Overriding Lints in Cargo.toml

## Typos

_You notice that in your Rust project, often times spelling errors make it
through code reviews because they are not specifically looking for them. This
leads to a lot of small follow-up pull requests only to fix obvious spelling
errors. Sometimes, they are not detected before a new version is released,
leading to spelling issues in the public documentation. How could spelling
errors be detected automatically so they do not make it in?_

Using a spell checker for a software projects helps ensure that no silly
mistakes make it into the `main` branch, without taking resources from processes
such as code review that better focus on the semantics of the change.

```admonish
Good communication is very valuable. Nothing is worse in a complex project than
having a situation where certain parts are only understood by a select few
people due to a lack of documentation. Having good documentation helps keep
projects agile by allowing faster onboarding of new developers and by helping
existing developers understand unfamiliar parts of a project. See the
*Documentation* section of this book for more information.
```

There is one crate in the Rust ecosystem that is specifically designed to detect
spelling errors: `typos-cli`.

The [`typos-cli`][typos-cli] crate helps do just that by providing a spell
checker specifically for Rust projects. It is designed to be fast enough to run
on monorepos and have a low false positive rate so that it can be run for pull
requests.

It can be installed using Cargo:

```
cargo install typos-cli
```

Once installed, it can be run against a project:

```
typos
```

If it detects a spelling error, it outputs a nonzero exit code and a diagnostic
message explaining what the error was and how it could be remedied. This is a
good tool to run in a CI job to keep pull requests from containing spelling
errors.

```admonish example title="Running <code>typos-cli</code> in CI"
TODO: add CI example here
```

## Static Analysis Results Interchange Format (SARIF)

https://github.com/psastras/sarif-rs

https://sarifweb.azurewebsites.net/

## Reading

[Static Analysis](https://abseil.io/resources/swe-book/html/ch20.html) in
Software Engineering at Google

[Rust Lints you may not know](https://www.possiblerust.com/pattern/rust-lints-you-may-not-know)
by Andrew Lilley Brinker

[clippy]: https://github.com/rust-lang/rust-clippy
[clippy-lints]: https://rust-lang.github.io/rust-clippy/
[typos-cli]: https://github.com/crate-ci/typos
