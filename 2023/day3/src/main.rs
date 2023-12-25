use day3::soln::{parse_input, part1::part1, part2::part2};

fn main() {
    let input = parse_input();
    println!("Part 1: {:?}", part1(&input));
    println!("Part 2: {:?}", part2(&input));
}
