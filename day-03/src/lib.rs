use std::cmp;

use regex::Regex;

pub fn read_lines() -> Vec<String> {
    let mut res = Vec::new();

    let content = include_str!("../input.txt");
    for line in content.lines() {
        res.push(line.to_string());
    }

    res
}

#[derive(Debug, PartialEq)]
pub struct Engine {
    pub schematic: Vec<Vec<char>>,
    pub lines: Vec<String>,
}

impl Engine {
    pub fn from_lines(lines: Vec<String>) -> Self {
        let mut schematic: Vec<Vec<char>> = Vec::new();

        for line in lines.clone() {
            let line_chars: Vec<char> = line.chars().collect();

            schematic.push(line_chars);
        }

        Self { schematic, lines }
    }

    pub fn part_no_is_adjacent(&self, part_no: PartNumber) -> bool {
        let start_line = part_no.position.line.saturating_sub(1);
        let end_line = cmp::min(self.lines.len() - 1, part_no.position.line + 1);

        let pattern = Regex::new("([^\\.0-9])").unwrap();
        for line_counter in start_line..=end_line {
            let line = self.lines.get(line_counter);

            for mat in pattern.find_iter(line.unwrap()) {
                let min_end = mat.end().saturating_sub(2);

                if mat.start() + 1 >= part_no.position.start && min_end <= part_no.position.end {
                    return true;
                }
            }
        }
        false
    }

    pub fn find_part_numbers(&self) -> Vec<PartNumber> {
        let mut output = Vec::new();
        let pattern = Regex::new("([0-9]+)").unwrap();

        for (line_counter, line) in self.lines.iter().enumerate() {
            let mats = pattern.find_iter(line);

            for mat in mats {
                output.push(PartNumber {
                    number: mat.as_str().to_string().parse::<u32>().unwrap(),
                    position: Position {
                        line: line_counter,
                        start: mat.start(),
                        end: mat.end() - 1,
                    },
                });
            }
        }

        output
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct PartNumber {
    pub number: u32,
    pub position: Position,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Position {
    pub line: usize,
    pub start: usize,
    pub end: usize,
}

pub fn part_one(inp: Vec<String>) -> u32 {
    let engine = Engine::from_lines(inp);
    let part_nos = engine.find_part_numbers();

    let mut output = 0;
    for part_no in part_nos {
        if !engine.part_no_is_adjacent(part_no) {
            continue;
        }

        output += part_no.number;
    }

    output
}

pub fn part_two(inp: Vec<String>) -> String {
    "".to_string()
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    pub fn test_part_no_is_adjacent() {
        let engine = Engine::from_lines(vec!["467..114..".to_string(), "...*......".to_string()]);

        let part_nos = engine.find_part_numbers();

        let res = engine.part_no_is_adjacent(*part_nos.get(0).unwrap());
        assert!(res);

        let res = engine.part_no_is_adjacent(*part_nos.get(1).unwrap());
        assert!(!res);
    }

    #[test]
    pub fn test_find_part_numbers() {
        let res = Engine::from_lines(vec!["467..114..".to_string(), "...*......".to_string()])
            .find_part_numbers();

        let expected = vec![
            PartNumber {
                number: 467,
                position: Position {
                    line: 0,
                    start: 0,
                    end: 2,
                },
            },
            PartNumber {
                number: 114,
                position: Position {
                    line: 0,
                    start: 5,
                    end: 7,
                },
            },
        ];

        assert_eq!(res, expected);
    }

    #[test]
    pub fn test_engine_from_lines() {
        let res = Engine::from_lines(vec!["467..114..".to_string(), "...*......".to_string()]);

        let expected = Engine {
            schematic: vec![
                vec!['4', '6', '7', '.', '.', '1', '1', '4', '.', '.'],
                vec!['.', '.', '.', '*', '.', '.', '.', '.', '.', '.'],
            ],
            lines: vec!["467..114..".to_string(), "...*......".to_string()],
        };

        assert_eq!(res, expected);
    }

    #[test]
    pub fn test_part_one() {
        let input = vec![
            "467..114..".to_string(),
            "...*......".to_string(),
            "..35..633.".to_string(),
            "......#...".to_string(),
            "617*......".to_string(),
            ".....+.58.".to_string(),
            "..592.....".to_string(),
            "......755.".to_string(),
            "...$.*....".to_string(),
            ".664.598..".to_string(),
        ];

        let res = part_one(input);

        assert_eq!(res, 4361);
    }

    #[test]
    pub fn test_part_two() {
        let input = Vec::from(["".to_string()]);

        let res = part_two(input);

        assert_eq!(res, "");
    }
}