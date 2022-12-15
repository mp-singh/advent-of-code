use std::collections::HashMap;

use regex::Regex;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    pub static ref STARTING: Regex = Regex::new(r"divisible by (\d+)").unwrap();
    pub static ref OPERATION: Regex = Regex::new(r"Operation: new = old (\+|\*) (\d+)").unwrap();
    pub static ref DIVISIBLE_BY: Regex = Regex::new(r"divisible by (\d+)").unwrap();
    pub static ref TEST: Regex = Regex::new(r"If (true|false): throw to monkey (\d+)").unwrap();
}

pub fn parse_input() -> String {
    include_str!("../input.txt").trim_end().to_string()
}

// pub fn part1(input: &str) -> i32 {
//     let monkeys = input.split('\n').map(Command::from_str).collect::<Vec<_>>();
// }

#[derive(Debug)]
pub struct Monkey {
    id: u32,
    starting_items: Vec<i32>,
    operation: Operation,
    operation_value: u32,
    test_operation: HashMap<String, u32>,
}

impl Monkey {}

#[derive(Debug)]
pub enum Operation {
    Add,
    Multiply,
}

impl Operation {
    fn from_str(s: &str) -> Self {
        match s {
            "+" => Operation::Add,
            "*" => Operation::Multiply,
            _ => panic!("Invalid operation: {}", s),
        }
    }
}

mod test {
    use super::*;

    #[test]
    fn test_regex() {
        let input = 
        r"Monkey 3:
        Starting items: 74
        Operation: new = old + 3
        Test: divisible by 17
          If true: throw to monkey 0
          If false: throw to monkey 1
        #";
        let monkey = parse_monkeys(input);
        println!("{:?}", monkey);

    }
}

fn parse_monkeys(input: &str) -> Monkey {
    // Split the input string into lines
    let lines: Vec<&str> = input.split('\n').collect();

    lines.iter().for_each(|l| println!("line: {}", l));
    // Parse the Monkey id
    let id_str = lines[0].split(':').collect::<Vec<&str>>()[0];
    println!("test[{:?}]", id_str.split_whitespace().collect::<Vec<&str>>()[1].replace("\"", ""));
    let id: u32 = id_str.split_whitespace().collect::<Vec<&str>>()[0]
        .parse::<u32>()
        .expect("Failed to parse id");

    // Parse the starting items
    let starting_items_str = lines[1].split(':').collect::<Vec<&str>>()[1];
    let starting_items: Vec<i32> = starting_items_str
        .split(',')
        .map(|x| x.trim().parse().expect("Failed to parse item"))
        .collect();

    // Parse the operation
    let operation_str = lines[2].split(':').collect::<Vec<&str>>()[1];
    let operation_split: Vec<&str> = operation_str.split('=').collect();
    let operation_value: u32 = operation_split[1]
        .trim()
        .parse()
        .expect("Failed to parse operation value");
    let operation = Operation::from_str(operation_split[0].trim());

    // Parse the test operation
    let test_operation_str = lines[4].split(':').collect::<Vec<&str>>()[1];
    let test_operation_value: u32 = test_operation_str.split_whitespace().collect::<Vec<&str>>()[3]
        .parse()
        .expect("Failed to parse test operation value");
    let test_operation = {
        let mut map = HashMap::new();
        // Parse the true and false branches of the test operation
        map.insert(
            "If true".to_string(),
            lines[5].split_whitespace().collect::<Vec<&str>>()[3]
                .parse()
                .expect("Failed to parse true branch"),
        );
        map.insert(
            "If false".to_string(),
            lines[6].split_whitespace().collect::<Vec<&str>>()[3]
                .parse()
                .expect("Failed to parse false branch"),
        );
        map
    };

    // Return the resulting Monkey structure
    Monkey {
        id,
        starting_items,
        operation,
        operation_value,
        test_operation: test_operation,
    }
}
