use std::ops::Range;

use regex::Regex;

pub fn read_lines() -> Vec<String> {
    let mut res = Vec::new();

    let content = include_str!("../input.txt");
    for line in content.lines() {
        res.push(line.to_string());
    }

    res
}

fn is_adjacent(range: Range<usize>, part_no: PartNumber) -> bool {
    let min_end = range.end.saturating_sub(2);

    range.start + 1 >= part_no.position.start && min_end <= part_no.position.end
}

fn is_neighboured_line(line_no: usize, p_no: PartNumber) -> bool {
    p_no.position.line >= line_no.saturating_sub(1) && p_no.position.line <= line_no + 1
}

#[derive(Debug, PartialEq)]
pub struct Engine {
    lines: Vec<String>,
    part_nos: Vec<PartNumber>,
}

impl Engine {
    pub fn from_lines(lines: Vec<String>) -> Self {
        Self {
            lines: lines.clone(),
            part_nos: find_part_numbers(lines),
        }
    }

    pub fn find_adj_part_nos(&self) -> Vec<u32> {
        let mut output = Vec::new();

        let pattern = Regex::new("([^\\.0-9])").unwrap();
        for (line_no, line) in self.lines.iter().enumerate() {
            for mat in pattern.find_iter(line) {
                let mut adj_part_nos: Vec<u32> = self
                    .part_nos
                    .clone()
                    .into_iter()
                    .filter(|p_no| is_neighboured_line(line_no, *p_no))
                    .filter(|p_no| is_adjacent(mat.range(), *p_no))
                    .map(|p_no| p_no.number)
                    .collect();

                output.append(&mut adj_part_nos);
            }
        }

        output
    }

    pub fn find_gear_ratios(&self) -> Vec<u32> {
        let mut output = Vec::new();

        let pattern = Regex::new("(\\*)").unwrap();
        for (line_no, line) in self.lines.iter().enumerate() {
            for mat in pattern.find_iter(line) {
                let gear_part_nos: Vec<PartNumber> = self
                    .part_nos
                    .clone()
                    .into_iter()
                    .filter(|p_no| is_neighboured_line(line_no, *p_no))
                    .filter(|p_no| is_adjacent(mat.range(), *p_no))
                    .collect();

                if gear_part_nos.len() != 2 {
                    continue;
                }

                output.push(
                    gear_part_nos
                        .iter()
                        .fold(1, |product, part_no| product * part_no.number),
                );
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

pub fn find_part_numbers(lines: Vec<String>) -> Vec<PartNumber> {
    let mut output: Vec<PartNumber> = Vec::new();
    let pattern = Regex::new("([0-9]+)").unwrap();

    for (line_counter, line) in lines.iter().enumerate() {
        let mut mats: Vec<PartNumber> = pattern
            .find_iter(line)
            .map(|mat| PartNumber {
                number: mat.as_str().to_string().parse::<u32>().unwrap(),
                position: Position {
                    line: line_counter,
                    start: mat.start(),
                    end: mat.end() - 1,
                },
            })
            .collect();

        output.append(&mut mats);
    }

    output
}

pub fn part_one(inp: Vec<String>) -> u32 {
    let engine = Engine::from_lines(inp);

    engine
        .find_adj_part_nos()
        .iter()
        .fold(0, |sum, p_no| sum + p_no)
}

pub fn part_two(inp: Vec<String>) -> u32 {
    let engine = Engine::from_lines(inp);

    engine
        .find_gear_ratios()
        .iter()
        .fold(0, |sum, ratio| sum + ratio)
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    pub fn test_find_adj_part_nos() {
        let engine = Engine::from_lines(vec!["467..114..".to_string(), "...*......".to_string()]);

        let res = engine.find_adj_part_nos();

        assert_eq!(res, vec![467]);
    }

    #[test]
    pub fn test_find_part_numbers() {
        let res = find_part_numbers(vec!["467..114..".to_string(), "...*......".to_string()]);

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
            lines: vec!["467..114..".to_string(), "...*......".to_string()],
            part_nos: vec![
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
            ],
        };

        assert_eq!(res, expected);
    }

    #[test]
    pub fn test_find_part_numbers_with_gears() {
        let engine = Engine::from_lines(vec![
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
        ]);

        let res = engine.find_gear_ratios();

        assert_eq!(res, vec![16345, 451490]);
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

        let res = part_two(input);

        assert_eq!(res, 467835);
    }
}
