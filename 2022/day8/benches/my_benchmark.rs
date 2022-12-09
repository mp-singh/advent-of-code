use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day8::{parse_input, part1, part2};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("day8-part1", |b| {
        b.iter(|| part1(black_box(&parse_input())))
    });
    c.bench_function("day8-part2", |b| {
        b.iter(|| part2(black_box(&parse_input())))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
