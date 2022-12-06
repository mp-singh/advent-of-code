use day4::{parse_input, part1, part2};

fn main() {
    let input = parse_input();
    let data = input.iter().map(|s| s.as_str()).collect::<Vec<_>>();
    println!("Part 1: {:?}", part1(&data));
    println!("Part 2: {:?}", part2(&data));
}
