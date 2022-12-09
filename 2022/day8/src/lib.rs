use std::fmt::Display;

pub fn parse_input() -> String {
    include_str!("../input.txt").trim_end().to_string()
}

pub struct Forest {
    trees: Vec<Vec<i32>>,
    width: usize,
    height: usize,
}

impl Forest {
    pub fn new(input: &str) -> Self {
        Self {
            trees: input
                .lines()
                .map(|line| {
                    line.chars()
                        .map(|c| c.to_digit(10).unwrap() as i32)
                        .collect()
                })
                .collect(),
            width: input.lines().next().unwrap().len(),
            height: input.lines().count(),
        }
    }
}

impl Display for Forest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "width: {}, height: {}", self.width, self.height)?;
        for row in &self.trees {
            write!(f, "[")?;
            for col in row {
                write!(f, " {} ", col)?;
            }
            write!(f, "]")?;
            writeln!(f)?;
        }
        Ok(())
    }
}

pub fn part1(input: &str) -> i32 {
    let forest = Forest::new(input);
    let trees = &forest.trees;
    let mut count = 0;
    for (i, row) in trees.iter().enumerate() {
        for (j, _) in row.iter().enumerate() {
            if is_visible(trees, i, j) {
                count += 1;
            }
        }
    }
    count
}

fn is_visible(trees: &[Vec<i32>], row: usize, col: usize) -> bool {
    let tree = &trees[row][col];
    let r = &trees[row];
    if r[0..col].iter().all(|&x| x < *tree) || r[col + 1..].iter().all(|&x| x < *tree) {
        return true;
    }
    let c = trees.iter().map(|r| r[col]).collect::<Vec<_>>();
    if c[0..row].iter().all(|&x| x < *tree) || c[row + 1..].iter().all(|&x| x < *tree) {
        return true;
    }
    false
}

pub fn part2(input: &str) -> usize {
    let forest = Forest::new(input);
    let trees = &forest.trees;
    let mut score = 0;
    for (i, row) in trees.iter().enumerate() {
        for (j, _) in row.iter().enumerate() {
            if i == 0 || j == 0 || i == trees.len() - 1 || j == row.len() - 1 {
                continue;
            } else {
                score = scenic_score(trees, i, j).max(score);
            }
        }
    }
    score
}

fn scenic_score(trees: &[Vec<i32>], row: usize, col: usize) -> usize {
    let mut score = 1;

    let tree = &trees[row][col];
    let r = &trees[row];
    let mut t = r[0..col].iter().rev().take_while(|&x| x < tree).count();
    if t < r[0..col].len() {
        t += 1;
    }
    score *= t;

    let mut t = r[col + 1..].iter().take_while(|&x| x < tree).count();
    if t < r[col + 1..].len() {
        t += 1;
    }
    score *= t;

    let c = trees.iter().map(|r| r[col]).collect::<Vec<_>>();
    let mut t = c[0..row].iter().rev().take_while(|&x| x < tree).count();
    if t < c[0..row].len() {
        t += 1;
    }
    score *= t;
    let mut t = c[row + 1..].iter().take_while(|&x| x < tree).count();
    if t < c[row + 1..].len() {
        t += 1;
    }
    score *= t;
    score
}
