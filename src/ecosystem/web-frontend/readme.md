# Web Frontend

This section discusses the frameworks you can use in Rust to build web frontends that run in the
browser. If you are already familiar with the architecture of single-page web applications, you can
skip down to the frameworks for a discussion of how they work.

## Background

Websites use HTML for both content and structure. CSS is used to style how the website looks. The
browser reads the HTML to get the content and layout, then applies CSS to style it, and finally
renders it to the screen. When writing web applications, the first question is where and when this
HTML is generated.

In traditional web applications, the HTML is created on the server. When the backend gets a request,
it processes it and generates an HTML response. This response is then sent to the browser. On any
interaction, such as a click on a link, press of a button or submission of a form, a new request is
made to the browser, and a new HTML response is sent.

<center>

![Traditional web application](traditional-webapp.svg)

</center>

The Rust web backend frameworks have good support for writing web applications this way, often
combined with templating crates such as [handlebars][] or [tera][]. The downside to this traditional
approach is higher latencies. Since the entire page needs to be regenerated, transmitted and
rendered on every interaction, there is a noticeable delay. When structing a web application this
way, it is difficult to implement interactive widgets on pages, or update information in real-time.

Modern web frontend applications are often [single-page applications][spa] (SPAs), written in
languages like JavaScript or TypeScript and run in the browser. They are called "single-page"
because the entire app is loaded in the initial request. After that, the frontend reacts to
interactions and dynamically updates the content, without needing to realod the page. Communication
with the backend typically happens through an API. This keeps the app responsive while waiting for
server responses and allows for real-time events from the server using technologies like
[WebSockets][WebSocket].

<center>

![Modern web application](modern-webapp.svg)

</center>

Since the standardization of [WebAssembly][wasm] and the broad [browser support][browser-support] it
has gained, it has been possible to write frontend web applications languages other than JavaScript.
This section explores Rust frameworks that allow you to write single-page web applications for
full-stack Rust projects.

Using Rust for web frontends has some benefits. It allows you to write performant frontends, make
use of the Rust crate ecosystem, and share type definitions between your backend and frontend
easily. However, the availability of this is a relatively new and JavaScript-based frameworks tend
to be more mature. Finding frontend engineers that are familiar with JavaScript-based frameworks is
also a lot easier. If you want to build a prototype frontend for an existing Rust project, it may be
worth exploring these as it allows you use a single language across the project.

[spa]: https://en.wikipedia.org/wiki/Single-page_application
[handlebars]: https://github.com/sunng87/handlebars-rust
[tera]: https://keats.github.io/tera/
[WebSocket]: https://en.wikipedia.org/wiki/WebSocket

### The Component Model

All Rust web frontend frameworks discussed here use the component model to implement applications.
In web frontend development, the component model is a way to build applications using reusable and
self-contained pieces called components. Each component has its own logic and can manage its own
state and appearance. Components can be nested within other components to build complex user
interfaces. If you are familiar with [React][react] or similar JavaScript frontend libraries, then
you should already be familiar with the component model.

![Component application](app.svg)

Typically, web frameworks use a HTML-like domain-specific language to represent the outputs of
components. For example, the root component of this example application might look like this:

```rust
html! {
    <main>
        <Header />
        <div class="content">
            <SideBar />
            <Content />
        </div>
    </main>
}
```

Like functions can have arguments, components can have _properties_. These are inputs to the
component. In this example, the HTML `div` element has the property `class="content"`. In the same
way, Rust components can have properties, which can be any Rust type.

As a convention, HTML native components are usually lowercased (such as `main`, `div`, `p`) whereas
Rust components are uppercased (such as `Header`, `SideBar`, `Content`).

Components can also have _state_.

Finally, many frameworks also support _context_. Unlike _properties_, which a parent explicitly
passes down to its child components, context is implicitly passed down to all child components (even
children-of-children). This is often useful to pass global state such as whether the user is logged
in down to all components in the tree, or utilities such as data caches.

The the web frameworks do is they handle _changing_ of data. If any of the inputs to a component
changes, whether that be properties, state or context, the component is re-rendered.

<center>

![Component data model](component.svg)

</center>

