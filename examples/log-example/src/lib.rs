use log::*;
use std::time::Instant;

pub fn do_something() -> f64 {
    info!("started doing something");

    // run and measure runtime
    let now = Instant::now();
    let mut value = 1.0;
    for _ in 0..1000000 {
        value *= 1.00001;
    }
    let time = now.elapsed();
    debug!("took {time:?}");

    // log result
    info!("result is {value}");

    value
}
