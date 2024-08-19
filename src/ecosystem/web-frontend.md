# Web Frontend

Traditionally, web frontend applications are written in languages such as
JavaScript, or TypeScript which compiles down to it. Running code in the
browser allows you to store state client-side, implement responsive
applications that don't need to reload every time a user performs an action.

### WebAssembly

Since the advent of [WebAssembly][wasm], and the broad [support of browsers for
WebAssembly][browser-support], it has been possible to write frontend web
applications in other languages than JavaScript.

Thanks to Rust's use of [LLVM][llvm], a compiler infrastructure that makes it
easy to write new backends for different targets, it gained support for
targetting WebAssembly relatively early. This means you can write entire
applications that live and run in the browser in Rust, and make use of Rust's
extensive ecosystem. 

Not all of Rust crates will work on WebAssembly out-of-the-box, for example
because they access native operating system APIs that do not exist in
WebAssembly, but many will work out-of-the-box or have feature flags that can
be enabled to add support for it.

All of the low-level APIs that are relevant for running in the browser are
exposed by the [web_sys](https://docs.rs/web-sys/latest/web_sys/index.html)
crate. This is a large crate that is automatically generated, and you need to
enable features to enable it's various APIs. Ergonomic wrappers for a lot of
functionality are exposed by the [gloo](https://docs.rs/gloo/latest/gloo/)
crate, and you should use this if you can.

### Async Support

Thanks to the hard work of the community, it is even possible to use Rust async
code in a WebAssembly environment, through the use of
[wasm-bindgen-futures](https://docs.rs/wasm-bindgen-futures/latest/wasm_bindgen_futures/).
These map the interface of Rust's Futures to JavaScript Promises.

For example, you can use this to spawn a future in the background to make a
network request and get the body of some web resource using the
[reqwest](https://docs.rs/reqwest/latest/reqwest/#wasm) library:

```rust
wasm_bindgen_futures::spawn_local(async {
    let test = reqwest::get("https://www.rust-lang.org")
        .await?
        .text()
        .await?;
});
```

Most frameworks have some kind of wrapper around these raw futures to be able to
use them in the applications.

### How these frameworks work

Most Rust web frameworks work similar to [React][react].  They have a component
model, where every component can have parameters called *props*, can keep some
state, and their output is a tree containing either raw HTML or child
components. The frameworks handle rendering the components to the browser, and
take care of handling changes in state (by re-rendering affected components and
child components that changed). 

- animation of updating state in component using d3/recursion-visualize

Some of the differences are in how they render the components to the [Document
Object Model][dom], which is what the browser uses to represent the HTML that
is rendered. Some frameworks render a shadow DOM and synchronize it with the
browser, while others directly update the DOM.

[dom]: https://developer.mozilla.org/en-US/docs/Web/API/Document_Object_Model/Introduction

Deploying a Rust frontend web application in the browser is a bit more complex
than just running `cargo build`, since the resulting WebAssembly blob still
needs to be packaged in a way that a browser can consume, and it needs some
JavaScript glue to make it usable. For this, a lot of frameworks use
[Trunk][trunk] to bundle and ship the raw Rust WebAssembly binaries into
something the browser can understand.

In this section, we will not cover all available frontend frameworks, only a
few of the post popular. As this is a relatively new development, there is a
lot of activity in the various frameworks and you should expect some volatility
in which frameworks are the most popular.

[react]: https://react.dev/learn

## [Yew](https://yew.rs/)

Yew is currently the most popular framework for web frontend development in Rust. 
It uses a reactive component model, has a useful ecosystem of plugins, supports
server-side rendering, routing, and has a HTML macro that makes it relatively easy
to get started.

To define a component, you can either implement the [Component][yew::html::Component]
trait, or use the [`function_component`][yew::functional::function_component] derive 
macro. In general, the latter leads to more concise code, and is the recommended way.

```rust
#[function_component]
fn app() -> Html {
    html! {
        <h1>{ "Hello World" }</h1>
    }
}
```

You can think of this function as always being run whenever your component needs to
re-render, for example if any of the inputs (props or state) have changed. To declare
state in your component, you use *hooks*. Here is an example:

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

The idea is that you can compose these small components into bigger applications. Yew
also comes with a plugin for [routing](https://docs.rs/yew-router/latest/yew_router/).

### Example: Todo App

Here is an example of a todo-list application written in Yew. It is a rewrite
of this [example todo app written in
React](https://www.digitalocean.com/community/tutorials/how-to-build-a-react-to-do-app-with-react-hooks).

```files
path = "todo-yew"
git_ignore = true
files = ["!.git"]
default_file = "src/lib.rs"
```

[Here](https://rust-project-primer.gitlab.io/todo-yew/) you can see this application running.


[yew::html::Component]: https://docs.rs/yew/0.21.0/yew/html/trait.Component.html
[yew::functional::function_component]: https://docs.rs/yew/0.21.0/yew/functional/attr.function_component.html

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

### Example: Todo App

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

### Example: Todo App

## [Trunk][trunk]

Trunk is not a frontend framework at all, but it is a build tool. It handles some of
the nitty-gritty in getting a WebAssembly blog runnable in a browser. You can install
it by running:

    cargo install trunk --locked

If you have not done so already, you also need to enable compiling to WebAssembly. If you
installed Rust using rustup, you can do this easily:

    rustup target add wasm32-unknown-unknown

Some interesting points is that it has some integration with external tooling, such
as wasm-opt to optimize and slim down WebAssembly binaries, and Tailwind CSS for generating
CSS styles.

### Configuration

### Assets

- how to include css
- how to include assets
- how to use tailwind css

### Request Forwarding

- how to proxy API requests to backend

### Output of Trunk

- example build output of trunk

## Reading

[Are We Web Yet: Web Frameworks](https://www.arewewebyet.org/topics/frameworks/)

*Keeps a list of frontend web frameworks for Rust along with some statistics indicating popularity.*

[Rust Web Framework Comparison](https://github.com/flosse/rust-web-framework-comparison)

Compares different Rust web frameworks.

[Full-stack Rust: A complete tutorial with examples](https://blog.logrocket.com/full-stack-rust-a-complete-tutorial-with-examples/)

*A tutorial showing how to build a full-stack Rust web application using Yew,
Tokio, Postgres, and Warp.*

[Full Stack Rust with Leptos](https://benw.is/posts/full-stack-rust-with-leptos)



[Rust and WebAssembly Book](https://rustwasm.github.io/docs/book/introduction.html)

*Book that explains how to use Rust to target WebAssembly.*

[trunk]: https://trunkrs.dev/
[wasm]: https://webassembly.org/
[browser-support]: https://caniuse.com/wasm
[llvm]: https://llvm.org/
