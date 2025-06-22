# Nix

Nix is a declarative package manager and build system.  It lets you defined
dependencies and configurations in a functional language, and uses build
isolation to ensure consistend and reproducible builds across machines. 

The declarative nature of Nix makes it great at dealing with complex
environment. It handles cross-platform builds correctly. Despite being over 20
years old, it has recently gained a lot of support. It is useful for providing
consistent development setups between teams, ensuring that the code has the
same environments between developers, CI machines and deployment machines.

Nix is quite versatile. It can be used to configure your system, setup a
hygienic development shell containing only the dependencies you explicitly
requested, build Docker images with the minimal set of runtime dependencies.

## Nix Explainer

- derivations
- sandboxes
- flakes

### Nix Limitations

- versioning is a bit awkward

## What can you use Nix for?

Nix is a bit of an oddball in this section because it is more than just a build
system. You can use it, or even combine it with other build systems.
Some common setups are:

- Using Nix to define a development environment
- Using Nix to define CI tasks that can be easily run locally
- Using Nix as a build system
- Using Nix to deploy your application

Nix has great support for caching. This is one of the principal reasons why it
is useful as a build system.

## Nix Development Environment

The Rust project comes with `rustup`, which you can use to manage your Rust
toolchains. It allows you to install multiple versions of Rust side-by-side,
update them, and select a toolchain version per-project. You can even put a
`rust-toolchain.toml` file in your project root, and have `rustup` pick this up
and select the appropriate toolchain for you. This is explained in the
[Cargo](./cargo.md) chapter.

However, this doesn't quite solve all of your environment needs. What if you
need to have a specific C library in your environment? What if you need to have
specific tooling in your environment? Rustup is great at managing Rust
toolchains, that is the primary purpose it serves. But it will not manage all
of your native dependencies.

This is where Nix comes in. With Nix, you can declaratively define an environment,
and you can use `nix-shell` to spawn a new shell with everything declared
in that environment accessible. That way, you can declare which native dependencies
you need *once*, and make sure that no matter what platform your developers happen
to use, Nix can make sure that all requirements are satisfied.

```nix
{
  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk";

    nixpkgs-mozilla = {
      url = "github:mozilla/nixpkgs-mozilla";
      flake = false;
    };
  };

  outputs = { self, flake-utils, naersk, nixpkgs, nixpkgs-mozilla }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = (import nixpkgs) {
          inherit system;

          overlays = [
            (import nixpkgs-mozilla)
          ];
        };

        toolchain = (pkgs.rustChannelOf {
          rustToolchain = ./rust-toolchain;
          sha256 = "";
          #        ^ After you run `nix build`, replace this with the actual
          #          hash from the error message
        }).rust;

        naersk' = pkgs.callPackage naersk {
          cargo = toolchain;
          rustc = toolchain;
        };

      in rec {
        # For `nix build` & `nix run`:
        defaultPackage = naersk'.buildPackage {
          src = ./.;
        };

        # For `nix develop` (optional, can be skipped):
        devShell = pkgs.mkShell {
          nativeBuildInputs = [ toolchain ];
        };
      }
    );
}
```

### Example: Cargo and OpenSSL

## Nix for Continuous Integration

https://nix-ci.com/

https://serokell.io/blog/continuous-delivery-with-nix

https://github.com/NixOS/hydra

https://hercules-ci.com/


## Nix as a build system

### Building C/C++ dependencies

### Building TypeScript dependencies

### Building WebAssembly component

https://jordankaye.dev/posts/rust-wasm-nix/

## Nix for deployment

https://garnix.io/blog/hosting-nixos

https://x86.lol/generic/2024/08/28/systemd-sysupdate.html

## Reading

~~~reading
style: book
title: Nix Reference Manual
url: https://nix.dev/manual/nix/latest/
author: Nix Project
---
Reference manual for the Nix package manager.
~~~

~~~reading
style: article
title: Rust
url: https://nixos.wiki/wiki/Rust
author: NixOS Wiki
---
~~~

[ipetkov/crane](https://github.com/ipetkov/crane) on GitHub

~~~reading
style: article
title: Building a Rust service with Nix
url: https://fasterthanli.me/series/building-a-rust-service-with-nix
author: Amos Wenger
---
Amos shows how to build a Rust service in this article.
~~~

~~~reading
style: article
title: "Introducing Crane: Composable and Cacheable Builds with Cargo and Nix"
url: https://ipetkov.dev/blog/introducing-crane/
author: Ivan Petkov
archived: ipetkov-introducing-crane.pdf
---
Ivan introduces Crane in this article, a Nix library for building Cargo
projects. He explains how it works and how to use it to build Rust projects.
~~~

~~~reading
style: article
title: Building Nix Flakes from Rust Workspaces
url: https://www.tweag.io/blog/2022-09-22-rust-nix/
author: Tor Hovland
archived: tweag-rust-nix.pdf
---
Tor explains how to package your Rust code using Nix. He explains the
different options you have for doing so: the Nix built-in `buildRustPackage`,
Naersk, Crane and Cargo2Nix. He shows how to build a sample application that
consists of a Rust crate that is compiled into WebAssembly, a Rust library and
a Rust application that depends on both of these. He also discusses some
potential other options for building and packaging Rust code in Nix.
~~~

~~~reading
style: article
title: Zero to Nix
url: https://zero-to-nix.com/)
author: Determinate Systems
---
This is a guide on how to get started using Nix. It teaches you how to install it,
how to use it for development, how to package your software with it, and how to
manage your system with it.
~~~

~~~reading
style: article
title: What is Nix?
url: https://serokell.io/blog/what-is-nix
author: Alexander Bantyev
---
~~~

~~~reading
style: article
title: The Nix Thesis 
url: https://jonathanlorimer.dev/posts/nix-thesis.html
author: Jonathan Lorimer
---
~~~

~~~reading
style: article
title: Some notes on NixOS
url: https://jvns.ca/blog/2024/01/01/some-notes-on-nixos/
author: Julia Evans
---
~~~

https://jvns.ca/blog/2023/03/03/how-do-nix-builds-work-/

https://jvns.ca/blog/2023/02/28/some-notes-on-using-nix/

Alternative Nix implementations:

https://tvix.dev/
https://lix.systems/about/
