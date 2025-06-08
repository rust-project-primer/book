# Bazel

Bazel is an open-source port of the Blaze build system used internally at
Google. It is, in some ways, purpose built to solve the kinds of problems that
Google faces: building large amounts of code in a giant monorepo with a very
diverse set of client machines. 

It excels at mixing and matching multiple programming languages, which makes it
a great fit when you're trying to integrate Rust into an existing C or C++
codebase, or build a web application that uses components written in different
languages (such as TypeScript for the frontend, and Rust for the backend) but
still want to have a simple build process.

It is also an *artifact-centric* rather than a *task-centric* build system.

## Why Bazel?

It uses a high-level build language and supports multiple languages and
platforms. One of Bazel's key features is its ability to create reproducible
builds, meaning that it ensures the output of a build is the same regardless of
the environment it's run in. This is achieved through strict dependency
tracking and sandboxed execution environments. Bazel's performance is enhanced
by its advanced caching and parallel execution strategies, allowing it to only
rebuild parts of the project that have changed since the last build,
significantly reducing build times. It also scales seamlessly, facilitating its
use in both small projects and massive codebases like those at Google. This
makes Bazel particularly appealing for large, multi-language projects with
complex dependencies, where build speed and consistency are critical. 

## How does Bazel work?

When you use Bazel, you declare how your project should be built in `BUILD`
files containing a descriptin in the Starlark language, which is similar to
Python. In this language, you define all of the targets and dependencies.  From
this, Bazel builds a graph of all targets and their dependencies.

Bazel will try to perform hygienic builds, meaning that you should not rely on
native dependencies being available, but rather you tell Bazel how to build
them itself. You can also have platform-specific targets and rules to ensure
that your project can be built on any platform (that your developers use or
deploy to).

Any external resources you rely on, you specifiy with a hash-sum to ensure that
the compilation process is always deterministic.

## Getting Started with Bazel

Bazel's build configuration *replaces* or *coexists* with the typical
Cargo metadata. This means that if you want to migrate a Rust project
to use Bazel, you may need to duplicate some definitions.

### Installing Bazel

