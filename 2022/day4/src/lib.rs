use itertools::Itertools;

pub fn parse_input() -> Vec<String> {
    let input_str = include_str!("../input.txt");
    input_str.lines().map(|line| line.to_string()).collect()
}

pub fn part1(arr: &[&str]) -> i32 {
    let result = arr
        .iter()
        .map(|elm| elm.split(',').collect::<Vec<_>>())
        .map(|e| (e[0], e[1]))
        .collect::<Vec<_>>();

    let mut count = 0;
    for (a, b) in &result {
        let split_a: Vec<&str> = a.split('-').collect();
        let split_b: Vec<&str> = b.split('-').collect();
        let start_a = split_a[0].parse::<i32>().unwrap();
        let end_a = split_a[1].parse::<i32>().unwrap();
        let start_b = split_b[0].parse::<i32>().unwrap();
        let end_b = split_b[1].parse::<i32>().unwrap();

        if start_a <= start_b && end_b <= end_a || start_b <= start_a && end_a <= end_b {
            count += 1;
        }
    }
    count
}

pub fn part2(arr: &[&str]) -> i32 {
    let result = arr
        .iter()
        .map(|elm| elm.split(',').collect::<Vec<_>>())
        .map(|e| (e[0], e[1]))
        .collect::<Vec<_>>();

    let mut count = 0;
    for (a, b) in &result {
        let split_a: Vec<&str> = a.split('-').collect();
        let split_b: Vec<&str> = b.split('-').collect();
        let start_a = split_a[0].parse::<i32>().unwrap();
        let end_a = split_a[1].parse::<i32>().unwrap();
        let start_b = split_b[0].parse::<i32>().unwrap();
        let end_b = split_b[1].parse::<i32>().unwrap();

        // (start_a, end_a) overlaps (start_b, end_b) if it is not completely before (to the left) or completely after (to the right).
        if !(end_a < start_b || start_a > end_b) {
            count += 1;
        }
    }
    count
}

pub fn part1_fast(data: &str) -> usize {
    data.lines()
        .map(|l| {
            l.split(['-', ','])
                .map(|v| v.parse::<usize>().unwrap())
                .collect_tuple::<(_, _, _, _)>()
                .unwrap()
        })
        .filter(|(s1, e1, s2, e2)| (s1 <= s2 && e1 >= e2) || (s2 <= s1 && e2 >= e1))
        .count()
}

pub fn part2_fast(data: &str) -> usize {
    data.lines()
        .map(|l| {
            l.split(&['-', ','][..])
                .map(|v| v.parse::<usize>().unwrap())
                .collect_tuple::<(_, _, _, _)>()
                .unwrap()
        })
        .filter(|(s1, e1, s2, e2)| (s1 <= s2 && e1 >= s2) || (s2 <= s1 && e2 >= s1))
        .count()
}
