# Crate Registry

The default way to release Rust crates (libraries and binaries) is to a Crate
registry.  The default crate registry for Rust is [crates.io][], with 150k
crates published, almost a million crate versions and (at the time of writing)
77 million crate downloads, it is the largest repository of Rust code.

Publishing your crates on there is free, and it allows others to use the
libraries you have written as simple as running `cargo add your-library`.
If you publish your code there, the documentation for it is also generated
automatically and published on [docs.rs][], Rust's public crate documentation
browser.

Any binary crates you have published there are easily installable by other
Rust users with `cargo install`. For example, if you want to install [ripgrep][],
a useful tool for searching through local git repositories, you can do so simply
by running:

    cargo install ripgrep

But in a commerical setting, you may have some internal crates that you want
to share, but not publically. Using a crate repository makes for a more pleasant
experience than having direct git dependencies, because Cargo supports semantic
versioning (but this does not work with git dependencies). [RFC 2141][rfc2141]
specifies how this works, and today there are some commercial private crate
registries that you can use, or you can even host your own registry.

## Rust Crate Registry

[Crates.io][crates.io]
 is the public Rust package index. It is free and used by the Rust
community to share libraries and tooling. It integrates with [docs.rs][] to
automatically build and host documentation for any crates that are published
to it.

To use it, all you need is a GitHub account. You can log in on their website and
generate an API token. With that, you can log in using Cargo:

    cargo login <api-token>

Once you are ready to publish your crate, you can do so using Cargo:

    cargo publish

If you accidentally published a version that you did not intend to publish, you
can yank it. Yanking does not delete it, to avoid situations like the [left-pad
disaster](https://www.theregister.com/2016/03/23/npm_left_pad_chaos/), where a
developer deleted a package from NPM that a lot of JavaScript libraries relied
upon, temporarily breaking the internet.

    cargo yank

In order to be able to publish your package, it must contain some required
metadata.  See [Publishing on
crates.io](https://doc.rust-lang.org/cargo/reference/publishing.html) for more
details on what is required and how you can manage your packages.

## Shipyard

[Shipyard](https://shipyard.rs/)
is a private cargo registry service. It replicates

## JFrog


## Kellnr

[Kellnr](https://kellnr.io/)

## Reading

[Chapter 14.2: Publishing to Crates.io](https://doc.rust-lang.org/book/ch14-02-publishing-to-crates-io.html) in The Rust Book

*In this section of the Rust book, it shows you how you can write a Rust crate and
publish it on Rust's crate index, crates.io.*

[Using the Shipyard private crate registry with
Docker](https://fasterthanli.me/series/building-a-rust-service-with-nix/part-7)
by Amos Wenger

*Amos explains how you can publish your crates to a private crate registry
hosted by Shipyard. He shows how you can configure Cargo to authenticate with
Shipyard, and how to push packages to it both locally and from CI.*

[ripgrep]: https://github.com/BurntSushi/ripgrep
[crates.io]: https://crates.io
[docs.rs]: https://docs.rs
[rfc2141]: https://rust-lang.github.io/rfcs/2141-alternative-registries.html
