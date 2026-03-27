# Logging

Logging is the process of recording significant events, actions, or errors
within a software system. Typically, it involves recording them in a textual
format as _log messages_, with the ability to designate each at different levels
(such as error, warning, info, debug). This can be used to observe a system
(such as flagging error logs) or to debug issues (such as deducing why a system
is failing from debug or info logs).

There are some additional ways that logging can be implemented:

- **Structured logging** involves adding metadata to log messages, often in the
  form of key-value pairs. This data can be used to filter log messages, for
  example by user or by resource.
- **Tracing** means generating log events to be able to trace the propagation of
  asynchronous tasks. For example, it might mean that the log library issues log
  events whenever an asynchronous task (request handler) is launched, and when
  it is completed, or attach metadata to a log event to be able to trace its
  propagation through several services.

The Rust ecosystem has centred around three crates which are used for logging.
These vary in terms of their intended use-case, and to some extent can even be
mixed through interop libraries. In the next sections, we will discuss each of
them and finally show some ways to mix-and-match them.

For your Rust project, it makes sense to think about what you want out of your
logging system and choose the right kind of logging infrastructure. Part of the
consideration should also be what other libraries or frameworks you are using,
because many of them come with logging support built-in that you can enable. For
example, many of the asynchronous HTTP libraries have built-in support for the
`tracing` crate.

## Log

The `log` crate is the most popular logging infrastructure. It uses the façade
pattern, which decouples the users of the logging facilities (which use the
`log` crate) from the implementation of the logging output (such as
`env_logger`).

Using it is therefore a two-step process:

1. You use the `log` crate in your libraries and binaries, which exposes some
   macros that you can use for emitting log messages, such as `log::info!` or
   `log::error!`.
2. In your binaries, you import and initialize a log handler crate, such as
   `env_logger`. This will subscribe to the logs that are sent to the `log`
   crate, and do with them whatever you configure it to (such as emit them on
   standard output).

The advantage of doing it this way is that the `log` crate itself is very
light-weight and is used in a lot of libraries. It does not pull in any code
related to emitting logs, and it does not prescribe how you output your logs. It
gives binary authors the flexibility to setup their logging subscriber in
whichever way best fits with the application.

````admonish info
The façade pattern is quite common for decoupling generic interfaces (logging, tracing,
metrics collection, randomness generation, hashing) from the actual implementation. You
will see it in multiple places.

In the case of the `log` crate, it is implemented by the log crate having a mutable global
which holds a reference to the currently used logger, and having all of the logging implementations
set this on initialization. This allows for decoupling the logging interface (which can be used
in a lot of crates) from the implementation.

```rust
static mut LOGGER: &dyn Log = &NopLogger;
```

In general, using mutable globals is discouraged. Care must be taken when updating
them from multiple threads, because this can lead to race conditions. However, this façade
pattern is one case where it makes sense. If you want to implement something similar,
you can look into using [OnceLock][], which is thread-safe.

[OnceLock]: https://doc.rust-lang.org/stable/std/sync/struct.OnceLock.html
````

For example, you might have a function like this:

```rust
{{#include ../../examples/log-example/src/lib.rs}}
```

You can use this function after registering your logging implementation, in this
case `env_logger`:

```rust
{{#include ../../examples/log-example/src/main.rs}}
```

When you run this, for example with `cargo run`, then you will see this output
on the console:

```
{{#include ../../examples/log-example/output/output.txt}}
```

Many libraries in the Rust crate ecosystem either use `log`, or have an optional
feature that can be turned on to enable the use of the `log` crate, allowing you
to capture logs from them. Many logging subscribers let you filter not only by
log level, but also by the source. This allows you to filter out logs from other
crates that you are not interested in seeing.

### Logging Backends

The simplest and most popular logging implementation is `env_logger`, which
simply prints log messages to standard error in a structured way. You can find a
full list of logging implementation in the documentation for the `log` crate.
These are some of the popular ones:

| Name                                               | Description                                                                              |
| -------------------------------------------------- | ---------------------------------------------------------------------------------------- |
| [`android_log`][android_log]                       | Log to the Android logging subsystem. Useful when building Android applications in Rust. |
| [`console_log`][console_log]                       | Log to the browser's console. Useful when building WASM applications in Rust.            |
| [`db_logger`][db_logger]                           | Log to a database, supports Postgres and SQLite out-of-the-box.                          |
| [`env_logger`][env_logger]                         | Prints log messages on standard error.                                                   |
| [`logcontrol_log`][logcontrol_log]                 | Control logging settings via DBUS. Does not do logging itself.                           |
| [`syslog`][syslog]                                 | Log to syslog, supports UNIX sockets and TCP/UDP remote servers.                         |
| [`systemd_journal_logger`][systemd_journal_logger] | Log to the systemd journal.                                                              |
| [`win_dbg_logger`][win_dbg_logger]                 | Log to a Windows debugger.                                                               |

