# Performance

Rust often attracts people that care about performance. Often times, performance is not the end
goal: instead, higher performance means higher efficiency. In an era of cloud computing, this
translates to lower costs per request.

Performance optimizations are a large subject, and this book will not go into depth when it comes to
it. There are other books that do a better job of summarizing what can be done to optimize
applications, such as the
[Rust Performance Book](https://nnethercote.github.io/perf-book/title-page.html). But this book does
make a point that performance is something that should be tested and tracked over time, that is the
only way to ensure that a project is heading in the right direction and not regressing.

The way you can do that in Rust is by writing benchmarks. In fact, Cargo comes with built-in support
for doing so. While the Cargo build-in benchmarking harness is still unstable, there are some crates
that allow you to easily build benchmarks for both blocking and async code, and track their
performance over time.

Writing benchmarks makes it easy to experiement with different options of implementing a feature,
because it makes it easy to compare the performance differences between various approaches. Another
application is tracking the performance of your code over time, by running benchmarks on every
commit or periodically by a platform such as [Bencher][bencher] or the [Continuous Benchmark GitHub
Action][continuous-benchmark].

Often times, performance is a tradeoff. While Rust has some zero-cost abstrations that allow you to
write simple code that is still fast, there are many situations where you have to make a choice
between a simpler implementation or some tech debt, and doing it properly, resulting in more
development time or more complex code. The only way to make these decisions properly is to have data
for them. How much runtime performance are you trading by keeping your simple implementation? How
much performance are you gaining by having a more complex implementation? Projects should make these
decisions based on measurements, and not guesses.

## Criterion

Typically, the way that you write these is using the [criterion][] crate[^1]. This lets you test
both synchronous and asynchronous code, and it provides some support for statistical analysis of the
benchmark results. The Rust standard library also has some benchmarking support, but this is
currently a nightly-only feature.

### Examples

TODO:

- simple benchmarking with criterion
- async benchmarking with criterion
- benchmarking published to bencher

## Valgrind

- idea: repeatable measurements (on same architecture).

## Flamegraph

## Debugging Performance

So, what do you do if you notice that your Rust code is not performing well? There are some common
issues you might run into:

- **Build mode**: Are you building your code in release mode (eg. `cargo build --release`)? It makes
  a large difference for Rust projects.
- **Optimization level**: Have you changed the optimization level, for example to optimize for size
  rather than speed? This can also make a large difference.
- **Link-time optimization**: Have you tried enabling `lto` in your compilation profile?
- **Build target**: Are you building for musl libc instead of glibc (eg.
  `--target x86_64-unknown-linux-musl`)? Musl tends to produce slower code.
- **Allocator**: Is your application allocation-heavy? Then try using
  [`jemallocator`][jemallocator], it might give you a performance boost.
- **Data structures**: Have you tried using different data structures? For example, the
  [hashbrown](https://docs.rs/hashbrown/latest/hashbrown/) crate has a `HashMap` implementation that
  is significantly faster than the standard library.

If these didn't fix your performance issues, the next step to do is to find out why your performance
isn't good. When it comes to improving performance, the best thing to do is to be guided by data
rather than intuition. There are many microoptimizations you can do in your code that lead to
negligible benefits. Letting yourself be guided by data allows you to focus on the most important
optimizations, this is known as [Amdahl's law](https://en.wikipedia.org/wiki/Amdahl's_law).

### Visualizing Performance

To get an understanding of _where_ you are losing performance, you want to get some insight into
which code in your program is responsible for the majority of the runtime. Doing this guides you to
where you should focus your attention towards when trying optimization approaches.

[cargo-flamegraph](https://github.com/flamegraph-rs/flamegraph) is a Cargo subcommand that lets you
visualize what code in your project is taking up the majority of the runtime.

## Reading

```reading
style: book
title: Rust Performance Book
url: https://nnethercote.github.io/perf-book/index.html
author: Various Authors
---
This book summarizes various approaches of benchmarking and profiling code,
and offers some suggestions to use to improve performance.
```

```reading
style: book
title: Criterion.rs Book
url: https://bheisler.github.io/criterion.rs/book/index.html
author: Brook Heisler
---
The Criterion Book explains how to get started using Criterion, and what
features it has.
```

```reading
style: article
title: Benchmark It!
url: https://www.justanotherdot.com/posts/benchmark-it
author: Ryan James Spencer
archived: justanotherdot-benchmark-it.pdf
---
Ryan argues in this blog post that you should benchmark code. He said that
users can feel performance and you should care about it. He explains how to get
started doing performance benchmarkis in Rust using criterion.
```

```reading
style: article
title: Continuous Benchmarking
url: https://bencher.dev/docs/explanation/continuous-benchmarking/
author: Bencher
---
This blog post from Bencher explains the concept of continuous benchmarking. It
also talks about some myths surrounding benchmarking, for example benchmarking
in CI.
```

```reading
style: article
title: Continuous benchmarking for rustls
url: https://ochagavia.nl/blog/continuous-benchmarking-for-rustls/
author: Adolfo Ochagavía
---
Adolfo explains in this blog post how he was able to implement continuous
benchmarking for the [rustls][] library, and how he was able to leverage this
to find performance regressions easily. He explains that using `cachegrind` was
instrumental, because it is able to count CPU instructions and easily diff them
per function for different benchmark runs, which allows for tracking down which
function introduced a regression.
```

```reading
style: article
title: Criterion Flamegraphs
url: https://andi-zimmerer.com/posts/criterion-flamegraphs.html
author: Andi Zimmerer
---
```

```reading
style: article
title: Making slow Rust code fast
url: https://patrickfreed.github.io/rust/2021/10/15/making-slow-rust-code-fast.html
author: Patrick Freed
archived: patrickfreed-making-slow-rust-code-fast.pdf
---
```

```reading
style: article
title: Guidelines on Benchmarking and Rust
url: https://nickb.dev/blog/guidelines-on-benchmarking-and-rust/
author: Nick Babcock
archived: nickb-guidelines-on-benchmarking-and-rust.pdf
---
```

```reading
style: article
title: Benchmarking and analyzing Rust performance with Criterion and iai
url: https://blog.lambdaclass.com/benchmarking-and-analyzing-rust-performance-with-criterion-and-iai/
author: Lambda Class
---
```

```reading
style: article
title: Benchmarking Rust code using Criterion-rs
url: https://engineering.deptagency.com/benchmarking-rust-code-using-criterion-rs
author: Ashwin Sundar
---
```

Windtunnel CI

https://lib.rs/crates/iai-callgrind

https://github.com/bheisler/iai

```reading
style: article
title: Rust Heap Profiling with Jemalloc
url: https://www.magiroux.com/rust-jemalloc-profiling/
author: Marc-Andre Giroux
archived: magiroux-rust-jemalloc-profiling.pdf
---
Marc-Andre explains in this article how to use jemallocator's built-in support
for emitting heap dumps, and how to analyze them with `jeprof`. He explains how to
control the profiling behaviour from inside Rust, and gives an anecdote about Facebook
using this in production for many services with little overhead.
```

```reading
style: article
title: Exploring the Rust compiler benchmark suite
url: https://kobzol.github.io/rust/rustc/2023/08/18/rustc-benchmark-suite.html
author: Jakub Beránek
archived: kobzol-rustc-benchmark-suite.pdf
---
```

https://blog.anp.lol/rust/2016/07/24/profiling-rust-perf-flamegraph/

[Benchmarking](https://nnethercote.github.io/perf-book/benchmarking.html) in The Rust Performance
Book

[Achieving warp speed with Rust](https://gist.github.com/jFransham/369a86eff00e5f280ed25121454acec1)

[criterion]: https://docs.rs/criterion/latest/criterion/
[bencher]: https://bencher.dev/
[continuous-benchmark]: https://github.com/marketplace/actions/continuous-benchmark
[rustls]: https://github.com/rustls/rustls
[jemallocator]: https://docs.rs/jemallocator/latest/jemallocator/
