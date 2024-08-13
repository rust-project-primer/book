# Web Frontend

Traditionally, web frontend applications are written in languages such as
JavaScript, or TypeScript which compiles down to it. Running code in the
browser allows you to store state client-side, implement responsive
applications that don't need to reload every time a user performs an action.

Since the advent of [WebAssembly][wasm], and the broad [support of browsers for
WebAssembly][browser-support], it has been possible to write frontend web
applications in other languages than JavaScript.

Thanks to Rust's use of [LLVM][llvm], a compiler infrastructure that makes it
easy to write new backends for different targets, it had support for targetting
WebAssembly relatively early. This means you can write entire applications that
live and run in the browser in Rust, and make use of Rust's extensive
ecosystem. 

Not all of Rust crates will work on WebAssembly out-of-the-box, for example
because they access native operating system APIs that do not exist in
WebAssembly, but many will work out-of-the-box or have feature flags that can
be enabled to add support for it.

Thanks to the hard work of the community, it is even possible to use Rust async
code in a WebAssembly environment, through the use of
[wasm-bindgen-futures](https://docs.rs/wasm-bindgen-futures/latest/wasm_bindgen_futures/).
These map the interface of Rust's Futures to JavaScript Promises.

Most web frameworks work similar to React, in that they have a component model
and keep the DOM in sync, either by directly updating it or by rendering to a
shadow DOM.

Running Rust frontend web application in the browser is a bit more complex than
just running `cargo build`, since the result WebAssembly still needs to be
packaged in a way that a browser can consume, and it needs some JavaScript glue
to make it usable. For this, a lot of frameworks use [Trunk][trunk] to bundle
and ship the raw Rust WebAssembly binaries into something the browser can
understand.

In this section, we will not cover all available frontend frameworks, only a
few of the post popular. As this is a relatively new development, there is a
lot of activity in the various frameworks and you should expect some volatility
in which frameworks are the most popular.

## [Yew](https://yew.rs/)

Yew is currently the most popular framework for web frontend development in Rust. 
It uses a reactive component model, has a useful ecosystem of plugins, supports
server-side rendering, routing, and has a HTML macro that makes it relatively easy
to get started.

The way you use it is by defining components, using the `functional_component`
prop macro.

```rust
#[function_component]
fn app() -> Html {
    html! {
        <h1>{ "Hello World" }</h1>
    }
}
```

Components can have inputs, these are called *props*. They can also have state
and react to things using *hooks*. 

```rust
#[function_component]
fn App() -> Html {
    let state = use_state(|| 0);

    let incr_counter = {
        let state = state.clone();
        Callback::from(move |_| state.set(*state + 1))
    };

    let decr_counter = {
        let state = state.clone();
        Callback::from(move |_| state.set(*state - 1))
    };

    html! {
        <>
            <p> {"current count: "} {*state} </p>
            <button onclick={incr_counter}> {"+"} </button>
            <button onclick={decr_counter}> {"-"} </button>
        </>
    }
}
```

Simple hooks come
[built-in](https://docs.rs/yew/latest/yew/functional/index.html), but there are
also external crates offering [more
hooks](https://docs.rs/yew-hooks/latest/yew_hooks/).

In older versions of Yew, functional components did not exist yet, so in some
code bases you may see people [deriving Component
manually](https://github.com/yewstack/yew/blob/master/examples/counter/src/main.rs).

The idea is that you can compose these small components into bigger applications. Yew
also comes with a plugin for [routing](https://docs.rs/yew-router/latest/yew_router/).

- todo: example application

## [Dioxus](https://dioxuslabs.com/)

Dioxus has a similar experience as Yew, also using a reactive component model.
One of the main differences is that it does not use a HTML macro the way Yew does,
which understands (mostly) vanilla HTML, but it has an `rsx` macro that uses a slightly
different syntax.

```rust
fn app() -> Element {
    rsx! {
        div { "Hello, world!" }
    }
}
```

The upside is that it does not use proc macros, which helps with IDE support
for the developer experience.

## [Leptos](https://www.leptos.dev/)

Leptop is quite similar to Yew. The primary difference is in how it renders:
Yew renders to a shadow DOM, and then synchronizes it to the real DOM, while
Leptos directly updates the DOM. This has some implications in terms of speed.

```rust
#[component]
pub fn SimpleCounter(initial_value: i32) -> impl IntoView {
    // create a reactive signal with the initial value
    let (value, set_value) = create_signal(initial_value);

    // create event handlers for our buttons
    // note that `value` and `set_value` are `Copy`, so it's super easy to move them into closures
    let clear = move |_| set_value(0);
    let decrement = move |_| set_value.update(|value| *value -= 1);
    let increment = move |_| set_value.update(|value| *value += 1);

    // create user interfaces with the declarative `view!` macro
    view! {
        <div>
            <button on:click=clear>Clear</button>
            <button on:click=decrement>-1</button>
            // text nodes can be quoted or unquoted
            <span>"Value: " {value} "!"</span>
            <button on:click=increment>+1</button>
        </div>
    }
}
```

## [Trunk][trunk]

Trunk is not a frontend framework at all, but it is a build tool. It handles some of
the nitty-gritty in getting a WebAssembly blog runnable in a browser. You can install
it by running:

    cargo install trunk --locked

Some interesting points is that it has some integration with external tooling, such
as wasm-opt to optimize and slim down WebAssembly binaries, and Tailwind CSS for generating
CSS styles.

## Reading

[Are We Web Yet: Web Frameworks](https://www.arewewebyet.org/topics/frameworks/)

*Keeps a list of frontend web frameworks for Rust along with some statistics indicating popularity.*

[Rust Web Framework Comparison](https://github.com/flosse/rust-web-framework-comparison)

Compares different Rust web frameworks.

[Full-stack Rust: A complete tutorial with examples](https://blog.logrocket.com/full-stack-rust-a-complete-tutorial-with-examples/)

*A tutorial showing how to build a full-stack Rust web application using Yew,
Tokio, Postgres, and Warp.*

[Full Stack Rust with Leptos](https://benw.is/posts/full-stack-rust-with-leptos)


[trunk]: https://trunkrs.dev/
[wasm]: https://webassembly.org/
[browser-support]: https://caniuse.com/wasm
[llvm]: https://llvm.org/
