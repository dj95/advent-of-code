use nom::{
    branch::alt,
    character::complete::{self, line_ending, space1},
    combinator::{eof, recognize},
    multi::{many1, separated_list1},
    sequence::terminated,
    IResult,
};
use tracing::trace;

pub fn read_lines() -> Vec<String> {
    let mut res = Vec::new();

    let content = include_str!("../input.txt");
    for line in content.lines() {
        res.push(line.to_string());
    }

    res
}

#[derive(Debug, PartialEq)]
pub struct Series {
    pub initial: Vec<i64>,
    pub extrapolation: Vec<Vec<i64>>,
}

impl Series {
    pub fn from_initial(initial: Vec<i64>) -> Self {
        let extrapolation = Self::extrapolate(initial.clone());

        Self {
            initial,
            extrapolation,
        }
    }

    #[tracing::instrument]
    pub fn extend_by_one(&mut self) {
        let mut prev_num = 0;

        let last_nums: Vec<i64> = self
            .extrapolation
            .iter()
            .rev()
            .map(|s| *s.last().unwrap())
            .collect();

        trace!(?last_nums);
        for (i, ln) in last_nums.into_iter().enumerate() {
            prev_num += ln;

            let e_index = self.extrapolation.len() - 1 - i;
            trace!(e_index);

            let mut e = self.extrapolation.get(e_index).unwrap().clone();
            e.push(prev_num);

            let _ = std::mem::replace(&mut self.extrapolation[e_index], e);
        }

        let last_num = self.initial.last().unwrap();
        self.initial.push(last_num + prev_num);
    }

    #[tracing::instrument]
    pub fn extend_left_by_one(&mut self) {
        let mut prev_num = 0;

        let first_nums: Vec<i64> = self
            .extrapolation
            .iter()
            .rev()
            .map(|s| *s.first().unwrap())
            .collect();

        trace!(?first_nums);
        for (i, ln) in first_nums.into_iter().enumerate() {
            prev_num = ln - prev_num;

            let e_index = self.extrapolation.len() - 1 - i;
            trace!(e_index);

            let mut e = self.extrapolation.get(e_index).unwrap().clone();
            e.insert(0, prev_num);

            let _ = std::mem::replace(&mut self.extrapolation[e_index], e);
        }

        let first_num = self.initial.first().unwrap();
        self.initial.insert(0, first_num - prev_num);
    }

    #[tracing::instrument]
    fn extrapolate(initial: Vec<i64>) -> Vec<Vec<i64>> {
        let mut output = vec![];

        let mut current = initial.clone();
        while !current.iter().all(|c| *c == 0) {
            let diff = Self::get_diff(current);
            trace!(?diff);
            output.push(diff.clone());
            current = diff;
        }

        output
    }

    fn get_diff(a: Vec<i64>) -> Vec<i64> {
        let mut output = vec![];
        for i in 0..a.len() - 1 {
            let x = a.get(i).unwrap();
            let y = a.get(i + 1).unwrap();

            output.push(y - x);
        }

        output
    }
}

#[tracing::instrument]
pub fn parse_input(input: &str) -> IResult<&str, Vec<Vec<i64>>> {
    many1(terminated(
        separated_list1(space1, complete::i64),
        alt((recognize(line_ending), recognize(eof))),
    ))(input)
}

pub fn part_one(inp: Vec<String>) -> i64 {
    let inp = inp.join("\n");

    let (_, series) = parse_input(&inp).unwrap();

    series
        .iter()
        .map(|s| Series::from_initial(s.to_vec()))
        .map(|mut s| {
            s.extend_by_one();
            *s.initial.last().unwrap()
        })
        .sum::<i64>()
}

pub fn part_two(inp: Vec<String>) -> i64 {
    let inp = inp.join("\n");

    let (_, series) = parse_input(&inp).unwrap();

    series
        .iter()
        .map(|s| Series::from_initial(s.to_vec()))
        .map(|mut s| {
            s.extend_left_by_one();
            *s.initial.first().unwrap()
        })
        .sum::<i64>()
}

#[cfg(test)]
mod test {
    use std::vec;

    use crate::*;

    #[test_log::test]
    pub fn test_series_extend_left_by_one() {
        let mut s = Series::from_initial(vec![0, 3, 6, 9, 12, 15]);
        s.extend_left_by_one();
        assert_eq!(s.initial, vec![-3, 0, 3, 6, 9, 12, 15]);

        let mut s = Series::from_initial(vec![1, 3, 6, 10, 15, 21]);
        s.extend_left_by_one();
        assert_eq!(s.initial, vec![0, 1, 3, 6, 10, 15, 21]);

        let mut s = Series::from_initial(vec![10, 13, 16, 21, 30, 45]);
        s.extend_left_by_one();
        assert_eq!(s.initial, vec![5, 10, 13, 16, 21, 30, 45]);
    }

    #[test_log::test]
    pub fn test_series_extend_by_one() {
        let mut s = Series::from_initial(vec![0, 3, 6, 9, 12, 15]);
        s.extend_by_one();
        assert_eq!(s.initial, vec![0, 3, 6, 9, 12, 15, 18]);

        let mut s = Series::from_initial(vec![1, 3, 6, 10, 15, 21]);
        s.extend_by_one();
        assert_eq!(s.initial, vec![1, 3, 6, 10, 15, 21, 28]);

        let mut s = Series::from_initial(vec![10, 13, 16, 21, 30, 45]);
        s.extend_by_one();
        assert_eq!(s.initial, vec![10, 13, 16, 21, 30, 45, 68]);
    }

    #[test_log::test]
    pub fn test_series_from_initial() {
        assert_eq!(
            Series::from_initial(vec![0, 3, 6, 9, 12, 15]),
            Series {
                initial: vec![0, 3, 6, 9, 12, 15],
                extrapolation: vec![vec![3, 3, 3, 3, 3], vec![0, 0, 0, 0]],
            }
        );

        assert_eq!(
            Series::from_initial(vec![1, 3, 6, 10, 15, 21]),
            Series {
                initial: vec![1, 3, 6, 10, 15, 21],
                extrapolation: vec![vec![2, 3, 4, 5, 6], vec![1, 1, 1, 1], vec![0, 0, 0]],
            }
        );

        assert_eq!(
            Series::from_initial(vec![10, 13, 16, 21, 30, 45]),
            Series {
                initial: vec![10, 13, 16, 21, 30, 45],
                extrapolation: vec![
                    vec![3, 3, 5, 9, 15],
                    vec![0, 2, 4, 6],
                    vec![2, 2, 2],
                    vec![0, 0]
                ],
            }
        );
    }

    #[test_log::test]
    pub fn test_parse_input() {
        assert_eq!(
            parse_input(
                "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"
            ),
            Ok((
                "",
                vec![
                    vec![0, 3, 6, 9, 12, 15],
                    vec![1, 3, 6, 10, 15, 21],
                    vec![10, 13, 16, 21, 30, 45],
                ]
            ))
        );
    }

    #[test]
    pub fn test_part_one() {
        let input = vec![
            "0 3 6 9 12 15".to_string(),
            "1 3 6 10 15 21".to_string(),
            "10 13 16 21 30 45".to_string(),
        ];

        let res = part_one(input);

        assert_eq!(res, 114);
    }

    #[test]
    pub fn test_part_two() {
        let input = vec![
            "0 3 6 9 12 15".to_string(),
            "1 3 6 10 15 21".to_string(),
            "10 13 16 21 30 45".to_string(),
        ];

        let res = part_two(input);

        assert_eq!(res, 2);
    }
}
