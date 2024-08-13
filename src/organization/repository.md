# Repository

In software development, one of the longstanding questions is: should you use a monorepo,
or should you split components into separate repositories?

Unless you work in a large company with resources to build custom solutions, monorepos will
likely run into scaling issues. Keeping an entire company in sync on a single repository with
standard technology like `git` can run into issues.

At the same time, dealing with multiple repositories is also a headache. How do you easily
make a change in a library and test that it doesn't break any of the repositories that depend
on it?

## Advantages and disadvantages of monorepos


## Start out with a single repository

For your new Rust project, it probably makes sense to start out with 

- bubble graph with big bubbge containing crates

## Split out libraries only if they are stable

- git dependencies
- private registry


## Examples

- tokio project

## Reading


