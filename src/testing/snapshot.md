# Snapshot Testing

Snapshot testing captures the output of some code and saves it as a reference
file. On subsequent test runs, the output is compared against the saved
snapshot, and any difference is flagged as a failure. The idea is simple: rather
than writing expected values by hand, you let the framework record them for you
and then verify that they don't change unexpectedly.

```admonish
Some people also refer to this as *golden testing* (the snapshot being the
*golden master*). *Transcript tests* are a related concept that focus on
testing only the external interface of a tool.
```

Snapshot testing is not a replacement for unit testing. It is a complementary
technique that makes it easy to add test cases and maintain them when output
changes. This is especially valuable for code whose output is large or complex
enough that writing expected values by hand would be tedious and error-prone.

### Snapshot testing vs unit testing

With traditional unit testing, you tend to compare the output of some process to
some known result. This requires you to be able to specify what the desired
output should be.

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

With snapshot testing, you assert the output of some process. Generally, you
don't specify what that output is (the snapshot testing framework will help you
with that), all you care about is that it stays the same.

```rust
#[test]
fn test_to_json() {
    let input = MyType {
        name: "Name".to_string(),
        email: "name@example.com".to_string(),
    };

    // the framework records the output on first run
    // and compares against the saved snapshot on subsequent runs
    assert_snapshot!(input.to_json());
}
```

The snapshot testing framework will ensure that the output of `input.to_json()`
will stay the same. If it does change, usually the frameworks will show you a
diff so that you can find out what the change is. You can then choose if you
accept the change (it was intended) or not.

### Use Cases

Snapshot testing works well for code that transforms data into a textual
representation:

- **Serialization formats**: ensuring that a type always encodes to the same
  JSON, TOML, or YAML.
- **Data transformations**: capturing the output of a pipeline or compiler pass.
- **UI component rendering**: capturing the generated HTML output of frontend
  components, to make sure they don't change.
- **Command-line tools**: recording the stdout/stderr of a CLI invocation for
  various inputs.

```admonish note
The test suite for Cargo uses snapshot testing, but with a twist: it checks not
only the output (standard error and standard output), it also tests the before
and after filesystem state. It does that using [fixtures](https://github.com/rust-lang/cargo/tree/master/tests/testsuite/cargo_add/add_basic)
which have the start state, the command to run, the expected console output,
and the expected filesystem state after the command is run.
```

### How snapshot testing works

The first time this test runs, it records the output and saves it. On subsequent
runs, it compares the current output to the saved snapshot. If the output
changes (for example, because you reordered the JSON fields), the snapshot tool
shows you a diff and lets you accept the new output rather than forcing you to
copy-paste updated values into your test source.

## Insta

[Insta][insta] ([docs][insta-docs], [repo][insta-repo]) is the most widely used
snapshot testing framework in the Rust ecosystem. It ships with multiple
serialization formats and a command-line tool for reviewing and accepting
snapshot changes.

### Macros

Insta provides several assertion macros that differ in how they serialize the
value being snapshotted:

| Macro                    | Serialization                      |
| ------------------------ | ---------------------------------- |
| `assert_snapshot!`       | Uses the `Display` representation. |
| `assert_debug_snapshot!` | Uses the `Debug` representation.   |
| `assert_json_snapshot!`  | Uses JSON serialization.           |
| `assert_yaml_snapshot!`  | Uses YAML serialization.           |
| `assert_toml_snapshot!`  | Uses TOML serialization.           |
| `assert_csv_snapshot!`   | Uses CSV serialization.            |
| `assert_ron_snapshot!`   | Uses RON serialization.            |

The serde-based macros (JSON, YAML, TOML, CSV, RON) require the snapshotted type
to implement `Serialize`.

### Workflow

The typical insta workflow has three steps:

1. **Run tests**: `cargo insta test` runs your test suite and writes any new or
   changed snapshots to `.snap.new` files next to your code.
2. **Review**: `cargo insta review` opens an interactive terminal UI that shows
   you each pending snapshot change as a diff. You can accept or reject each one
   individually.
3. **Commit**: accepted snapshots are promoted from `.snap.new` to `.snap`
   files, which you commit alongside your code.

These can be combined into a single command with `cargo insta test --review`.

