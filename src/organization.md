# Organization

Rust has a number of ways that code can be structured. It comes with files,
modules, crates and workspaces. This chapter attempts to give you some advice
for how to use these to achieve what you want.

Generally, there are two things that this chapter tries to optimize for:
*development speed* and *loose coupling*.

## Development Speed

One of the features that Rust really tries to push is something called *zero-cost
abstractions*. These are abstractions that are useful to a programmer, but have 
no cost at runtime. However, they may have a cost at compile time.

The consequence of that is that Rust code is often optimized to execute fast,
but not necessarily be fast to compile.

However, fast compile times are still important, to some degree: they allow for
a faster development loop.

There are some choices that can be made when setting up a Rust project that 
can allow for a faster compile time. This chapter attempts to explain what these
choices are and their implications.

## Loose Coupling

Nobody wants to work in a giant, monolithic application
that is tightly coupled and complex to change. Ideally, we want to make it as
easy as possible to produce code that is composed of small units which can be
tested individually.
