# Development Environment

This chapter explains what you need to get started writing a Rust project.  It
outlines how your can install a Rust toolchain, and what editors or IDEs you
can use to write Rust code. If you already have a Rust toolchain installed and
you have an editor or an IDE that you are comfortable using, you can safely
skip this chapter.

## Rust Toolchain

The bare minimum you need to get started with to write and build Rust code is a
text editor and `rustc`.  However, to do meaningful work, you will likely also
need Cargo and some way to manage it, for example to update your Rust
toolchains or install support for other targets like WebAssembly.

The recommended approach to install a Rust toolchain that comes with all the
components you will need is to install [Rustup][rustup]. This will manage your
Rust toolchains, allow you to install and use both the stable and nightly
versions of Rust alongside, and will install and manage additional components
such as `rustfmt` for you.

```admonish warning
Your operating system might have Rust available in its package manager, however
you should be careful about using it. The version available might be outdated,
or there might not be a way to use Rust nightly or install a different target.
For some tasks, such as writing WebAssembly web frontends in Rust or doing
embedded development, you will need to install additional targets so that Rust
knows how to compile your code.
```

To install `rustup` on Linux, you can run the following command. If you are
using Windows, you can find installation instructions on the [website][rustup].

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

```admonish
A lot of this book is very command-line centric and as such you may find the
experience of using these tools slightly easier on UNIX-like operating systems
such as Linux or macOS. This should not come as a surprise, as the [majority of
Rust developers work on and target Linux][survey2023]. However, Rust loves
Windows too, and most of the tools explained here should work on any platform.
I try to point out any commands that either don't work on natively on Windows
or require special setup. You can always try [WSL2][wsl2] to to run things if you
run into any issues.
```

With Rustup installed, you should now have access to Cargo and you can use it
to manage your Rust installation. Here are some useful commands for reference:

```
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

```
# build and run tests using the nightly toolchain
cargo +nightly test
```

If you have Rustup installed and Cargo works, then you are set up for using Rust.

[rustup]: https://rustup.rs/

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

### Recommended IDEs

In the rest of this section, we will take a look at some of the IDEs that have
good support for Rust.  When asked in a survey in 2023, [the majority of people
in the Rust community use either VSCode or vim/neovim][survey2023].  The [Are
We IDE Yet][areweideyet] page tracks the progress of Rust integrations. 

However, we will focus on three IDEs, because I think they have the least amount
of friction.

## Background

This section provides background information on how IDEs integrate with Rust.
You can safely skip this if you don't care how it works under the hood.

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

[Rust Analyzer Manual](https://rust-analyzer.github.io/manual.html)

[LSP could have been better](https://matklad.github.io/2023/10/12/lsp-could-have-been-better.html)

[LSP: The good, the bad and the ugly](https://www.michaelpj.com/blog/2024/09/03/lsp-good-bad-ugly.html)

[Improving "Extract Function" in Rust Analyzer](https://dorianlistens.com/2022/07/improving-extract-function-in-rust-analyzer/)

[survey2023]: https://blog.rust-lang.org/2024/02/19/2023-Rust-Annual-Survey-2023-results.html
[wsl2]: https://learn.microsoft.com/en-us/windows/wsl/about

### IDEs


https://rust-lang.github.io/rustup/index.html

[areweideyet]: https://areweideyet.com/
[vim]: https://vim.org
[neovim]: https://neovim.org
[rust-analyzer]: https://github.com/rust-lang/rust-analyzer
[lsp]: https://microsoft.github.io/language-server-protocol/


[zed]: https://zed.dev/
[helix]: https://github.com/helix-editor/helix
