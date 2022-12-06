use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day1::{parse_input, part1, part2};

fn criterion_benchmark(c: &mut Criterion) {
    let input = parse_input();
    c.bench_function("day1-part1", |b| b.iter(|| part1(black_box(&input))));
    c.bench_function("day1-part2", |b| b.iter(|| part2(black_box(&input))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
