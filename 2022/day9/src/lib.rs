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

    let mut head = Point::new(0, 0);
    let mut tail = Point::new(0, 0);
    let mut visited: HashSet<Point> = HashSet::new();
    visited.insert(tail);

    instructions
        .iter()
        .for_each(|instruction| match instruction {
            Instruction::Move(direction, distance) => {
                for _ in 0..*distance {
                    match direction {
                        Direction::Up => {
                            head = Point::new(head.row - 1, head.col);
                            if (head.row - tail.row).abs() > 1 {
                                tail = Point::new(tail.row - 1, head.col);
                                visited.insert(tail);
                            }
                        }
                        Direction::Down => {
                            head = Point::new(head.row + 1, head.col);
                            if (head.row - tail.row).abs() > 1 {
                                tail = Point::new(tail.row + 1, head.col);
                                visited.insert(tail);
                            }
                        }
                        Direction::Left => {
                            head = Point::new(head.row, head.col - 1);
                            if (head.col - tail.col).abs() > 1 {
                                tail = Point::new(head.row, tail.col - 1);
                                visited.insert(tail);
                            }
                        }
                        Direction::Right => {
                            head = Point::new(head.row, head.col + 1);
                            if (head.col - tail.col).abs() > 1 {
                                tail = Point::new(head.row, tail.col + 1);
                                visited.insert(tail);
                            }
                        }
                    }
                }
            }
        });
    visited.len()
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
}
