#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2024::*;
#[macro_use]
extern crate std;
fn main() {
    let mapping = {
        let mut temp_map = ::std::collections::BTreeMap::new();
        temp_map.insert("joesmith", "joe.smith@example.com");
        temp_map.insert("djb", "djb@example.com");
        temp_map.insert("elon", "musk@example.com");
        temp_map
    };
}
