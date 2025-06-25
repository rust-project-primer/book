# Nix

Nix is a declarative package manager and build system. It lets you define
dependencies and configurations in a functional language, and uses build
isolation to ensure consistent and reproducible builds across machines. 

The declarative nature of Nix makes it great at dealing with complex
environment. It handles cross-platform builds correctly. Despite being over 20
years old, it has recently gained a lot of support. It is useful for providing
consistent development setups between teams, ensuring that the code has the
same environments between developers, CI machines and deployment machines.

Nix is quite versatile. It can be used to configure your system, setup a
hygienic development shell containing only the dependencies you explicitly
requested, build Docker images with the minimal set of runtime dependencies.

## Nix Explainer

There are three main ways you can use Nix:

- **Operating system**: NixOS, which is built on top of Nix, is an entire
  operating system that you can use. It allows you to define everything on
  your work machine with the Nix language.
- **Package manager**: you can use Nix as a package manager. For example,
  you can use it to define (in your project) which dependencies it should make
  available (think libraries, frameworks, compilers). Nix will make these
  available, and it will make sure that no matter what machine or platform you
  build on, you always have *exactly* the same versions of those dependencies.
  On top of this, you can use a different build system, for example Buck or
  Bazel. In this configuration, Nix is only responsible for providing the
  dependencies your project needs to build.
- **Build system**: using something like Nix Flakes (we will discuss them
  later), you can define how every part of your project is built. Then Nix will
  build your project. Depending on what you are building, it can be a bit of
  work to get it building with Nix. The advantage you have if you do this is
  that you get reproducible builds, so no matter which machine you build on, you
  always get exactly the same output. You can also use caching, which makes
  builds faster for developers.

In this section, we will not take a look at NixOS. Mainly we will focus on
using Nix as a build system, but we will also show how you could use it as
a package manager in combination with another build system.

### Nix Terminology

If you are new to Nix, it can be a bit confusing. Nix is both a language,
and a package manager, and a build system. It uses Flakes and derivation.
If you already know them, you can skip past the subheadings here, but it
makes sense to explain how this all works together.

### Derivations

At the very core of Nix is a *derivation*. This is how Nix tracks how to
compile things. It can take other derivations as input (via
`nativeBuildInputs`, `buildInputs`), some files (via `src`), and it has some
shell scripts that define how it is built, and how it is installed.

<center>

![Nix derivation example](nix-derivation.svg)

</center>

Here's an example derivation (this is just a snippet, and not a full, working
Nix config):

```nix
pkgs.stdenv.mkDerivation {
  src = {
    url = "https://github.com/xfbs/passgen/releases/v0.1.2/passgen-v0.1.2.tar.gz";
    hash = "sha256-0000000000000000000000000000000000";
  };
  nativeBuildInputs = [ pkgs.cmake pkgs.ruby pkgs.python3 ];
  buildPhase = ''
    mkdir build
    cd build
    cmake ..
    make -j
  '';
  installPhase = ''
    make install
  '';
};
```

Derivations are *deterministic*, which means that if you execute them again at
a later date, or on a different machine, they are expected to produce exactly
the same output. Nix uses some strategies to make that happen. For example,
when your derivation is built, it runs in a sandbox where it only has access to
the derivations it declared as inputs, nothing else. When it attempts to get
the current time, it receives a timestamp of zero. Network access is blocked.
Any external data the derivation uses must have a hashsum, and Nix checks it to
make sure the data is still the same.

This allows Nix to use an aggressive caching strategy. It can use the hash
of the derivation (this includes the hash of all transitive dependencies)
as a key, and the output of it as values.

One of the important problems that Nix addresses here is that even Rust code
has *implicit* dependencies. For example, your Rust program is linked with some
kind of libc, typically `glibc` or `musl`. Which version you have depends on
your distribution, and how frequently you install updates. So if some code
works on your machine, it might not work on someone else's machine, because you
don't use the same versions. Similarly, if you use native dependencies like
SQLite, it is possible that you don't have the same version as your coworker.
What Nix ensures is that, when you do build your code, everyone builds it with
*exactly* the same versions of all dependencies (compilers, libraries,
headers).

### Nixpkgs