Snapshots are stored as `.snap` files in a `snapshots/` directory next to your
test code by default.

### Inline snapshots

Insta also supports [inline snapshots][insta-inline], where the reference value
is stored directly in the test source code using a `@"..."` syntax —
`cargo insta review` updates the source file automatically when you accept a
change.

### CI

In CI, you want tests to fail if any snapshot is out of date, without writing
new snapshot files. Setting the `CI` environment variable (which most CI
providers set automatically) enables this behavior. You can also explicitly
control it:

```bash
# fail if any snapshot doesn't match, don't write .snap.new files
INSTA_UPDATE=no cargo test
```

### Testing Command-Line Tools

Insta has an optional extension called [insta-cmd][insta-cmd-docs]
([repo][insta-cmd-repo]) for snapshotting the output of external commands:

```rust
use std::process::Command;
use insta_cmd::assert_cmd_snapshot;

#[test]
fn test_command() {
    assert_cmd_snapshot!(Command::new("echo").arg("hello"));
}
```

## Expect-Test

[`expect-test`][expect-test-docs] ([repo][expect-test-repo]) takes a different
approach: instead of storing snapshots in separate files, it stores them inline
in your test source code. When the output changes, running the tests with
`UPDATE_EXPECT=1` rewrites the expected value in your source file directly.

```rust
use expect_test::expect;

#[test]
fn test_greeting() {
    let actual = greet("World");
    expect![[r#"Hello, World!"#]].assert_eq(&actual);
}
```

This makes expect-test a hybrid between unit testing and snapshot testing: the
expected values live in the test code (like a unit test), but they are
maintained automatically (like a snapshot test). Insta supports a similar
workflow through its [inline snapshots][insta-inline] feature.

## Runt

[Runt][runt-repo] ([docs][runt-docs]) is a tool for snapshot-testing
command-line programs. It implements _transcript tests_: you write a file
containing commands and their expected output, and runt verifies that running
the commands still produces the same output. This is related to snapshot testing
but focuses specifically on testing the external behavior of text-processing
tools.

## Reading

```reading
style: article
title: What if writing tests was a joyful experience?
url: https://blog.janestreet.com/the-joy-of-expect-tests/
author: James Somers
---
Describes how expect tests at Jane Street make testing feel like a REPL
session: developers write minimal test code with blank expect blocks, the
system fills in the actual output, and you accept the diff with a keybinding.
Argues that by removing the friction of writing assertions, expect tests
encourage more comprehensive testing because "by relieving you from having to
dream up exactly what you want to assert, expect tests make it easier to
implicitly assert more."
```

```reading
style: article
title: Try Snapshot Testing for Compilers and Compiler-Like Things
url: https://www.cs.cornell.edu/%7Easampson/blog/turnt.html
author: Adrian Sampson
---
Argues that snapshot testing is ideal for programs that transform text into
other text — compilers, linters, formatters, and similar tools. Introduces
turnt, a minimal snapshot testing tool, and makes the case that prioritizing
easy test creation over precise assertions is a worthwhile tradeoff when human
review of output changes is cheap.
```

```reading
style: article
title: Building Industrial Strength Software without Unit Tests
url: https://chrispenner.ca/posts/transcript-tests
author: Chris Penner
---
Introduces transcript tests: markdown files that document expected behavior
through executable code blocks and their outputs, serving as both tests and
user-facing documentation. The key insight is that testing the external
interface (rather than internal implementation) means refactors don't break
tests unless observable behavior changes, removing a major psychological
barrier to improving code.
```

```reading
style: article
title: Insta - Snapshot Testing for Rust
url: https://blog.bryantluk.com/insta-rs-snapshot-testing-library/
author: Bryant Luk
---
Walkthrough of using insta in a Rust project, highlighting how snapshot
testing speeds up development because code changes don't require manually
fixing test cases — you review snapshot diffs instead. Demonstrates insta's
glob feature for running tests against multiple input files.
```

```reading
style: article
title: Using Insta for Rust snapshot testing
url: https://blog.logrocket.com/using-insta-rust-snapshot-testing/
author: Agustinus Theodorus
---
Step-by-step tutorial showing how to set up insta, write snapshot tests, and
use `cargo-insta` to review and accept changes. Good starting point if you
want a hands-on introduction.
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
