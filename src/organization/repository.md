# Repository

In software development, one of the longstanding questions is: should you use a monorepo, or should
you split components into separate repositories?

Unless you work in a large company with resources to build custom solutions, monorepos will likely
run into scaling issues. Keeping an entire company in sync on a single repository with standard
technology like `git` can run into issues.

At the same time, dealing with multiple repositories is also a headache. How do you easily make a
change in a library and test that it doesn't break any of the repositories that depend on it?

## Advantages and disadvantages of monorepos

Pro:

- easy to test changes to libraries upstream
- easy to refactor code
- no need to do proper versioning

Cons:

- all consumers of a library have to be refactored at the same time if interface changes, or
  backwards compatibility needs to be ensured (slows down development)
- complexity of rebasing as repository grows

## Start out with a single repository

For your new Rust project, it probably makes sense to start out with a single repository, set up a
single crate (or a Cargo Workspace) and start from there. Only when code is stabilized, you can
start to factor out atomic pieces into their own crates. When functionality is useful enough, it can
be put into it's own repository, and versioned properly.

- bubble graph with big bubbge containing crates

## Split out libraries only if they are stable

- git dependencies
- private registry (see [Releasing Crates](../releasing/crates.md)).

## Examples

- tokio project

## Reading
