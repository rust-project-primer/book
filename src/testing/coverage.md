# Measure Coverage

The only way to ensure that you are doing a good job of testing all the paths in
the code is by measuring them. 

In general, I believe that library crates should aim to get as close to 100% of
test coverage as possible. Binary crates may not be able to achieve that, also
because they might use libraries that make it difficult to test them at all.
This is another good reason for splitting up project into small crates: it allows
you to have and enforce a good test coverage for all of the library crates that can
be tested, while allowing certain binary crates to not be well-tested.

In Rust, you can use `cargo-llvm-cov` to determine code coverage. It can output
in different formats, including HTML, JSON and text.

```
$ cargo llvm-cov
```

## Reading

- [cargo-llvm-cov](https://github.com/taiki-e/cargo-llvm-cov)
- [tarpaulin](https://github.com/xd009642/tarpaulin)
- [grcov](https://github.com/mozilla/grcov)
- Rustc Book: [Instrumentation-based Code Coverage](https://doc.rust-lang.org/rustc/instrument-coverage.html)

