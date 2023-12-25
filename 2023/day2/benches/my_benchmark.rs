use std::time::Duration;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day2::soln::{parse_input, part1::part1, part2::part2};

fn criterion_benchmark(c: &mut Criterion) {
    let input = parse_input();
    let mut group = c.benchmark_group("day2");
    group.bench_function("part1", |b| b.iter(|| part1(black_box(&input))));
    group.bench_function("part2", |b| b.iter(|| part2(black_box(&input))));
    group.finish();
}

criterion_group! {
    name = benches;
    config = Criterion::default().measurement_time(Duration::from_secs(10));
    targets = criterion_benchmark
}
criterion_main!(benches);
