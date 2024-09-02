# Nix

While originally a package management and declarative system configuration
tool, through it's focus on being fully reproducible, Nix has become a popular
tool for reproducible builds and deployments.

Nix is quite versatile. It can be used to configure your system, setup a
hygienic development shell containing only the dependencies you explicitly
requested, build Docker images with the minimal set of runtime dependencies.

## Why Nix?

## How does Nix work?

## Examples

### Building C/C++ dependencies

### Building TypeScript dependencies

### Building WebAssembly component

## Reading

[Rust](https://nixos.wiki/wiki/Rust) in the NixOS Wiki

[ipetkov/crane](https://github.com/ipetkov/crane) on GitHub

[Building a Rust service with Nix](https://fasterthanli.me/series/building-a-rust-service-with-nix) by Amos Wenger

*Amos shows how to build a Rust service in this article.*

[Introducing Crane: Composable and Cacheable Builds with Cargo and Nix](https://ipetkov.dev/blog/introducing-crane/) by Ivan Petkov

*Ivan introduces Crane in this article, a Nix library for building Cargo
projects. He explains how it works and how to use it to build Rust projects.*

[Building Nix Flakes from Rust Workspaces](https://www.tweag.io/blog/2022-09-22-rust-nix/) by Tor Hovland

*Tor explains how to package your Rust code using Nix. He explains the
different options you have for doing so: the Nix built-in `buildRustPackage`,
Naersk, Crane and Cargo2Nix. He shows how to build a sample application that
consists of a Rust crate that is compiled into WebAssembly, a Rust library and
a Rust application that depends on both of these. He also discusses some
potential other options for building and packaging Rust code in Nix.*

[Zero to Nix](https://zero-to-nix.com/) by Determinate Systems

*This is a guide on how to get started using Nix. It teaches you how to install it,
how to use it for development, how to package your software with it, and how to
manage your system with it.*
