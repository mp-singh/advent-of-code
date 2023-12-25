use crate::soln::Color;

use super::parse_into_game;

pub fn part1(input: &str) -> u32 {
    let games = parse_into_game(input);
    let mut game_id = 0;
    games.iter().for_each(|g| {
        let mut is_valid = true;
        g.sets.iter().for_each(|s| {
            s.color.iter().for_each(|c| match c {
                Color::BLUE(n) => {
                    if n > &14 {
                        is_valid = false;
                    }
                }
                Color::RED(n) => {
                    if n > &12 {
                        is_valid = false;
                    }
                }
                Color::GREEN(n) => {
                    if n > &13 {
                        is_valid = false;
                    }
                }
            })
        });
        if is_valid {
            game_id += g.id;
        }
    });
    game_id
}
