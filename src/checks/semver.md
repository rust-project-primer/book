# Semantic Versioning

Rust is built heavily on [Semantic Versioning][semver] to make it easy to compose
software and update dependencies without needing to worry about everything breaking
when dependencies are updated.

If you don't know what exactly Semantic Versioning is, here is a quick summary of it:

- Versions consist of three numbers, the *major*, *minor* and *patch* number. Usually, they are written with dots, such as `1.3.4`.
- Any time you create a breaking change, you must increment the major version.
- Any time you add a public API, you must increment the minor version.
- Any time you release a bugfix which does not change the public API or behaviour (besides fixing the bug), you must increment the patch number.

This also puts some responsibilities on you as a crate author: you have to ensure
that when you build and publish crates, you do not violate semantic versioning by
accidentally publishing versions with breaking changes but not marking them as such
by incrementing the major version.

Doing this manually is very hard. The answer here is to use good tooling.
`cargo-semver-checks` addresses this by mechanically checking your crate for
public API changes.

## Examples

## Usage

### GitHub

### GitLab

## Reading

- [Semantic Versioning][semver]
- Article: [Semver violations are common, better tooling is the answer](https://predr.ag/blog/semver-violations-are-common-better-tooling-is-the-answer/)
- Article: [Checking semver in the presence of doc(hidden) items](https://predr.ag/blog/checking-semver-for-doc-hidden-items/)
- GitHub: [obi1kenobi][cargo-semver-checks]

[semver]: https://semver.org/
[cargo-semver-checks]: https://github.com/obi1kenobi/cargo-semver-checks
