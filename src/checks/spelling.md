# Spelling

*You notice that in your Rust project, often times spelling errors make it
through code reviews because they are not specifically looking for them. This
leads to a lot of small follow-up pull requests only to fix obvious spelling
errors. Sometimes, they are not detected before a new version is released,
leading to spelling issues in the public documentation. How could spelling
errors be detected automatically so they do not make it in?*

## Solution

```admonish
Good communication is very valuable. Nothing is worse in a complex project than
having a situation where certain parts are only understood by a select few
people due to a lack of documentation. Having good documentation helps keep
projects agile by allowing faster onboarding of new developers and by helping
existing developers understand unfamiliar parts of a project. See the
*Documentation* section of this book for more information.
```

Avoiding spelling errors is a small part in making sure that documentation
(which includes comments) is usable. The [`typos-cli`][typos-cli] crate helps
do just that by providing a spell checker specifically for Rust projects.

It can be installed using Cargo:

```
cargo install typos-cli
```

Once installed, it can be run against a project:

```
typos
```

### Examples



[typos-cli]: https://github.com/crate-ci/typos

