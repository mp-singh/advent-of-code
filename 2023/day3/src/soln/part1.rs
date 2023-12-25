pub fn part1(input: &str) -> u32 {
    let r = input
        .lines()
        .map(|l| {
            println!("{}", l);
            l.chars().for_each(|c| {
                if c.is_numeric() {
                    // println!("{}", c);
                }
            })
        })
        .collect::<Vec<_>>();
    0
}
