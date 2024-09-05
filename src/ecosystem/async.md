# Concurrency

The biggest question that you need to ask yourself is: do you require the use
of async code? This is typically the case when your application is I/O bound
and wants to process a lot of requests, or when you want to make use of heavy,
lightweight concurrency.

The main difference between async and blocking programming paradigms is the
introduction of *futures*, which represent a computation. While in blocking code,
when you run some code, your thread will do only that:

```rust
std::time::sleep(Duration::from_secs(1));
```

In async code, you split the *definition* of a computation from its *execution*.
Every async function returns a *future* that you need to *await*.

```rust
tokio::time::sleep(Duration::from_secs(1)).await;
```

The advantage of this is that it lets you perform high-level operations on
computations. It lets you compose them. For example, you can execute
multiple futures at once:

```rust
let future_1 = tokio::time::sleep(Duration::from_secs(1));
let future_1 = tokio::time::sleep(Duration::from_secs(1));
futures::join(future_1, future_2).await;
```

You can also wrap your futures into something else, for example adding a timeout
to some computation that will cancel it when the time runs out:

```rust
tokio::time::timeout(Duration::from_secs(1), handle_request()).await;
```

## When should you use async?

When should you consider async code:

- You're writing something that is heavily I/O bound, such as a web server,
  and you want it to be able to scale to a lot of requests and still stay efficient.
- You're writing firmware for a microcontroller, and you want it to perform
  multiple things simultaneously.
- You want to be able to compose computation in a high-level way, for example
  wrapping some computation in a timeout.

When should you stick so synchronous code (see also [When not to use Tokio](https://tokio.rs/tokio/tutorial#when-not-to-use-tokio)):

- You're writing a command-line application that only does one thing.
- You're writing an application that mainly performes computation and not I/O,
  such as cryptographic libraries, data structures.
- Most of the I/O your applications performs is file I/O.

If your crate performs mainly computations, then [Rayon][rayon] is most likely what
you want to use. The Rust standard library also comes with code to let you easily and
safely create and manage threads.

The caveate around file I/O comes from the fact that in many operating systems
there are no asynchronous interfaces for reading from and writing to files.
While there is [a crate that lets Tokio use
`io_uring`](https://github.com/tokio-rs/tokio-uring), this only works on Linux
and is experimental. For that reason, Tokio spawns a dedicated thread for file
I/O and uses blocking calls.

[rayon]: https://docs.rs/rayon/latest/rayon/

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

Although support for async-await style programming was only added in [Rust
1.39](https://blog.rust-lang.org/2019/11/07/Async-await-stable.html), it has
caught on and the Rust community has seen a large number of frameworks being
built for async, and a lot of crates that support it.

![bubble graph of popular async crates](/graphics/crate-popularity-async.svg)

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



[async book]: https://rust-lang.github.io/async-book/
[tokio]: https://tokio.rs/
[smol]: https://github.com/smol-rs/smol
[async-std]: https://async.rs/


## Reading

[Sans-IO](https://www.firezone.dev/blog/sans-io)

*This article expalains an approach to architecting asynchronous applications that stricly
separate IO code from business logic.*

[\Device\Afd, Or the Deal With the Devil that makes async Rust work on
Windows](https://notgull.net/device-afd/)

*That Windows has some odd design choices and cruft is has accumulated over the
years is not news to any developers that have had to interact with it. This
article explains the dark magic that needs to be performed to make async work
on Windows for Rust.*


https://kkourt.io/blog/2017/10-14-linux-aio.html


https://v5.chriskrycho.com/journal/async-rust-complexity/

https://github.com/alexpusch/rust-magic-patterns/blob/master/rust-stream-visualized/Readme.md

https://github.com/jkarneges/rust-async-bench



https://rust-lang.github.io/async-book/

Stats on blocking vs async:

https://emschwartz.me/async-rust-can-be-a-pleasure-to-work-with-without-send-sync-static/
