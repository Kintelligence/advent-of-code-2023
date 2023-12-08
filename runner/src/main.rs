use std::time::Duration;

use runner::day;
use shared::{parse::Parsable, *};
extern crate shared;

fn main() {
    let mut time: Duration = Duration::new(0, 0);
    for i in 1..=50 {
        let (function, input, id) = day(i);
        let day = id.bytes().next_number().unwrap();

        time += execute(function, input, id, day_name(day));
    }
    total(time);
}
