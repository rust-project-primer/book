# Caching

_While Rust does have a local build cache in the `target` folder, you notice that this is not always
useful. Especially in the CI system, the entire project is always rebuilt. It makes you wonder if
there is some way to cache the build artifacts in a global cache that your team and the CI server
can make use of._

Caching build artifacts is a large quality of life improvement: typically, the dependencies do not
change too much, and not all of the crates in your project change all the time either. With a good
build cache, compiling the project can become very fast.

```admonish info
If you are using a [Build System](../build-system/readme.md), you may get this
for free: Bazel, Buck2 and Nix all support caching compilations.
```

A simple tool to deploy from the Rust ecosystem is [`sccache`][], which allows you to cache
compilation outputs in various storage options, such as a memcached instance or a cloud storage
bucket.

## `sccache`

`sccache` is a tool that allows you to cache built artifacts in the cloud. It allows you to get
faster builds, however it also comes with the cost of having to setup and maintain a storage bucket
and access tokens for it.

```admonish example title="Using sccache in CI"
*TODO*
```

## Reading

```reading
style: article
title: "sccache"
url: https://github.com/mozilla/sccache
author: Mozilla
---
A ccache-like tool that provides shared compilation caching with various storage backends including cloud storage buckets, Redis, and memcached. Originally developed by Mozilla for Firefox builds.
```
