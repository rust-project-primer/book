# Organization

This chapter gives some advice on how to best organize code.

What I try to optimize for is:

- **Development speed**. Having an organization that fits well with how Cargo
  expects projects to look like allows for faster (re-)compilation and thereby
  faster iteration times.
- **Loose coupling**. Nobody wants to work in a giant, monolithic application
  that is tighly coupled and complex to change. Ideally, we want to make it as
  easy as possible to produce code that is composed of small units which can be
  tested individually.
