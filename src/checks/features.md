# Crate Features

*Following advice from [Crate Features](../organization/features.md), you have
added optional features into your crate to reduce compilation times for when
they are not required by downstream users. This has been working well, however
in a recent release you have received a bug report what a specific combination
of enabled features triggers a compilation error.  You have fixed the error,
which was introduced by some refactoring that moved a `#[cfg]` block.  However,
you are wondering whether it is possible to catch these kinds of issues
automatically by CI rather than by downstream users.*

<!-- TODO: example of how this happens -->

Similar to using `#ifdef` statements in C and C++, using `#[use]` blocks is
inherently brittle. Using a crate such as [`cfg_if`][cfg_if] instead can help
make it more manageable, but it does not address the root issue: you really
need to test the code you write for all features.

Fortunately, the `cargo-hack` tool allows you to do just that.

## `cargo-hack`

What [`cargo-hack`][cargo-hack] lets you do is run some check (for example
`cargo check` to see if it will compile, or `cargo test` to run unit tests) for
*every possible feature* or even for *every possible set of features*.

To use `cargo-hack`, you need to give it two pieces of information: which
sets of features to test for and which command to run. 

### Feature Sets

For the set of features,
the two popular options are `--each-feature` and `--feature-powerset`. To illustrate
the different, consider if you have a crate with the features `a`, `b`, `c` and `d`.

| Flag | Feature Sets |
| --- | --- |
| `--each-feature` | `a`; `b`; `c` |
| `--feature-powerset` | `a`; `b`; `c`; `a,b`; `a,c`; `b,c`; `a,b,c` |

### Commands

You also need to tell `cargo-hack` what command to run.

| Name | Description |
| --- | --- |
| `check` | Runs `cargo check` for each of the selected feature sets |
| `test` | Runs `cargo test` for each of the selected feature sets |

### Examples

```admonish example
Test
```

[cfg_if]: https://docs.rs/cfg-if/latest/cfg_if/
[cargo-hack]: https://github.com/taiki-e/cargo-hack
