# Metrics

Metrics is the process of collecting numerical data from deployed software to measure how well it
performs, to count how many errors occur, and to track the performance of individual components. You
can use them to monitor the health of a system, identify performance bottlenecks, and detect
anomalies. Sometimes you just want to plot that data so you have pretty graphs to look at. Other
times you want to watch them closely, as you deploy a new version of a service, and you want to see
if your latency or error rate increases.

Generally, you could say that metrics collection is, along with tracing, part of the observability
stack, which allows you to gain insight over how a system is performing.

Whatever your reason is for collecting metrics, there are some crates in Rust that can help you
achieve this. Before we look into them, let's take a look at an overview of how metrics collection
usually works, and what you usually do with that data.

### Metrics collection

Metrics collection usually involves instrumenting your code to record data about the execution of
your program. Data is collected by counters, gauges, histograms, and summaries inside your code.
Just to make sure this terminology is clear:

| Name      | Description                                                                                                                                                                                     |
| --------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Counter   | Counts occurrences of something happen. For example, you might have a counter that you increment each time a request is served.                                                                 |
| Gauge     | Represents a single numerical value that can go up or down. For example, you might have a gauge that represents the current number of active users.                                             |
| Histogram | Measures the distribution of values in a stream of data. For example, you might have a histogram that measures the response time of your service.                                               |
| Summary   | Measures the distribution of values in a stream of data, but with a different approach than a histogram. For example, you might have a summary that measures the response time of your service. |

These metrics are then collected by a metrics aggregator, which can be a standalone service or part
of a larger monitoring system. A common example of an aggregator is [Prometheus][prometheus]. We'll
explain it later, but Prometheus will send a request to your service periodically to collect the
metrics. There are aggregators that work in a different way, for example such that you send them, or
you could even store them directly in a database.

```admonish info
Prometheus is more than just a metrics aggregator. It is a time-series database,
in other words a database that is optimized for storing and querying data which
is indexed by time. It can store such data more efficiently than a traditional
database. That is why it is commonly used for monitoring and alerting purposes.
```

Once the metrics are collected, they can be used for various purposes, such as monitoring the health
of a system, identifying performance bottlenecks, and detecting anomalies. They can also be used to
generate alerts, trigger automated actions, or provide insights into the behavior of your system. As
I mentioned earlier, you can also just use [Grafana][grafana] to plot them.

[grafana]: https://grafana.com/
[prometheus]: https://prometheus.io/docs/introduction/overview/

<!-- add grafana plot -->

