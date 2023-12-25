use day1::soln::{
    parse_input,
    part1::{part1, part1_rev1, part1_rev2},
    part2::{part2, part2_rev1, part2_rev2},
};

fn main() {
    let input = parse_input();
    println!("Part 1: {:?}", part1(&input));
    println!("Part 1 rev1: {:?}", part1_rev1(&input));
    println!("Part 1 rev2: {:?}", part1_rev2(&input));
    println!("Part 2: {}", part2(&input));
    println!("Part 2 rev1: {}", part2_rev1(&input));
    println!("Part 2 rev2: {}", part2_rev2(&input));
}
