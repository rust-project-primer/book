# Testing

All of the software that I love using has one thing in common: it is thoroughly
and carefully tested. In my opinion, testing has three purposes:

1. Ensure correctness of a project's code over time.
2. Make it possible to make nontrivial changes in a large, potentially unfamiliar codebase.
3. Reduce the iteration time by noticing issues at test time rather than at deploy time.

Rust recognizes the importance of testing, and as such has several facilities
built-in for writing unit tests as part of the code base.

## Reading

- [How SQLite is tested](https://www.sqlite.org/testing.html)[^1]
- [Chapter 11: Writing automated tests](https://doc.rust-lang.org/book/ch11-00-testing.html) in The Rust Book
- [cargo-nextest book](https://nexte.st/index.html)


[^1]: SQLite is the most-deployed database. You likely have hundreds of SQLite databases in your phone. 