If you just want simple metrics, the [metrics](#metrics-1) crate is a good bet. If you want a more
integrated solution, you can use [OpenTelemetry](#opentelemetry), which comes at a cost of
complexity but has more features (such as including tracing). If you just want to use Prometheus,
the [prometheus](#prometheus) crate is a good bet.

## Metrics

The [metrics crate][metrics-rs] ([website][metrics-web]) is a light-weight crate that has
implementations for various metrics counters and aggregations. It is designed to be used by your
application to record metrics, and have another crate to export them to a monitoring system. It
works similar to the `log` crate with a façade pattern, allowing you to use any metrics
implementation you want.

For the backend, it supports two implementations: Prometheus and TCP.

| Backend                                   | Description                    |
| ----------------------------------------- | ------------------------------ |
| [Prometheus][metrics-exporter-prometheus] | Exports metrics to prometheus  |
| [TCP][metrics-exporter-tcp]               | Streams metrics out over TCP   |
| [DogStatsD][metrics-exporter-dogstatsd]   | Sends metrics to Datadog       |
| [HTTP][metrics-exporter-http]             | Sends metrics to HTTP endpoint |

[metrics-exporter-http]: https://lib.rs/crates/metrics-exporter-http
[metrics-exporter-dogstatsd]: https://lib.rs/crates/metrics-exporter-dogstatsd
[metrics-exporter-tcp]: https://docs.rs/metrics-exporter-tcp/latest/metrics_exporter_tcp/
[metrics-exporter-prometheus]:
  https://docs.rs/metrics-exporter-prometheus/latest/metrics_exporter_prometheus/
[metrics-axum-prometheus]: https://docs.rs/axum-prometheus/latest/axum_prometheus/
[metrics-rs]: https://github.com/metrics-rs/metrics
[metrics-web]: https://metrics.rs/

There are also some crates that allow bridging the `metrics` facade with other libraries:

- [metrics-prometheus](https://lib.rs/crates/metrics-prometheus) allows you to bridge the
  `prometheus` crate with the `metrics` facade. This can be useful if you use some Rust libraries
  that use the `prometheus` crate internally, but you want to expose metrics using the `metrics`
  facade.

### Examples

```rust
use metrics::{counter, histogram};

pub fn process(query: &str) -> u64 {
    let start = Instant::now();
    let row_count = run_query(query);
    let delta = start.elapsed();

    histogram!("process.query_time").record(delta);
    counter!("process.query_row_count").increment(row_count);

    row_count
}
```

## OpenTelemetry

[OpenTelemetry][opentelemetry-docs] is an observability framework designed for collecting,
processing, and exporting telemetry data—such as traces, metrics, and logs—from applications. It is
a standard that works across programming languages and framworks

OpenTelemetry has a [Rust crate][OpenTelemetry-rust], that you can use to export data to
OTel-compatible observability systems.

[opentelemetry-docs]: https://opentelemetry.io/docs/
[opentelemetry-rust]: https://github.com/open-telemetry/opentelemetry-rust

## Prometheus

There is a Rust crate for Prometheus called [prometheus][prometheus-rs]. It has built-in primitives
for creating counters, gauges, histograms, and summaries. You declare them as global variables and
use them to track performance data, and you define and endpoint that Prometheus can scrape
periodically.

### Examples

With the `prometheus` crate, you can define your metrics and register them with a registry. If you
don't specify a specific one, it will register them with a default, global registry. When you want
to expose them to Prometheus, you can use the `prometheus::gather()` function to gather all the
metrics and then encode them using the `TextEncoder` struct.

<!-- TODO: turn this into proper example. -->

```rust
use prometheus::{self, IntCounter, TextEncoder, Encoder, register_int_counter};
use lazy_static::lazy_static;

lazy_static! {
    static ref HIGH_FIVE_COUNTER: IntCounter =
        register_int_counter!("highfives", "Number of high fives received").unwrap();
}

HIGH_FIVE_COUNTER.inc();
assert_eq!(HIGH_FIVE_COUNTER.get(), 1);

let mut buffer = Vec::new();
let encoder = TextEncoder::new();

// Gather the metrics.
let metric_families = prometheus::gather();
// Encode them to send.
encoder.encode(&metric_families, &mut buffer).unwrap();

let output = String::from_utf8(buffer.clone()).unwrap();
const EXPECTED_OUTPUT: &'static str = "# HELP highfives Number of high fives received\n# TYPE highfives counter\nhighfives 1\n";
assert!(output.starts_with(EXPECTED_OUTPUT));
```

[prometheus-rs]: https://docs.rs/prometheus/latest/prometheus/

## Autometrics

https://crates.io/crates/autometrics/2.0.0

## Reading

```reading
title: How to setup and use metrics in rust
author: Hamza Khchichine
url: https://www.hamzak.xyz/blog-posts/how-to-setup-and-use-metrics-in-rust
style: article
archived: hamza-setup-use-metrics.pdf
---
Todo
```

```reading
style: article
title: OpenTelemetry, time series, metrics and a bit of Rust
url: https://www.elias.sh/posts/opentelemetry_timeseries_metrics_and_a_bit_of_rust
author: Elias Granja
archived: elias-telemetry-metrics.pdf
---
Todo
```
