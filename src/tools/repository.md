# Readme

Open-source Rust projects have several places for documentation. Often times they have a README file
that contains some general overview of wwhat the crate does, as well as some crate-level
documentation in the `main.rs` or `lib.rs` file. In many cases the content for these two is similar,
or even the same.

For ease of maintenance, it can be beneficial to keep the two in sync.

## Cargo Readme

[`cargo-readme`](https://lib.rs/crates/cargo-readme) is a tool that allows you to generate a README
file from the crate-level documentation strings of your Rust crate.

You can install it using Cargo:

```bash
cargo install cargo-readme
```

## Cargo Rdme

[`cargo-rdme`](https://github.com/orium/cargo-rdme)
