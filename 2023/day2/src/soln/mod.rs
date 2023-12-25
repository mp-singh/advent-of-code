pub mod part1;
pub mod part2;

pub fn parse_input() -> String {
    let input = include_str!("../../input.txt");
    input.to_string()
}

#[derive(Debug)]
pub struct Game {
    pub id: u32,
    pub sets: Vec<Set>,
}

impl Game {
    pub fn new(id: u32, sets: Vec<Set>) -> Self {
        Self { id, sets }
    }
}

#[derive(Debug, Clone)]
pub struct Set {
    pub color: Vec<Color>,
}

#[derive(Debug, Clone)]
pub enum Color {
    BLUE(u32),
    RED(u32),
    GREEN(u32),
}

impl Color {
    pub fn new(color: &str, count: u32) -> Self {
        match color {
            "blue" => Self::BLUE(count),
            "red" => Self::RED(count),
            "green" => Self::GREEN(count),
            _ => panic!("Invalid color"),
        }
    }
    pub fn increment(&mut self) {
        match self {
            Self::BLUE(n) => *n += 1,
            Self::RED(n) => *n += 1,
            Self::GREEN(n) => *n += 1,
        }
    }
}

pub fn parse_into_game(input: &str) -> Vec<Game> {
    input
        .lines()
        .map(|line| {
            let iter: Vec<_> = line.split(": ").collect();
            let id = iter[0].split("Game ").collect::<Vec<_>>()[1]
                .parse::<u32>()
                .unwrap();
            let set: Vec<_> = iter[1].split("; ").collect();
            let result: Vec<Set> = set
                .iter()
                .map(|color_set| {
                    color_set
                        .split(", ")
                        .map(|c| {
                            let color: Vec<_> = c.split(' ').collect();
                            let count = color[0].parse::<u32>().unwrap();
                            Color::new(color[1], count)
                        })
                        .collect::<Vec<Color>>()
                })
                .map(|color| Set { color })
                .collect();
            Game::new(id, result)
        })
        .collect::<Vec<Game>>()
}
