# Web Backend

A common use-case of Rust is building backends for web applications. To do this,
you often need some kind of framework to handle the HTTP protocol, request routing,
authentication, parameter deserialization and response.

![bubble graph of popular web crates](/graphics/crates-web.svg)

### Template engines in Rust

*TODO*

https://blog.logrocket.com/top-3-templating-libraries-for-rust/

https://lib.rs/template-engine

## [Axum](https://github.com/tokio-rs/axum)

Axum is currently the most popular web framework in the Rust ecosystem. It is
developed by the same people that wrote Tokio, and uses hyper as the underlying
HTTP implementation.  It supports WebSockets, has built-in routing and
parameter decoding. It also integrates with the
[tracing](https://github.com/tokio-rs/tracing) ecosystem and uses
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
implement routing or request handling, which makes it easier to use it with
IDEs that might not understand the syntax. The downside is that it's generics
approach sometimes leads to difficult-to-understand error messages.

## [Actix-Web](https://actix.rs/)

Actix started out as a framework implementing the actor model for message-passing
concurrency. Actix-Web, a framework for building web application on top of it gained
quite a lot of popularity. It remains the second-most popular framework for building
web backend application.

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

## [Rocket](https://rocket.rs)

Rocket was an early framework for building web backends. Initially, it only
supported blocking code and used threads, but since version 0.5.0 it supports
async as well.

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

## Reading


[Are We Web Yet: Web
Frameworks](https://www.arewewebyet.org/topics/frameworks/) maintains a list of
web frameworks along with some stats on them.


https://www.arewewebyet.org/


[Web Frameworks Benchmark: Rust](https://web-frameworks-benchmark.netlify.app/result?asc=0&l=rust&order_by=language)

*Compares the performance (as measured by requests-per-second) of various web
frameworks.*
