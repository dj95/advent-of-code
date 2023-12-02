pub fn read_lines() -> Vec<String> {
    let mut res = Vec::new();

    let content = include_str!("../input.txt");
    for line in content.lines() {
        res.push(line.to_string());
    }

    res
}

pub fn find_first_number(inp: String) -> Option<u32> {
    inp.chars()
        .find(|c| c.is_ascii_digit())
        .map(|c| c.to_digit(10).unwrap())
}

pub fn find_last_number(inp: String) -> Option<u32> {
    inp.chars()
        .rev()
        .find(|c| c.is_ascii_digit())
        .map(|c| c.to_digit(10).unwrap())
}

pub fn get_calibration_number(inp: String) -> Option<u32> {
    let first_number = find_first_number(inp.clone());
    let last_number = find_last_number(inp);

    if first_number.is_none() || last_number.is_none() {
        return None;
    }

    let combined = format!("{}{}", first_number.unwrap(), last_number.unwrap());

    match combined.parse::<u32>() {
        Ok(c) => Some(c),
        Err(_) => None,
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn test_find_first_number() {
        let res = find_first_number("asd".to_string());
        assert_eq!(res, None);

        let res = find_first_number("as12d".to_string());
        assert_eq!(res, Some(1));
    }

    #[test]
    fn test_find_last_number() {
        let res = find_last_number("asd".to_string());
        assert_eq!(res, None);

        let res = find_last_number("as12d".to_string());
        assert_eq!(res, Some(2));
    }

    #[test]
    fn test_get_calibration_number() {
        let res = get_calibration_number("asd".to_string());
        assert_eq!(res, None);

        let res = get_calibration_number("as12d".to_string());
        assert_eq!(res, Some(12));
    }
}
