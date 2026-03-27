# Web Backend

A common use-case of Rust is building backends for web applications. Rust is
particularily suited for this, because it offers great performance and a strong
async ecosystem that allows you to scale to many concurrent requests easily.

While you can build a web backend manually by using crates such as [hyper][] for
HTTP and [h3][] for HTTP/3, generally you will want to use a framework to
implement the backend. Web backend frameworks handle things such as request
routing, route authentication, parameter deserialization and building responses
for you to make sure your application stays maintainable.

But the important question is then: which framework do you use? The rust crate
ecosystem has come up with a large amount of web framework crates with varying
levels of popularity.

In general, the two most popular frameworks are Axum and Actix-Web, and they
should be your go-to frameworks of choice if you have no specific requirements.
Axum is nice because it integrates into the Tower ecosystem of middleware,
meaning that you will easily find some existing middleware implementations for
whatever you are trying to do, such as adaptive rate limiting. Actix-Web is
known for being easy to get started with, and for being very fast.

On a reasonably powerful system, either one of these can handle up to one
million requests per second, meaning that most likely your database will be the
bottleneck in scaling Rust web backends.

[hyper]: https://hyper.rs/
[h3]: https://github.com/hyperium/h3

### Template engines in Rust

_TODO_

https://blog.logrocket.com/top-3-templating-libraries-for-rust/

https://lib.rs/template-engine

### Routing

- macro-based vs dynamic

### Query Parsing

### Middleware

- tower ecosystem

### WebSockets

- websocket support

### Tracing

### Metrics

### State

### Testing

## Axum

[Axum](https://github.com/tokio-rs/axum) is currently the most popular web
framework in the Rust ecosystem. It is developed by the same people that wrote
Tokio, and uses hyper as the underlying HTTP implementation. It supports
WebSockets, has built-in routing and parameter decoding. It also integrates with
the [tracing](https://github.com/tokio-rs/tracing) ecosystem and uses
[tower](https://github.com/tower-rs/tower) to build middleware.

```rust
use axum::{
    routing::get,
    Router,
};

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new().route("/", get(|| async { "Hello, World!" }));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
```

One thing that is nice about Axum is that it does not use custom proc-macros to
implement routing or request handling, which makes it easier to use it with IDEs
that might not understand the syntax. The downside is that it's generics
approach sometimes leads to difficult-to-understand error messages.

## [Actix-Web](https://actix.rs/)

Actix started out as a framework implementing the actor model for
message-passing concurrency. Actix-Web, a framework for building web application
on top of it gained quite a lot of popularity. It remains the second-most
popular framework for building web backend application.

```rust
use actix_web::{get, web, App, HttpServer, Responder};

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {}!", name)
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(greet)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
```

Actix-Web is quite fast

## Rocket

[Rocket](https://rocket.rs) was an early framework for building web backends.
Initially, it only supported blocking code and used threads, but since version
0.5.0 it supports async as well.

```rust
extern crate rocket;

#[get("/")]
fn hello() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![hello])
}
```

## Salvo

[Salvo][salvo] is a web framework for Rust.

[salvo]: https://salvo.rs/

## Warp

https://github.com/seanmonstar/warp

## Tide

https://github.com/http-rs/tide

## Poem

https://github.com/poem-web/poem

## Deploying

### Shuttle

https://www.shuttle.rs/

### AWS Lambda

## Reading

```reading
style: article
title: "Are We Web Yet: Web Frameworks"
url: https://www.arewewebyet.org/topics/frameworks/
author: Are We Web Yet
---
List of web frameworks along with some stats on them.
```

```reading
style: article
title: "Web Frameworks Benchmark: Rust"
url: https://web-frameworks-benchmark.netlify.app/result?asc=0&l=rust&order_by=language
author: The Benchmarker
---
Compares the performance (as measured by requests-per-second) of various web
frameworks.
```

```reading
style: article
title: Rusts Axum style magic function params example
url: https://alexpusch/rust-magic-patterns/blob/master/axum-style-magic-function-param/Readme.md
author: Alex Puschinsky
---
In this article, Alex explains how Axum's magic function parameter handling is
implemented in Rust.
```