The way frameworks can track these changes depend on the framework itself, but generally they are
able to do so because they have _hooks_ that allow them to track what is changed and when.

- animation of changes propagating

[dom]: https://developer.mozilla.org/en-US/docs/Web/API/Document_Object_Model/Introduction

In this section, we will not cover all available frontend frameworks, only a few of the post
popular. As this is a relatively new development, there is a lot of activity in the various
frameworks and you should expect some volatility in which frameworks are the most popular.

[react]: https://react.dev/learn

### Raw Web APIs with `web-sys`

While most of the Rust web frameworks handle all of the interactions with the underlying web APIs,
sometimes you may find the need to go "deeper" and interact with the raw APIs. The way you do this
is by using the `web-sys` crate, which has safe Rust wrappers for all of the APIs the browser
exposes.

```admonish
One quirk of the `web-sys` crate is that it puts every single API behind a
feature flag. In doing so, it has over 1,500 features, and as such needs an
exception to bypass the [crates.io crate features
limit](https://blog.rust-lang.org/2023/10/26/broken-badges-and-23k-keywords.html).
If you use it, don't be surprised if you get compiler errors, make sure that
you have enabled the correct set of features. The crate documentation shows you
for every interface, which feature it requires.
```

Most frontend libraries will allow you to get raw access to the underlying DOM nodes and perform raw
operations on them. One example is when you want to use a `<canvas>` element, you can use this to
draw on it. Here is an example of what this looks like in Yew:

```rust
// todo
```

You must keep in mind how the framework renders, to make sure that your raw access is not broken by
components refreshing.

### Compiling and Deploying Frontend Applications

