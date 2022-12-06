pub fn parse_input() -> Vec<(OponentsHand, MyHand)> {
    let input_str = include_str!("../input.txt");
    input_str
        .split("\n")
        .map(|line| {
            let mut iter = line.split(" ");
            let ps1 = iter.next().unwrap();
            let ps2 = iter.next().unwrap();
            (OponentsHand::from(ps1), MyHand::from(ps2))
        })
        .collect()
}

pub fn part1(arr: &Vec<(OponentsHand, MyHand)>) -> i32 {
    let mut answer = 0;
    for (f1, f2) in arr {
        answer += match f2 {
            MyHand::Rock => {
                1 + match f1 {
                    OponentsHand::Rock => 3,
                    OponentsHand::Paper => 0,
                    OponentsHand::Scissors => 6,
                }
            }
            MyHand::Paper => {
                2 + match f1 {
                    OponentsHand::Rock => 6,
                    OponentsHand::Paper => 3,
                    OponentsHand::Scissors => 0,
                }
            }
            MyHand::Scissors => {
                3 + match f1 {
                    OponentsHand::Rock => 0,
                    OponentsHand::Paper => 6,
                    OponentsHand::Scissors => 3,
                }
            }
        }
    }
    answer
}

pub fn part2(arr: &Vec<(OponentsHand, MyHand)>) -> i32 {
    let mut answer = 0;
    for (f1, f2) in arr {
        answer += match f1 {
            OponentsHand::Rock => match f2 {
                MyHand::Rock => 3 + 0,
                MyHand::Paper => 1 + 3,
                MyHand::Scissors => 2 + 6,
            },
            OponentsHand::Paper => match f2 {
                MyHand::Rock => 1 + 0,
                MyHand::Paper => 2 + 3,
                MyHand::Scissors => 3 + 6,
            },
            OponentsHand::Scissors => match f2 {
                MyHand::Rock => 2 + 0,
                MyHand::Paper => 3 + 3,
                MyHand::Scissors => 1 + 6,
            },
        }
    }
    answer
}

#[derive(Debug, Clone, Copy)]
pub enum OponentsHand {
    Rock,
    Paper,
    Scissors,
}

impl From<&str> for OponentsHand {
    fn from(s: &str) -> Self {
        match s {
            "A" => OponentsHand::Rock,
            "B" => OponentsHand::Paper,
            "C" => OponentsHand::Scissors,
            _ => panic!("Invalid strategy"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum MyHand {
    Rock,
    Paper,
    Scissors,
}
impl From<&str> for MyHand {
    fn from(s: &str) -> Self {
        match s {
            "X" => MyHand::Rock,
            "Y" => MyHand::Paper,
            "Z" => MyHand::Scissors,
            _ => panic!("Unknown strategy: {}", s),
        }
    }
}
