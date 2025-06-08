# Logging

Logging is the process of recording significant events, actions, or errors
within a software system.  Typically, it involves recording them in a textual
format as *log messages*, with the ability to designate each at different
levels (such as error, warning, info, debug). This can be used to observe a
system (such as flagging error logs) or to debug issues (such as deducing why a
system is failing from debug or info logs).

There are some additional ways that logging can be implemented:

- **Structured logging** involves adding metadata to log messages, often in the
  form of key-value pairs. This data can be used to filter log messages, for
  example by user or by resource.
- **Tracing** means generating log events to be able to trace the propagation
  of asynchronous tasks. For example, it might mean that the log library issues log
  events whenever an asynchronous task (request handler) is launched, and when it is
  completed, or attach metadata to a log event to be able to trace its propagation
  through several services.

The Rust ecosystem has centred around three crates which are used for logging.
These vary in terms of their intended use-case, and to some extend can even
be mixed through interop libraries. In the next sections, we will discuss each
of them and finally show some ways to mix-and-match them.

For your Rust project, it makes sense to think about what you want our of your logging
system and choose the right kind of logging infrastructure. Part of the consideration
should also be what other libraries or frameworks you are using, because many of them
come with logging support built-in that you can enable. For example, many of the
asynchronous HTTP libraries have built-in support for the `tracing` crate.

## Log

The `log` crate is the most popular logging infrastructure.

## Tracing

The `tracing` crate implements scoped structured logging. It is maintained by
the Tokio developers and is commonly used in async projects, as it excels at
tracing asynchronous functions.

## Slog

[slog](https://github.com/slog-rs/slog) is a logging crate for Rust with the tagline
*structured, contextual, extensible, composable logging for Rust*.

## Metrics

## Interopability

| Crate | Description |
| --- | --- |
| `tracing-slog` | `slog` to `tracing` |
| `tracing-log` | `log` to `tracing` |
| `slog-stdlog` | `slog` to `log`, or `log` to `slog` |

## Reading

~~~reading
style: book
title: Structured logging
url: https://rust-exercises.com/telemetry/01_structured_logging/00_intro
author: Rust telemetry exercises
---
This Rust exercise walks through various logging facilities in the Rust
ecosystem, from using the log crate, to using tracing for asynchronous code, to
collecting metrics and exporting them to Prometheus.
~~~

~~~reading
style: article
title: What is the Difference Between Tracing and Logging?
url: https://www.baeldung.com/cs/trace-vs-log
author: Amanda Viescinski
---
This article explains the difference between logging and tracing.
~~~
