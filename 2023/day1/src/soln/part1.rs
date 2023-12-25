pub fn part1(input: &str) -> i32 {
    let mut numbers: Vec<i32> = Vec::new();
    for line in input.lines() {
        let mut temp_nums = vec![];
        for c in line.chars() {
            if c.is_ascii_digit() {
                temp_nums.push(c);
            }
        }
        let str = format!(
            "{:?}{:?}",
            temp_nums[0].to_digit(10).unwrap(),
            temp_nums[temp_nums.len() - 1].to_digit(10).unwrap()
        );
        numbers.push(str.parse::<i32>().unwrap());
    }
    numbers.iter().sum()
}

pub fn part1_rev1(input: &str) -> i32 {
    let mut numbers: Vec<i32> = Vec::new();
    for line in input.lines() {
        let arr = line
            .chars()
            .filter(|c| c.is_ascii_digit())
            .map(|c| c.to_digit(10).unwrap() as i32)
            .collect::<Vec<i32>>();
        numbers.push(arr[0] * 10 + arr[arr.len() - 1]);
    }
    numbers.iter().sum()
}

pub fn part1_rev2(input: &str) -> i32 {
    input
        .lines()
        .map(|line| {
            let arr = line
                .chars()
                .filter(|c| c.is_ascii_digit())
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect::<Vec<i32>>();
            arr[0] * 10 + arr[arr.len() - 1]
        })
        .sum()
}
#[cfg(test)]
mod tests {

    use crate::soln::parse_input;

    use super::*;

    #[test]
    fn test_part1() {
        let input = parse_input();
        assert_eq!(part1(&input), 54968);
    }

    #[test]
    fn test_part1_rev1() {
        let input = parse_input();
        assert_eq!(part1_rev1(&input), 54968);
    }

    #[test]
    fn test_part1_rev2() {
        let input = parse_input();
        assert_eq!(part1_rev2(&input), 54968);
    }
}
