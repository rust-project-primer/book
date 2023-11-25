# Spelling

As they say, cache invalidation and naming things are the hardest problems in computer science.
But what is even worse than an identifier that is named inappropriately, is one that is spelled
wrong. Thankfully, this is a problem that is rather easy to solve: using a spell checker.
The [`typos-cli`][typos-cli] crate provides just that: a spell checker for Rust projects.

## Usage

You can install it using `cargo`: 

```
cargo install typos-cli
```

Once installed, you can run it on your project like this:

```
typos
```

[typos-cli]: https://github.com/crate-ci/typos

