use std::fs::read_to_string;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day3::{part1, part2};

fn parse_input() -> Vec<String> {
    let input_str = read_to_string("input.txt").unwrap();
    input_str.lines().map(|line| line.to_string()).collect()
}

fn criterion_benchmark(c: &mut Criterion) {
    let arr = parse_input();
    let input = arr.iter().map(|s| s.as_str()).collect::<Vec<_>>();
    c.bench_function("day3-part1", |b| b.iter(|| part1(black_box(&input))));
    c.bench_function("day3-part2", |b| b.iter(|| part2(black_box(&input))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
