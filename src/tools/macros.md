# Expand Macros

On a high level, a macro is some code that generates code. In languages such
as C or C++, they are expanded by the preprocessor in a step just before
compilation happens. They are commonly used to reduce code repetition,
avoid boilerplate.

Instead of relying on a preprocessor, the Rust compiler has built-in support
for macros. It supports two kinds of macros: [declarative
macros](https://doc.rust-lang.org/reference/macros-by-example.html) and
[procedural
macros](https://doc.rust-lang.org/reference/procedural-macros.html).
Declarative macros work as a kind of pattern-match-and-replace on tokens.  They
are fast and functional, but are limited in terms of what they can do.
Procedural macros work by compiling a separate Rust program, which is fed the
arguments of the macro and outputs Rust code that it is replaced with.  They
are more powerful, can do potentially non-deterministic things, but have higher
overhead.

Declarative macros can be used to implement Domain-Specific Languages within
Rust. For example, the
[`json!`][json] macro
allows you to write JSON within Rust, or the
[`html!`](https://yew.rs/docs/next/concepts/html) macro allows you to write HTML
within Rust. Procedural derive macros are often used to allow you to derive
traits for your types automatically. Commonly used examples are the `Serialize`
and `Deserialize` derive macros from the
[serde](https://docs.rs/serde/latest/serde/index.html) crate. Procedural
attribute macros such as
[`rocket::get`](https://docs.rs/rocket/latest/rocket/attr.get.html) are used to
provide metadata for routing requests in the Rocket web backend framework.

Using macros, where appropriate, is good style because it allows you to reduce
boilerplate code. At times, they can feel quite magic. However, there are
downsides to relying on them heavily as well:

1. When you use procedural macros, a separate Rust application needs to be
   built and run for the compilation, slowing down your compilations.
2. Formatting often does not work within macro invocations. Some projects
   work around this by providing their own formatting tools that are able to do
   this, for example [leptosfmt](https://github.com/bram209/leptosfmt).
3. Macros can be difficult to understand. Because macros are expanded at
   compile-time, it can be difficult to inspect or debug them, because you cannot
   see what code the macro expands to.

This section looks at how you can work around (3), by showing you how you can
inspect what your code looks like after macro expansion.

## Cargo Expand

[cargo expand](https://github.com/dtolnay/cargo-expand) is a Cargo plugin that
allows you to view your code after macro expansion. In addition to performing
macro expansion, it will also run `rustfmt` over the result (because the code
that macros expands to is often machine-generated and therefore unformatted)
and syntax-highlights the result.

You can install it simply using Cargo:

    cargo install cargo-expand

To run it, simply run it as a Cargo subcommand within a Rust crate:

    cargo expand

It has some command-line options that you can use to control the output
options, for example turning off the syntax highlighting or selecting a
different theme that plays nicer with your terminal color scheme.

### Example: Inspecting your own macro

If you want to create a `Vec<T>`, Rust has a built-in macro for doing so:
`vec![]`. However, the same is not true for creating maps, such as
`BTreeMap<T>`. You can work around this by creating your own macro:

```rust
macro_rules! btreemap {
    ( $($x:expr => $y:expr),* $(,)? ) => ({
        let mut temp_map = std::collections::BTreeMap::new();
        $(
            temp_map.insert($x, $y);
        )*
        temp_map
    });
}
```

But how do you verify that this macro works correctly? Besides writing unit tests for
it, you can write a small test program that uses this macro, for example:

```rust
fn main() {
    let mapping = btreemap!{
        "joesmith" => "joe.smith@example.com",
        "djb" => "djb@example.com",
        "elon" => "musk@example.com"
    };
}
```

Finally, you can run `cargo expand` on this test program to verify that it is expanding
to the right thing.

```rust
#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
fn main() {
    let values = {
        let mut temp_map = ::std::collections::BTreeMap::new();
        temp_map.insert("joesmith", "joe.smith@example.com");
        temp_map.insert("djb", "djb@example.com");
        temp_map.insert("elon", "musk@example.com");
        temp_map
    };
}
```

### Example: Inspecting the [`json!`][json] macro

The [`json!`][json] macro from `serde_json` allows you to write JSON inline in Rust,
and get a JSON `Value` back. It supports all of JSON syntax, and allows you to
interpolate Rust values inside it as well.

```rust
use serde_json::json;
use uuid::Uuid;

fn main() {
    let id = Uuid::new_v4();
    let person = json!({
        "name": "Jeff",
        "age": 24,
        "interests": ["guns", "trucks", "bbq"],
        "nationality": "us",
        "state": "tx",
        "id": id.to_string()
    });
}
```

To see what this code actually does, calling `cargo expand` on it yields the
following:

```rust
#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use serde_json::json;
use uuid::Uuid;
fn main() {
    let id = Uuid::new_v4();
    let person = ::serde_json::Value::Object({
        let mut object = ::serde_json::Map::new();
        let _ = object.insert(("name").into(), ::serde_json::to_value(&"Jeff").unwrap());
        let _ = object.insert(("age").into(), ::serde_json::to_value(&24).unwrap());
        let _ = object
            .insert(
                ("interests").into(),
                ::serde_json::Value::Array(
                    <[_]>::into_vec(
                        #[rustc_box]
                        ::alloc::boxed::Box::new([
                            ::serde_json::to_value(&"guns").unwrap(),
                            ::serde_json::to_value(&"trucks").unwrap(),
                            ::serde_json::to_value(&"bbq").unwrap(),
                        ]),
                    ),
                ),
            );
        let _ = object
            .insert(("nationality").into(), ::serde_json::to_value(&"us").unwrap());
        let _ = object.insert(("state").into(), ::serde_json::to_value(&"tx").unwrap());
        let _ = object
            .insert(("id").into(), ::serde_json::to_value(&id.to_string()).unwrap());
        object
    });
}
```

This shows that under the hood, the macro expands to manual creations of a map,
filling it with values.

### Example: Inspecting the `Serialize` procedural macro

The `Serialize` procedural macro auto-generates an implementation for the
`Serialize` trait that the `serde` crate uses to be able to serialize your
struct to arbitrary data formats. If you have some struct which uses this
derive macro:

```rust
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
pub struct Person {
    name: String,
    id: Uuid,
    age: u16,
}
```

You may want to know what the expanded code looks like. Again, running `cargo expand`
can show you this.

```rust
#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use serde::Serialize;
use uuid::Uuid;
pub struct Person {
    name: String,
    id: Uuid,
    age: u16,
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl _serde::Serialize for Person {
        fn serialize<__S>(
            &self,
            __serializer: __S,
        ) -> _serde::__private::Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
        {
            let mut __serde_state = _serde::Serializer::serialize_struct(
                __serializer,
                "Person",
                false as usize + 1 + 1 + 1,
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "name",
                &self.name,
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "id",
                &self.id,
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "age",
                &self.age,
            )?;
            _serde::ser::SerializeStruct::end(__serde_state)
        }
    }
};
```

## Reading

```reading
style: book
title: "Chapter 19.5: Macros"
url: https://doc.rust-lang.org/book/ch19-06-macros.html
author: The Rust Book
---
Section in The Rust Book introducing and explaining macros. It explains the
difference declarative and procedural macros, and the different types of
procedural macros (attribute macros, derive macros, function-like macros) and
how they are implemented.
```

```reading
style: article
title: Rust Macros and inspection with cargo expand
url: https://medium.com/@adamszpilewicz/rust-macros-and-inspection-with-cargo-expand-9236b6ccff17
author: Adam Szpilewicz
---
Adam explains Rust macros and how they can be inspected with `cargo expand`.
```

[json]: https://docs.rs/serde_json/latest/serde_json/macro.json.html
[serde_json]: https://docs.rs/serde_json/latest/
