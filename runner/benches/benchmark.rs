use criterion::{black_box, criterion_group, criterion_main, Criterion};
use shared::Solution;

extern crate day_01;
extern crate day_02;
extern crate day_03;
extern crate day_04;
extern crate day_05;
extern crate day_06;
extern crate day_07;
extern crate day_08;
extern crate day_09;
extern crate day_10;
extern crate day_11;
extern crate day_12;
extern crate day_13;
extern crate day_14;
extern crate day_15;
extern crate day_16;
extern crate day_17;
extern crate day_18;
extern crate day_19;
extern crate day_20;
extern crate day_21;
extern crate day_22;
extern crate day_23;
extern crate day_24;
extern crate day_25;
extern crate shared;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("Total", |b| {
        b.iter(|| {
            day_01::part_1(black_box(day_01::_INPUT));
            day_01::part_2(black_box(day_01::_INPUT));
            day_02::part_1(black_box(day_02::_INPUT));
            day_02::part_2(black_box(day_02::_INPUT));
            day_03::part_1(black_box(day_03::_INPUT));
            day_03::part_2(black_box(day_03::_INPUT));
            day_04::part_1(black_box(day_04::_INPUT));
            day_04::part_2(black_box(day_04::_INPUT));
            day_05::part_1(black_box(day_05::_INPUT));
            day_05::part_2(black_box(day_05::_INPUT));
            day_06::part_1(black_box(day_06::_INPUT));
            day_06::part_2(black_box(day_06::_INPUT));
            day_07::part_1(black_box(day_07::_INPUT));
            day_07::part_2(black_box(day_07::_INPUT));
            day_08::part_1(black_box(day_08::_INPUT));
            day_08::part_2(black_box(day_08::_INPUT));
            day_09::part_1(black_box(day_09::_INPUT));
            day_09::part_2(black_box(day_09::_INPUT));
            day_10::part_1(black_box(day_10::_INPUT));
            day_10::part_2(black_box(day_10::_INPUT));
            day_11::part_1(black_box(day_11::_INPUT));
            day_11::part_2(black_box(day_11::_INPUT));
            day_12::part_1(black_box(day_12::_INPUT));
            day_12::part_2(black_box(day_12::_INPUT));
            day_13::part_1(black_box(day_13::_INPUT));
            day_13::part_2(black_box(day_13::_INPUT));
            day_14::part_1(black_box(day_14::_INPUT));
            day_14::part_2(black_box(day_14::_INPUT));
            day_15::part_1(black_box(day_15::_INPUT));
            day_15::part_2(black_box(day_15::_INPUT));
            day_16::part_1(black_box(day_16::_INPUT));
            day_16::part_2(black_box(day_16::_INPUT));
            day_17::part_1(black_box(day_17::_INPUT));
            day_17::part_2(black_box(day_17::_INPUT));
            day_18::part_1(black_box(day_18::_INPUT));
            day_18::part_2(black_box(day_18::_INPUT));
            day_19::part_1(black_box(day_19::_INPUT));
            day_19::part_2(black_box(day_19::_INPUT));
            day_20::part_1(black_box(day_20::_INPUT));
            day_20::part_2(black_box(day_20::_INPUT));
            day_21::part_1(black_box(day_21::_INPUT));
            day_21::part_2(black_box(day_21::_INPUT));
            day_22::part_1(black_box(day_22::_INPUT));
            day_22::part_2(black_box(day_22::_INPUT));
            day_23::part_1(black_box(day_23::_INPUT));
            day_23::part_2(black_box(day_23::_INPUT));
            day_24::part_1(black_box(day_24::_INPUT));
            day_24::part_2(black_box(day_24::_INPUT));
            day_25::part_1(black_box(day_25::_INPUT));
            day_25::part_2(black_box(day_25::_INPUT));
        })
    });
    if let Solution::None = day_01::part_1(day_01::_INPUT) {
    } else {
        c.bench_function("01.1", |b| {
            b.iter(|| day_01::part_1(black_box(day_01::_INPUT)))
        });
    }
    if let Solution::None = day_01::part_2(day_01::_INPUT) {
    } else {
        c.bench_function("01.2", |b| {
            b.iter(|| day_01::part_2(black_box(day_01::_INPUT)))
        });
    }
    if let Solution::None = day_02::part_1(day_02::_INPUT) {
    } else {
        c.bench_function("02.1", |b| {
            b.iter(|| day_02::part_1(black_box(day_02::_INPUT)))
        });
    }
    if let Solution::None = day_02::part_2(day_02::_INPUT) {
    } else {
        c.bench_function("02.2", |b| {
            b.iter(|| day_02::part_2(black_box(day_02::_INPUT)))
        });
    }
    if let Solution::None = day_03::part_1(day_03::_INPUT) {
    } else {
        c.bench_function("03.1", |b| {
            b.iter(|| day_03::part_1(black_box(day_03::_INPUT)))
        });
    }
    if let Solution::None = day_03::part_2(day_03::_INPUT) {
    } else {
        c.bench_function("03.2", |b| {
            b.iter(|| day_03::part_2(black_box(day_03::_INPUT)))
        });
    }
    if let Solution::None = day_04::part_1(day_04::_INPUT) {
    } else {
        c.bench_function("04.1", |b| {
            b.iter(|| day_04::part_1(black_box(day_04::_INPUT)))
        });
    }
    if let Solution::None = day_04::part_2(day_04::_INPUT) {
    } else {
        c.bench_function("04.2", |b| {
            b.iter(|| day_04::part_2(black_box(day_04::_INPUT)))
        });
    }
    if let Solution::None = day_05::part_1(day_05::_INPUT) {
    } else {
        c.bench_function("05.1", |b| {
            b.iter(|| day_05::part_1(black_box(day_05::_INPUT)))
        });
    }
    if let Solution::None = day_05::part_2(day_05::_INPUT) {
    } else {
        c.bench_function("05.2", |b| {
            b.iter(|| day_05::part_2(black_box(day_05::_INPUT)))
        });
    }
    if let Solution::None = day_06::part_1(day_06::_INPUT) {
    } else {
        c.bench_function("06.1", |b| {
            b.iter(|| day_06::part_1(black_box(day_06::_INPUT)))
        });
    }
    if let Solution::None = day_06::part_2(day_06::_INPUT) {
    } else {
        c.bench_function("06.2", |b| {
            b.iter(|| day_06::part_2(black_box(day_06::_INPUT)))
        });
    }
    if let Solution::None = day_07::part_1(day_07::_INPUT) {
    } else {
        c.bench_function("07.1", |b| {
            b.iter(|| day_07::part_1(black_box(day_07::_INPUT)))
        });
    }
    if let Solution::None = day_07::part_2(day_07::_INPUT) {
    } else {
        c.bench_function("07.2", |b| {
            b.iter(|| day_07::part_2(black_box(day_07::_INPUT)))
        });
    }
    if let Solution::None = day_08::part_1(day_08::_INPUT) {
    } else {
        c.bench_function("08.1", |b| {
            b.iter(|| day_08::part_1(black_box(day_08::_INPUT)))
        });
    }
    if let Solution::None = day_08::part_2(day_08::_INPUT) {
    } else {
        c.bench_function("08.2", |b| {
            b.iter(|| day_08::part_2(black_box(day_08::_INPUT)))
        });
    }
    if let Solution::None = day_09::part_1(day_09::_INPUT) {
    } else {
        c.bench_function("09.1", |b| {
            b.iter(|| day_09::part_1(black_box(day_09::_INPUT)))
        });
    }
    if let Solution::None = day_09::part_2(day_09::_INPUT) {
    } else {
        c.bench_function("09.2", |b| {
            b.iter(|| day_09::part_2(black_box(day_09::_INPUT)))
        });
    }
    if let Solution::None = day_10::part_1(day_10::_INPUT) {
    } else {
        c.bench_function("10.1", |b| {
            b.iter(|| day_10::part_1(black_box(day_10::_INPUT)))
        });
    }
    if let Solution::None = day_10::part_2(day_10::_INPUT) {
    } else {
        c.bench_function("10.2", |b| {
            b.iter(|| day_10::part_2(black_box(day_10::_INPUT)))
        });
    }
    if let Solution::None = day_11::part_1(day_11::_INPUT) {
    } else {
        c.bench_function("11.1", |b| {
            b.iter(|| day_11::part_1(black_box(day_11::_INPUT)))
        });
    }
    if let Solution::None = day_11::part_2(day_11::_INPUT) {
    } else {
        c.bench_function("11.2", |b| {
            b.iter(|| day_11::part_2(black_box(day_11::_INPUT)))
        });
    }
    if let Solution::None = day_12::part_1(day_12::_INPUT) {
    } else {
        c.bench_function("12.1", |b| {
            b.iter(|| day_12::part_1(black_box(day_12::_INPUT)))
        });
    }
    if let Solution::None = day_12::part_2(day_12::_INPUT) {
    } else {
        c.bench_function("12.2", |b| {
            b.iter(|| day_12::part_2(black_box(day_12::_INPUT)))
        });
    }
    if let Solution::None = day_13::part_1(day_13::_INPUT) {
    } else {
        c.bench_function("13.1", |b| {
            b.iter(|| day_13::part_1(black_box(day_13::_INPUT)))
        });
    }
    if let Solution::None = day_13::part_2(day_13::_INPUT) {
    } else {
        c.bench_function("13.2", |b| {
            b.iter(|| day_13::part_2(black_box(day_13::_INPUT)))
        });
    }
    if let Solution::None = day_14::part_1(day_14::_INPUT) {
    } else {
        c.bench_function("14.1", |b| {
            b.iter(|| day_14::part_1(black_box(day_14::_INPUT)))
        });
    }
    if let Solution::None = day_14::part_2(day_14::_INPUT) {
    } else {
        c.bench_function("14.2", |b| {
            b.iter(|| day_14::part_2(black_box(day_14::_INPUT)))
        });
    }
    if let Solution::None = day_15::part_1(day_15::_INPUT) {
    } else {
        c.bench_function("15.1", |b| {
            b.iter(|| day_15::part_1(black_box(day_15::_INPUT)))
        });
    }
    if let Solution::None = day_15::part_2(day_15::_INPUT) {
    } else {
        c.bench_function("15.2", |b| {
            b.iter(|| day_15::part_2(black_box(day_15::_INPUT)))
        });
    }
    if let Solution::None = day_16::part_1(day_16::_INPUT) {
    } else {
        c.bench_function("16.1", |b| {
            b.iter(|| day_16::part_1(black_box(day_16::_INPUT)))
        });
    }
    if let Solution::None = day_16::part_2(day_16::_INPUT) {
    } else {
        c.bench_function("16.2", |b| {
            b.iter(|| day_16::part_2(black_box(day_16::_INPUT)))
        });
    }
    if let Solution::None = day_17::part_1(day_17::_INPUT) {
    } else {
        c.bench_function("17.1", |b| {
            b.iter(|| day_17::part_1(black_box(day_17::_INPUT)))
        });
    }
    if let Solution::None = day_17::part_2(day_17::_INPUT) {
    } else {
        c.bench_function("17.2", |b| {
            b.iter(|| day_17::part_2(black_box(day_17::_INPUT)))
        });
    }
    if let Solution::None = day_18::part_1(day_18::_INPUT) {
    } else {
        c.bench_function("18.1", |b| {
            b.iter(|| day_18::part_1(black_box(day_18::_INPUT)))
        });
    }
    if let Solution::None = day_18::part_2(day_18::_INPUT) {
    } else {
        c.bench_function("18.2", |b| {
            b.iter(|| day_18::part_2(black_box(day_18::_INPUT)))
        });
    }
    if let Solution::None = day_19::part_1(day_19::_INPUT) {
    } else {
        c.bench_function("19.1", |b| {
            b.iter(|| day_19::part_1(black_box(day_19::_INPUT)))
        });
    }
    if let Solution::None = day_19::part_2(day_19::_INPUT) {
    } else {
        c.bench_function("19.2", |b| {
            b.iter(|| day_19::part_2(black_box(day_19::_INPUT)))
        });
    }
    if let Solution::None = day_20::part_1(day_20::_INPUT) {
    } else {
        c.bench_function("20.1", |b| {
            b.iter(|| day_20::part_1(black_box(day_20::_INPUT)))
        });
    }
    if let Solution::None = day_20::part_2(day_20::_INPUT) {
    } else {
        c.bench_function("20.2", |b| {
            b.iter(|| day_20::part_2(black_box(day_20::_INPUT)))
        });
    }
    if let Solution::None = day_21::part_1(day_21::_INPUT) {
    } else {
        c.bench_function("21.1", |b| {
            b.iter(|| day_21::part_1(black_box(day_21::_INPUT)))
        });
    }
    if let Solution::None = day_21::part_2(day_21::_INPUT) {
    } else {
        c.bench_function("21.2", |b| {
            b.iter(|| day_21::part_2(black_box(day_21::_INPUT)))
        });
    }
    if let Solution::None = day_22::part_1(day_22::_INPUT) {
    } else {
        c.bench_function("22.1", |b| {
            b.iter(|| day_22::part_1(black_box(day_22::_INPUT)))
        });
    }
    if let Solution::None = day_22::part_2(day_22::_INPUT) {
    } else {
        c.bench_function("22.2", |b| {
            b.iter(|| day_22::part_2(black_box(day_22::_INPUT)))
        });
    }
    if let Solution::None = day_23::part_1(day_23::_INPUT) {
    } else {
        c.bench_function("23.1", |b| {
            b.iter(|| day_23::part_1(black_box(day_23::_INPUT)))
        });
    }
    if let Solution::None = day_23::part_2(day_23::_INPUT) {
    } else {
        c.bench_function("23.2", |b| {
            b.iter(|| day_23::part_2(black_box(day_23::_INPUT)))
        });
    }
    if let Solution::None = day_24::part_1(day_24::_INPUT) {
    } else {
        c.bench_function("24.1", |b| {
            b.iter(|| day_24::part_1(black_box(day_24::_INPUT)))
        });
    }
    if let Solution::None = day_24::part_2(day_24::_INPUT) {
    } else {
        c.bench_function("24.2", |b| {
            b.iter(|| day_24::part_2(black_box(day_24::_INPUT)))
        });
    }
    if let Solution::None = day_25::part_1(day_25::_INPUT) {
    } else {
        c.bench_function("25.1", |b| {
            b.iter(|| day_25::part_1(black_box(day_25::_INPUT)))
        });
    }
    if let Solution::None = day_25::part_2(day_25::_INPUT) {
    } else {
        c.bench_function("25.2", |b| {
            b.iter(|| day_25::part_2(black_box(day_25::_INPUT)))
        });
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
