# Memory Usage

Generally, Rust programs have three kinds of memory:

- **Static memory**: Allocated at program startup, fixed size. Used for global statics.
- **Stack**: Allocated at program startup, fixed size. Used to store local variables in function calls.
- **Heap**: Allocated dynamically during program execution.

It is not too difficult to estimate static memory and stack memory, because you
can measure the sizes of the types stored in them, for example using
[`std::mem::size_of()`][size_of].  However, how do you measure memory that is
allocated dynamically? You might want to do this because you want to evaluate
different data structures, or you want to evaluate the impact on memory
usage a code change has.

To do this, you can use some tools that measure externally. For example,
Valgrind and its [Dynamic Heap Analysis Tool][dhat] let you capture all
allocations, and later examine them to see where they came from, and which code
accessed the memory.

Another strategy is to measure *internally*. This relies on the fact that Rust
allows you to override the global memory allocator that is used by implementing
[`std::alloc::GlobalAlloc`][global-alloc]. By implementing this trait and
setting it as the `#[global_allocator]`, you can intercept allocation and
deallocation requests.

There are some libraries that have helpers to let you do this. This section
discusses how they work and how they can help you.

## DHAT

[dhat-rs](https://docs.rs/dhat/0.3.3/dhat/) tries to achieve the same
functionality as Valgrind's DHAT.

### Examples

## Tracing Allocator

[tracking-allocator](https://docs.rs/tracking-allocator/latest/tracking_allocator/)
is a replacement allocator that allows you to implement tracing hooks to count
memory usage.  It does not perform the actual allocation, this is deferred to
the system allocator.  But it does allow you to measure the peak memory usage
of code sections.

### Examples

## Reading

[Heap Allocations](https://nnethercote.github.io/perf-book/heap-allocations.html) in The Rust Performance Books

*In this chapter, strategies for profiling and optimizing heap memory usage
is discussed.*

[Allocator Designs](https://os.phil-opp.com/allocator-designs/) by Philipp Oppermann

*Philipp explains different designs of allocators, and shows you how you can
implement them in Rust. This is good background knowledge to have if you want
to learn more about how allocators work and how they track and manage
allocations. It can also be useful if you want fine-grained control over how
memory is allocated, for example if you want to use an arena-style allocator
for a specific data structure.*

[size_of]: https://doc.rust-lang.org/stable/std/mem/fn.size_of.html
[dhat]: https://valgrind.org/docs/manual/dh-manual.html
[global-alloc]: https://doc.rust-lang.org/std/alloc/trait.GlobalAlloc.html
