use indicatif::ParallelProgressIterator;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_till, take_until, take_while1},
    character::complete::line_ending,
    combinator::{eof, iterator, map_res, recognize},
    sequence::{terminated, tuple},
    IResult,
};
use rayon::iter::{IntoParallelIterator, ParallelIterator};

pub fn read_lines() -> Vec<String> {
    let mut res = Vec::new();

    let content = include_str!("../input.txt");
    for line in content.lines() {
        res.push(line.to_string());
    }

    res
}

#[derive(Debug, PartialEq)]
struct Almanach {
    pub seeds: Vec<u64>,
    pub maps: Vec<Vec<[u64; 3]>>,
}

impl Almanach {
    pub fn lowest_location_from_ranges(&self) -> u64 {
        let seeds: Vec<u64> = self
            .seeds
            .chunks(2)
            .flat_map(|r| r[0]..r[0] + r[1])
            .collect();

        seeds
            .into_par_iter()
            .progress()
            .map(|s| self.location_for_seed(s))
            .min()
            .unwrap()
    }

    pub fn lowest_location(&self) -> u64 {
        self.seeds
            .iter()
            .map(|s| self.location_for_seed(*s))
            .min()
            .unwrap()
    }

    pub fn location_for_seed(&self, seed: u64) -> u64 {
        self.maps
            .iter()
            .fold(seed, |seed, map| find_dest_in_map(map.to_vec(), seed))
    }
}

pub fn find_dest_in_map(map: Vec<[u64; 3]>, seed: u64) -> u64 {
    match map.iter().find(|e| e[1] <= seed && (e[1] + e[2]) > seed) {
        Some(res) => res[0] + (seed - res[1]),
        None => seed,
    }
}

pub fn from_str(input: &str) -> Result<u64, std::num::ParseIntError> {
    input.parse::<u64>()
}

pub fn is_digit(input: char) -> bool {
    input.is_ascii_digit()
}

pub fn parse_digit(input: &str) -> IResult<&str, u64> {
    let (input, _) = take_till(|c: char| c.is_ascii_digit())(input)?;
    map_res(take_while1(is_digit), from_str)(input)
}

pub fn parse_line(input: &str) -> IResult<&str, [u64; 3]> {
    let (line, res) = tuple((parse_digit, parse_digit, parse_digit))(input)?;

    Ok((line, res.into()))
}

pub fn parse_endless_line(input: &str) -> IResult<&str, Vec<u64>> {
    let (input, digits) = take_till(|c| c == '\n')(input)?;

    let res: Vec<u64> = iterator(
        digits,
        terminated(parse_digit, alt((tag(" "), recognize(eof)))),
    )
    .collect();

    Ok((input, res))
}

fn map(line: &str) -> IResult<&str, Vec<[u64; 3]>> {
    let (input, _) = take_until(":")(line)?;

    let mut input = input;
    let mut map_input = input;
    if line.contains("\n\n") {
        (input, map_input) = take_until("\n\n")(line)?;
    }
    let res: Vec<[u64; 3]> = iterator(
        map_input,
        terminated(parse_line, alt((recognize(eof), recognize(line_ending)))),
    )
    .collect();

    let (input, _) = alt((take_till(|c: char| c.is_alphabetic()), recognize(eof)))(input)?;

    Ok((input, res))
}

fn almanach(input: &str) -> IResult<&str, Almanach> {
    let (input, _) = take_till(|c| c == ' ')(input)?;
    let (input, seeds) = terminated(parse_endless_line, recognize(line_ending))(input)?;

    let mut maps = vec![];
    let mut input = input;
    while !input.is_empty() {
        let (inp, m) = map(input)?;
        input = inp;
        maps.push(m);
    }

    Ok((input, Almanach { seeds, maps }))
}

pub fn part_one(inp: Vec<String>) -> u64 {
    let (_, almanach) = almanach(&inp.join("\n")).unwrap();

    almanach.lowest_location()
}

