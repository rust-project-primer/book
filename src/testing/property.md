# Property Testing

One common issue you will run into when writing unit tests is that there is a
lot of repetition and artificial test cases. For example, if you test a simple
piece of code which appends something to a vector, you might end up with test
cases like this:

~~~admonish note
There is some overlap between property testing and [Fuzzing](./fuzzing.md).
Both are testing strategies that rely on randomly generating input cases.
Usually, the difference is that property testing focusses on testing a single
component, whereas fuzzing tries to test a whole program. Additionally, fuzzing
usually employs instrumentation, where it monitors at runtime which branches
are taken and attempts to try to archieve full coverage. You can replicate some
of that by measuring [Test Coverage](../measure/coverage.md).
~~~

```rust
#[test]
fn test_append() {
    assert_eq!(vec![].append(1), vec![1]);
    assert_eq!(vec![1, 2].append(3), vec![1, 2, 3]);
    assert_eq!(vec![0, 0, 0].append(0), vec![0, 0, 0, 0]);
}
```

In this example, you are making sure that your append function works correctly,
but you have actually only tested that it works for these particular cases. What
if you have missed an edge case in your tests? Ideally, you want to be able to
write a test case like *given any array, when you call `.append()` on it with
a value, make sure that the resulting array is the original array with the
value appended to it*.

Property-based testing allows you to express exactly that. What property-based
testing lets you do is consume randomized inputs, such that properties of the
code you are testing hold true under *all possible* inputs, rather than just the
couple you can come up with and hard-code.

For example, this is how you might write a reasonable test for your code:

```rust
fn test_append(list: Vec<u8>, value: u8) {
    let appended = list.append(value);

    // length of appended is one more than original list.
    assert_eq!(appended.len(), list.len() + 1);

    // appended consists of the original list, plus `value` at the end.
    assert_eq!(appended[0..list.len()], &list[..]);
    assert_eq!(appended[list.len()], value);
}
```

In some sense, using property testing is a lot like using [fuzzing][], but it
usually works at the function level rather than the whole program. Because you
are testing smaller pieces of code, you can test it more thoroughly and in less
time.

To use property testing, you need a framework. Two popular ones in Rust are
[quickcheck](https://github.com/BurntSushi/quickcheck) and
[proptest](https://docs.rs/proptest/latest/proptest/). While they are both
good, I recommend you use the latter.

## Proptest

Proptest is a framework that makes it easy to set up property-based testing in
Rust. It lets you generate randomized inputs for your property-based tests.
When it hits a failure, it attempts to reduce the input to a minimal example.
It records failing test inputs such that they will be retried.

If you use `proptest`, I recommend you to use it with the `test-strategy` crate,
which just contains some macros that make it simpler to set it up and use it
to test async code, for example.

An example proptest, using the `test-strategy` crate looks like this:

```rust
#[proptest]
fn test_parser(input: &str) {
    let ast = parse(input);
}
```

## Example

## Test-Strategy

[test_strategy Crate](https://docs.rs/test-strategy/latest/test_strategy/)

## QuickCheck

https://github.com/BurntSushi/quickcheck

## Arbitrary and Arbtest

https://crates.io/crates/arbtest


# Reading

~~~reading
style: book
title: Proptest Book
url: https://proptest-rs.github.io/proptest/
author: Proptest Project
---
The official book of the proptest crate. This is a valuable read if you want
to understand how it works and how you can customize it, for example by
implementing custom strategies for generating test inputs.
~~~

~~~reading
style: article
title: "Complete Guide to Testing Code in Rust: Property testing"
url: https://zerotomastery.io/blog/complete-guide-to-testing-code-in-rust/#Property-testing
author: Jayson Lennon
---
~~~

~~~reading
style: article
title: Property-testing async code in Rust to build reliable distributed systems
url: https://www.youtube.com/watch?v=ms8zKpS_dZE
author: Antonio Scandurra
---
In this presentation, Antonio explains how he used property testing to test
the Zed editor for correctness. Being a concurrent, futures-based application,
it is important that the code is correct. By testing random permutations of the
futures execution ordering, he was able to find bugs in edge cases that would
otherwise have been very difficult to discover or reproduce.
~~~

~~~reading
style: article
title: An Introduction to Property-Based Testing in Rust
url: https://www.lpalmieri.com/posts/an-introduction-to-property-based-testing-in-rust/
author: Luca Palmieri
archived: lpalmieri-an-introduction-to-property-based-testing-in-rust.pdf
---
An exerpt from his book, *Zero to Production in Rust*, Luca does a deep-dive
into property testing in Rust. He shows how to test a web backend using its
REST API using both the `proptest` crate and the `quickcheck` crate.
~~~

~~~reading
style: article
title: Property-Based Testing in Rust with Arbitrary
url: https://www.greyblake.com/blog/property-based-testing-in-rust-with-arbitrary/
author: Serhii Potapov
archived: greyblake-property-based-testing-in-rust-with-arbitrary.pdf
---
Serhii shows how to use the `arbitrary` crate and the `arbtest` crate to
implement property-testing in Rust.
~~~

~~~reading
style: article
title: Bridging fuzzzing and property testing
url: https://blog.yoshuawuyts.com/bridging-fuzzing-and-property-testing/
author: Yoshua Wuyts
archived: yoshuawuyts-bridging-fuzzing-and-property-testing.pdf
---
Yoshua notices that fuzzing and property testing are fundamentally similar, in
that they generate random test-cases for programs. He mentions the `arbitrary`
crate, which is used for fuzzing in Rust. He explains how to use this same
crate to generate random test-cases for property testing, and explains his
crate to do this, called `heckcheck`. He also mentions that there is another
crate for doing this, called `proptest-arbitrary-interop`. The advantage of
using these crates is that they unify the library ecosystem used for fuzzing
with that used for property testing.
~~~

~~~reading
style: article
title: Property-based testing in Rust with Proptest
url: https://tinkering.xyz/property-based-testing-with-proptest/
author: Zach Mitchell
archived: tinkering-property-based-testing-with-proptest.pdf
---
Zack shows how to use the proptest crate to write property tests. He gives
an example of writing a parser using the `pest` crate, shows how to implement
custom strategies for generating arbitrary test cases, and uses them to
test his parser.
~~~

[fuzzing]: https://en.wikipedia.org/wiki/Fuzzing
