use criterion::{black_box, criterion_group, criterion_main, Criterion};
use runner::day;
use shared::{day_name, Solution};

extern crate shared;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("Total: All Days", |b| {
        b.iter(|| {
            for i in 1..=50 {
                let (function, input, _) = day(i);
                function(black_box(input));
            }
        })
    });

    let mut bench_group = c.benchmark_group("Individual");
    for i in 1..=50 {
        let (function, input, name) = day(i);
        if let Solution::None = function(input) {
        } else {
            let title = day_name((i + 1) / 2);
            bench_group.bench_function(&format!("{}: {}", name, title), |b| {
                b.iter(|| function(black_box(input)))
            });
        }
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
