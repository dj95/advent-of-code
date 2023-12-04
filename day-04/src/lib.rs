use nom::{
    branch::alt,
    bytes::complete::{tag, take_till, take_while_m_n},
    combinator::{eof, iterator, map_res, recognize},
    sequence::{delimited, terminated},
    IResult,
};

pub fn read_lines() -> Vec<String> {
    let mut res = Vec::new();

    let content = include_str!("../input.txt");
    for line in content.lines() {
        res.push(line.to_string());
    }

    res
}

#[derive(Debug, PartialEq)]
pub struct Card {
    pub winning_no: Vec<u8>,
    pub numbers: Vec<u8>,
}

impl Card {
    pub fn wins(&self) -> usize {
        self.numbers
            .iter()
            .filter(|no| self.winning_no.contains(no))
            .count()
    }

    pub fn points(&self) -> u32 {
        self.numbers
            .iter()
            .filter(|no| self.winning_no.contains(no))
            .fold(1, |res, _| res * 2)
            / 2
    }
}

pub fn from_str(input: &str) -> Result<u8, std::num::ParseIntError> {
    input.parse::<u8>()
}

pub fn is_digit(input: char) -> bool {
    input.is_ascii_digit()
}

pub fn parse_digit(input: &str) -> IResult<&str, u8> {
    let (input, _) = take_till(|c: char| c.is_ascii_digit())(input)?;
    map_res(take_while_m_n(1, 3, is_digit), from_str)(input)
}

pub fn parse_numbers(input: &str) -> IResult<&str, Vec<u8>> {
    let (input, res) = take_till(|c: char| !c.is_ascii_digit() && c != ' ')(input)?;

    let it: Vec<u8> = iterator(
        res,
        terminated(parse_digit, alt((tag(" "), recognize(eof)))),
    )
    .collect();

    Ok((input, it))
}

pub fn card(line: &str) -> IResult<&str, Card> {
    let (input, _) = delimited(tag("Card "), parse_digit, tag(":"))(line)?;
    let (input, winning_no) = delimited(tag(" "), parse_numbers, tag("|"))(input)?;
    let (input, numbers) = parse_numbers(input)?;

    Ok((
        input,
        Card {
            winning_no,
            numbers,
        },
    ))
}

pub fn part_one(inp: Vec<String>) -> u32 {
    inp.iter()
        .map(|l| {
            let (_, c) = card(l).unwrap();
            c.points()
        })
        .sum()
}

pub fn part_two(inp: Vec<String>) -> u32 {
    let cards = inp.iter().map(|l| {
        let (_, c) = card(l).unwrap();
        c.wins() as u32
    });
    let mut multiplier: Vec<usize> = vec![];

    let mut output = 0;
    for card in cards {
        let mul = match multiplier.first() {
            Some(m) => *m + 1,
            None => 1,
        };

        if !multiplier.is_empty() {
            multiplier.remove(0);
        }

        output += mul as u32;

        for i in 0..card {
            if multiplier.get(i as usize).is_some() {
                multiplier[i as usize] += mul;
            } else {
                multiplier.push(mul);
            }
        }
    }

    output
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    pub fn test_card() {
        assert_eq!(
            card("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53"),
            Ok((
                "",
                Card {
                    winning_no: vec![41, 48, 83, 86, 17],
                    numbers: vec![83, 86, 6, 31, 17, 9, 48, 53],
                }
            )),
        );
    }

    #[test]
    pub fn test_points() {
        let card = Card {
            winning_no: vec![41, 48, 83, 86, 17],
            numbers: vec![83, 86, 6, 31, 17, 9, 48, 53],
        };

        assert_eq!(card.points(), 8);
    }

    #[test]
    pub fn test_part_one() {
        let input = vec![
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53".to_string(),
            "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19".to_string(),
            "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1".to_string(),
            "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83".to_string(),
            "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36".to_string(),
            "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11".to_string(),
        ];

        let res = part_one(input);

        assert_eq!(res, 13);
    }

    #[test]
    pub fn test_part_two() {
        let input = vec![
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53".to_string(),
            "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19".to_string(),
            "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1".to_string(),
            "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83".to_string(),
            "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36".to_string(),
            "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11".to_string(),
        ];

        let res = part_two(input);

        assert_eq!(res, 30);
    }
}