Deploying a Rust frontend web application in the browser is a bit more complex than just running
`cargo build`, since the resulting WebAssembly blob still needs to be packaged in a way that a
browser can consume, and it needs some JavaScript glue to make it usable. For this, a lot of
frameworks use [Trunk][trunk] to bundle and ship the raw Rust WebAssembly binaries into something
the browser can understand. The [Trunk](#trunk) section below explains how that works and how you
can configure it.

Some Rust web frontend frameworks also support server-side rendering, where it can fallback to a
traditional web application style where the HTML is generated server-side. This can also help search
engines index the websites better by not needing WebAssembly support to render the website. The
frameworks support partial hydration, where parts of the website are rendered server-side, or full
hydration where every page can be fully rendered server-side.

If you use this feature, you also need to integrate your frontend application with your backend.

### Rendering Methods

Browsers represent a loaded website (with HTML and styling) in their [Document Object Model][dom].
Web frontend frameworks have to update this DOM whenever components change their outputs. One
important difference between frameworks is in how they do this.

Some frameworks have a _shadow DOM_ (sometimes also called _virtual DOM_), which is a copy of the
DOM that is in the browser, that components modify. The framework then synchronizes this copy with
the real DOM.

Other frameworks modify the DOM directly, which can have some performance benefits.

### WebAssembly Support in the Ecosystem

Thanks to Rust's use of [LLVM][llvm], a compiler infrastructure that makes it easy to write new
backends for different targets, it gained support for targetting WebAssembly relatively early. This
means you can write entire applications that live and run in the browser in Rust, and make use of
Rust's extensive ecosystem.

Not all of Rust crates will work on WebAssembly out-of-the-box, for example because they access
native operating system APIs that do not exist in WebAssembly, but many will work out-of-the-box or
have feature flags that can be enabled to add support for it.

All of the low-level APIs that are relevant for running in the browser are exposed by the
[web_sys](https://docs.rs/web-sys/latest/web_sys/index.html) crate. This is a large crate that is
automatically generated, and you need to enable features to enable it's various APIs. Ergonomic
wrappers for a lot of functionality are exposed by the [gloo](https://docs.rs/gloo/latest/gloo/)
crate, and you should use this if you can.

### Async Support

Thanks to the hard work of the community, it is even possible to use Rust async code in a
WebAssembly environment through the use of
[wasm-bindgen-futures](https://docs.rs/wasm-bindgen-futures/latest/wasm_bindgen_futures/). These map
the interface of Rust's Futures to JavaScript Promises.

For example, you can use this to spawn a future in the background to make a network request and get
the body of some web resource using the [reqwest](https://docs.rs/reqwest/latest/reqwest/#wasm)
library:

```rust
wasm_bindgen_futures::spawn_local(async {
    let test = reqwest::get("https://www.rust-lang.org")
        .await?
        .text()
        .await?;
});
```

Most frameworks have some kind of wrapper around these raw futures to be able to use them in the
applications.

### Server-Side Rendering

### Differences between frameworks

The rest of this section discusses some frameworks for Rust-based frontend programming. Generally,
the conceptual model of these frameworks is very similar, because they used the same component
model.

Differences between the frameworks exist between:

- The language they use to describe the output of a component. Usually, this is some kind of macro
  that allows you to specify a tree of components (HTML or native), their properties and children.
- The method in which they render the output of the components into the browser (using direct
  rendering or a shadow DOM). Either rendering methods can have advantages, it depends on what you
  are doing. Unless you are rendering a large amount of data or update frequently, it likely does
  not make a difference.
- The ecosystem of premade components and hooks. Some frameworks are more established and have
  third-part support for premade hooks and component libraries. These make your life easier.
- The degree to which they allow you to access raw browser APIs. Frameworks that have multiple
  rendering backends might be more limited in their support for raw browser APIs for compatibility.
- The syntax they use for defining components, properties and create and access hooks.
- The build system they use and support (either Trunk or a custom build system)
- Support for server-side rendering, for example having plugins for popular web backend crates such
  as `axum` or `actix-web`.

In the next sections, we will showcase some popular frameworks and attempt to give an overview of
their features.

## Yew

[Yew](https://yew.rs/) is currently the most popular framework for web frontend development in Rust.
It uses a reactive component model, has a useful ecosystem of plugins, supports server-side
rendering, routing, and has a `html!` macro that makes it relatively easy to get started.

To define a component, you can either implement the [Component][yew::html::Component] trait, or use
the [`function_component`][yew::functional::function_component] derive macro. In general, the latter
leads to more concise code, and is the recommended way. Functional components return `Html`, using
the `html` macro. This macro can output raw HTML, or other child components.

```rust
#[function_component]
fn app() -> Html {
    html! {
        <h1>{ "Hello World" }</h1>
    }
}
```

You can think of this function as always being run whenever your component needs to re-render, for
example if any of the inputs (props or state) have changed. To declare state in your component, you
use _hooks_. Here is an example:

```rust
#[function_component]
fn App() -> Html {
    let state = use_state(|| 0);

    let increment_counter = {
        let state = state.clone();
        Callback::from(move |_| state.set(*state + 1))
    };

    let decrement_counter = {
        let state = state.clone();
        Callback::from(move |_| state.set(*state - 1))
    };

    html! {
        <>
            <p> {"current count: "} {*state} </p>
            <button onclick={increment_counter}> {"+"} </button>
            <button onclick={decrement_counter}> {"-"} </button>
        </>
    }
}
```

Simple hooks come [built-in](https://docs.rs/yew/latest/yew/functional/index.html), but there are
also external crates offering [more hooks](https://docs.rs/yew-hooks/latest/yew_hooks/).

The idea is that you can compose these small components into bigger applications. Yew also comes
with a plugin for [routing](https://docs.rs/yew-router/latest/yew_router/).

One thing that is nice about Yew is that the `html!` macro it uses very closely resembles HTML.
There is not a steep learning curve if you are familiar with it. The only downside with it is that
values require quoting, you can see that to have text inside a paragraph element, you need to write
`<p>{"Text here"}</p>`. Another downside of it is that the state handles it uses require cloning,
which adds some clutter to the code.

### Example: Yew Todo App

Here is an example of a todo-list application written in Yew. It showcases props, child components,
raw HTML rendering, the `use_state` hook and how to package it with trunk.

```files
path = "todo-yew"
git_ignore = true
files = ["!.git"]
default_file = "src/lib.rs"
```

You can see this application in action [here][todo-yew]. In this example, you can see how properties
in Yew are structs that derive the `Properties` trait. You can also see how state is represented
with the `use_state()` hook, and how `Callback` is used to pass callbacks down to child components.
The `html!` macro is used to output HTML elements and child components, and the `classes!` macro is
used to create a list of classes.

[todo-yew]: https://rust-project-primer.gitlab.io/todo-yew/
[yew::html::Component]: https://docs.rs/yew/0.21.0/yew/html/trait.Component.html
[yew::functional::function_component]:
  https://docs.rs/yew/0.21.0/yew/functional/attr.function_component.html

## Leptos

[Leptos](https://www.leptos.dev/) is a web frontend framework for Rust that is quite similar to Yew.
The primary difference is in how it renders: Yew renders to a shadow DOM, and then synchronizes it
to the real DOM, while Leptos directly updates the DOM. This has some implications in terms of
speed.

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

One thing to note is that the `view!` macro it uses to create the component output tree has some
slightly different syntax from regular HTML. For example, it uses `on:click=value` instead of
`onclick=value`. An upside is that values do not require being put into braces, so
`<span>"Hello"</span>` is valid. Also, the state handles it uses do not require cloning as they do
in Yew.

### Example: Todo App

Here is an example of a todo-list application written using Leptos. It showcases defining
components, rendering child components, passing down properties, handling state and passing
callbacks to child components.

```files
path = "todo-leptos"
git_ignore = true
files = ["!.git"]
default_file = "src/lib.rs"
```

You can see this app in action [here][todo-leptos]. You can see how Leptos represents properties
using function parameters when defining components. You can also see how Leptos manages state using
`create_signal()`, which returns a getter and a setter for the signal value. It further showcases
how the `view!` macro is used to construct a tree of HTML elements and child components, and how
`Callback` can be used to pass callbacks down to child components.

[todo-leptos]: https://rust-project-primer.gitlab.io/todo-leptos

## Dioxus

[Dioxus](https://dioxuslabs.com/) is another frontend framework. Like Yew and Leptos, it also uses
the component model, hooks and has a domain-specific language for describing the graph of HTML
elements and components that a component renders into.

What makes Dioxus interesting is that it is easy to build Desktop and Mobile applications with it.
The Dioxus team is also working on [Blitz](https://github.com/DioxusLabs/blitz), a minimal web
renderer for use with writing Desktop applications with Dioxus but without the need for a full
browser engine. Dioxus also used to support rendering to the Terminal, but it appears as if the
support for this has been dropped since `0.4.3`.

The domain-specific language of Dioxus uses the `rsx!` macro and is distinct from the XML-style that
the other frameworks use.

```rust
fn app() -> Element {
    rsx! {
        div { "Hello, world!" }
    }
}
```

Dioxus comes with its own CLI to use for initializing, building and serving Dioxus applications. I
was not able to get it working with Trunk.

### Example: Todo App

This is an example todo application written using Dioxus. It looks and functions similar to the
example applications written with Yew and Leptos.

```files
path = "todo-dioxus"
git_ignore = true
files = ["!.git"]
default_file = "src/lib.rs"
```

You can see this application in action [here][todo-dioxus]. Note that this implementation is
slightly different from the Yew and Leptos implementations, because here we pass the signal that
contains the list of todo items directly down to the child components and have them change it,
rather than using callbacks to update it.

[todo-dioxus]: https://rust-project-primer.gitlab.io/todo-dioxus

## Trunk

[Trunk][trunk] is is a build tool for Rust web frontends. It handles some of the nitty-gritty in
getting a WebAssembly blog runnable in a browser. You can install it by running:

    cargo install trunk --locked

If you have not done so already, you also need to enable compiling to WebAssembly. If you installed
Rust using rustup, you can do this easily:

    rustup target add wasm32-unknown-unknown

Some interesting points is that it has some integration with external tooling, such as `wasm-opt` to
optimize and slim down WebAssembly binaries, and [Tailwind CSS][tailwind] for generating CSS styles.

[tailwind]: https://tailwindcss.com/

### Setup

To get started with Trunk, you need to create an `index.html` file. This is used by Trunk as a
template, and it contains some metadata for Trunk that tells it what assets you want to include in
the build.

```html
<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="utf-8" />
    <title>Hello World</title>
    <link data-trunk rel="rust" data-wasm-opt="z" />
  </head>
  <body></body>
</html>
```

```admonish info
The `data-wasm-opt` property here tells Trunk to call `wasm-opt` over the resulting
WebAssembly output when doing a release build.
```

### Assets

Most of the content of this does not matter. Trunk only cares about any tags that have the
`data-trunk` property. In this example, we have only one entry that Trunk processes, which is the
`rel=rust` one. This tells Trunk to link the current crate into this site, and run `wasm-opt` on it
to optimize the WebAssembly.

You can include some CSS in the output of your site like this:

```html
<link data-trunk rel="rust" data-wasm-opt="z" />
```

If you want to use Tailwind CSS, you can use this to tell Trunk to run it and include the generated
CSS file in your site:

```html
<link data-trunk rel="tailwind-css" href="src/tailwind.css" />
```

See the [Trunk Assets](https://trunkrs.dev/assets/) documentation page for a full list of the types
of assets that Trunk supports including in your application. It can run the SASS preprocessor, copy
static assets such as images, inline content, copy files or directory,

### Configuration

Trunk also has an additional configuration file that you can use to configure how it works,
`Trunk.toml`. In this file, you can configure some hooks, which are run before, during or after the
build for custom steps, set up proxying for the Trunk development server, or change where and how
your site is built.

### Request Forwarding

A common pattern for developing is to use `trunk serve` to build and serve your frontend, and to
have it talk to your backend via API requests. To make it easier to route the API requests to your
backend, you can tell Trunk to forward proxy requests matching a specific route to another service.

```toml
[[proxy]]
rewrite = "/api/v1/"
backend = "http://localhost:9000/"
```

### Example: Trunk and Tailwind CSS

### Example: Proxying API requests to backend

## Reading

```reading
style: article
title: "Are We Web Yet: Web Frameworks"
url: https://www.arewewebyet.org/topics/frameworks/)
author: Are We Web Yet
---
List of frontend web frameworks for Rust along with some statistics indicating
popularity. Good for discovery of new and rising frameworks or to explore all
the different ideas.
```

```reading
style: article
title: Rust Web Framework Comparison
url: https://github.com/flosse/rust-web-framework-comparison
author: Markus Kohlhase
---
Overview of different Rust frontend and backend frameworks. Unfortunately, it
marks some frameworks that are still heavily used as outdated, so take that
with a grain of salt.
```

```reading
style: article
title: "Full-stack Rust: A complete tutorial with examples"
url: https://blog.logrocket.com/full-stack-rust-a-complete-tutorial-with-examples/
author: Mario Zupan
---
Tutorial showing how to build a full-stack Rust web application using Yew,
Tokio, Postgres, and Warp. Good tutorial to see how everything fits together,
unfortunately it is a bit older and uses an outdated version of Yew that is
pre-functional components. But it is still a good article to get a feeling for
how a full-stack Rust application fits together.
```

```reading
style: article
title: Full Stack Rust with Leptos
url: https://benw.is/posts/full-stack-rust-with-leptos
author: Ben Wishowich
archived: benwis-full-stack-rust-with-leptos.pdf
---
```

```reading
style: book
title: Rust and WebAssembly Book
url: https://rustwasm.github.io/docs/book/introduction.html
author: Rust-Wasm Project
---
Book that explains how to use Rust to target WebAssembly. Has some good
low-level information, such as how to debug and profile WebAssembly
applications, keeping code size small, interoperation with JavaScript.
```

```reading
style: article
title: "A Rust web server / frontend setup like it's 2022 (with axum and yew)"
url: https://robert.kra.hn/posts/2022-04-03_rust-web-wasm/
author: Robert Krahn
---
Shows how to setup a full-stack Rust web application with Yew and Axum
from scratch.
```

```reading
style: article
title: Using Dioxus with Rust to build performant single-page apps
url: https://blog.logrocket.com/using-dioxus-rust-build-single-page-apps/
author: Eze Sunday
---
Eze shows how to use Dioxus to implement a todo application. Uses an older
version of Dioxus, the interface has since changed.
```

[trunk]: https://trunkrs.dev/
[wasm]: https://webassembly.org/
[browser-support]: https://caniuse.com/wasm
[llvm]: https://llvm.org/
