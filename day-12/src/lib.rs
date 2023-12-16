use cached::{proc_macro::cached, UnboundCache};
use itertools::Itertools;
pub(crate) use nom::{
    bytes::complete::{is_a, tag},
    character::complete::{self, space1},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};
use rayon::prelude::*;

pub fn read_lines() -> Vec<String> {
    let mut res = Vec::new();

    let content = include_str!("../input.txt");
    for line in content.lines() {
        res.push(line.to_string());
    }

    res
}

#[derive(Debug, PartialEq, Clone)]
pub struct Record {
    pub condition: String,
    pub required_groups: Vec<u64>,
}

pub fn parse_record(input: &str) -> IResult<&str, Record> {
    let (input, (condition, location)) = separated_pair(
        is_a(".#?"),
        space1,
        separated_list1(tag(","), complete::u64),
    )(input)?;

    Ok((
        input,
        Record {
            condition: condition.to_string(),
            required_groups: location,
        },
    ))
}

impl Record {
    pub fn get_possible_arrangement_count(&self) -> u64 {
        num_valid_solutions(&self.condition, &self.required_groups)
    }

    pub fn expand(&mut self) -> &mut Self {
        self.condition = std::iter::repeat(&self.condition).take(5).join("?");
        self.required_groups = self.required_groups.repeat(5);
        self
    }
}

#[cached(
    type = "UnboundCache<(String, Vec<u64>), u64>",
    create = "{ UnboundCache::new() }",
    convert = r#"{ (record.to_string(), groups.to_vec()) }"#
)]
fn num_valid_solutions(record: &str, groups: &[u64]) -> u64 {
    if record.is_empty() {
        return match groups.is_empty() {
            true => 1,
            false => 0,
        };
    }

    if groups.is_empty() {
        return match record.matches('#').count() != 0 {
            true => 0,
            false => 1,
        };
    }

    let (chr, rest_of_record) = record.split_at(1);

    if chr == "." {
        return num_valid_solutions(rest_of_record, groups);
    }

    if chr == "#" {
        let group = groups[0];

        if record.len() < group as usize {
            return 0;
        }

        if !record.as_bytes()[0..group as usize]
            .iter()
            .all(|c| *c != b'.')
        {
            return 0;
        }

        if record.len() != group as usize && record.chars().nth(group as usize).unwrap() == '#' {
            return 0;
        }

        if record.len() == group as usize {
            return num_valid_solutions("", &groups[1..]);
        }

        return num_valid_solutions(&record[group as usize + 1..], &groups[1..]);
    }

    if chr == "?" {
        return num_valid_solutions(&format!("#{rest_of_record}"), groups)
            + num_valid_solutions(&format!(".{rest_of_record}"), groups);
    }

    todo!("invalid char");
}

#[tracing::instrument(skip_all)]
pub fn part_one(inp: Vec<String>) -> u64 {
    inp.iter()
        .map(|line| {
            parse_record(line)
                .unwrap()
                .1
                .get_possible_arrangement_count()
        })
        .sum()
}

pub fn part_two(inp: Vec<String>) -> u64 {
    inp.par_iter()
        .map(|line| {
            parse_record(line)
                .unwrap()
                .1
                .expand()
                .get_possible_arrangement_count()
        })
        .sum::<u64>()
}

#[cfg(test)]
mod test {
    use crate::*;

    use rstest::rstest;

    #[rstest]
    #[case(
        Record {
            condition: ".#".to_string(),
            required_groups: vec![1]
        },
        Record {
            condition: ".#?.#?.#?.#?.#".to_string(),
            required_groups: vec![1, 1, 1, 1, 1]
        }
    )]
    #[case(
        Record {
            condition: "???.###".to_string(),
            required_groups: vec![1, 1, 3]
        },
        Record {
            condition: "???.###????.###????.###????.###????.###".to_string(),
            required_groups: vec![1,1,3,1,1,3,1,1,3,1,1,3,1,1,3]
        }
    )]
    #[test_log::test]
    pub fn test_expand(#[case] input: Record, #[case] expected: Record) {
        let mut input = input;
        input.expand();

        assert_eq!(input, expected);
    }

    #[rstest]
    #[case(Record { condition: "???.###".to_string(), required_groups: vec![1, 1, 3] }, 1)]
    #[case(Record { condition: ".??..??...?##.".to_string(), required_groups: vec![1, 1, 3] }, 4)]
    #[case(Record { condition: "?#?#?#?#?#?#?#?".to_string(), required_groups: vec![1,3,1,6] }, 1)]
    #[case(Record { condition: "????.#...#...".to_string(), required_groups: vec![4, 1, 1] }, 1)]
    #[case(Record { condition: "????.######..#####.".to_string(), required_groups: vec![1, 6, 5] }, 4)]
    #[test_log::test]
    pub fn test_get_possible_arrangement_count(
        #[case] record: Record,
        #[case] expected_count: u64,
    ) {
        assert_eq!(record.get_possible_arrangement_count(), expected_count);
    }

    #[test_log::test]
    pub fn test_parse_input() {
        let input = "???.### 1,1,3";

        let res = parse_record(input);

        assert_eq!(
            res,
            Ok((
                "",
                Record {
                    condition: "???.###".to_string(),
                    required_groups: vec![1, 1, 3]
                }
            ))
        );
    }

    #[test_log::test]
    pub fn test_part_one() {
        let input = vec![
            "???.### 1,1,3".to_string(),
            ".??..??...?##. 1,1,3".to_string(),
            "?#?#?#?#?#?#?#? 1,3,1,6".to_string(),
            "????.#...#... 4,1,1".to_string(),
            "????.######..#####. 1,6,5".to_string(),
            "?###???????? 3,2,1".to_string(),
        ];

        let res = part_one(input);

        assert_eq!(res, 21);
    }

    #[test_log::test]
    pub fn test_part_two() {
        let input = vec![
            "???.### 1,1,3".to_string(),
            ".??..??...?##. 1,1,3".to_string(),
            "?#?#?#?#?#?#?#? 1,3,1,6".to_string(),
            "????.#...#... 4,1,1".to_string(),
            "????.######..#####. 1,6,5".to_string(),
            "?###???????? 3,2,1".to_string(),
        ];

        let res = part_two(input);

        assert_eq!(res, 525152);
    }
}