When you build some code, you typically need a compiler. You might also need
some libraries, and you may want to use some tools (linters, maybe script
interpreters if your build process involves running scripts). Instead of having
to define derivations for each of these, Nix has a centralized repository
called *nixpkgs*, which contains Nix derivations for most popular packages. In
the derivation earlier, we showed that we used the `nativeBuildInputs`.  The
`pkgs` that we wrote there refers to nixpkgs.

### Nix Shell

Nix Shell is the feature that you can use if you want to use Nix as a package
manager. When you define a Nix Shell, you can tell Nix which dependencies you
need. When you launch it, Nix will open a new shell that has the dependencies
you specified available in its `$PATH`.

For example, this is what a simple shell might look like. Typically, you
will save this as `shell.nix`:

```nix
{
  # todo
}
```

You can launch the shell with `nix-shell`. It will recognize the `shell.nix`
file in your current directory, and create a shell that links the tools you
specified.

You can use this in combination with other build systems. For example, if you
use Bazel, then you can use a simple definition that includes Bazel.

Here is an example:

```nix
# todo
```

Note that even if you only use the Nix Shell, you may still want to use Nix
Flakes, for reasons that we will explain later (it has to do with pinning the
version of `nixpkgs` that you are using).

### Nix Flakes

We've explained what a derivation is. But how do you write one? Nix has an
*experimental* feature called *flakes*, which is typically what you want to
use. Nix Flakes make it easy for you to specify the version of nixpkgs (that
is where all preexisting software is packaged) and import Nix definitions from
other repositories.

When you write your Nix derivations to build your code components, you typically
want to use existing code. For example, you might want to use a Rust compiler
toolchain, the SQLite library, and some tools. Nix has a large repository called
*nixpkgs* which contain Nix definitions for most packages that you would find
in other package managers.

But you might also want to import derivations from another source. For example,
you might want to import some Nix code that helps you turn Rust's build metadata
(your `Cargo.toml`) into something Nix can understand and build. Or you might
import derivations from another repository that you use.

<center>

![Nix Flakes diagram](nix-flakes.svg)

</center>

Nix Flakes allow you to write Nix code that has two definitions: a set of
*inputs*, which are typically Git repositories. This can be `nixpkgs`, or
helpers, or other flakes (in which case you can access their exported
derivations). And you can export *outputs*, which can be packagess (derivations),
*apps* (which are commands you can run) and definitions for how to spawn a
development shell.

Here is an example for what a derivation looks like:

```nix
{
  description = "A very basic flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
  };

  outputs = { self, nixpkgs }: {
    packages.x86_64-linux.hello = nixpkgs.legacyPackages.x86_64-linux.hello;
    packages.x86_64-linux.default = self.packages.x86_64-linux.hello;
  };
}
```

Even if the synax might be unfamiliar, you can see two things:

- The flake has a description, which is just an informative string.
- The flake has some inputs, which are specified by URL.
- The flake has some outputs. This is a function that takes the parsed
  input flakes as input, and return some kind of structure. In this example,
  we define some keys in the `packages` field of the output structure.
- We have hard-coded only output packages for the `x86_64-linux` architecture.
  We could also hard-code outputs for other architectures, or use some Nix
  features to automatically make this work for a set of platforms we want to
  support.

With this simple configuration, we can run it if we save it as `flake.nix` and
run `nix run`:

```bash
$ nix run
Hello
```

Earlier, I mentioned that Nix is *deterministic*. But how does that work here?
We have referenced other Git repositories by their branches, but the branches
might change. However, when you run any Nix command, Nix will resolve the
inputs to a commit hash, and record that in the `flake.lock` file.

### Nix Limitations

As explained earlier, Nix has a central repository called *nixpkgs* that
contains definitions for how to build packages. Nix does not store each and
every version for each package. Rather, it always points to the latest release
of each package.

For example, you cannot tell Nix that you want SQLite version 3.12.1. Instead,
you can only tell Nix that you want SQLite version 3, which is the package
`sqlite3`. If for some reason you need to use an older version of SQLite (which
is not recommended), you need to use an earlier version of the entire nixpkgs
(which means you will also get older versions of other packages).

In general, this is a good thing. Because usually, you do want to use the latest
versions of packages, in order to get the latest features, but most importantly,
to get the latest security fixes. But if for some reason you don't, then you it
can get in your way.

~~~admonish info
You can always manually write derivations for the packages where you need a
specific version, and otherwise use the latest nixpkgs.
~~~

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

### Example: Bazel and Rust

