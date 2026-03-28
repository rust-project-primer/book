# Preface

This book exists because learning Rust and learning how to run a Rust project
are two different things. There are excellent resources for the language itself:
The Rust Book, Rust by Example, to name just a few. But once you know the syntax
and the borrow checker, you still need to figure out how to structure a
codebase, set up CI, manage dependencies across a workspace, write tests that
actually catch bugs, and ship the result to users. There are established
patterns in the Rust community, but that knowledge is scattered across blog
posts, README files, and tribal experience. I wrote this book to bring it
together in one place.

The book assumes you already know Rust. It is not a language tutorial. Instead,
it focuses on the practical side of running a Rust project: how to organize
code, which tools to use for formatting, linting, testing, and benchmarking, how
to set up continuous integration, how to document and release your work, and how
to make good choices when the ecosystem gives you several options. Where
possible, I try to explain the tradeoffs rather than just prescribing a single
answer.

Much of what is in here comes from years of working with Rust professionally and
learning from open-source projects, and from studying how well-run projects in
the ecosystem handle the same problems. I have tried to make it
information-dense and practical: something you can read through once to get the
lay of the land, and come back to later when you need to set up a specific tool
or make a specific decision.

Rust is not perfect, but it is a language that rewards investment. The tooling
is good, the ecosystem is maturing rapidly, and the community cares deeply about
quality. I hope this book helps you build on that foundation, and helps give you
the tools you need to structure your project well, so that you can focus on
writing great code and get the most out of the ecosystem.
