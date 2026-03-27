# Checks

The Rust compiler is very good at finding bugs in the code, thanks to the type
system and the borrow checker. However, there are some other things in a Rust
project that can be checked continuously to avoid writing broken code.

This chapter deals with some other aspects of Rust projects that should be
checked, and offers some tooling that can be used to check them. It gives you
suggestions that you can adopt in your workflows or CI jobs to give you
confidence that your project is correct.

Not all of these checks might be interesting or relevant to you. You can use
your own judgement of which checks you find are valuable and which ones are not
worth adopting.

For every check, you need to decide what the process is. Is it something that
you want your developers to be able to run locally? If so, you need to give them
instructions on how to install the necessary tooling locally, and how to ensure
that they are all using the same version. Is it something you want to run in the
CI, or periodically? At the end of this chapter, I provide an overview of each
of the tools discussed, and how I would apply them.

_This chapter includes sections that show you how to check properties of your
entire project, rather than just your Rust code._

## Reading