### Example: Cargo and OpenSSL

## Nix as a build system

You can use Nix as your primary build system. Doing so gives you reproducible
builds, and caching for free. The downside is that you need to write (and maintain)
the Nix configuration for building your project. You can't just use Cargo directly,
because Cargo defaults to downloading dependencies from the internet. Instead,
you need to use some kind of wrapper that provides you with a Rust toolchain
of your choice, parses your Cargo dependencies lock file and makes your Rust
dependencies available in a Nix-native way.

There are some popular wrappers that make this easy:

| Name | Description |
| --- | --- |
| [Crane][crane] | ... |
| [Naersk][naersk] | ... |

[crane]: https://github.com
[naersk]: https://github.com

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


### Building Rust code

### Building C/C++ dependencies

### Building TypeScript dependencies

### Building WebAssembly component

https://jordankaye.dev/posts/rust-wasm-nix/


## Nix for Continuous Integration

A common issue that developers have is that software works on one machine, but
doesn't work on another one. Usually, this is caused by differences in the
environment.

It is very frustrating when tests work perfectly locally, but fail in CI.
Often times, the CI system uses runner nodes that are not easily accessible,
making it hard to debug or reproduce the issue.

Because Nix is deterministic, it can help alleviate this. It makes for a good
development experience, where there is trust that when tests work locally, they
also work in CI (and vice versa).

Nix has built-in support for running tests. Nix calls them *checks*. In your
`flake.nix`, you can define a set of commands to run when checking code:

```nix
{
  ..
  checks = {
    unit-tests {
      drv = utils.mkShellScript {
        run = ''
          cargo test
        '';
      };
    };
  };
}
```

When you define your tests this way, then you can run them with:

  nix flake check

An added bonus is that if you do use some tools for checking crates,
such as `cargo-hack`, Nix is able to provide them for you. 

There are even some CI systems that focus on running Nix checks:

| Name | Description |
| --- | --- |
| [Hydra][hydra] | Continuous Integration system built by the Nix community. |
| [Nix CI][nix-ci] | ... |
| [Hercules CI][hercules-ci] | ... |

[hydra]: https://github.com/NixOS/hydra
[nix-ci]: https://nix-ci.com/
[hercules-ci]: https://hercules-ci.com/

https://serokell.io/blog/continuous-delivery-with-nix

## Nix for deployment

https://garnix.io/blog/hosting-nixos

https://x86.lol/generic/2024/08/28/systemd-sysupdate.html

## Nix as a build cache

By default, Nix will cache build outputs on your local machine. But if many
people work on a project, and tend to compile the same code frequently, then it
makes sense to use a shared build cache.

<center>

![Nix build cache](nix-build-cache.svg)

</center>

In a typical configuration, the CI system has write access to the build cache.
Any commits that are pushed and run through it, have their build outputs
uploaded to the cache. Developer machines have read-only access to the cache.
This ensures that builds that don't change frequently (such as dependencies,
tooling) are always in the build cache. New code is in the cache, as soon as it
is pushed to the repository, and is available for example when other developers
do code review.

You can use hosted solutions like Cachix for your build cache, or you can setup
a S3 bucket on some provider (Hetzner, Wasabi, Backblaze, AWS) and configure
it. You should take care that only trusted people or machines are able to write
into it, because this can be a security issue.

## Nix as distributed compiler

Finally, you can use Nix to speed up compilation by using it as a distributed
compiler. 

*Todo*

## Summary

In this section, we've shown that Nix has a very strict determinism. This
allows you to use it for reproducible builds, have confidence that software
built on one machine behaves the same way on a different machine. It also
allows it to use very aggressive caching, and to run compilation on different
machines.

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

~~~reading
style: article
title: "Practical Nix flake anatomy: a guided tour of flake.nix"
url: https://vtimofeenko.com/posts/practical-nix-flake-anatomy-a-guided-tour-of-flake.nix/
author: Vladimir Timofeenko
---
Vladimit explains how a `flake.nix` file is constructed. He explains the high-level
concepts (inputs, outputs) and shows syntax examples for how to write them.
~~~


https://jvns.ca/blog/2023/03/03/how-do-nix-builds-work-/

https://jvns.ca/blog/2023/02/28/some-notes-on-using-nix/

Alternative Nix implementations:

https://tvix.dev/
https://lix.systems/about/
