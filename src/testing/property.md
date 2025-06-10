# Property Testing

Property testing is a testing methodology that allows you to generalize your
unit tests by running them with randomized inputs and testing *properties* of
the resulting state, rather than coming up with individual test cases. This
gives you confidence that your code is *generally* correct, rather than just
correct for the specific inputs you are testing. It is often effective at
finding edge cases you haven't considered.

<center>

![Property testing flow](proptest-flow.svg)

</center>

What property-testing frameworks typically do is:

- **Generate** arbitrary (random) test-cases for your tests, with constraints
  that you specify.
- **Simplify** failing inputs to crate a small failing test-case. Also called
  *test case shrinking*.
- **Record** failing test-cases, so you can replay them.


~~~admonish note
There is some overlap between property testing and [fuzzing](./fuzzing.md).
Both are testing strategies that rely on randomly generating input cases.
Usually, the difference is that property testing focusses on testing a single
component, whereas fuzzing tries to test a whole program. Additionally, fuzzing
usually employs instrumentation, where it monitors at runtime which branches
are taken and attempts to try to archieve full coverage. You can replicate
some of that by measuring [Test Coverage](../measure/coverage.md).

Usually, property tests run fast and can be part of your regular unit tests,
while fuzzing tests are run for hours and are not part of your regular testing
routing.
~~~

### Examples

When you write unit tests, they allow you to prove that for some specific
inputs, your program produces the expected output. This goes a long way to
ensuring that your code works as you intend it to.

```rust
#[test]
fn test_append() {
    assert_eq!(sort(vec![]), vec![]);
    assert_eq!(sort(vec![1, 2]), vec![1, 2]);
    assert_eq!(sort(vec![3, 1, 2]), vec![1, 2, 3]);
}
```

But writing tests like this does not prove that your code is correct. It just
proves that for these particular inputs, your code produces the intended
outputs.

How can you prove that your code is correct for *all* possible inputs? Doing
that is really only possible if you write a formal proof for the correctness
of your program, which is generally not something that you have time for.

Property testing is a testing methodology that gives you more confidence that
your program is generally correct. What it does is it allows you to define
a property that you are testing, and it will generate randomized testcases
and check that your program upholds the property.



For example, if we want to test a sorting algorithm, the property that we
want to verify is that for any given input, the output is *sorted*. You
can imagine a property test to look like this:

```rust
fn check_sorting(input: Vec<u64>) {
    assert!(is_sorted(sort(input)));
}
```

What the property testing framework does for you is generate the randomized
inputs, and it runs the test you define against some number of random inputs.
It cannot check the program exhaustively (check it against all possible
inputs), or mechanically (use formal proofs and verify them), instead it does
it stochastically.  What that means is that if one thousand random inputs get
correctly sorted, you can be reasonably confident that your sorting algorithm
works correctly.  Every time you run the tests, it will test different random
inputs.



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

### Action Strategy

One common pattern when doing property testing is letting the property
testing framework come up with a sequence of actions, and performing those.
This approach lets you test more complex interactions.

The way this works is that you create an enum that holds possible actions.
These actions can be anything, for example if you are testing a data structure
you might mimick the public interface of the data structure. If you are testing
a REST API, this struct would mimick the API endpoints that you want to test.

```rust
pub enum Action {
    CreateUser(Uuid),
    DeleteUser(Uuid),
}
```

You allow the property testing framework to generate a list of these actions,
and then you run them.

```rust
fn test_interaction(actions: Vec<Action>) {
    let service = Service::new();
    for action in actions {
        match action {
            Action::CreateUser(uuid) => {
                service.create_user(uuid);
            },
            Action::DeleteUser(uuid) => {
                service.delete_user(uuid);
            },
        }
    }
}
```

One of the ways you can use this is with a proxy object, which tracks a subset
of the state.

### Rust Crates

There are three ecosystems of property-testing frameworks that you can
use. 

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
 
https://www.tedinski.com/2018/12/11/fuzzing-and-property-testing.html

https://jo3-l.dev/posts/proptest/

[fuzzing]: https://en.wikipedia.org/wiki/Fuzzing
