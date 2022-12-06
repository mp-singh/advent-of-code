use day5::{parse_input, part1, part2};

fn main() {
    let (instructions, stack) = parse_input();
    println!("Part 1: {:?}", part1(stack.clone(), &instructions));
    println!("Part 2: {:?}", part2(stack, &instructions));
}
