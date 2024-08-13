# Versioning

Rust comes with built-in support for [Semantic Versioning][semver], and you
should use it unless you have a strong reason not to.

Semantic versioning encodes information into the version string of the application.
A version looks like `1.2.3`. These three numbers are called *major*, *minor* and *patch*.

When you make a change that only fixes a bug, but does not change the
interface, you increment the last number, the patch number. These releases are
always safe to apply.

When you add a new interface that does not break existing users, you increment
the second number, the minor number.

When you change an interface in a backwards-incompatible way, you increment the
first number, the major number.

Cargo understands semantic versioning and lets you express dependency versions
as bounds.

If you want to make a prerelease of an upcoming version, for example to let
users test it (but not let Cargo choose it unless explicitly requested), you
can add a suffix.  For example `1.3.0-rc.0` would be a prerelease called
`rc.0`. The numbering there exists so that you can make multiple prereleases to
fix bugs, before you release version `1.3.0` properly.

There is some tooling you can use to enforce proper versioning, discussed in
[Semantic Versioning](../checks/semver.md).


[semver]: https://semver.org/
