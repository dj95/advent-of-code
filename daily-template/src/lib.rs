pub fn read_lines() -> Vec<String> {
    let mut res = Vec::new();

    let content = include_str!("../input.txt");
    for line in content.lines() {
        res.push(line.to_string());
    }

    res
}


pub fn part_one(inp: Vec<String>) -> String {
    "".to_string()
}

pub fn part_two(inp: Vec<String>) -> String {
    "".to_string()
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    pub fn test_part_one() {
        let input = Vec::from([
            "".to_string(),
        ]);

        let res = part_one(input);

        assert_eq!(res, "");
    }

    #[test]
    pub fn test_part_two() {
        let input = Vec::from([
            "".to_string(),
        ]);

        let res = part_two(input);

        assert_eq!(res, "");
    }
}
