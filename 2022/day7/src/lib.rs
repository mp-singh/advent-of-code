use std::{cmp, collections::HashMap};

pub fn parse_input() -> String {
    include_str!("../input.txt").trim_end().to_string()
}

pub fn part1(input: &str) -> i32 {
    create_dir_map(input)
        .values()
        .filter(|v| **v <= 100000)
        .sum::<i32>()
}

pub fn part2(input: &str) -> i32 {
    let size_map = create_dir_map(input);
    let max_used = 40000000;
    let need_size = size_map.get("/").unwrap() - max_used;
    let mut best = 1000000000;
    for (_k, v) in size_map {
        if v >= need_size {
            best = cmp::min(best, v)
        }
    }
    best
}

fn create_dir_map(input: &str) -> HashMap<String, i32> {
    let mut path: Vec<&str> = Vec::new();
    let mut size_map: HashMap<String, i32> = HashMap::new();
    for line in input.lines() {
        let word = line.split(" ").collect::<Vec<&str>>();
        if word[1] == "cd" {
            if word[2] == ".." {
                path.pop();
            } else {
                path.push(word[2]);
            }
        } else if word[1] == "ls" || word[0] == "dir" {
            continue;
        } else {
            for i in 1..path.len() + 1 {
                let key = format!("{}", (&path[..i].join("/")).clone());
                if size_map.contains_key(&key) {
                    size_map.insert(
                        key.clone(),
                        size_map.get(&key).unwrap() + word[0].parse::<i32>().unwrap(),
                    );
                } else {
                    size_map.insert(key.clone(), word[0].parse::<i32>().unwrap());
                }
            }
        }
    }
    size_map
}
