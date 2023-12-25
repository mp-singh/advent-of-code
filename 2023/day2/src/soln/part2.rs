use crate::soln::parse_into_game;

use super::Color;

pub fn part2(input: &str) -> u32 {
    let games = parse_into_game(input);
    let mut sum = 0;
    games.iter().for_each(|g| {
        let mut blue_max = 0;
        let mut red_max = 0;
        let mut green_max = 0;
        g.sets.iter().for_each(|s| {
            s.color.iter().for_each(|c| match c {
                Color::BLUE(n) => {
                    if n > &blue_max {
                        blue_max = *n;
                    }
                }
                Color::RED(n) => {
                    if n > &red_max {
                        red_max = *n;
                    }
                }
                Color::GREEN(n) => {
                    if n > &green_max {
                        green_max = *n;
                    }
                }
            })
        });
        sum += blue_max * red_max * green_max;
    });
    sum
}
