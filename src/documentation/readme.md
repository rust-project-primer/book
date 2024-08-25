# Documentation

If writing software was just about telling the machine what to do, then we
would be writing code in assembly. But writing software is as much
communicating to *others* as it is communicating with the machine we expect it
to run in.

Documentation is essential for any project. It makes it easy to onboard people
onto projects, and generally prevents projects from turning into a mess. 
Having documentation about the software architecture makes it easier to reason
about it when things don't work, and it allows non-technical people to understand
how the system works on a high level without needing to understand the code.

For this reason, you see a lot of Rust project having two kinds of documentation:

- **Code-level documentation**. This is most commonly implemented using
  Rustdoc, is quite technical and is targetted mostly at other developers or
  users of libraries.
- **High-level documentation**. This is most commonly implemented using mdBook,
  is less technical and is targetted at users and other developers for getting
  an overview.

In this chapter, I will explain how both of these documentation strategies can
be employed effectively in a Rust project.

## Reading

[Documentation](https://abseil.io/resources/swe-book/html/ch10.html) in Software Engineering at Google

*Tom explains why documentation is needed for software projects to scale,
because they communicate important information about how things work and why
they work the way they do. They save valuable engineering time by giving
engineers access to the information they need quickly, without needing to look
into the code.  He explains what good documentation looks like, and what Google
does to keep it accurate and of high quality.*
