# Snapshot Testing

Snapshot testing is a software testing technique that captures the output of a
component or system and saves it as a reference for future comparisons. It helps
ensure that the output **remains consistent over time** by comparing current
results to previously stored snapshots, highlighting any changes that may
indicate bugs or unintended modifications. In other words, a snapshot test
asserts that _the output of this should not change_.

```admonish
The name *snapshot testing* comes from the idea that the first time you run
the tests, they will record the current output (take a *snapshot*), and then
on subsequent runs compare the output to the previously recorded output.

Some people also refer to it as *golden testing* (the snapshot being the
*golden master*). *Transcript tests* is a related concept, that focusses
on using the external APIs only.
```

Generally, to use snapshot testing you will use some kind of framework which is
responsible for creating, reading and updating the snapshots.

Snapshot testing is not a replacement for traditional unit-testing, rather it is
a tool in your toolbelt that makes it easier for you to add more test cases, and
maintain them when your code changes. Making it easy to add and maintain tests
leads to having more test cases, which is beneficial for your codebase.

## Example

If you want to test that a function works correctly, with regular unit testing
you would come up with test cases where you define and input and compare it with
the expected output. You would then use some kind of assertion macro (in Rust,
`assert_eq!` is used) to ensure that the output matches the expected output.

```rust
#[test]
fn test_to_json() {
    let input = MyType {
        name: "Name".to_string(),
        email: "name@example.com".to_string(),
    };

    // you have to write this by hand
    let expected = "{\"name\":\"Name\",\"email\":\"name@example.com\"}";

    assert_eq!(expected, input.to_json());
}
```

Coming up with these test cases takes time. Also, there are situations where you
don't care what the exact output is, just that it doesn't change. Also,
maintaining the tests takes time, because you need to update the source code.

With snapshot testing, instead you use the testing framework to record and
compare the expected outputs. Here, we use the imagined `assert_snapshot!` macro
to compare the output of `input.to_json()` to whatever was previously recorded.

```rust
#[test]
fn test_to_json() {
    let input = MyType {
        name: "Name".to_string(),
        email: "name@example.com".to_string(),
    };

    // pseudocode, replace this with whatever framework you use.
    assert_snapshot!(input.to_json());
}
```

The first time you run this, it will run `input.to_json()`, record the output
and save it in your repository as a snapshot. Depending on your framework, it
may have a tool that lets you review the output before you save it.

On subsequent test runs, it will use the saved snapshot as reference and compare
it to the current output. If the output is different, it may be able to show you
a diff that highlights where the output differs from what was previously
recorded.

If you ever change the implementation of the function you are testing, for
example to alphabetically order the fields (so _email_ is serialized before
_name_), the snapshot testing tool may help you by allowing you to review each
changed output and accept it, rather than having to copy-and-paste changed
output into the test sources.

## Use-Cases

Some common use-cases of snapshot testing are:

- **Data formats**: If you want to write tests for an encoding format, you can
  use snapshot testing to ensure that the library will always encode types in
  the same way deterministically.
- **Data transformations**: When you want to test a function that transforms
  data, you can use it to record outputs.
- **Command-line tools**: If you want to write unit tests for a CLI tool, you
  can use snapshot testing to capture and compare the standard output that the
  CLI tool produces for various invocations.

# Insta

Insta ([website][insta], [docs][insta-docs], [repo][insta-repo]) is at the time
of writing the most popular snapshot testing framework in the Rust ecosystem. It
comes batteries-included with support for different snapshot serialization
formats and a command-line tool that allows you to review and record the outputs
of your snapshot tests.

### Insta Concepts

Insta has a command-line tool called `cargo-insta` that you can use to record
and update stored snapshots. Using this is not mandatory, but it makes it easier
to manage, review and update snapshots. You can install it with cargo:

    cargo install cargo-insta

It has several macros that allow you to take a snapshot of some value, which
will be written into a file. They differ in how they serialize the value:
depending on what you want to take a snapshot of, you can use the `Display`
output, the `Debug` output, or a variety of serde encodings (JSON, TOML, YAML,
CSV, RON). When using the serde encodings, the types that you are storing
snapshots of must implement serde's `Serialize`.

