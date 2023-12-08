use std::collections::BTreeMap;

use nom::{
    branch::alt,
    bytes::complete::{tag, take_till, take_until},
    character::complete::{alphanumeric1, line_ending},
    combinator::{eof, iterator, recognize},
    sequence::{delimited, preceded, terminated},
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

#[tracing::instrument]
pub fn node(input: &str) -> IResult<&str, (&str, (&str, &str))> {
    let (input, name) = take_until(" ")(input)?;
    let (input, left) = preceded(tag(" = ("), alphanumeric1)(input)?;
    let (input, right) = delimited(tag(", "), alphanumeric1, tag(")"))(input)?;

    Ok((input, (name, (left, right))))
}

#[tracing::instrument]
pub fn network(input: &str) -> IResult<&str, BTreeMap<&str, (&str, &str)>> {
    let res: BTreeMap<&str, (&str, &str)> = iterator(
        input,
        terminated(node, alt((recognize(line_ending), recognize(eof)))),
    )
    .collect();

    trace!(?res);
    Ok(("", res))
}

pub fn parse_input(input: &str) -> IResult<&str, (&str, BTreeMap<&str, (&str, &str)>)> {
    let (input, instructions) =
        terminated(take_till(|c: char| c == '\n'), recognize(line_ending))(input)?;
    trace!(input);
    trace!(instructions);

    let (input, network) = preceded(recognize(line_ending), network)(input)?;
    trace!(input);
    trace!(?network);

    Ok((input, (instructions, network)))
}

pub fn find_destination_steps(
    start_id: &str,
    instructions: &[u8],
    network: BTreeMap<&str, (&str, &str)>,
    full_match: bool,
) -> usize {
    let mut counter: usize = 1;
    let mut found_dest = false;
    let mut iterator: usize = 0;
    let mut node = network.get(start_id).unwrap();

    while !found_dest {
        trace!(?node);
        trace!("{}", instructions[iterator]);
        let next_node = match instructions[iterator] {
            b'L' => node.0,
            b'R' => node.1,
            0_u8..=75_u8 | 77_u8..=81_u8 | 83_u8..=u8::MAX => todo!(),
        };
        trace!(?next_node);

        if !full_match && next_node.ends_with('Z') {
            found_dest = true;

            continue;
        }

        if full_match && next_node == "ZZZ" {
            found_dest = true;

            continue;
        }

        node = network.get(next_node).unwrap();

        iterator = (iterator + 1) % instructions.len();
        counter += 1;
    }

    trace!(counter);

    counter
}

pub fn part_one(inp: Vec<String>) -> usize {
    let binding = inp.join("\n");
    let (_, (instructions, network)) = parse_input(&binding).unwrap();
    let instructions = instructions.as_bytes();
    trace!(?instructions);
    trace!(?network);

    let network_keys = network.keys().copied().collect::<Vec<&str>>();
    let first_node_id = network_keys.first().unwrap();

    find_destination_steps(first_node_id, instructions, network, true)
}

pub fn part_two(inp: Vec<String>) -> usize {
    let binding = inp.join("\n");
    let (_, (instructions, network)) = parse_input(&binding).unwrap();
    let instructions = instructions.as_bytes();
    trace!(?instructions);
    trace!(?network);

    let start_nodes = network
        .keys()
        .copied()
        .filter(|n| n.ends_with('A'))
        .collect::<Vec<&str>>();

    trace!(?start_nodes);
    todo!("walk simultaniously");

    start_nodes
        .iter()
        .map(|n| find_destination_steps(n, instructions, network.clone(), false))
        .sum::<usize>()
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test_log::test]
    pub fn test_parse_input() {
        assert_eq!(
            parse_input(
                "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)
"
            ),
            Ok((
                "",
                (
                    "RL",
                    BTreeMap::from([
                        ("AAA", ("BBB", "CCC")),
                        ("BBB", ("DDD", "EEE")),
                        ("CCC", ("ZZZ", "GGG")),
                        ("DDD", ("DDD", "DDD")),
                        ("EEE", ("EEE", "EEE")),
                        ("GGG", ("GGG", "GGG")),
                        ("ZZZ", ("ZZZ", "ZZZ")),
                    ])
                )
            ))
        )
    }

    #[test_log::test]
    pub fn test_node() {
        assert_eq!(node("AAA = (BBB, CCC)"), Ok(("", ("AAA", ("BBB", "CCC")))));
    }

    #[test_log::test]
    pub fn test_network() {
        assert_eq!(
            network(
                "AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)"
            ),
            Ok((
                "",
                BTreeMap::from([
                    ("AAA", ("BBB", "CCC")),
                    ("BBB", ("DDD", "EEE")),
                    ("CCC", ("ZZZ", "GGG")),
                    ("DDD", ("DDD", "DDD")),
                    ("EEE", ("EEE", "EEE")),
                    ("GGG", ("GGG", "GGG")),
                    ("ZZZ", ("ZZZ", "ZZZ")),
                ])
            ))
        );
    }

    #[test_log::test]
    pub fn test_part_one() {
        let input = vec![
            "RL".to_string(),
            "".to_string(),
            "AAA = (BBB, CCC)".to_string(),
            "BBB = (DDD, EEE)".to_string(),
            "CCC = (ZZZ, GGG)".to_string(),
            "DDD = (DDD, DDD)".to_string(),
            "EEE = (EEE, EEE)".to_string(),
            "GGG = (GGG, GGG)".to_string(),
            "ZZZ = (ZZZ, ZZZ)".to_string(),
        ];

        let res = part_one(input);

        assert_eq!(res, 2);
    }

    #[test_log::test]
    pub fn test_part_two() {
        let input = vec![
            "LR".to_string(),
            "".to_string(),
            "11A = (11B, XXX)".to_string(),
            "11B = (XXX, 11Z)".to_string(),
            "11Z = (11B, XXX)".to_string(),
            "22A = (22B, XXX)".to_string(),
            "22B = (22C, 22C)".to_string(),
            "22C = (22Z, 22Z)".to_string(),
            "22Z = (22B, 22B)".to_string(),
            "XXX = (XXX, XXX)".to_string(),
        ];

        let res = part_two(input);

        assert_eq!(res, 6);
    }
}
