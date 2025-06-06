# Concurrency

One of Rust's themes is [fearless
concurrency](https://blog.rust-lang.org/2015/04/10/Fearless-Concurrency.html),
and due to the focus on this, Rust has many safeguards built-in to the language
that enable you to easily write correct concurrent (and parallel) code.
Because of these safeguards, Rust is one of the most pleasant languages to
write heavily concurrent (and parallel) code in. In this section, we will
discuss some high-level concepts, strategies and libraries that you can use in
your code to make use of this capability. Some of these involve choices that
you have to make which affect how you should structure your project.

Before we launch into this section, we should clarify what *concurrency*
and *parallelism* actually mean. 

- **Concurrency** is your program's ability to track and execute multiple things
  at the same time, but not neccessarily *in parallel*. One example is a single-threaded
  asynchronous runtime, which can execute multiple futures by switching
  between them.
- **Parallelism** is when your program executes multiple tasks at the same time,
  for example using a multi-threaded model. It implies concurrency.

There are different methods in Rust to write concurrent or parallel
programs, depending on the kind of workload you have. Your choice of these
impacts the shape of the Rust code you write, so it is important to figure
out which model suits your particular project. However, it is possible, to 
some extent, to mix the two models.

The building blocks that Rust give you to write concurrent applications are:

- Multi-threading with synchronous code
- Asynchronous concurrency or parallelism

### Primer on Multithreading


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

## Async

### When should you use async?

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

### What even is async?

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

### What runtimes are recommended?

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

### Thread-per-Core vs Shared-Nothing

### Epoll vs io_uring

### How does aysnc work in Rust?

There is the [Async Book][async book] that goes into much greater depth. But in
general, async in Rust 

### What are some common pitfalls with async in Rust?

Function coloring: design "sync core, async shell"

https://www.thecodedmessage.com/posts/async-colors/



[async book]: https://rust-lang.github.io/async-book/
[tokio]: https://tokio.rs/
[smol]: https://github.com/smol-rs/smol
[async-std]: https://async.rs/


## Reading

[Why Async Rust](https://without.boats/blog/why-async-rust/) by David Lee Aronson

*In this article, David explains the history of the development of async Rust.*

[Sans-IO](https://www.firezone.dev/blog/sans-io)

*This article expalains an approach to architecting asynchronous applications
that stricly separate IO code from business logic. This concept helps you
design applications that can be easily tested, but can run with an asynchronous
executor. While this article is written with Python in mind, the lessons are
equally valid for Rust: good software design keeps a synchronous core (without
I/O) and wraps it in a thin, asynchronous shell. That way, your business logic
is decoupled from your runtime strategy.*

[\Device\Afd, Or the Deal With the Devil that makes async Rust work on
Windows](https://notgull.net/device-afd/)

*That Windows has some odd design choices and cruft is has accumulated over the
years is not news to any developers that have had to interact with it. This
article explains the dark magic that needs to be performed to make async work
on Windows for Rust.*

[Thread-per-core](https://without.boats/blog/thread-per-core/) by David Lee Aronson

Todo

[Linux AIO](https://kkourt.io/blog/2017/10-14-linux-aio.html)


[Async Rust Complexity](https://v5.chriskrycho.com/journal/async-rust-complexity/) by Chris Krycho

*Chris argues that one of the reasons why doing async is difficult in Rust is
because of the sheer amount of choice. Various async runtimes and libraries
exist, and for a beginner it is difficult to pick one without investigating all
of the options. This is less true today, as most of the Rust community has
centered around the Tokio ecosystem for async.*

[Rust Stream Visualized](https://github.com/alexpusch/rust-magic-patterns/blob/master/rust-stream-visualized/Readme.md) by Alex Pushinsky

*Visually explains how the Rust asynx stream API works, using diagrams to
illustrate the behaviour.*

[Rust Async Bench](https://github.com/jkarneges/rust-async-bench)



[Async Book](https://rust-lang.github.io/async-book/)

Stats on blocking vs async:

[Async Rust can be a pleasure to work with without Send, Sync and 'static](https://emschwartz.me/async-rust-can-be-a-pleasure-to-work-with-without-send-sync-static/)

[How to deadlock a Tokio application in Rust with just a single Mutex](https://turso.tech/blog/how-to-deadlock-tokio-application-in-rust-with-just-a-single-mutex)

[Asynchronous I/O: The next billion dollar mistake?](https://yorickpeterse.com/articles/asynchronous-io-the-next-billion-dollar-mistake/)

[Measuring Context switching and memory overheads for Linux threads](https://eli.thegreenplace.net/2018/measuring-context-switching-and-memory-overheads-for-linux-threads/) by Eli Bendersky

*Eli measures the overhead of using threads in Linux. While Linux threads have
a relatively low overhead, the requirement to do a context switch to switch
between threads has a mimimum overhead of about 1.2 to 1.5 µs when using CPU
core pinning, and 2.2 µs without.  This limits how many requests can be served
when using a thread-per-request architecture.*

[Confusing or misunderstood topics in systems programming: Part 0](https://pthorpe92.dev/programming/systems/common-misunderstandings) by Preston Thorpe

*Preston explains processes, threads, context switches and communication
between threads. This article provides a good background explainer to be able
to understand how asynchronouns programming works behind the scenes.*

[Rust Tokio task cancellations patterns](https://cybernetist.com/2024/04/19/rust-tokio-task-cancellation-patterns/) by Milos Gajdos

*In this article, Milos explains different patterns used in asynchronous, Tokio-powered
Rust software to cancel tasks.*

[Async-Task explained](https://notgull.net/async-task-explained-part1/) John Nunley

*John explains the internals of the `async-task` crate from the grounds up in
this article. It gives a good background on how async works behind the scenes.*

[Async Rust in Three Parts](https://jacko.io/async_intro.html) by Jack O'Connor

[Async Rust is not safe with io_uring](https://tonbo.io/blog/async-rust-is-not-safe-with-io-uring)

[Notes on io_uring](https://without.boats/blog/io-uring/)

[Waiting for many things at once with io_uring](https://mazzo.li/posts/uring-multiplex.html)

[Threads beat async/await](https://lucumr.pocoo.org/2024/11/18/threads-beat-async-await/)

https://trouble.mataroa.blog/blog/asyncawait-is-real-and-can-hurt-you/

[Async: What is blocking?](https://ryhl.io/blog/async-what-is-blocking/)

[Async Rust is about concurrency, not (just) performance](https://kobzol.github.io/rust/2025/01/15/async-rust-is-about-concurrency.html) by Jakub Beránek

*Jacub argues that the primary benefit of async/await is that it lets us
concisely express complex concurrency; any (potential) performance improvements
are just a second-order effect. He suggests that we should thus judge async
primarily based on how it simplifies our code, not how (or if) it makes the
code faster.*
