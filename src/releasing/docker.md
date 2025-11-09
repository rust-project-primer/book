# Containerize

_To deploy your Rust services, you have some `Dockerfile`s in the repository. Sometimes, developers
like to build these locally for testing purposes. However, you have received complaints that these
are very slow, as there is no caching going on. This makes you wonder: what is a good way to build
Rust projects with Docker while making use of caching?_

Deploying code using Docker is quite popular, as there is a lot of great tooling around deploying,
monitoring and scaling containers. The downside is that usually Docker builds are hermetic, meaning
that they do not have access to stateful things such as a target folder containing a build cache.

There are some solutions to getting Docker to play nice with Rust.

## Docker Cache

https://gist.github.com/noelbundick/6922d26667616e2ba5c3aff59f0824cd

## `cargo-chef`

[`cargo-chef`][cargo-chef] is a Cargo subcommand which is designed to help with making use of
Docker's caching capabilities. It works by splitting your Dockerfile build process into two stages:
one where only the dependencies are fetched and built, and the second one where the project is
built. The advantage of doing this split is that the dependency layer can be cached and reused as it
rarely changes.

[cargo-chef]: https://github.com/LukeMathWalker/cargo-chef
