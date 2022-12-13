use std::{collections::HashMap, fmt::Display};

const SCREEN_WIDTH: i32 = 40;
const SCREEN_SIZE: i32 = 240;

pub fn parse_input() -> String {
    include_str!("../input.txt").trim_end().to_string()
}

pub fn part1(input: &str) -> i32 {
    let commands = input.split('\n').map(Command::from_str).collect::<Vec<_>>();
    let mut clock = Clock::new();
    for command in commands {
        clock.tick_p1();
        match &command {
            Command::Addx(x) => {
                clock.tick_p1();
                clock.register_value += x;
            }
            _ => continue,
        }
    }
    clock.total
}

pub fn part2(input: &str) -> Clock {
    let commands = input.split('\n').map(Command::from_str).collect::<Vec<_>>();
    let mut clock = Clock::default();
    for command in commands {
        clock.tick_p2();
        match &command {
            Command::Addx(x) => {
                clock.tick_p2();
                clock.register_value += x;
            }
            _ => continue,
        }
    }
    clock
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Command {
    Noop,
    Addx(i32),
}

impl Command {
    fn from_str(s: &str) -> Self {
        let mut parts = s.split_whitespace();
        let command = parts.next().unwrap();
        match command {
            "noop" => Command::Noop,
            "addx" => {
                let value = parts.next().unwrap().parse::<i32>().unwrap();
                Command::Addx(value)
            }
            _ => panic!("Invalid command: {}", command),
        }
    }
}

impl Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Command::Noop => write!(f, "noop"),
            Command::Addx(value) => write!(f, "addx {}", value),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Clock {
    cycle: i32,
    register_value: i32,
    threshold: i32,
    total: i32,
    pixels: HashMap<i32, char>,
}

impl Clock {
    pub fn new() -> Self {
        Self {
            cycle: 0,
            register_value: 1,
            threshold: 20,
            total: 0,
            pixels: HashMap::new(),
        }
    }
    fn tick_p1(&mut self) {
        self.cycle += 1;
        if self.cycle == self.threshold {
            self.total += self.register_value * self.threshold as i32;
            self.threshold += 40;
        }
    }
    fn tick_p2(&mut self) {
        let pos = self.cycle % SCREEN_WIDTH;
        match ((self.register_value - 1)..=(self.register_value + 1)).contains(&pos) {
            true => self.pixels.insert(self.cycle, '#'),
            false => self.pixels.insert(self.cycle, '.'),
        };
        self.cycle += 1;
    }
    pub fn display(self) {
        for i in 0..SCREEN_SIZE {
            if i % SCREEN_WIDTH == 0 {
                println!();
            }
            print!("{}", self.pixels.get(&i).unwrap_or(&' '));
        }
        println!();
    }
}

impl Default for Clock {
    fn default() -> Self {
        Self::new()
    }
}