pub fn part_two(inp: Vec<String>) -> u64 {
    let (_, almanach) = almanach(&inp.join("\n")).unwrap();

    almanach.lowest_location_from_ranges()
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    pub fn test_almanach() {
        let res = almanach(
            "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4",
        );

        assert_eq!(
            res,
            Ok((
                "",
                Almanach {
                    seeds: vec![79, 14, 55, 13],
                    maps: vec![
                        vec![[50, 98, 2], [52, 50, 48]],
                        vec![[0, 15, 37], [37, 52, 2], [39, 0, 15]],
                        vec![[49, 53, 8], [0, 11, 42], [42, 0, 7], [57, 7, 4]],
                        vec![[88, 18, 7], [18, 25, 70]],
                        vec![[45, 77, 23], [81, 45, 19], [68, 64, 13]],
                        vec![[0, 69, 1], [1, 0, 69]],
                        vec![[60, 56, 37], [56, 93, 4]],
                    ],
                }
            ))
        );
    }

    #[test]
    pub fn test_map() {
        let res = map("foo-map:
50 98 2
52 50 48

");

        assert_eq!(res, Ok(("", vec![[50, 98, 2], [52, 50, 48],])))
    }

    #[test]
    pub fn test_find_dest_in_map() {
        assert_eq!(find_dest_in_map(vec![[50, 98, 2], [52, 50, 48]], 79), 81);
        assert_eq!(find_dest_in_map(vec![[50, 98, 2], [52, 50, 48]], 14), 14);
        assert_eq!(find_dest_in_map(vec![[50, 98, 2], [52, 50, 48]], 55), 57);
        assert_eq!(find_dest_in_map(vec![[50, 98, 2], [52, 50, 48]], 13), 13);
        assert_eq!(
            find_dest_in_map(vec![[49, 53, 8], [0, 11, 42], [42, 0, 7], [57, 7, 4]], 53),
            49
        );
    }

    #[test]
    pub fn test_location_for_seed() {
        let (_, alm) = almanach(
            "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4",
        )
        .unwrap();

        assert_eq!(alm.location_for_seed(79), 82);
        assert_eq!(alm.location_for_seed(14), 43);
        assert_eq!(alm.location_for_seed(55), 86);
        assert_eq!(alm.location_for_seed(13), 35);
    }

    #[test]
    pub fn test_part_one() {
        let input = vec![
            "seeds: 79 14 55 13".to_string(),
            "".to_string(),
            "seed-to-soil map:".to_string(),
            "50 98 2".to_string(),
            "52 50 48".to_string(),
            "".to_string(),
            "soil-to-fertilizer map:".to_string(),
            "0 15 37".to_string(),
            "37 52 2".to_string(),
            "39 0 15".to_string(),
            "".to_string(),
            "fertilizer-to-water map:".to_string(),
            "49 53 8".to_string(),
            "0 11 42".to_string(),
            "42 0 7".to_string(),
            "57 7 4".to_string(),
            "".to_string(),
            "water-to-light map:".to_string(),
            "88 18 7".to_string(),
            "18 25 70".to_string(),
            "".to_string(),
            "light-to-temperature map:".to_string(),
            "45 77 23".to_string(),
            "81 45 19".to_string(),
            "68 64 13".to_string(),
            "".to_string(),
            "temperature-to-humidity map:".to_string(),
            "0 69 1".to_string(),
            "1 0 69".to_string(),
            "".to_string(),
            "humidity-to-location map:".to_string(),
            "60 56 37".to_string(),
            "56 93 4".to_string(),
        ];

        let res = part_one(input);

        assert_eq!(res, 35);
    }

    #[test]
    pub fn test_part_two() {
        let input = vec![
            "seeds: 79 14 55 13".to_string(),
            "".to_string(),
            "seed-to-soil map:".to_string(),
            "50 98 2".to_string(),
            "52 50 48".to_string(),
            "".to_string(),
            "soil-to-fertilizer map:".to_string(),
            "0 15 37".to_string(),
            "37 52 2".to_string(),
            "39 0 15".to_string(),
            "".to_string(),
            "fertilizer-to-water map:".to_string(),
            "49 53 8".to_string(),
            "0 11 42".to_string(),
            "42 0 7".to_string(),
            "57 7 4".to_string(),
            "".to_string(),
            "water-to-light map:".to_string(),
            "88 18 7".to_string(),
            "18 25 70".to_string(),
            "".to_string(),
            "light-to-temperature map:".to_string(),
            "45 77 23".to_string(),
            "81 45 19".to_string(),
            "68 64 13".to_string(),
            "".to_string(),
            "temperature-to-humidity map:".to_string(),
            "0 69 1".to_string(),
            "1 0 69".to_string(),
            "".to_string(),
            "humidity-to-location map:".to_string(),
            "60 56 37".to_string(),
            "56 93 4".to_string(),
        ];

        let res = part_two(input);

        assert_eq!(res, 46);
    }
}
