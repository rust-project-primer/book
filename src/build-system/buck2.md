# Buck2

[Buck2][buck2] ([source][buck2-repo]) is written and maintained by Facebook,
and is very similar to Bazel. What makes it interesting is that it is written
in Rust, which makes it rather likely that it has good support for building
Rust projects.

Interestingly, Buck2 uses the same language to write configuration as Bazel
does, which is called Starlark. Both the syntax and the APIs are quite similar,
but not close enough to say that they are compatible. Buck2 is quite new,
having only been released in 2022. <!-- TODO factcheck -->

What makes Buck2 exciting for us Rustaceans is that it itself is written in
Rust, and that it has good support for Rust out-of-the-box, without needing any
external plugins (as Bazel does with `rules_rust`).

## Why Buck2?

As per their website, Buck2 is an extensible and performant build system
written in Rust and designed to make your build experience faster and more
efficient. 

## How does it work?

## Examples

### Building C/C++ code

### Building JavaScript

### Building WebAssmebly

## Reading

[Using Buck to build Rust Projects](https://steveklabnik.com/writing/using-buck-to-build-rust-projects) by Steve Klabnik

*Steve explains how Buck2 can be used to build Rust projects.*

[Using Crates.io with Buck](https://steveklabnik.com/writing/using-cratesio-with-buck) by Steve Klabnik

*Steve shows how crates from [crates.io][] can be used in projects
built by Buck2.*

[Updating Buck](https://steveklabnik.com/writing/updating-buck) by Steve Klabnik

*Steve shows how Buck2 can be updated.*

[buck2]: https://buck2.build/
[buck2-repo]: https://github.com/facebook/buck2
[crates.io]: https://crates.io
