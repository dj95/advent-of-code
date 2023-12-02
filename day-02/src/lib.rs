use anyhow::{bail, Result};
use regex::Regex;

pub fn read_lines() -> Vec<String> {
    let mut res = Vec::new();

    let content = include_str!("../input.txt");
    for line in content.lines() {
        res.push(line.to_string());
    }

    res
}

pub struct Game {
    pub id: u32,
    pub sets: Vec<Set>,
}

impl Game {
    pub fn from_string(inp: String) -> Result<Self> {
        let id = extract_game_id(inp.clone());
        if id.is_none() {
            bail!("cannot get game id");
        }

        Ok(Self {
            id: id.unwrap(),
            sets: sets_from_string(inp),
        })
    }

    pub fn minimal_set(&self) -> Set {
        let mut output = Set::from_string("".to_string());

        for set in &self.sets {
            if set.blue > output.blue {
                output.blue = set.blue;
            }

            if set.green > output.green {
                output.green = set.green;
            }

            if set.red > output.red {
                output.red = set.red;
            }
        }

        output
    }

    pub fn is_valid(&self, total: Set) -> bool {
        for set in self.sets.iter() {
            if set.is_valid(total.clone()) {
                continue;
            }

            return false;
        }

        true
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Set {
    pub blue: u32,
    pub green: u32,
    pub red: u32,
}

impl Set {
    pub fn from_string(inp: String) -> Self {
        Self {
            blue: extract_color_count(inp.clone(), "blue".to_string()),
            green: extract_color_count(inp.clone(), "green".to_string()),
            red: extract_color_count(inp, "red".to_string()),
        }
    }

    pub fn power(&self) -> u32 {
        self.blue * self.green * self.red
    }

    pub fn is_valid(&self, total: Set) -> bool {
        self.red <= total.red && self.blue <= total.blue && self.green <= total.green
    }
}

fn sets_from_string(inp: String) -> Vec<Set> {
    let mut output = Vec::new();
    let parts: Vec<&str> = inp.split(':').collect();
    let parts: Vec<&str> = parts[1].split(';').collect();

    for part in parts {
        let set = Set::from_string(part.to_string());

        output.push(set);
    }

    output
}

fn extract_color_count(inp: String, color: String) -> u32 {
    let pattern = Regex::new(format!("(\\d+) {}", color).as_str()).unwrap();

    match pattern.captures(&inp) {
        Some(cap) => (cap[1]).parse::<u32>().unwrap_or(0),
        None => 0,
    }
}

fn extract_game_id(inp: String) -> Option<u32> {
    let pattern = Regex::new("Game (\\d+):").unwrap();
    let captures = pattern.captures(&inp).unwrap();

    match (captures[1]).parse::<u32>() {
        Ok(num) => Some(num),
        Err(_) => None,
    }
}

pub fn part_one(inp: Vec<String>, total: Set) -> u32 {
    let mut output = 0;

    for line in inp {
        let game = Game::from_string(line.to_string());

        if game.is_err() {
            continue;
        }

        let game = game.unwrap();

        if !game.is_valid(total.clone()) {
            continue;
        }

        output += game.id;
    }

    output
}

pub fn part_two(inp: Vec<String>) -> u32 {
    let mut output = 0;

    for line in inp {
        let game = Game::from_string(line.to_string());

        if game.is_err() {
            continue;
        }

        output += game.unwrap().minimal_set().power();
    }

    output
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    pub fn test_part_two() {
        let input = Vec::from([
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green".to_string(),
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue".to_string(),
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red".to_string(),
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red".to_string(),
            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green".to_string(),
        ]);

        let res = part_two(input);

        assert_eq!(res, 2286);
    }

    #[test]
    pub fn test_minimal_set() {
        let game =
            Game::from_string("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green".to_string())
                .unwrap();

        let res = game.minimal_set();
        assert_eq!(
            res,
            Set {
                blue: 6,
                green: 2,
                red: 4,
            },
        );
    }

    #[test]
    pub fn test_set_power() {
        let power = Set::from_string("6 blue, 2 green, 4 red".to_string()).power();
        assert_eq!(power, 48);
    }

    #[test]
    pub fn test_part_one() {
        let input = Vec::from([
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green".to_string(),
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue".to_string(),
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red".to_string(),
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red".to_string(),
            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green".to_string(),
        ]);

        let total_set = Set {
            blue: 14,
            green: 13,
            red: 12,
        };

        let res = part_one(input, total_set);

        assert_eq!(res, 8);
    }

    #[test]
    pub fn test_game_is_valid() {
        let game =
            Game::from_string("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green".to_string());

        assert!(game.is_ok());
        let game = game.unwrap();

        let res = game.is_valid(Set {
            blue: 6,
            green: 2,
            red: 4,
        });
        assert!(res);

        let res = game.is_valid(Set {
            blue: 5,
            green: 2,
            red: 4,
        });
        assert!(!res);
    }

    #[test]
    pub fn test_set_is_valid() {
        let set = Set::from_string("3 blue, 4 red".to_string());
        let res = set.is_valid(Set {
            blue: 6,
            green: 5,
            red: 4,
        });

        assert!(res);

        let res = set.is_valid(Set {
            blue: 2,
            green: 5,
            red: 4,
        });

        assert!(!res);
    }

    #[test]
    pub fn test_sets_from_string() {
        let res =
            sets_from_string("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green".to_string());

        assert_eq!(
            res,
            Vec::from([
                Set {
                    blue: 3,
                    green: 0,
                    red: 4
                },
                Set {
                    blue: 6,
                    green: 2,
                    red: 1
                },
                Set {
                    blue: 0,
                    green: 2,
                    red: 0
                },
            ])
        );
    }

    #[test]
    pub fn test_set_from_string() {
        let res = Set::from_string("3 blue, 4 red".to_string());

        assert_eq!(res.blue, 3);
        assert_eq!(res.green, 0);
        assert_eq!(res.red, 4);
    }

    #[test]
    pub fn test_extract_color_count() {
        let res = extract_color_count("3 blue, 4 red".to_string(), "blue".to_string());
        assert_eq!(res, 3);

        let res = extract_color_count("3 blue, 4 red".to_string(), "red".to_string());
        assert_eq!(res, 4);
    }

    #[test]
    pub fn test_extract_game_id() {
        let res =
            extract_game_id("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green".to_string());

        assert!(res.is_some());
        assert_eq!(res.unwrap(), 1)
    }

    #[test]
    pub fn test_from_string() {
        let res =
            Game::from_string("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green".to_string());

        assert!(res.is_ok());

        let res = res.unwrap();
        assert_eq!(res.id, 1);
        assert_eq!(
            res.sets,
            Vec::from([
                Set {
                    blue: 3,
                    green: 0,
                    red: 4
                },
                Set {
                    blue: 6,
                    green: 2,
                    red: 1
                },
                Set {
                    blue: 0,
                    green: 2,
                    red: 0
                },
            ])
        )
    }
}
