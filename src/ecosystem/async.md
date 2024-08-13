# Async

The biggest question that you need to ask yourself is: do you require the use
of async code? This is typically the case when your application is I/O bound
and wants to process a lot of requests, or when you want to make use of heavy,
lightweight concurrency.

## What even is async?

In short, async programming is a paradigm that lets you write scalable applications
that have to do a lot of waiting.

If you have some code that is computation-heavy, it will generally not do a lot waiting
but rather utilise the CPU efficiently. It might look something like this:

- graphic of compute-bound thread

However, if you think of a typical request handler, it involves a lot of waiting.
It has to accept requests, parse headers, wait for the full request, make some queries
to the database, and finally send a response. In terms of CPU utilisation, it means
that it will spend the majority of it's time waiting for things from the network
(from client or responses from the database).

- graphic of network request thread

In traditional applications, you would spawn a thread per connection. Waiting for
responses would be handled by the kernel, which would schedule other threads to run
while it is waiting. However, the issue with this approach is that switching between
threads is a relatively expensive operation, so this approach does not scale well.
This means you can run into the [C10k problem](https://en.wikipedia.org/wiki/C10k_problem).

A better approach here is an [event-driven one](https://en.wikipedia.org/wiki/Event-driven_programming), where you handle multiple connections in a single thread, asking the operating system
to notify you if any of them can progress. This lets you use a [thread-per-core](https://without.boats/blog/thread-per-core/) model, where you don't spawn one thread per request, but you spawn
as many threads as you have CPU cores, and distribute the requests amongs them.

- graphic of thread-per-core

If you were to implement this in C, you would be using an event-loop library like [libuv](https://libuv.org/), which lets you register callbacks when certain operations complete. In Rust, the
async runtimes handle all of this for you, letting you write your code "as if" it were
running on a thread by itself.

The async runtimes have wrappers for any operations that are "blocking", meaning that
they cause your thread to stall until some event happens. Examples of these are:

- Waiting for new network connection to come in
- Waiting for data on a network connection (receiving or sending)
- Waiting for a write or a read from disk
- Waiting for a timer to expire



## What runtimes are recommended?

In general, there are three runtimes that are recommended:

- [Tokio][tokio] is the go-to runtime for all things async. With over 200
  million downloads, it is by far the most popular. At the time of writing, 20,000 crates depend on it (as of August 2024), meaning that it enjoys a very broad support by other libraries.
- [Smol][smol] is a small and fast async runtime. It does not have the same
  amount of features as Tokio does, but due to it's simplicity it is good for resource-constrained
  environments or if you want to be able to understand all of the code.
- [async-std][] is the main competitor to Tokio. It was a project that aimed at making writing
  async code as simple as using the standard library. It is not as actively developed
  as Tokio, and in general is not recommended to be used for new projects. A lot of
  the ideas it came with have been incorporated into Tokio.

## How does aysnc work in Rust?

There is the [Async Book][async book] that goes into much greater depth. But in
general, async in Rust 



## What are some common pitfalls with async in Rust?

Function coloring: design "sync core, async shell"

https://www.thecodedmessage.com/posts/async-colors/

## Summary

When should you stick so synchronous code:

- You're writing a command-line application that only does one thing.
- You're writing an application that performs computation, such as cryptographic
  library.

When should you consider async code:

- You're writing something that is heavily I/O bound, such as a web server.
- You're writing firmware for a microcontroller, and you want it to perform
  multiple things simultaneously.




[async book]: https://rust-lang.github.io/async-book/
[tokio]: https://tokio.rs/
[smol]: https://github.com/smol-rs/smol
[async-std]: https://async.rs/




https://kkourt.io/blog/2017/10-14-linux-aio.html
