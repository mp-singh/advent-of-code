use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day11::{parse_input, RE};

fn criterion_benchmark(c: &mut Criterion) {
    // c.bench_function("day11-part1", |b| {
    //     b.iter(|| b.iter(|| sum(black_box(1), black_box(2))))
    // });
    // c.bench_function("day11-part2", |b| {
    //     b.iter(|| part2(black_box(&parse_input())))
    // });
    c.bench_function("day11-regex", |b| b.iter(|| test_regex()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

fn test_regex() {
    let input = r"
    Monkey 3:
    Starting items: 74
    Operation: new = old + 3
    Test: divisible by 17
      If true: throw to monkey 0
      If false: throw to monkey 1
    #";
    let result = RE.captures_iter(input).collect::<Vec<_>>();
    // println!("{:?}", result);
    // assert_eq!(result.get(1).unwrap().as_str(), "123");
}
