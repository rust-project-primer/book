# Benchmarking

If there is some part of your crate where you care about performance, then you
should test for it. The way to test the performance of some code in Rust is by writing
a benchmark.

Typically, the way that you write these is using the [criterion][] crate[^1]. This
lets you test both synchronous and asynchronous code, and it provides some support
for statistical analysis of the benchmark results.

## Examples

*TODO*

## Reading

- [Criterion.rs Documentation](https://bheisler.github.io/criterion.rs/book/index.html)

[criterion]: https://docs.rs/criterion/latest/criterion/
[^1]: The Rust standard library also has some benchmarking support, but this is currently a nightly-only feature.


https://www.justanotherdot.com/posts/benchmark-it
