use itertools::Itertools;

pub fn parse_input() -> String {
    include_str!("../input.txt").to_string()
}

pub fn part1(input: &str) -> usize {
    let sliding_window = 4;
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

pub fn part2(input: &str) -> usize {
    let sliding_window = 14;
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
