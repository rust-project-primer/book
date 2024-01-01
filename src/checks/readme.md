# Checks

The Rust compiler is very good at finding bugs in the code, thanks to the type system and
the borrow checker. However, there is some other things that need to be checked continuously
in a project to avoid writing incorrect code.

This chapter deals with some other aspects of Rust projects that should be checked, and
offers some tooling that can be used to check them.

In general, all of these checks should run in a CI system to prevent invalid code
from being merged in the first place. However, these checks should also be available
to run locally to allow for a smooth developer experience.
