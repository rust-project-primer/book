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
  that you specify. Typically, this works by generating a random seed, and
  using that in combination with a [pseudorandom number generator][prng] to
  randomly generate data structures that are used as input.
- **Simplify** failing inputs to crate a small failing test-case, also called
  *test case shrinking*. This attempts to reduce the input test case to
  something smaller to eliminate parts of the input data that don't matter,
  and to make it easier to reproduce and track down the bug.
- **Record** failing test-cases, so you can replay them. Usually this works
  by recording the initial seed, so that the same input can be generated
  again. 
- **Replay**: When running tests, recorded failing seeds are replayed first
  (before generating more randomized inputs) to ensure that there are no
  regressions where previously-found bugs resurface.

[prng]: https://en.wikipedia.org/wiki/Pseudorandom_number_generator

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

### General Principle

When you write unit tests, you know the inputs and expected outputs. When you
use property testing, your inputs will be randomized, so you don't know ahead
of time what they will be. What you do here is that you test *properties*
of the output state.

In general, all property tests are structured the same way: it is a test
function that is provided with some randomized inputs of a predefined shape,
runs some *action* on the input, and then verifies the output.

<center>

![Proptest flow](proptest-test.svg)

</center>

If you are testing a stateful system, then the initial state of the system will
be the input, and the resulting state will be the output.

For exampe: if you have an API, and you are testing the *crate user* functionality,
then your initial API (and database) state will be the input. Then you will run
the action (create user). The property that you will test for in the output state
will be that the user exists.

### Testing against a reference

Rather than manually testing properties, you can also write property tests to
apply some operations onto both your implementation and a reference implementation.
For example, if you are implementing a specific data structure, you can test it
against another data structure (that might not be as optimized as yours, but you
know is correct).

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
                service.user_create(uuid);
                assert!(service.user_exists(uuid));
            },
            Action::DeleteUser(uuid) => {
                service.user_delete(uuid);
                assert!(!service.user_exists(uuid));
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

### Example

Imagine that you are trying to implement a novel sorting algorithm. You've
read the paper, and you've tried your best to follow along and implement it
in Rust. You came up with this implementation:

```rust
{{#include ../../examples/property-testing/src/lib.rs:sort_broken}}
```

Now, you want to test it. You can start by writing some simple unit tests
for it, or maybe you already have as you were implementing your algorithm
because you used *test-driven development*.

```rust
{{#include ../../examples/property-testing/src/lib.rs:unit_test}}
```

Running these works:

```
{{#include ../../examples/property-testing/output/unit_tests.txt}}
```

The issue now is that these working unit tests do not prove that your algorithm
works in general. All they do is prove that your algorithm works for these
specific inputs. What if there is a bug in your algorithm that is only
triggered on an edge case? Hint: there is, and we will find it.

We can use property testing to test the algorithm for randomized inputs.  While
with unit testing, we test specific inputs and outputs, with property testing
we run our algorithm on unknown (random) inputs, and verify that certain
properties hold.

In this case, the function is supposed to sort an array of numbers. Sorting
implies two properties:

- The output should be sorted. This means that for any pair of adjacent numbers,
  the first should be lower or equal than the second.
- The output should contain the same numbers as the input (but maybe in a
  different order).

From this, we can derive some property checking functions. For each of our two
properties (that the output is sorted, and that the output should contain the
same elements), we write a proptest. Notice how this works: a proptest is just
a Rust unit test that takes a `Vec<u16>`. Proptest takes care of generating
this for us. Also, we use `prop_assert!()`, this is not required but makes the
proptest framework play nicer.

```rust
{{#include ../../examples/property-testing/tests/tests.rs}}
```

When you run this, you will see that it finds a failure. Because of a bug in
the implementation of our sorting algorithm, it does not work for all inputs.

```
{{#include ../../examples/property-testing/output/proptest_fail.txt:0:18}}
...
{{#include ../../examples/property-testing/output/proptest_fail.txt:297:}}
```

Helpfully, proptest records this failure. Typically, it will save the failing
seeds into a file adjacant to the source file that contains the test. In our
case, it saves them into `tests/tests.proptest-regressions`.

```
{{#include ../../examples/property-testing/output/proptest-regressions.txt}}
```

Can we fix this? For sure. Looking at the test, we can deduce what the issue
is. The problem seems to be that we remove all values from the input array, but
we only add it to the output once. So when the input array contains duplicate
values, the output will only contain a single one. We can fix this in the code
by counting the occurences, and adding that many to the output:

```rust
{{#include ../../examples/property-testing/src/lib.rs:sort_fixed}}
```

Finally, we can run the property test again to verify that it works now.

```
{{#include ../../examples/property-testing/output/proptest_ok.txt}}
```

This example was maybe a bit simplistic, unit testing could have also caught
this issue. But it shows the general principle of doing property testing: you
identify general properties that your application should uphold after certain
actions. It works well for stateless code that has an input and an output, like
this. But you can also use it to test state transitions, which we will explain
next.

~~~admonish warning
Property testing is not guaranteed to find an issue, because it is randomized.
There are some things you can do to increase the chances that proptest can find
issues. For example, you can tweak how many iterations it performs.  You can
also reduce the search space, for example by operating on `Vec<u8>` instead of
`Vec<u64>`.

But if proptest does catch an issue, it makes it easy to reproduce it, debug it
and ensure that it does not occur again (regression).
~~~


## Test-Strategy

The [test_strategy](https://docs.rs/test-strategy/latest/test_strategy/) is
a supplementary crate for proptest. It has three useful features:

- It allows you to write proptests more easily with an attribute macro
- It allows you to write proptests that are async (with support for `tokio` and
  `async-std` executors)
- It allows you derive an `Arbitrary` implementation for your custom types,
  making it easier to use them in property tests.

## QuickCheck

[QuickCheck](https://github.com/BurntSushi/quickcheck) is another property
testing crate, named after the similarly named Haskell
[QuickCheck](https://hackage.haskell.org/package/QuickCheck) package.

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