While you can [install Bazel](https://bazel.build/install), the recommended way
to use it is to install [bazelisk](https://github.com/bazelbuild/bazelisk).
Bazelisk is to Bazel as Rustup is to Rust: it manages multiple versions of
Bazel and ensures that you are using the appropriate version in each project.

If you do use bazelisk, then you should add a file into your repository telling
it which version of Bazel your project should use. The simplest way to achieve
this is by creating a `.bazelversion` file containing the desitred version of
Bazel:

```
7.3.1
```

The advantage of doing this is that you ensure all users will use exactly the
same version of Bazel.

### Project Setup

To use Bazel, you need to configure a Repository (used to be called a Workspace).
You can do this by creating a `MODULE.bazel` or `REPO.bazel` file in the
root of your repository.

Typically, if you work with Rust you will want to use [rules_rust][], which
is a module that teaches Bazel how to build and interact with Rust projects.
A sample Repository configuration might look like this

```python
bazel_dep(name = "rules_rust", version = "0.48.0")
```

## Examples

This sections shows off some example projects that showcase what using Bazel in
a Rust project looks like. Bazel comes with some [Rust
examples](https://github.com/bazelbuild/examples/tree/main/rust-examples) and
the `rules_rust` comes with a more extensive [set of
examples](https://github.com/bazelbuild/rules_rust/tree/main/examples) that are
also worth looking into.

### Bazel Rust Hello World

- smallest possible bazel + rust project

### Bazel Rust Workspace

- smallest possible bazel + rust workspace project

### Mixing Rust and C

- smallest possible rust + native C code project

### Full-stack Rust web application

- smallest possible backend + frontend project

### Mixing Rust and JavaScript

- smallest possible rust + javascript (react) project

## Integrating with Nix

It is possible to integrate Bazel with Nix. The idea is that Nix is a little
bit better of a package manager, and that Bazel is a bit better as a build
system.  Nix is used to bootstrap the environment: the compiler, the native
libraries.  Bazel is then used as a build system.

If you don't Nix, to get a true hermetic build environment you need to instruct
it to build all native dependencies from source. You can avoid that when using
Nix. And the fact that Nix has a public binary cache means that you rarely need
to actually compile the thing you are using, most of the case Nix will be able
to just pull it from the cache.

- <https://nix-bazel.build/>
- <https://www.tweag.io/blog/2022-12-15-bazel-nix-migration-experience/>
- <https://www.tweag.io/blog/2018-03-15-bazel-nix/>
- <https://www.tweag.io/blog/2024-02-29-remote-execution-rules-nixpkgs/>
- <https://github.com/tweag/rules_nixpkgs>


## Reading

~~~reading
style: article
title: Scaling Rust builds with Bazel
url: https://mmapped.blog/posts/17-scaling-rust-builds-with-bazel.html
author: Roman Kashitsyn
archived: mmapped-scaling-rust-builds-with-bazel.pdf
---
Roman explains how and why the Internet Computer project switched to using
Bazel as it's build system. He explains how Bazel is good at setting up builds
that involve several languages or build targets, such as building some code for
WebAssembly and using the resulting binaries as inputs to other builds.  He
walks you through the process they used to incrementally switch a large project
to using Bazel and the implications it had. He considers the migration a
success.
~~~

~~~reading
style: article
title: Using Bazel with Rust to Build and Deploy an Application
url: https://earthly.dev/blog/bazel-with-rust/
author: Enoch Chejieh
archived: earthly-bazel-with-rust.pdf
---
Enoch walks you through how to get started with a simple Rust project that
uses Bazel to build. In particular, he shows to get get dependencies between
several crates working, and unit tests running in Bazel.
~~~

~~~reading
style: article
title: Rewriting the Modern Web in Rust
url: https://implfuture.dev/blog/rewriting-the-modern-web-in-rust
author: Kevin King
archived: implfuture-rewriting-the-modern-web-in-rust.pdf
---
Kevin shows how to set up a full-stack Rust application using
[Axum](https://docs.rs/axum) for the backend and [Yew](https://docs.rs/yew) and
the Tailwind CSS framework for the frontend. He shows how to use the Bazel
build system to wrap it all together, including getting interactive rebuilds
working. This is a good example to show how powerful Bazel is, as it involves
building the frontend to WebAssembly and embedding it into the frontend.
~~~

~~~reading
style: article
title: Building Rust Workspace with Bazel
url: https://www.tweag.io/blog/2023-07-27-building-rust-workspace-with-bazel/
author: Ilya Polyakovskiy
archived: tweag-building-rust-workspace-with-bazel.pdf
---
Ilya shows you how you can make existing Rust Workspaces build with Bazel, by
taking the `ripgrep` crate, which is a popular search tool written in Rust and
converting it to use Bazel for building and testing.
~~~

~~~reading
style: article
title: Bazel rules_rust
url: https://github.com/bazelbuild/rules_rust
author: rules_rust project
---
The `rules_rust` project is the official Rust bindings for Bazel. It lets you
tell Bazel about the crates you have, and how they depend on each other. If you
want to use Bazel to build Rust code, you should use this plugin.
~~~

~~~reading
style: article
title: "Bazel: What It Is, How It Works, and Why Developers Need It"
url: https://hackernoon.com/bazel-what-it-is-how-it-works-and-why-developers-need-it
author: David Mavrodiev
---
This article is an overview of Bazel, it discusses the basics of hot it operates
and what advantages it has for developers.
~~~

~~~reading
style: article
title: Birth of the Bazel
url: https://blog.engflow.com/2024/10/01/birth-of-the-bazel/
author: Han-Wen Nienhuys
archived: engflow-birth-of-the-bazel.pdf
---
Han-Wen explains how Bazel was born as an open source build system out of
Google's internal Blaze build system, and why the decision was made to
open-source it.
~~~
