# Test runners

Rust comes with support for unit tests built-in. You can annotate a function
with `#[test]` anywhere and it will be treated as a unit test.

```rust
#[test]
fn example_test() {
    assert_eq!(42, 40 + 2);
}
```

Often times, tests will be put into a separate module called `tests`, so that
you can import things you need in the tests and not get warnings because those
are not used in normal compilation.

You can also write integration tests by putting them into a `tests/` folder in
the crate root. Finally, code you put into crate documentation is also built,
these are called *doctests*.

Cargo has a subcommnd called `cargo test`, which will build and run all of the
tests in a crate. When you use a workspace, then using `cargo test --all` will
run all tests from all crates in the workspace.

This subcommand works well, and you should use it. However, there are a few
limitations with it that are especially noticeable when you are working in
a large Rust workspace with a lot of crates (and thus, a lot of tests and
integration tests): it runs a bit slow.

## Cargo Nextest

[Cargo Nextest][nextest] is a tool that you can use as a drop-in replacement
for `cargo test`. It has some useful features for running tests in CI, but the
main advantage is that it is [up to 3x faster][benchmarks] according to their
own benchmarks.

In my testing, I've observed a 10% speedup, but it depends on how many tests
you have and what they are bottlenecked by. In my case, the tests were
bottlenecked by external services, so there is not too much `cargo nextest` can
do. But if you have a lot of tests and/or a large Cargo workspace, be sure to
give `cargo nextest` a try, you may see a marked improvement in testing time
and the output is also more terse, which I like.

## Reading

[How (and why) nextest uses
Tokio](https://sunshowers.io/posts/nextest-and-tokio/) by Siddharth Agarwal

*Siddharth explains in this article how (and why) nextests uses Tokio.
Generally, using Tokio often comes with the assumption that some software
uses networking, however here it turns out that the async model maps very
well to scheduling tests as well. It is a fascinating peek into how nextest
works.*

[nextest]: https://nexte.st/index.html
[benchmarks]: https://nexte.st/book/benchmarks.html
