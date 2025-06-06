# Snapshot Testing

Snapshot testing (sometimes called *golden tests*) is a software testing
technique that captures the output of a component or system and saves it as a
reference for future comparisons. It helps ensure that the output **remains
consistent over time** by comparing current results to previously stored
snapshots, highlighting any changes that may indicate bugs or unintended
modifications.

Generally, to use snapshot testing you will use some kind of framework which is
responsible for creating, reading and updating the snapshots.

Snapshot testing is not a replacement for traditional unit-testing, rather it
is a tool in your toolbelt that makes it easier for you to add more test cases,
and maintain them when your code changes. Making it easy to add and maintain
tests leads to having more test cases, which is beneficial for your codebase.

## Example

If you want to test that a function works correctly, with regular unit testing
you would come up with test cases where you define and input and compare it
with the expected output. You would then use some kind of assertion macro (in
Rust, `assert_eq!` is used) to ensure that the output matches the expected
output.

```rust
#[test]
fn test_to_json() {
    let input = MyType {
        name: "Name".to_string(),
        email: "name@example.com".to_string(),
    };

    // you have to write this by hand
    let output = "{\"name\":\"Name\",\"email\":\"name@example.com\"}";

    assert_eq!(output, input.to_json());
}
```

Coming up with these test cases takes time. Also, there are situations where
you don't care what the exact output is, just that it doesn't change. Also,
maintaining the tests takes time, because you need to update the source code.

With snapshot testing, instead you use the testing framework to record and
compare the expected outputs. Here, we use the imagined `assert_snapshot!`
macro to compare the output of `input.to_json()` to whatever was previously
recorded.

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
and save it in your repository. Depending on your framework, it may have a tool
that lets you review the output before you save it.

On subsequent test runs, it will always use the saved output as reference and
compare it. If the output is incorrect, it may be able to show you a diff that
highlights where the output differs from what was previously recorded.

If you ever change the implementation of the function you are testing, for
example to alphabetically order the fields (so *email* is serialized before
*name*), the snapshot testing tool may help you by allowing you to review each
changed output and accept it, rather than having to copy-and-paste changed
output into the test sources.

## Use-Cases

Some common use-cases of snapshot testing are:

- **Data formats**: If you want to write tests for an encoding format, you can
  use snapshot testing to ensure that the library will always encode types in
  the same way deterministically.
- **Data transformations**: When you want to test a function that transforms
  data, you can use it to record outputs.
- **Command-line tools**: If you want to write unit tests for a CLI tool, you can use snapshot testing
  to capture and compare the standard output that the CLI tool produces for
  various invocations.

# Insta

Insta ([website][insta], [docs][insta-docs], [repo][insta-repo]) is at the time
of writing the most popular snapshot testing framework in Rust. It comes
batteries-included with a command-line tool that allows you to review and
record the outputs of your snapshot tests.

### Using Insta

Insta has a command-line tool called `cargo-insta` that you can use to
record and update stored snapshots. Using this is not mandatory, but it
makes it easier to manage, review and update snapshots.

It has some macros that allow you to take a snapshot of some value, which will
be written into a file. Depending on what you want to take a snapshot of, you
can use the `Display` output, the `Debug` output, or a variety of serde
encodings (JSON, TOML, YAML, CSV, RON). When using the serde encodings, the
types that you are storing snapshots of must implement serde's `Serialize`.

| Macro | Description |
| --- | --- |
| `insta::assert_snapshot!` | Uses the `Display` representation. |
| `insta::assert_debug_snapshot!` | Uses the `Debug` representation. |
| `insta::assert_json_snapshot!` | Uses the JSON serialisation. |
| `insta::assert_yaml_snapshot!` | Uses the YAML serialisation. |
| `insta::assert_toml_snapshot!` | Uses the TOML serialisation. |
| `insta::assert_csv_snapshot!` | Uses the CSV serialisation. |
| `insta::assert_ron_snapshot!` | Uses the RON serialisation. |

Instead of running `cargo test`, you can run `cargo insta test`. The first
time, this will record the initial values. The tool will show you each
snapshot, allow you to review it, and if you are happy with it, store it.

Snapshots are by default stored in a `snapshot/` folder next to your code.

### Testing Command-Line Tools

Insta has an optional extension insta-cmd ([repo][insta-cmd-repo],
[docs][insta-cmd-docs]) that makes it easy to test command-line tools.

## Examples

# Expect-Test

expect-test is a bit of an odd snapshot testing tool. It is much more minimal
than insta is.

Insta has support for doing this as well, with the [Inline Snapshots][insta-inline]

## Runt

Runt ([repo][runt-repo], [docs][runt-docs]) is a tool for snapshot-testing binaries.

## Reading

[Using Insta for Rust snapshot
testing](https://blog.logrocket.com/using-insta-rust-snapshot-testing/) by
Agustinus Theodorus

*In this article, Agustinus explains how to use insta-rs to do snapshot testing
in Rust.*

[Try Snapshot Testing for Compilers and Compiler-Like
Things](https://www.cs.cornell.edu/%7Easampson/blog/turnt.html) by Adrian
Sampson

*Adrian argues that snapshot testing is a good tool for programs that turn text
into other text, which describes compilers and lots of compiler-like things. He
describes turnt, a Python implementation of a snapshot-testing tool. He
explains that using snapshot testing makes it easy to add new tests, and that
the resulting tests can serve as a kind of documentation.*

[Snapshot Testing](https://avi.im/blag/2024/snapshot-testing/) Avinash
Sajjanshetty


[runt-repo]: https://github.com/rachitnigam/runt
[runt-docs]: https://docs.rs/runt/latest/runt/
[insta]: https://insta.rs/
[insta-inline]: https://docs.rs/insta/latest/insta/#inline-snapshots
[insta-repo]: https://github.com/mitsuhiko/insta
[insta-docs]: https://docs.rs/insta/latest/insta/
[insta-cmd-repo]: https://github.com/mitsuhiko/insta-cmd
[insta-cmd-docs]: https://docs.rs/insta-cmd/latest/insta_cmd/
