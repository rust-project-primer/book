# Property-Based

When writing unit tests, generally they all look the same way:

- Setup preconditions
- Run some code you are testing
- Check postconditions

One of the issues you have is that the space of possible preconditions can be quite
large. If you choose preconditions manually, you run the risk of leaking some knowledge
or assumptions into the test, and only testing those inputs that you expect to happen.

Property-based testing can help you here by automatically and randomly generating
preconditions. You can think of this as running a fuzzer against your functions.

```rust
#[proptest]
fn test_parser(input: &str) {
    let ast = parse(input);
}
```


## Reading

- [Proptest Book](https://proptest-rs.github.io/proptest/)
- [test_strategy Crate](https://github.com/frozenlib/test-strategy)
- [Complete Guide to Testing Code in Rust: Property testing](https://zerotomastery.io/blog/complete-guide-to-testing-code-in-rust/#Property-testing)
