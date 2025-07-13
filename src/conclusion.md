# Conclusion

Rust is an exciting programming language. The language is unique in that it
shifts responsibility for certain correctness principles, such as memory safety,
from the developers and maintainers to the compiler. In the long term, it is
cheaper and more scalable to have this correctness validated by a machine than
by a programmer.

The same principle applies to the tooling which the Rust ecosystem has come up
with. The tools discussed in this book allow one to shift responsibility of
certain project-level correctness principles from the developers and maintainers
of Rust projects to machines. These principles include correct versioning,
correct code, comprehensively tested code, correct use of features, and many
more.

In my opinion, software development can only be sustainable and scale if we can
automate the boring parts. I hope that this book does a good job of teaching you
just how to do that, in the context of working on Rust software projects.

[Ready-to-go CI pipelines](https://www.lpalmieri.com/posts/2020-06-06-zero-to-production-1-setup-toolchain-ides-ci/#6-2-ready-to-go-ci-pipelines)