[console_log]: https://docs.rs/console_log/latest/console_log/
[logcontrol_log]: https://docs.rs/logcontrol-log/latest/logcontrol_log/
[db_logger]: https://docs.rs/db_logger/latest/db_logger/
[win_dbg_logger]: https://docs.rs/win_dbg_logger/latest/win_dbg_logger/
[android_log]: https://docs.rs/android_log/latest/android_log/
[env_logger]: https://docs.rs/env_logger/latest/env_logger
[syslog]: https://docs.rs/syslog/latest/syslog
[systemd_journal_logger]:
  https://docs.rs/systemd-journal-logger/latest/systemd_journal_logger/

## defmt

When building firmware for embedded applications in Rust, often authors want to
avoid using the Rust built-in formatting system. While the built-in formatting
system is useful, it takes up some code space. On embedded system, code size is
a constrained resource. For that reason, the `defmt` project consists of a
number of crates that allow one to implement logging without making use of
Rust's formatting support, with the goal of producing smaller binaries.

It stands for _deferred formatting_. It supports `println!()`-style formatting,
multiple logging levels and compile-time filtering of logging statements, while
aiming for small binary size. It defers the formatting of log messages, which
means that the formatting itself is done on a secondary machine.

```admonish warning
Unless you know you are targetting an embedded system, it does not make sense
to use the `defmt` crate. You're better off starting with the `log` or `tracing`
crates, which are widely supported in the Rust ecosystem.
```

[The book][defmt-docs] is a good resource to get started with it.

[defmt-repo]: https://github.com/knurling-rs/defmt
[defmt-docs]: https://defmt.ferrous-systems.com/

## Tracing

The `tracing` crate implements scoped structured logging. It is maintained by
the Tokio developers and is commonly used in async projects, as it excels at
tracing asynchronous functions.

Asynchronous code often has a lot of _concurrency_, meaning that multiple
futures are running at the same time. This makes using traditional logging
difficult, because it is hard to associate which log messages are generated by
which asynchronous task.

Tracing improves on this by being _scoped_, this means that tracing events are
associated to the task they belong to. Tracing also often emits events when a
task is started, and when it is finished. This allows for measuring the time it
takes each task to complete (which corresponds to _latency_, if each task
handles a request).

## Slog

[slog](https://github.com/slog-rs/slog) is a logging crate for Rust with the
tagline _structured, contextual, extensible, composable logging for Rust_.

## Interopability

| Crate          | Description                         |
| -------------- | ----------------------------------- |
| `tracing-slog` | `slog` to `tracing`                 |
| `tracing-log`  | `log` to `tracing`                  |
| `slog-stdlog`  | `slog` to `log`, or `log` to `slog` |

## Reading

```reading
style: article
title: Getting started with Tracing
url: https://tokio.rs/tokio/topics/tracing
author: Tokio Project
---
This article explains the basics of the `tracing` crate, and shows how to use
it in an example project.
```

```reading
style: article
title: Crate log documentation
url: https://docs.rs/log/latest/log/
author: log crate authors
---
The documentation of the `log` crate explains what it is, and how to use it.
It also lists popular consumers of the crate.
```

```reading
style: book
title: defmt book
url: https://defmt.ferrous-systems.com/
author: Ferrous Systems
---
This is the book for defmt, a resource constrained logging crate. It explains
the motivation, implementation of the crate and how to get started using it.
```

```reading
style: book
title: Structured logging
url: https://rust-exercises.com/telemetry/01_structured_logging/00_intro
author: Rust telemetry exercises
---
This Rust exercise walks through various logging facilities in the Rust
ecosystem, from using the log crate, to using tracing for asynchronous code, to
collecting metrics and exporting them to Prometheus.
```

```reading
style: article
title: What is the Difference Between Tracing and Logging?
url: https://www.baeldung.com/cs/trace-vs-log
author: Amanda Viescinski
---
This article explains the difference between logging and tracing.
```

https://lpalmieri.com/posts/2020-09-27-zero-to-production-4-are-we-observable-yet
