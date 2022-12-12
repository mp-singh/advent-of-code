use std::collections::HashSet;

use crate::models::{Direction, Instruction, Point};

mod models;

pub fn parse_input() -> String {
    include_str!("../input.txt").trim_end().to_string()
}

pub fn part1(input: &str) -> usize {
    let instructions = input
        .split('\n')
        .map(Instruction::from_str)
        .collect::<Vec<_>>();

    let mut snake = [Point { row: 0, col: 0 }; 2].to_vec();
    let mut visited: HashSet<Point> = HashSet::new();

    for instruction in instructions {
        match instruction {
            Instruction::Move(direction, distance) => {
                for _ in 0..distance {
                    move_snake(&mut snake, &direction, &mut visited);
                }
            }
        }
    }
    visited.len()
}

pub fn part2(input: &str) -> usize {
    let instructions = input
        .split('\n')
        .map(Instruction::from_str)
        .collect::<Vec<_>>();

    let mut snake = [Point { row: 0, col: 0 }; 10].to_vec();
    let mut visited: HashSet<Point> = HashSet::new();
    visited.insert(snake[0]);
    for instruction in instructions {
        match instruction {
            Instruction::Move(direction, distance) => {
                for _ in 0..distance {
                    move_snake(&mut snake, &direction, &mut visited);
                }
            }
        }
    }
    visited.len()
}

fn move_snake(snake: &mut Vec<Point>, direction: &Direction, visited: &mut HashSet<Point>) {
    let head = snake[0];
    snake[0] = match direction {
        Direction::Up => Point::new(head.row - 1, head.col),
        Direction::Down => Point::new(head.row + 1, head.col),
        Direction::Left => Point::new(head.row, head.col - 1),
        Direction::Right => Point::new(head.row, head.col + 1),
    };

    for i in 1..snake.len() {
        let head = snake[i - 1];
        let mut tail = snake[i];
        let row_diff = (head.row - tail.row).abs();
        let col_diff = (head.col - tail.col).abs();
        if row_diff > col_diff {
            if head.row - tail.row > 1 {
                tail = Point::new(tail.row + 1, head.col);
            }
            if tail.row - head.row > 1 {
                tail = Point::new(tail.row - 1, head.col);
            }
        } else if row_diff < col_diff {
            if head.col - tail.col > 1 {
                tail = Point::new(head.row, tail.col + 1);
            }
            if tail.col - head.col > 1 {
                tail = Point::new(head.row, tail.col - 1);
            }
        } else if row_diff > 1 {
            if head.row - tail.row > 1 {
                tail = Point::new(tail.row + 1, tail.col);
            }
            if tail.row - head.row > 1 {
                tail = Point::new(tail.row - 1, tail.col);
            }
            if head.col - tail.col > 1 {
                tail = Point::new(tail.row, tail.col + 1);
            }
            if tail.col - head.col > 1 {
                tail = Point::new(tail.row, tail.col - 1);
            }
        }
        snake[i] = tail;
        if i == snake.len() - 1 {
            visited.insert(tail);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "R 4
        U 4
        L 3
        D 1
        R 4
        D 1
        L 5
        R 2";
        assert_eq!(part1(input), 13);
    }
    #[test]
    fn test_part2() {
        let input = "R 5
        U 8
        L 8
        D 3
        R 17
        D 10
        L 25
        U 20";
        assert_eq!(part2(input), 36);
    }
}
