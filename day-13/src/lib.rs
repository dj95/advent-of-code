use itertools::Itertools;
use tracing::trace;

pub fn read_lines() -> Vec<String> {
    let mut res = Vec::new();

    let content = include_str!("../input.txt");
    for line in content.lines() {
        res.push(line.to_string());
    }

    res
}

pub fn detect_horizontal_fold(inp: &[String], with_smudge: bool) -> Option<usize> {
    inp.iter()
        .enumerate()
        .tuple_windows()
        .filter(|((_, a), (_, b))| {
            if !with_smudge {
                return a == b;
            }

            a.chars().zip(b.chars()).filter(|(c, d)| c != d).count() <= 1
        })
        .find_map(|(a, b)| {
            let inp_a = inp[0..=a.0].iter().map(|l| l.chars()).rev();
            let inp_b = inp[b.0..inp.len()].iter().map(|l| l.chars());

            let c = match with_smudge {
                true => 1,
                false => 0,
            };

            inp_a
                .flatten()
                .zip(inp_b.flatten())
                .filter(|(a, b)| a != b)
                .count()
                .eq(&c)
                .then_some(a.0 + 1)
        })
}

pub fn detect_vertical_fold(inp: &[String], with_smudge: bool) -> Option<usize> {
    let inp: Vec<String> = (0..inp[0].len())
        .map(|i| {
            inp.iter()
                .map(|inner| inner.chars().nth(i).unwrap())
                .join("")
        })
        .collect();

    detect_horizontal_fold(&inp, with_smudge)
}

pub fn get_fold_pos(inp: Vec<String>) -> Option<usize> {
    match detect_vertical_fold(&inp, false) {
        Some(res) => Some(res),
        None => detect_horizontal_fold(&inp, false).map(|x| x * 100),
    }
}

#[tracing::instrument(skip_all)]
pub fn get_smudge_fold_pos(inp: Vec<String>) -> Option<usize> {
    match detect_horizontal_fold(&inp, true) {
        Some(res) => Some(res * 100),
        None => detect_vertical_fold(&inp, true),
    }
}

pub fn part_one(inp: Vec<String>) -> usize {
    inp.into_iter()
        .fold(vec![vec![]], |mut acc, x| {
            if x.is_empty() {
                acc.push(vec![]);

                return acc;
            }

            acc.last_mut().unwrap().push(x);

            acc
        })
        .into_iter()
        .filter_map(get_fold_pos)
        .sum::<usize>()
}

#[tracing::instrument(skip_all)]
pub fn part_two(inp: Vec<String>) -> usize {
    let lines = inp.into_iter().fold(vec![vec![]], |mut acc, x| {
        if x.is_empty() {
            acc.push(vec![]);

            return acc;
        }

        acc.last_mut().unwrap().push(x);

        acc
    });

    return lines
        .into_iter()
        .filter_map(get_smudge_fold_pos)
        .inspect(|x| trace!("x: {:?}", x))
        .sum::<usize>();
}

#[cfg(test)]
mod test {
    use crate::*;
    use rstest::rstest;

