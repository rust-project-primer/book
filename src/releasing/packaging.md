# Packaging

Once you have a compiled binary, you can distribute it as a tarball or a
standalone download, but system packages are a better experience for users. A
package bundles your binary with metadata (version, description, dependencies)
and any additional files it needs (man pages, configuration files, systemd
units), and integrates with the system's package manager for installation,
upgrades, and removal. This chapter covers the Linux-focused packaging tools
available for Rust projects and briefly touches on macOS with Homebrew.

## Debian Packages

[`cargo-deb`][cargo-deb] builds `.deb` packages directly from a Cargo project.
It reads your `Cargo.toml` metadata and figures out which binaries the project
produces. Additional Debian-specific metadata goes under
`[package.metadata.deb]`:

```toml
[package.metadata.deb]
maintainer = "Alice Example <alice@example.com>"
depends = "$auto"
section = "utility"
priority = "optional"
assets = [
    ["target/release/my-tool", "usr/bin/", "755"],
    ["README.md", "usr/share/doc/my-tool/README", "644"],
]
```

The `$auto` value for `depends` tells `cargo-deb` to use `dpkg-shlibdeps` to
detect shared library dependencies automatically. The `assets` array maps build
outputs and project files to their installation paths in the package.

Once configured, building a package is a single command:

```bash
cargo install cargo-deb
cargo deb
```

`cargo-deb` also supports [systemd service integration][cargo-deb-systemd] for
daemons, build variants for different configurations, and cross-compilation for
other architectures. If you want to support automatic updates, you can [host
your own APT repository][apt-repo].

## RPM Packages

[`cargo-generate-rpm`][cargo-generate-rpm] builds `.rpm` packages for Red Hat,
Fedora, openSUSE, and other RPM-based distributions. It generates RPM files
directly using the `rpm` crate rather than requiring `rpmbuild` to be installed.
Configuration goes under `[package.metadata.generate-rpm]` in `Cargo.toml`, with
an `assets` array similar to `cargo-deb`:

```bash
cargo install cargo-generate-rpm
cargo build --release
cargo generate-rpm
```

The generated package lands in `target/generate-rpm/`. The tool supports setting
dependencies, pre/post-install scripts, and PGP signing.

## Flatpak

[Flatpak](https://flatpak.org/) is a sandboxed packaging format for desktop
Linux applications. There is no Cargo plugin for Flatpak; instead, you write a
Flatpak manifest and build with `flatpak-builder`. Since Flatpak builds are
sandboxed and cannot fetch crates from the network during build, you need to
vendor dependencies ahead of time. The
[`flatpak-cargo-generator`][flatpak-cargo-generator] script reads your
`Cargo.lock` and generates the manifest sources needed to include all
dependencies offline. Flatpak packaging is most relevant for GUI applications
using frameworks like GTK (via [gtk-rs](https://gtk-rs.org/)) or
[egui](https://www.egui.rs/).

## AppImage

[`cargo-appimage`][cargo-appimage] bundles a Rust binary into an
[AppImage](https://appimage.org/), a self-contained executable that runs on most
Linux distributions without installation. Configuration goes under
`[package.metadata.appimage]` in `Cargo.toml`, where you can specify an icon,
desktop entry, and additional assets. It can optionally embed shared libraries
into the bundle so the AppImage works on systems that lack them:

```bash
cargo install cargo-appimage
cargo appimage
```

AppImage is a good fit for distributing desktop applications to end users who
may not want to add a repository or install a package manager.

## Homebrew

[Homebrew](https://brew.sh/) is the most common package manager on macOS. There
is no Cargo plugin for it; instead, you write a Ruby formula that tells Homebrew
how to build and install your project. For a Rust project, the formula typically
declares Rust as a build dependency and runs `cargo install` during the build
phase. Alternatively, you can host pre-built binaries on GitHub Releases and
have the formula download them directly, which is faster for users. Either way,
you publish the formula through a Homebrew tap (a Git repository) that users add
with `brew tap`.

<!-- TODO: add a concrete formula example, either source-build or binary download -->

## Reading

```reading
style: article
title: "Distribution - Command Line Applications in Rust"
url: https://rust-cli.github.io/book/tutorial/packaging.html
author: Rust CLI Working Group
---
The official Rust CLI book's chapter on distribution. Covers cargo-install,
pre-built binaries with CI, and packaging for various platforms. Brief but a
good starting point with links to platform-specific guides.
```

[cargo-deb]: https://github.com/kornelski/cargo-deb
[cargo-deb-systemd]: https://github.com/kornelski/cargo-deb/blob/main/systemd.md
[apt-repo]:
  https://earthly.dev/blog/creating-and-hosting-your-own-deb-packages-and-apt-repo/
[cargo-generate-rpm]: https://github.com/cat-in-136/cargo-generate-rpm
[flatpak-cargo-generator]:
  https://github.com/flatpak/flatpak-builder-tools/tree/master/cargo
[cargo-appimage]: https://github.com/stratusfearme21/cargo-appimage
