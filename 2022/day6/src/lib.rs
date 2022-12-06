use itertools::Itertools;

pub fn parse_input() -> String {
    include_str!("../input.txt").to_string()
}

pub fn part1(input: &str) -> usize {
    find_marker(input, 4)
}

pub fn part2(input: &str) -> usize {
    find_marker(input, 14)
}

fn find_marker(input: &str, sliding_window: usize) -> usize {
    for i in 0..(input.len() - 1 - sliding_window) {
        let chunk = &mut input[i..i + sliding_window]
            .chars()
            .unique()
            .collect::<Vec<char>>();
        if chunk.len() == sliding_window {
            return i + sliding_window;
        }
    }
    0
}
