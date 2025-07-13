# Buck2

[Buck2][buck2] ([source][buck2-repo]) is written and maintained by Facebook, and
is very similar to Bazel. What makes it interesting is that it is written in
Rust, which makes it rather likely that it has good support for building Rust
projects.

Interestingly, Buck2 uses the same language to write configuration as Bazel
does, which is called Starlark. Both the syntax and the APIs are quite similar,
but not close enough to say that they are compatible. Buck2 is quite new, having
only been released in 2022. <!-- TODO factcheck -->

What makes Buck2 exciting for us Rustaceans is that it itself is written in
Rust, and that it has good support for Rust out-of-the-box, without needing any
external plugins (as Bazel does with `rules_rust`).

## Why Buck2?

As per their website, Buck2 is an extensible and performant build system written
in Rust and designed to make your build experience faster and more efficient.

## How does it work?

## Examples

There are some
[examples](https://github.com/facebookincubator/reindeer/tree/main/example)
using reindeer, which is used to translate Cargo dependencies into Buck2
configurations.

### Building C/C++ code

### Building JavaScript

### Building WebAssmebly

## Reading

```reading
style: article
title: "Build faster with Buck2: Our open source build system"
url: https://engineering.fb.com/2023/04/06/open-source/buck2-open-source-large-scale-build-system/
author: Chris Hopman and Neil Mitchell
---
Introduction article of the Buck2 build system. Explains the features Buck2
has.
```

```reading
style: article
title: "Buck2 build: Getting started"
url: https://buck2.build/docs/getting_started/
author: Buck2 Project
---
Getting started guide of the Buck2 build system.
```

```reading
style: article
title: Using Buck to build Rust Projects
url: https://steveklabnik.com/writing/using-buck-to-build-rust-projects
author: Steve Klabnik
archived: steveklabnik-using-buck-to-build-rust-projects.pdf
---
Steve explains how Buck2 can be used to build Rust projects.
```

```reading
style: article
title: Using Crates.io with Buck
url: https://steveklabnik.com/writing/using-cratesio-with-buck
author: Steve Klabnik
archived: steveklabnik-using-crates-io-with-rust.pdf
---
Steve shows how crates from [crates.io][] can be used in projects
built by Buck2.
```

```reading
style: article
title: Updating Buck
url: https://steveklabnik.com/writing/updating-buck
author: Steve Klabnik
archived: steveklabnik-updating-buck.pdf
---
Steve shows how Buck2 can be updated.
```

[buck2]: https://buck2.build/
[buck2-repo]: https://github.com/facebook/buck2
[crates.io]: https://crates.io
