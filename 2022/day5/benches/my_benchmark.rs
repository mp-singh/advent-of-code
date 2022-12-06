use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day5::{parse_input, part1, part2};

fn criterion_benchmark(c: &mut Criterion) {
    let (instructions, stack) = parse_input();
    c.bench_function("day5-part1", |b| {
        b.iter(|| part1(black_box(stack.clone()), black_box(&instructions)))
    });
    c.bench_function("fay5-part2", |b| {
        b.iter(|| part2(black_box(stack.clone()), black_box(&instructions)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
