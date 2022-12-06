pub fn parse_input() -> (Vec<Instruction>, Vec<Vec<char>>) {
    let data = include_str!("../input.txt").trim_end();
    let parts = data.split("\n\n").collect::<Vec<_>>();

    let instructions = parts[1]
        .split('\n')
        .map(Instruction::from)
        .collect::<Vec<Instruction>>();

    let mut stacks: Vec<Vec<char>> = Vec::new();
    parts[0].lines().rev().skip(1).for_each(|line| {
        line.chars()
            .skip(1)
            .step_by(4)
            .enumerate()
            .filter(|(_, ch)| !ch.is_ascii_whitespace())
            .for_each(|(i, ch)| {
                if i >= stacks.len() {
                    let crates_vec = vec![ch];
                    stacks.push(crates_vec)
                } else {
                    stacks[i].push(ch);
                }
            })
    });
    (instructions, stacks)
}

pub fn part1(mut stack: Vec<Vec<char>>, instructions: &Vec<Instruction>) -> String {
    for instruction in instructions {
        let mut crates = stack[instruction.from - 1].clone();
        let mut crates_to_move = crates.split_off(crates.len() - instruction.mv);
        crates_to_move.reverse();
        stack[instruction.to - 1].append(&mut crates_to_move);
        stack[instruction.from - 1] = crates;
    }
    stack.iter().map(|e| e[e.len() - 1]).collect::<String>()
}

pub fn part2(mut stack: Vec<Vec<char>>, instructions: &Vec<Instruction>) -> String {
    for instruction in instructions {
        let mut crates = stack[instruction.from - 1].clone();
        let mut crates_to_move = crates.split_off(crates.len() - instruction.mv);
        stack[instruction.to - 1].append(&mut crates_to_move);
        stack[instruction.from - 1] = crates;
    }
    stack.iter().map(|e| e[e.len() - 1]).collect::<String>()
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Instruction {
    pub mv: usize,
    pub from: usize,
    pub to: usize,
}

impl From<&str> for Instruction {
    fn from(s: &str) -> Self {
        let iter = s.split(' ');
        let mv = iter.to_owned().nth(1).unwrap().parse::<usize>().unwrap();
        let from = iter
            .to_owned()
            .nth(3)
            .unwrap()
            .parse::<usize>()
            .unwrap()
            .to_owned();
        let to = iter
            .to_owned()
            .nth(5)
            .unwrap()
            .parse::<usize>()
            .unwrap()
            .to_owned();
        Instruction { mv, from, to }
    }
}
