#[derive(Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn from_char(c: char) -> Self {
        match c {
            'U' => Self::Up,
            'D' => Self::Down,
            'L' => Self::Left,
            'R' => Self::Right,
            _ => panic!("Invalid direction: {}", c),
        }
    }
}

#[derive(Debug)]
pub enum Instruction {
    Move(Direction, i32),
}

impl Instruction {
    pub fn from_str(s: &str) -> Self {
        let mut split = s.split_whitespace();
        let dir = split.next().unwrap();
        let dist = split.next().unwrap();
        Self::Move(
            Direction::from_char(dir.chars().next().unwrap()),
            dist.parse().unwrap(),
        )
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Point {
    pub row: i32,
    pub col: i32,
}
impl Point {
    pub fn new(row: i32, col: i32) -> Self {
        Self { row, col }
    }
}
