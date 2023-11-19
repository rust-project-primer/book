# Introduction

This is a guide for how to best set up your Rust project. This is by no means a
definite guide, think of it more as my opinionated set of best practises.
However, while I do not claim for these to be definite or required in any way,
I am hoping that they might be useful to you if you are setting up a new Rust
project, or if you are a crate author looking to find some inspiration.

The challenges you face in developing a small Rust project are vastly different
from those you may encounter when working in large, complex pieces of software
that have a lot of people contributing to them. This guide may show tools that
are only relevant once the complexity grows.

## Recommended Reading

This guide does not explain the fundamentals of the Rust programming language.
Rather, it shows some practical things that I have found to be useful to do in
Rust projects to allow them to scale and be of high quality.

Before reading this guide, if you have not already, I highly recommend reading
some books that give you a vast understanding of the Rust programming language
and explain concepts in much more detail than I ever could.

Here is a few books which I consider to be essential for understanding Rust.
Where possible, I will link to relevant chapters in these books from within
this guide, so that you may read about topics more in-depth.

### Rust Programming Language, 2nd Edition

Available [online][rust-book] and as [paperback][rust-book-nostarch].

### Rust for Rustaceans

### Effective Rust

### Zero to Production

Available for purchase [here][zero-to-production].

### Rust Design Patterns

Available online [here][rust-design-patterns].

[zero-to-production]: https://www.zero2prod.com/
[rust-design-patterns]: https://rust-unofficial.github.io/patterns/
[effective-rust]: https://www.lurklurk.org/effective-rust/
[rustonimicon]: https://doc.rust-lang.org/nightly/nomicon/
[rust-book]: https://doc.rust-lang.org/book/
[rust-book-nostarch]: https://nostarch.com/rust-programming-language-2nd-edition
[rust-book-image]: https://nostarch.com/sites/default/files/styles/uc_product/public/RustProgramming2ndEd_comp.png
[rust-for-rustaceans]: https://nostarch.com/rust-rustaceans
[rust-for-rustaceans-image]: https://nostarch.com/sites/default/files/styles/uc_product/public/RustforRustaceans_cover.png
