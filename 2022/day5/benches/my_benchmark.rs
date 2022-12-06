use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day5::{parse_input, part1, part2};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("day5-part1", |b| {
        b.iter(|| {
            let (instructions, stack) = parse_input();
            part1(black_box(stack), black_box(&instructions))
        })
    });
    c.bench_function("day5-part2", |b| {
        b.iter(|| {
            let (instructions, stack) = parse_input();
            part2(black_box(stack), black_box(&instructions))
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
