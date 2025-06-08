#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2024::*;
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
                        ::alloc::boxed::box_new([
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