| Macro                           | Description                        |
| ------------------------------- | ---------------------------------- |
| `insta::assert_snapshot!`       | Uses the `Display` representation. |
| `insta::assert_debug_snapshot!` | Uses the `Debug` representation.   |
| `insta::assert_json_snapshot!`  | Uses the JSON serialisation.       |
| `insta::assert_yaml_snapshot!`  | Uses the YAML serialisation.       |
| `insta::assert_toml_snapshot!`  | Uses the TOML serialisation.       |
| `insta::assert_csv_snapshot!`   | Uses the CSV serialisation.        |
| `insta::assert_ron_snapshot!`   | Uses the RON serialisation.        |

Instead of running `cargo test`, you can run `cargo insta test`. The first time,
this will record the initial values. The tool will show you each snapshot, allow
you to review it, and if you are happy with it, store it.

Snapshots are by default stored in a `snapshot/` folder next to your code.

### Managing Snapshots

_TODO_

### Testing Command-Line Tools

Insta has an optional extension called insta-cmd ([repo][insta-cmd-repo],
[docs][insta-cmd-docs]) that makes it easy to test command-line tools.

```rust
use std::process::Command;
use insta_cmd::assert_cmd_snapshot;

#[test]
fn test_command() {
    assert_cmd_snapshot!(Command::new("echo").arg("hello"));
}
```

## Examples

_TODO_

# Expect-Test

expect-test ([repo][expect-test-repo], [docs][expect-test-docs]) is another
crate that allows you to do snapshot testing. It is very minimal, offering only
an `expect!` macro. It uses a different approach to insta, in that it stores the
snapshots _inside your source code_. The interesting bit about it is that it is
able to modifiy your source code to update the snapshot value, if needed. In
some ways, that makes it a hybrid of unit-testing and snapshot testing.

Insta has support for doing this as well, with the [Inline
Snapshots][insta-inline] feature.

## Runt

Runt ([repo][runt-repo], [docs][runt-docs]) is a tool for snapshot-testing
binaries. It implements _transcript tests_, which are related to snapshot tests
but generally work a bit different.

## Reading

```reading
style: article
title: Insta - Snapshot Testing for Rust
url: https://blog.bryantluk.com/insta-rs-snapshot-testing-library/
author: Bryant Luk
---
Bryant explores insta, explaining how using it helps with development speed
because changes in code do not require fixing many test cases, but simply
reviewing changes in snapshots. He highlights a feature of insta that allows
for running tests against multiple input files.
```

```reading
style: article
title: Using Insta for Rust snapshot testing
url: https://blog.logrocket.com/using-insta-rust-snapshot-testing/
author: Agustinus Theodorus
---
In this article, Agustinus explains how to use insta-rs to do snapshot testing
in Rust. He shows how to set it up, and demonstrates using it by an example.
```

```reading
style: article
title: Try Snapshot Testing for Compilers and Compiler-Like Things
url: https://www.cs.cornell.edu/%7Easampson/blog/turnt.html
author: Adrian Sampson
---
Adrian argues that snapshot testing is a good tool for programs that turn text
into other text, which describes compilers and lots of compiler-like things. He
describes turnt, a Python implementation of a snapshot-testing tool. He
explains that using snapshot testing makes it easy to add new tests, and that
the resulting tests can serve as a kind of documentation.
```

```reading
style: article
title: Building Industrial Strength Software without Unit Tests
url: https://chrispenner.ca/posts/transcript-tests
author: Chris Penner
---
Chris describes a concept called transcript tests, which are a kind of
snapshot test that focus on testing the external behaviour of a tool, commonly
a text-based tool. He explains how these allow for writing tests without
actually writing tests.
```

```reading
style: article
title: What if writing tests was a joyful experience?
url: https://blog.janestreet.com/the-joy-of-expect-tests/
author: James Somers
---
James describes a pattern that is commonly used at Jane Street: expect tests.
He explains how writing tests this way makes them fun, and sometimes more
useful because all of the state is easily visible. He explains how they work,
and how they relate to some Rust libraries such as `expect-test` and `insta`.
```

[expect-test-docs]: https://docs.rs/expect-test/latest/expect_test/
[expect-test-repo]: https://github.com/rust-analyzer/expect-test
[runt-repo]: https://github.com/rachitnigam/runt
[runt-docs]: https://docs.rs/runt/latest/runt/
[insta]: https://insta.rs/
[insta-inline]: https://docs.rs/insta/latest/insta/#inline-snapshots
[insta-repo]: https://github.com/mitsuhiko/insta
[insta-docs]: https://docs.rs/insta/latest/insta/
[insta-cmd-repo]: https://github.com/mitsuhiko/insta-cmd
[insta-cmd-docs]: https://docs.rs/insta-cmd/latest/insta_cmd/
