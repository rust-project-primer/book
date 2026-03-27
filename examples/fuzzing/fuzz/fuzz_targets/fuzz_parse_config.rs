// ANCHOR: fuzz_target
#![no_main]

use libfuzzer_sys::fuzz_target;
use fuzzing_example::parse_config;

fuzz_target!(|data: &str| {
    // We don't care about the result, we just want to make
    // sure the parser does not panic on any input.
    let _ = parse_config(data);
});
// ANCHOR_END: fuzz_target
