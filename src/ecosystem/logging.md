# Logging

Logging is the process of recording significant events, actions, or errors
within a software system. Typically, it involves recording them in a textual
format as _log messages_, with the ability to designate each at different levels
(such as error, warning, info, debug). This can be used to observe a system
(such as flagging error logs) or to debug issues (such as deducing why a system
is failing from debug or info logs).

Beyond plain text messages, _structured logging_ adds metadata as key-value
pairs (user ID, request ID, resource name) that can be used to filter and
correlate log entries. _Tracing_ goes further by associating log events with
scoped spans that track the lifetime of operations — when a request handler
starts, what database queries it makes, and when it finishes. This is
particularly useful in async code where many tasks are interleaved on the same
thread.

The Rust ecosystem has three main logging crates: `log`, `tracing`, and `slog`.
They can be mixed through interop libraries, so choosing one does not lock you
out of the others. Your choice depends on what you need: `log` for simple text
logging, `tracing` for async applications that need scoped structured logging,
and `slog` for structured logging in synchronous code. Many libraries and
frameworks (especially async HTTP frameworks) have built-in support for
`tracing`. If you are writing code for an embedded platform where code size
matters, `defmt` is your friend.

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

The [`tracing`](https://docs.rs/tracing/latest/tracing/) crate implements
scoped, structured logging. It is maintained by the Tokio project and is the
standard choice for async Rust applications. Like `log`, it uses a facade
pattern: `tracing` defines the API, and a separate subscriber (typically
[`tracing-subscriber`](https://docs.rs/tracing-subscriber/latest/tracing_subscriber/))
handles the output.

The key concept in `tracing` is the _span_. A span represents a period of time
during which some operation is happening. Events (log messages) that occur
inside a span are associated with it, so you can see which request or task
produced which log output — even when many tasks are interleaved on the same
thread. Spans can be nested, forming a tree that mirrors the call structure of
your application.

The `#[instrument]` attribute macro is the most common way to create spans. It
automatically creates a span for a function, recording its arguments:

```rust
use tracing::{info, instrument};

#[instrument]
async fn handle_request(user_id: u64) {
    info!("processing request");
    let data = fetch_data(user_id).await;
    info!("request complete");
}
```

Every event inside `handle_request` will be associated with a span that includes
the `user_id`, making it easy to filter logs for a specific user even when
hundreds of requests are being handled concurrently.

To see any output, you need to register a subscriber. A minimal setup using
`tracing-subscriber`:

```rust
fn main() {
    tracing_subscriber::fmt::init();
    // ...
}
```

`tracing-subscriber` supports filtering by level and module (similar to
`env_logger`), JSON output for log aggregation services, and composing multiple
layers (for example, logging to both stdout and a file). For production
services, JSON output combined with a log aggregation system (ELK, Loki,
Datadog) is a common pattern.

## Slog

[`slog`](https://github.com/slog-rs/slog) is a structured logging framework that
predates `tracing`. It is built around the concept of _drains_ (output
destinations) that can be composed: you can chain a JSON formatter, an async
buffer, and a file writer together. Like `tracing`, it supports structured
key-value data on log messages, and like `log`, it works well in synchronous
code.

Slog's ecosystem includes separate crates for different output formats and
destinations: `slog-term` for terminal output, `slog-json` for JSON,
`slog-async` for non-blocking logging, and many others. Loggers in slog carry
context, so you can create child loggers with additional key-value pairs that
are automatically included in all messages from that logger — useful for tagging
all logs within a request handler with a request ID.

The slog maintainers acknowledge that `tracing` has become the default choice
for async Rust, but note that slog remains a stable, battle-tested library that
is preferable when async support is not needed or when you want finer control
over the logging pipeline.

## Interoperability

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
Official Tokio guide to the `tracing` crate. Covers creating spans,
recording events, using the `#[instrument]` macro, and setting up
`tracing-subscriber` with filtering. Good starting point if you are adding
tracing to a Tokio-based project.
```

```reading
style: article
title: Crate log documentation
url: https://docs.rs/log/latest/log/
author: log crate authors
---
API documentation for the `log` crate. Besides the API reference, the
overview section explains the facade pattern, how to choose a logging
implementation, and lists popular backends and consumers. The "compile time
filters" section is useful if you want to strip debug logs from release
builds.
```

```reading
style: book
title: defmt book
url: https://defmt.ferrous-systems.com/
author: Ferrous Systems
---
Guide to the `defmt` crate for resource-constrained embedded logging.
Explains how deferred formatting works (binary encoding on the device, text
formatting on the host), how to set up `defmt` with `probe-rs`, and how to
use its `println!`-style macros with compile-time log level filtering.
```

```reading
style: book
title: Structured logging
url: https://rust-exercises.com/telemetry/01_structured_logging/00_intro
author: Rust telemetry exercises
---
Hands-on exercise that progresses from basic `log` usage through `tracing`
with structured data, to collecting and exporting metrics to Prometheus.
Teaches by building a real telemetry pipeline incrementally.
```

```reading
style: article
title: What is the Difference Between Tracing and Logging?
url: https://www.baeldung.com/cs/trace-vs-log
author: Amanda Viescinski
---
Explains the conceptual difference between logging (recording discrete events)
and tracing (following the path of a request through a system). Not
Rust-specific, but useful background for understanding why the `tracing` crate
exists alongside the `log` crate.
```

```reading
style: article
title: "Are we observable yet? — Zero to Production #4"
url: https://lpalmieri.com/posts/2020-09-27-zero-to-production-4-are-we-observable-yet
author: Luca Palmieri
---
Walks through building an observable Rust web service from scratch. Starts
with basic `log` + `env_logger`, then migrates to `tracing` with
`tracing-subscriber` and `tracing-bunyan-formatter` for JSON output. Shows
how to use `#[tracing::instrument]` to reduce boilerplate, how to protect
sensitive data with `secrecy::Secret`, and how to configure logging differently
for tests vs production.
```
