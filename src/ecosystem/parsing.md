# Parsing

Parsing is a fundamental task in many applications, from configuration files to domain-specific
languages (although, configuration files are better parsed with a deserialization library such as
`serde`).

Rust has several popular parsing libraries, each with different approaches and strengths. This
section covers the most popular parsing libraries in the Rust ecosystem.

In general, if you want to parse binary data, the [`nom`](#nom) crate seems to be the most popular.
For parsing text, both [Chomsky](#chomsky) and [Pest](#pest) are popular. But the choice of parsing
library really boils down to the question of how you want to write your grammar: do you prefer a
declarative approach or a procedural one?

## nom

[nom](https://github.com/rust-bakery/nom) is a parser combinator library that enables you to build
parsers by combining smaller parsing functions. It focuses on performance and low memory usage,
making it ideal for binary formats, network protocols, and other performance-critical parsing tasks.
nom uses macros and functions to create composable parsers and is particularly well-suited for
byte-level parsing where you need fine-grained control over the parsing process.

## Chumsky

[Chumsky](https://github.com/zesterer/chumsky) is a parser combinator library designed with
ergonomics and error recovery in mind. It offers a friendly, declarative API that makes it easy to
build complex parsers. Chumsky excels at parsing programming languages and other text formats where
good error messages are important. It features strong typing, excellent error reporting, and the
ability to create parsers that can recover from errors and continue parsing.

## Pest

[Pest][pest] is a parsing library for Rust that consumes a PEG grammar and generates a parser,
complete with error reporting and recovery. Unlike nom and Chumsky, which build parsers through
code, Pest uses separate grammar files with a specialized syntax similar to other PEG (Parsing
Expression Grammar) tools.

[pest]: https://pest.rs/

## lalrpop

[LALRPOP](https://github.com/lalrpop/lalrpop) is a parser generator that turns grammar files into
Rust code. It uses LR(1) parsing techniques, making it powerful for parsing context-free grammars
like programming languages. LALRPOP is particularly well-suited for creating parsers for
statically-defined languages where you need precise control over grammar precedence and
associativity. It integrates well with the Rust build system through a build script, automatically
regenerating parser code when your grammar changes.

## Reading

```reading
style: book
title: LALRPOP book
url: https://lalrpop.github.io/lalrpop/
author: LALRPOP Team
---
```
