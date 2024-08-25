# Packaging

When you release software, packaging it often a good way to make it easy to
deploy, either by end-users or by other systems. Packaging your software allows
you to bundle the executables and libraries with assets (such as man pages,
configuration files, documentation or runtime data) into a single file that
contains metadata such as the version of your software and runtime dependencies
it has.  Doing so allows not only for easy installation, but it also makes
upgrades easy, as package managers will take care of removing the old version
and installing the new file.

Popular package formats are:

- Debian packages
- RPM packages
- FlatPak
- AppImage

The Rust ecosystem has some useful tools to package software. Typically, this
is useful for packaging it for Linux systems, as many other operating systems
do not have a useful package manager.

## Debian Package

If you want to create releases for Linux users, specifically ones that use the
[APT package manager][apt] (which includes [Debian][debian] and it's
derivatives, such as [Ubuntu][ubuntu] and [Linux Mint][linux mint]), then
[cargo-deb][] can help you do just that.

The advantage of releasing builds as Debian packages over [tarballs][tarball]
is that it contains metadata, and makes it easy to install and remove packages
for end-users. When running software from tarballs, often the installation
involves manually copying files to specific folders, without an easy way to
later remove the software.

If you want to support automatic updates, you can even [host your own APT
repository][apt repo].

### Configuration

The cargo-deb tool reads and understands your `Cargo.toml` metadata and can
automatically figure out which binaries your project produces, and will add
them to the package. However, Debian packages typically have more metadata than
is captured in there by default, for example paths to assets that need to be
installed or dependencies to other packages.

To give this data to the tool, you can capture it under the
`package.metadata.deb` key in the project definition. Here is an example for
what that looks like:

```toml
[package.metadata.deb]
maintainer = "Michael Aaron Murphy <mmstickman@gmail.com>"
copyright = "2017, Michael Aaron Murphy <mmstickman@gmail.com>"
license-file = ["LICENSE", "4"]
extended-description = """\
A simple subcommand for the Cargo package manager for \
building Debian packages from Rust projects."""
depends = "$auto"
section = "utility"
priority = "optional"
assets = [
    ["target/release/cargo-deb", "usr/bin/", "755"],
    ["README.md", "usr/share/doc/cargo-deb/README", "644"],
]
```

Once you have set up the metadata, creating your Debian package is as easy as
running `cargo deb` in the repository.

In addition to this, the tool also allows you to define variants, has an [integration
with systemd][cargo-deb-systemd] to  allow you to install systemd units, and even
supports cross-compilation for other architectures.

### Examples

*TODO*

## RPM Package

https://github.com/cat-in-136/cargo-generate-rpm

## Flatpak

## AppImage

[cargo-deb-systemd]: https://github.com/kornelski/cargo-deb/blob/main/systemd.md
[apt repo]: https://earthly.dev/blog/creating-and-hosting-your-own-deb-packages-and-apt-repo/
[tarball]: https://en.wikipedia.org/wiki/Tar_(computing)
[cargo-deb]: https://github.com/kornelski/cargo-deb
[apt]: https://en.wikipedia.org/wiki/APT_(software)
[debian]: https://www.debian.org/
[ubuntu]: https://www.ubuntu.com/
[linux mint]: https://www.linuxmint.com/
