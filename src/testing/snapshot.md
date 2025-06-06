# Snapshot Testing

Snapshot testing is a software testing technique that captures the output of a
component or system and saves it as a reference for future comparisons. It
helps ensure that the output **remains consistent over time** by comparing
current results to previously stored snapshots, highlighting any changes that
may indicate bugs or unintended modifications.

## Example

For example, if you wanted to test that a function works correctly, with regular
unit testing you would come up with test cases, where you define and input and
compare it with the expected output.

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
you don't care what the exact output is, just that it doesn't change. With snapshot
testing, instead you use the testing framework to record and compare the expected
outputs.

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
changed output and accept it.

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

Generally, snapshot testing does not replace regular unit testing, but it is a
good tool to have, and it can make your life easier.

# Insta

Insta ([website][insta], [docs][insta-docs], [repo][insta-repo]) is at the time
of writing the most popular snapshot testing framework in Rust. It comes
batteries-included with a command-line tool that allows you to review and
record the outputs of your snapshot tests.

It has some macros that allow you to take a snapshot of some value, which
will be written into a file. Depending on what you want to take a snapshot
of, you can use the `Display` output, the `Debug` output, or a variety of
serde encodings (JSON, TOML, YAML, CSV, RON).

### Testing Command-Line Tools

Insta has an optional extension insta-cmd ([repo][insta-cmd-repo],
[docs][insta-cmd-docs]) that makes it easy to test command-line tools.

## Examples

# Expect-Test

expect-test is a bit of an odd snapshot testing tool. It is much more minimal
than insta is.

Insta has support for doing this as well, with the [Inline Snapshots][insta-inline]

## Reading

[Using Insta for Rust snapshot testing](https://blog.logrocket.com/using-insta-rust-snapshot-testing/) by Agustinus Theodorus

*In this article, Agustinus explains how to use insta-rs to do snapshot testing in Rust.*


[insta]: https://insta.rs/
[insta-inline]: https://docs.rs/insta/latest/insta/#inline-snapshots
[insta-repo]: https://github.com/mitsuhiko/insta
[insta-docs]: https://docs.rs/insta/latest/insta/
[insta-cmd-repo]: https://github.com/mitsuhiko/insta-cmd
[insta-cmd-docs]: https://docs.rs/insta-cmd/latest/insta_cmd/
