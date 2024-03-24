# Performance

If there is some part of your crate where you care about performance, then you
should test for it. The way to test the performance of some code in Rust is by
writing some benchmarks.

Writing benchmarks allows you to quickly evaluate changes to some code, by
comparing the benchmark results from different versions. Another application is
tracking the performance of your code over time, by running benchmarks on every
commit or periodically by a platform such as [Bencher][bencher] or the [Continuous Benchmark GitHub Action][continuous-benchmark].

I must add that I am a big proponent of implementing this. You should care
about performance. Your users care about performance — even a 100ms change is
detectable by a human. And most importantly, performance does not happen by
accident. Only when you continuously measure it, does your team have the
feedback it needs to make good choices. In computer science, everything is a
tradeoff, and you can only make good tradeoffs if you have some hard data on
what you are trading in.

## Criterion

Typically, the way that you write these is using the [criterion][] crate[^1].
This lets you test both synchronous and asynchronous code, and it provides some
support for statistical analysis of the benchmark results.  The Rust standard
library also has some benchmarking support, but this is currently a
nightly-only feature.

## Examples

TODO:
- simple benchmarking with criterion
- async benchmarking with criterion
- benchmarking published to bencher

## Reading

[Criterion.rs Book](https://bheisler.github.io/criterion.rs/book/index.html)

*The Criterion Book explains how to get started using Criterion, and what
features it has.*

[Benchmark It!](https://www.justanotherdot.com/posts/benchmark-it) by Ryan James Spencer

*Ryan argues in this blog post that you should benchmark code. He said that
users can feel performance and you should care about it. He explains how to get
started doing performance benchmarkis in Rust using criterion.*

[Continuous Benchmarking](https://bencher.dev/docs/explanation/continuous-benchmarking/)

*This blog post from Bencher explains the concept of continuous benchmarking. It also talks about some myths surrounding benchmarking, for example benchmarking in CI.*

[Continuous benchmarking for rustls](https://ochagavia.nl/blog/continuous-benchmarking-for-rustls/) by Adolfo Ochagavía

*Adolfo explains in this blog post how he was able to implement continuous
benchmarking for the [rustls][] library, and how he was able to leverage this
to find performance regressions easily. He explains that using `cachegrind` was
instrumental, because it is able to count CPU instructions and easily diff them
per function for different benchmark runs, which allows for tracking down which
function introduced a regression.*

[criterion]: https://docs.rs/criterion/latest/criterion/
[bencher]: https://bencher.dev/
[continuous-benchmark]: https://github.com/marketplace/actions/continuous-benchmark
[rustls]: https://github.com/rustls/rustls
