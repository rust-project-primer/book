# Introduction

This guide is a collection of advice on how to best architect and maintain Rust
project. Most of this advice is either derived from looking at how popular Rust
projects are run, or from personal experience building Rust code.

This is by no means a definite guide, think of it more as my opinionated set of
issues you might encounter and how you could solve them. In fact, for some
issues multiple solutions are proposed.  However, while I do not claim for
these to be definite or required in any way, I am hoping that they might be
useful to you if you are setting up a new Rust project, or if you are a crate
author looking to find some inspiration.

Keep in mind that this book is a living document. It will be updated with
current advice and solutions where possible.

## Motivation

As the Rust programming language grows in popularity, it creates a situation
where some projects grow to have a large number of contributors. The challenges
faced by small projects are quite different from those faced by larger projects.

This guide is intended to share advice between projects to make sure that Rust
crates continue to deliver the same robustness and stability that they have
in the past.

