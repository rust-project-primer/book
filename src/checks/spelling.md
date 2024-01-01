# Spelling

*You notice that in your Rust project, often times spelling errors make it
through code reviews because they are not specifically looking for them. This
leads to a lot of small follow-up pull requests only to fix obvious spelling
errors. Sometimes, they are not detected before a new version is released,
leading to spelling issues in the public documentation. How could spelling
errors be detected automatically so they do not make it in?*

Using a spell checker for a software projects helps ensure that no silly
mistakes make it into the `main` branch, without taking resources from
processes such as code review that better focus on the semantics of the change.

```admonish
Good communication is very valuable. Nothing is worse in a complex project than
having a situation where certain parts are only understood by a select few
people due to a lack of documentation. Having good documentation helps keep
projects agile by allowing faster onboarding of new developers and by helping
existing developers understand unfamiliar parts of a project. See the
*Documentation* section of this book for more information.
```

There is one crate in the Rust ecosystem that is specifically designed
to detect spelling errors: `typos-cli`.

## `typos-cli`

The `typos-cli`[^typos-cli] crate helps do just that by providing a spell
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
message explaining what the error was and how it could be remedied.
This is a good tool to run in a CI job to keep pull requests from containing
spelling errors.

```admonish example title="Running <code>typos-cli</code> in CI"
TODO: add CI example here
```

[^typos-cli]: GitHub repository: <https://github.com/crate-ci/typos>