    #[rstest]
    #[case(
        vec![
            "#.##..##.".to_string(),
            "..#.##.#.".to_string(),
            "##......#".to_string(),
            "##......#".to_string(),
            "..#.##.#.".to_string(),
            "..##..##.".to_string(),
            "#.#.##.#.".to_string(),
        ],
        Some(300)
    )]
    #[case(
        vec![
            "#...##..#".to_string(),
            "#....#..#".to_string(),
            "..##..###".to_string(),
            "#####.##.".to_string(),
            "#####.##.".to_string(),
            "..##..###".to_string(),
            "#....#..#".to_string(),
        ],
        Some(100),
    )]
    #[case(
        vec![
            "..#.#..".to_string(),
            "##..#..".to_string(),
            "##..#..".to_string(),
            "...####".to_string(),
            "####.##".to_string(),
            "....#..".to_string(),
            "#####..".to_string(),
            "#..##..".to_string(),
            ".....##".to_string(),
            "..##...".to_string(),
            "....#..".to_string(),
            "##..###".to_string(),
            "...##..".to_string(),
        ],
        Some(1),
    )]
    #[test_log::test]
    pub fn test_get_smudge_fold_pos(#[case] input: Vec<String>, #[case] expected: Option<usize>) {
        assert_eq!(get_smudge_fold_pos(input), expected);
    }

    #[rstest]
    #[case(
        vec![
            "#.##..##.".to_string(),
            "..#.##.#.".to_string(),
            "##......#".to_string(),
            "##......#".to_string(),
            "..#.##.#.".to_string(),
            "..##..##.".to_string(),
            "#.#.##.#.".to_string(),
        ],
        Some(5)
    )]
    #[case(
        vec![
            "#...##..#".to_string(),
            "#....#..#".to_string(),
            "..##..###".to_string(),
            "#####.##.".to_string(),
            "#####.##.".to_string(),
            "..##..###".to_string(),
            "#....#..#".to_string(),
        ],
        Some(400),
    )]
    #[test_log::test]
    pub fn test_get_fold_pos(#[case] input: Vec<String>, #[case] expected: Option<usize>) {
        assert_eq!(get_fold_pos(input), expected);
    }

    #[rstest]
    #[case(
        vec![
            "#.##..##.".to_string(),
            "..#.##.#.".to_string(),
            "##......#".to_string(),
            "##......#".to_string(),
            "..#.##.#.".to_string(),
            "..##..##.".to_string(),
            "#.#.##.#.".to_string(),
        ],
        Some(5),
    )]
    #[case(
        vec![
            "#...##..#".to_string(),
            "#....#..#".to_string(),
            "..##..###".to_string(),
            "#####.##.".to_string(),
            "#####.##.".to_string(),
            "..##..###".to_string(),
            "#....#..#".to_string(),
        ],
        None,
    )]
    #[test_log::test]
    pub fn test_detect_vertical_fold(#[case] input: Vec<String>, #[case] expected: Option<usize>) {
        assert_eq!(detect_vertical_fold(&input, false), expected);
    }

    #[rstest]
    #[case(
        vec![
            "#.##..##.".to_string(),
            "..#.##.#.".to_string(),
            "##......#".to_string(),
            "##......#".to_string(),
            "..#.##.#.".to_string(),
            "..##..##.".to_string(),
            "#.#.##.#.".to_string(),
        ],
        None,
    )]
    #[case(
        vec![
            "#...##..#".to_string(),
            "#....#..#".to_string(),
            "..##..###".to_string(),
            "#####.##.".to_string(),
            "#####.##.".to_string(),
            "..##..###".to_string(),
            "#....#..#".to_string(),
        ],
        Some(4),
    )]
    #[test_log::test]
    pub fn test_detect_horizontal_fold(
        #[case] input: Vec<String>,
        #[case] expected: Option<usize>,
    ) {
        assert_eq!(detect_horizontal_fold(&input, false), expected);
    }

    #[test_log::test]
    pub fn test_part_one() {
        let input = vec![
            "#.##..##.".to_string(),
            "..#.##.#.".to_string(),
            "##......#".to_string(),
            "##......#".to_string(),
            "..#.##.#.".to_string(),
            "..##..##.".to_string(),
            "#.#.##.#.".to_string(),
            "".to_string(),
            "#...##..#".to_string(),
            "#....#..#".to_string(),
            "..##..###".to_string(),
            "#####.##.".to_string(),
            "#####.##.".to_string(),
            "..##..###".to_string(),
            "#....#..#".to_string(),
        ];

        let res = part_one(input);

        assert_eq!(res, 405);
    }

    #[test_log::test]
    pub fn test_part_two() {
        let input = vec![
            "#.##..##.".to_string(),
            "..#.##.#.".to_string(),
            "##......#".to_string(),
            "##......#".to_string(),
            "..#.##.#.".to_string(),
            "..##..##.".to_string(),
            "#.#.##.#.".to_string(),
            "".to_string(),
            "#...##..#".to_string(),
            "#....#..#".to_string(),
            "..##..###".to_string(),
            "#####.##.".to_string(),
            "#####.##.".to_string(),
            "..##..###".to_string(),
            "#....#..#".to_string(),
        ];

        let res = part_two(input);

        assert_eq!(res, 400);
    }
}
