use std::fs::read_to_string;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day4::{part1, part1_fast, part2, part2_fast};

fn parse_input() -> Vec<String> {
    let input_str = read_to_string("input.txt").unwrap();
    input_str.lines().map(|line| line.to_string()).collect()
}

fn criterion_benchmark(c: &mut Criterion) {
    let input = parse_input();
    let data = input.iter().map(|s| s.as_str()).collect::<Vec<_>>();
    c.bench_function("day4-part1", |b| b.iter(|| part1(black_box(&data))));
    c.bench_function("day4-part2", |b| b.iter(|| part2(black_box(&data))));
}

fn criterion_benchmark_2(c: &mut Criterion) {
    let input = include_str!("../input.txt");
    c.bench_function("day4-part1_fast", |b| {
        b.iter(|| part1_fast(black_box(&input)))
    });
    c.bench_function("day4-part2_fast", |b| {
        b.iter(|| part2_fast(black_box(&input)))
    });
}
criterion_group!(benches, criterion_benchmark, criterion_benchmark_2);
criterion_main!(benches);
