use aho_corasick::AhoCorasick;

const NUMBER_TO_WORDS_MAP: [(&str, &str); 10] = [
    ("zero", "zero0zero"),
    ("one", "one1one"),
    ("two", "two2two"),
    ("three", "three3three"),
    ("four", "four4four"),
    ("five", "five5five"),
    ("six", "six6six"),
    ("seven", "seven7seven"),
    ("eight", "eight8eight"),
    ("nine", "nine9nine"),
];

pub fn part2(input: &str) -> i32 {
    let mut numbers: Vec<i32> = Vec::new();
    for line in input.lines() {
        let arr = NUMBER_TO_WORDS_MAP
            .iter()
            .fold(line.to_string(), |acc, word| {
                if acc.contains(word.0) && word.0.len() <= acc.len() {
                    acc.replace(word.0, word.1)
                } else {
                    acc
                }
            })
            .chars()
            .filter(|c| c.is_ascii_digit())
            .map(|c| c.to_digit(10).unwrap() as i32)
            .collect::<Vec<i32>>();
        numbers.push(arr[0] * 10 + arr[arr.len() - 1]);
    }
    numbers.iter().sum()
}

pub fn part2_rev1(input: &str) -> i32 {
    input
        .lines()
        .map(|line| {
            let arr = NUMBER_TO_WORDS_MAP
                .iter()
                .fold(line.to_string(), |acc, word| {
                    if acc.contains(word.0) && word.0.len() <= acc.len() {
                        acc.replace(word.0, word.1)
                    } else {
                        acc
                    }
                })
                .chars()
                .filter(|c| c.is_ascii_digit())
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect::<Vec<i32>>();
            arr[0] * 10 + arr[arr.len() - 1]
        })
        .sum()
}

pub fn part2_rev2(input: &str) -> i32 {
    let mut sum = 0;
    let ac = AhoCorasick::new([
        "one", "1", "two", "2", "three", "3", "four", "4", "five", "5", "six", "6", "seven", "7",
        "eight", "8", "nine", "9",
    ])
    .unwrap();
    for line in input.lines() {
        let matches = ac.find_overlapping_iter(line).collect::<Vec<_>>();
        let first = matches.first().unwrap().pattern().as_i32() / 2 + 1;
        let last = matches.iter().last().unwrap().pattern().as_i32() / 2 + 1;
        sum += first * 10 + last;
    }
    sum
}

#[cfg(test)]
mod tests {
    use crate::soln::parse_input;

    use super::*;

    #[test]
    fn test_part2() {
        let input = parse_input();
        assert_eq!(part2(&input), 54094);
    }

    #[test]
    fn test_part2_rev1() {
        let input = parse_input();
        assert_eq!(part2_rev1(&input), 54094);
    }

    #[test]
    fn test_part2_rev2() {
        let input = parse_input();
        assert_eq!(part2_rev2(&input), 54094);
    }
}
