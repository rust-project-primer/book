# Development Environment

This chapter explains what you need to get started writing a Rust project.  It
outlines how your can install a Rust toolchain, and what editors or IDEs you
can use to write Rust code. If you already have a Rust toolchain installed and
you have an editor or an IDE that you are comfortable using, you can safely
skip this chapter.

Fundamentally, you need two pieces of software to get started with your Rust
project:

- **Rust toolchain**: with the components needed for formatting, linting Rust code, in the correct version, and with the right targets.
- **Code editor**: with support for Rust through syntax highlighting and ideally integration with `rust-analyzer`.

This section outlines how you can setup your environment to be able to write
Rust productively, by showing you ways to get a Rust toolchain installed and by
examining some popular code editors used by the Rust community.

```admonish
A lot of this book is very command-line centric and as such you may find the
experience of using these tools slightly easier on UNIX-like operating systems
such as Linux or macOS. This should not come as a surprise, as the majority of
Rust developers [work on][survey2023-os] and [target Linux][survey2023-target]
according to [the 2023 survery][survey2023]. However, Rust loves Windows too,
and most of the tools explained here should work on any platform.  I try to
point out any commands that either don't work on natively on Windows or require
special setup. You can always try [WSL2][wsl2] to to run things if you run into
any issues.
```

## Rust Toolchain

The bare minimum you need to get started with to write and build Rust code is a
text editor and `rustc`. However, to do meaningful work, you will likely also
need Cargo and some way to manage it, for example to update your Rust
toolchains or install support for other targets like WebAssembly.

Rust toolchain consists of:

| Item | Description |
| --- | --- |
| `rustc` | Rust compiler |
| `cargo` | Rust package manager and build system |
| `rustfmt` | Rust code formatter |
| `clippy` | Rust linter, and automatically fix code issues |
| `rust-std` | Rust standard library source code, used when requesting `rustc` to build it from source |
| `rust-docs` | Documentation for Rust's standard library |

There are different release channels. The `stable` channel tracks
stable Rust releases, such as `1.80`, while the `nightly` channel tracks
nightly releases that come with more features, but which might be unstable.
Generally, you want to stick to the `stable` release channels, unless you have
a specific reason to use the `nightly` ones (for example, you need to use a
feature that is unstable).

Depending on what you are writing software for, you may also want to install
toolchains for different targets. For example, you may need the targets
`x86_64-unknown-linux-gnu` to build software for Linux,
`wasm32-unknown-unknown` to build software for WebAssembly targets, or
`thumbv6m-none-eabi` to target Cortex-M0 ARM microcontrollers.

```admonish warning
Your operating system might have Rust available in its package manager, however
you should be careful about using it. The version available might be outdated,
or there might not be a way to use Rust nightly or install a different target.
For some tasks, such as writing WebAssembly web frontends in Rust or doing
embedded development, you will need to install additional targets so that Rust
knows how to compile your code.
```

You will likely want some way to not only install Rust, but also manage the
components and targets, update the toolchain and have the ability to install
different versions of the toolchain side-by-side to work on your project.

### Rustup

The recommended approach to install and manage Rust toolchains, components
and targets is [Rustup][rustup]. It lets you install different versions
of the toolchain side-by-side, switch between them either explicitly or
with some configuration inside your project.

To install `rustup` on Linux, you can run the following command. If you are
using Windows, you can find installation instructions on the [website][rustup].

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

With Rustup installed, you should now have access to Cargo and you can use it
to manage your Rust installation. Here are some useful commands for reference:

```bash
# install a different version of the toolchain (can also give a specific version)
rustup install nightly
rustup install 1.80.0

# install a target
rustup target add wasm32-unknown-unknown

# update your Rust toolchain
rustup update
```

When you use Cargo, Rustup will use your default toolchain. For most of your
development, this should be sufficient. However, you can always override this
to use a specific toolchain, for example to use `nightly` for a specific command
by adding `+<version>` to any command:

```bash
# build and run tests using the nightly toolchain
cargo +nightly test
```

If you have Rustup installed and Cargo works, then you are set up for using Rust.

[rustup]: https://rustup.rs/

### Nix

While Rustup is the most popular and preferred way to manage Rust toolchains,
it is not the only way you can use to install and manage Rust toolchains.
Another popular tool used by Rustaceans to manage their toolchains is Nix,
which is a declarative package manager and build system.

*Todo*

## Editors and IDEs

Preferences for development environments amongst developers varies widely.
Some developers prefer light-weight editors such as [vim][], [neovim][], or
[helix][]. These have the advantage of being fast and portable, tend to be easy
to extend and rely on keyboard shortcuts to avoid being slowed down by using a
mouse. Especially terminal-native developers tend to prefer enjoy these
editors, because it means they can do all of their development in the terminal
and can even use these editors remotely over SSH.

The other camp likes using IDEs, which are graphical tools for writing code.
They tend to integrate very well into the programming languages and have
compelling features such as jump-to-definition, show type information or have
debugging support built-in. IDEs used to have a bad reputation for being
rigid, but modern ones are just as extensible as command-line editors.

<figure>

