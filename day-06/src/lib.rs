use nom::{
    character::complete, character::complete::space1, multi::separated_list1, IResult, Parser,
};
use nom_supreme::{tag::complete::tag, ParserExt};

pub fn read_lines() -> Vec<String> {
    let mut res = Vec::new();

    let content = include_str!("../input.txt");
    for line in content.lines() {
        res.push(line.to_string());
    }

    res
}

pub fn data(input: &str) -> IResult<&str, (Vec<u64>, Vec<u64>)> {
    let (input, time) = tag("Time:")
        .precedes(space1)
        .precedes(separated_list1(space1, complete::u64))
        .parse(input)?;

    let (input, dist) = tag("\nDistance:")
        .precedes(space1)
        .precedes(separated_list1(space1, complete::u64))
        .parse(input)?;

    Ok((input, (time, dist)))
}

pub fn travel_for_hold(hold: u64, time: u64) -> u64 {
    (time - hold) * hold
}

pub fn find_winning_hold_count(time: u64, dist: u64) -> u64 {
    for i in 1..time {
        if travel_for_hold(i, time) <= dist {
            continue;
        }

        return time - (2 * i) + 1;
    }

    0
}

pub fn part_one(inp: Vec<String>) -> u64 {
    let (_, (time, dist)) = data(inp.join("\n").as_str()).unwrap();

    time.iter()
        .zip(dist)
        .map(|(t, d)| find_winning_hold_count(*t, d))
        .product()
}

pub fn part_two(inp: Vec<String>) -> u64 {
    let (_, (time, dist)) = data(inp.join("\n").as_str()).unwrap();

    let time = time
        .iter()
        .fold("".to_string(), |acc, t| format!("{acc}{t}"))
        .parse::<u64>()
        .unwrap();

    let dist = dist
        .iter()
        .fold("".to_string(), |acc, t| format!("{acc}{t}"))
        .parse::<u64>()
        .unwrap();

    find_winning_hold_count(time, dist)
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    pub fn test_data() {
        let res = data(
            "Time:      7  15   30
Distance:  9  40  200",
        );

        assert_eq!(res, Ok(("", (vec![7, 15, 30], vec![9, 40, 200]))));
    }

    #[test]
    pub fn test_find_winning_hold_count() {
        assert_eq!(find_winning_hold_count(7, 9), 4);
    }

    #[test]
    pub fn test_travel_for_hold() {
        assert_eq!(travel_for_hold(0, 7), 0);
        assert_eq!(travel_for_hold(1, 7), 6);
        assert_eq!(travel_for_hold(2, 7), 10);
        assert_eq!(travel_for_hold(3, 7), 12);
        assert_eq!(travel_for_hold(4, 7), 12);
        assert_eq!(travel_for_hold(5, 7), 10);
        assert_eq!(travel_for_hold(6, 7), 6);
        assert_eq!(travel_for_hold(7, 7), 0);
    }

    #[test]
    pub fn test_part_one() {
        let input = vec![
            "Time:      7  15   30".to_string(),
            "Distance:  9  40  200".to_string(),
        ];

        let res = part_one(input);

        assert_eq!(res, 288);
    }

    #[test]
    pub fn test_part_two() {
        let input = vec![
            "Time:      7  15   30".to_string(),
            "Distance:  9  40  200".to_string(),
        ];

        let res = part_two(input);

        assert_eq!(res, 71503);
    }
}
