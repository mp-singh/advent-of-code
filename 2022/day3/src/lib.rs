use std::collections::HashMap;

use itertools::Itertools;

lazy_static::lazy_static! {
    static ref APLHABET: HashMap<&'static str, i32> = HashMap::from([
    ("a", 1),
    ("b", 2),
    ("c", 3),
    ("d", 4),
    ("e", 5),
    ("f", 6),
    ("g", 7),
    ("h", 8),
    ("i", 9),
    ("j", 10),
    ("k", 11),
    ("l", 12),
    ("m", 13),
    ("n", 14),
    ("o", 15),
    ("p", 16),
    ("q", 17),
    ("r", 18),
    ("s", 19),
    ("t", 20),
    ("u", 21),
    ("v", 22),
    ("w", 23),
    ("x", 24),
    ("y", 25),
    ("z", 26),
    ("A", 27),
    ("B", 28),
    ("C", 29),
    ("D", 30),
    ("E", 31),
    ("F", 32),
    ("G", 33),
    ("H", 34),
    ("I", 35),
    ("J", 36),
    ("K", 37),
    ("L", 38),
    ("M", 39),
    ("N", 40),
    ("O", 41),
    ("P", 42),
    ("Q", 43),
    ("R", 44),
    ("S", 45),
    ("T", 46),
    ("U", 47),
    ("V", 48),
    ("W", 49),
    ("X", 50),
    ("Y", 51),
    ("Z", 52),
]);}
pub fn part1(arr: &[&str]) -> i32 {
    arr.iter()
        .map(|line| {
            let split = line.split_at(line.len() / 2);
            (split.0, split.1)
        })
        .map(|(a, b)| {
            a.chars()
                .filter(|c| b.contains(*c))
                .unique()
                .collect::<String>()
        })
        .map(|e| *APLHABET.get(e.as_str()).unwrap())
        .sum::<i32>()
}

pub fn part2(arr: &[&str]) -> i32 {
    arr.iter()
        .tuples()
        .map(|(a, b, c)| {
            a.chars()
                .filter(|s| b.contains(*s) && c.contains(*s))
                .unique()
                .collect::<String>()
        })
        .map(|e| *APLHABET.get(e.as_str()).unwrap())
        .sum::<i32>()
}

pub fn parse_input() -> Vec<String> {
    let input_str = include_str!("../input.txt");
    input_str.lines().map(|line| line.to_string()).collect()
}
