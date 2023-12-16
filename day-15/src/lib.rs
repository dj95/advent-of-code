use tracing::trace;

pub fn read_lines() -> Vec<String> {
    let mut res = Vec::new();

    let content = include_str!("../input.txt");
    for line in content.lines() {
        res.push(line.to_string());
    }

    res
}

pub fn hash(inp: &str) -> u8 {
    inp.chars().fold(0, |hash, c| (hash + c as u64) * 17 % 256) as u8
}

pub fn part_one(inp: Vec<String>) -> u64 {
    let inp = inp[0].split(',').collect::<Vec<_>>();

    inp.iter().map(|s| hash(s) as u64).sum()
}

pub fn part_two(inp: Vec<String>) -> u64 {
    let inp = inp[0].split(',').collect::<Vec<&str>>();

    let mut map: [Vec<(&str, u8)>; 256] = vec![Vec::new(); 256].try_into().expect("static");

    for inp in inp.iter() {
        if inp.contains('=') {
            let parts = inp.split('=').collect::<Vec<&str>>();
            let box_id = hash(parts[0]);

            // insert, if not exists
            if map[box_id as usize]
                .iter()
                .map(|(k, _)| k)
                .filter(|k| **k == parts[0])
                .count()
                == 0
            {
                map[box_id as usize].push((parts[0], parts[1].parse::<u8>().unwrap()));

                continue;
            }

            // replace, if exists
            map[box_id as usize] = map[box_id as usize]
                .clone()
                .into_iter()
                .map(|mut lens| {
                    if lens.0 == parts[0] {
                        lens.1 = parts[1].parse::<u8>().unwrap();
                    }

                    lens
                })
                .collect();
            trace!("= {:?}", map[box_id as usize]);
        }

        if inp.contains('-') {
            let parts = inp.split('-').collect::<Vec<&str>>();
            let box_id = hash(parts[0]);

            // remove labels from box
            map[box_id as usize]
                .clone()
                .into_iter()
                .enumerate()
                .filter_map(|(i, lens)| if lens.0 == parts[0] { Some(i) } else { None })
                .for_each(|id| {
                    map[box_id as usize].remove(id);
                });

            trace!("- {:?}", map[box_id as usize]);
        }
    }

    map.iter()
        .enumerate()
        .map(|(idx, val)| {
            val.iter()
                .enumerate()
                .map(move |(i, v)| (idx as u64 + 1) * (i as u64 + 1) * (v.1 as u64))
                .sum::<u64>()
        })
        .sum::<u64>()
}

#[cfg(test)]
mod test {
    use crate::*;

    use rstest::rstest;

    #[rstest]
    #[case("rn=1", 30)]
    #[case("cm-", 253)]
    #[case("qp=3", 97)]
    #[case("cm=2", 47)]
    pub fn test_hash(#[case] inp: &str, #[case] expected: u8) {
        assert_eq!(hash(inp), expected);
    }

    #[test]
    pub fn test_part_one() {
        let input = vec!["rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7".to_string()];

        let res = part_one(input);

        assert_eq!(res, 1320);
    }

    #[test_log::test]
    pub fn test_part_two() {
        let input = vec!["rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7".to_string()];

        let res = part_two(input);

        assert_eq!(res, 145);
    }
}
