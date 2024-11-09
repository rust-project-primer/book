# Snapshot Testing

Snapshot testing is a strategy to ensure that the output of some code doesn't
change over time. It make writing tests very simple, it lets you record an output
of some code once, save it as a unit test, and check that it stays the same.
If it does change, it shows you the difference.

## Insta

[Insta](https://insta.rs/)
is a crate that lets you to snapshot testing easily in Rust. It
comes with a tool that lets you record the output, and shows you the diff.

## Reading

[Using Insta for Rust snapshot testing](https://blog.logrocket.com/using-insta-rust-snapshot-testing/) by Agustinus Theodorus

*In this article, Agustinus explains how to use insta-rs to do snapshot testing in Rust.*
