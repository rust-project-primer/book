# Web Frontend

Writing Frontend application in Rust is possible thanks to Rust's support for
targetting WebAssembly. This means you can write entire applications that live
and run in the browser, enjoy fast latency (as the browser does not have to
reload the page any time a user clicks on something). At the same time, you can
make use of Rust's extensive ecosystem, even complex tasks such as parsing or
encryption is easily possible. 

Most web frameworks work similar to React, in that they have a component model
and keep the DOM in sync, either by directly updating it or by rendering to a
shadow DOM.

Due to how these run, they need some additional tooling. Commonly,
[Trunk][trunk] is used to bundle and ship the raw Rust WebAssembly binaries
into something the browser can understand.

## Yew

## Dioxus


## Reading

[Are We Web Yet: Web Frameworks](https://www.arewewebyet.org/topics/frameworks/) keeps
a list of frontend web frameworks for Rust along with some stats.

https://github.com/flosse/rust-web-framework-comparison


[trunk]: https://trunkrs.dev/
