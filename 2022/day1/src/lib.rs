pub fn parse_input() -> Vec<Vec<i32>> {
    let input_str = include_str!("../input.txt");
    input_str
        .split("\n\n")
        .map(|rations| {
            rations
                .split('\n')
                .map(|ration| ration.parse().unwrap())
                .collect()
        })
        .collect()
}

pub fn part1(arr: &[Vec<i32>]) -> i32 {
    arr.iter().map(|ration| ration.iter().sum()).max().unwrap()
}

pub fn part2(arr: &[Vec<i32>]) -> i32 {
    let mut sum: Vec<i32> = arr.iter().map(|ration| ration.iter().sum()).collect();
    sum.sort();
    sum.reverse();

    sum[0..3].iter().sum::<i32>()
}
