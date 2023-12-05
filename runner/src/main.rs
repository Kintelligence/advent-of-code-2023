use std::time::Duration;

use runner::day;
use shared::*;
extern crate shared;

fn main() {
    let mut time: Duration = Duration::new(0, 0);
    for i in 1..=50 {
        let (function, input, name) = day(i);
        time += execute(function, input, name);
    }
    total(time);
}
