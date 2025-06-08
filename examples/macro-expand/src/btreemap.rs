// ANCHOR: macro
macro_rules! btreemap {
    ( $($x:expr => $y:expr),* $(,)? ) => ({
        let mut temp_map = ::std::collections::BTreeMap::new();
        $(
            temp_map.insert($x, $y);
        )*
        temp_map
    });
}
// ANCHOR_END: macro

// ANCHOR: main
fn main() {
    let mapping = btreemap!{
        "joesmith" => "joe.smith@example.com",
        "djb" => "djb@example.com",
        "elon" => "musk@example.com"
    };
}
// ANCHOR_END: main