![Editors used by the Rust community in 2023](https://blog.rust-lang.org/images/2024-02-rust-survey-2023/what-ide-do-you-use.svg)

<figcaption>Responses in the 2023 Rust Survey to "Which editor or IDE setup do you use with Rust code on a regular basis?"</figcaption>
</figure>

This survey shows that the two most popular editors for Rust are VS Code, and
Vi-family editors (which I group together as Vim). The Zed editor is also
popular, but did not appear in this survey, likely because it was not stable at
the time the survey was run.

We can cluster the editors into two groups:

- **Graphical IDEs**: Includes VS Code, Rust Rover, Sublime Text, Visual Studio, Xcode, Atom.
- **Terminal-based editors**: Vim, Helix, Emacs

In general, Graphical IDEs are more friendly to beginners. For this reason, the
editors discussed in this chapter focusses mainly on these. The Terminal-based
editors have their own advantages, but they require more learning and unless
you are already familiar with them, it likely does not make sense to pick them
up.

In the subsections of this chapter, we take a look at three editors that
yield a good developer experience:

- **VS Code**: Partially open-source editor developed by Microsoft, has extensive plugin functionality, basically a clone of the once-popular Atom editor.
- **Zed**: Open-source editor written in Rust, comes with Rust support out of the box. Not available for Windows currently.
- **Rust Rover**: Commercial, but free-to-use for noncommerical applications, developed by JetBrains.

### Rust Analyzer

Language servers are tools that parse understand programming languages, and
expose this data to IDEs. Unline compilers, which run once and produce a
binary, language servers are designed to run continuously, generated metadata
such as inferred types of values, and implement high-level operations such as
refactoring code.

The original language server for Rust was called [Rust Language
Server](https://github.com/rust-lang/rls), and it used [rustc to parse
projects](https://github.com/rust-lang/rls/blob/master/architecture.md). This
approached worked initially, but there were issues with latency. Additionally,
rustc is not great at handling incomplete or broken code, which is important
for language servers as they run while you write code. As a result, RLS was
[deprecated](https://blog.rust-lang.org/2022/07/01/RLS-deprecation.html) in
2022.

- graph of [rls architecture](https://github.com/rust-lang/rls/blob/master/architecture.md)

As a result, a new approach was taken that used a custom parser to be more
error-resiliant than `rustc`, called
[rust-analyzer](https://blog.rust-lang.org/2022/02/21/rust-analyzer-joins-rust-org.html).

- graph of rust-analyzer architecture

The core piece that makes Rust IDEs possible is thus [rust-analyzer][], which
is a project that understands Rust projects and implements the [Language Server
Protocol][lsp], which is a way for IDEs to understand them too and display type
annotations, warnings, errors, suggestions.

In general, any IDE that supports the LSP protocol can be used for Rust
development using [rust-analyzer][]. The only exception is Rust Rover, which
implements it's own parser for Rust projects.

In general, you don't need to know much about Rust Analyzer to use it. In fact,
many Rust IDEs even bundle it, and will manage and update it for you. You will not
even be aware that it is running in the background. But there are some situations where
you might need to be aware of its existence. If you use build systems other than Cargo to
build your Rust project, for example, then Rust Analyzer might not be able to analyze
your project. There might also be cases where it has bugs, because it uses a different
parser for Rust than `rustc` has.

## Reading

~~~reading
style: book
url: https://rust-lang.github.io/rustup/index.html
title: The Rustup Book
author: Rust Language
---
Book for the Rustup tool used by the Rust community to install and manage Rust
toolchains. It explains core concepts such as channels, toolchains, components
and profiles, how to configure Rustup to use specific versions of the toolchain
on a per-project basis.
~~~

~~~reading
style: book
url: https://rust-analyzer.github.io/manual.html
title: Rust Analyzer Manual
author: Rust Analyzer
---
Explains what `rust-analyzer` is, and how to use it. It has instructions for the
best way to install it for every editor it supports, and outlines ways you can
configure it for your project.
~~~

~~~reading
style: article
url: https://matklad.github.io/2022/04/25/why-lsp.html
title: Why LSP?
author: Alex Kladov
---
Alex explains what problem LSPs solve.
~~~

~~~reading
style: article
url: https://matklad.github.io/2023/10/12/lsp-could-have-been-better.html
title: LSP could have been better
author: Alex Kladov
---
This article discusses architectural aspects of LSPs, that Alex does not find as brilliant.
~~~

~~~reading
style: article
url: https://www.michaelpj.com/blog/2024/09/03/lsp-good-bad-ugly.html
title: "LSP: The good, the bad and the ugly"
author: Michael Peyton Jones
---
~~~

~~~reading
style: article
url: https://dorianlistens.com/2022/07/improving-extract-function-in-rust-analyzer/
title: Improving 'Extract Function' in Rust Analyzer
author: Dorian Scheidt
---
~~~

[survey2023]: https://blog.rust-lang.org/2024/02/19/2023-Rust-Annual-Survey-2023-results.html
[survey2023-os]: https://blog.rust-lang.org/images/2024-02-rust-survey-2023/which-os-do-you-use.svg
[survey2023-target]: https://blog.rust-lang.org/images/2024-02-rust-survey-2023/which-os-do-you-target.svg
[wsl2]: https://learn.microsoft.com/en-us/windows/wsl/about
[areweideyet]: https://areweideyet.com/
[vim]: https://vim.org
[neovim]: https://neovim.org
[rust-analyzer]: https://github.com/rust-lang/rust-analyzer
[lsp]: https://microsoft.github.io/language-server-protocol/
[zed]: https://zed.dev/
[helix]: https://github.com/helix-editor/helix
