use onig::Regex;

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

pub fn get_numbers(inp: String, with_words: bool) -> Option<String> {
    let mut regex = "1|2|3|4|5|6|7|8|9|0".to_string();
    if with_words {
        regex = format!("{}|one|two|three|four|five|six|seven|eight|nine", regex);
    }

    let pattern = Regex::new(format!("(?=({}))", regex).as_str()).unwrap();

    let res = pattern
        .captures_iter(inp.as_str())
        .fold("".to_string(), |res, mat| {
            let r = mat.at(1).unwrap();

            let mut digit = r.to_string();
            if with_words {
                digit = word_to_digit(r.to_string());
            }

            format!("{}{}", res, digit)
        });

    if res.is_empty() {
        return None;
    }

    Some(res)
}

fn word_to_digit(inp: String) -> String {
    match inp.as_str() {
        "one" => "1".to_string(),
        "two" => "2".to_string(),
        "three" => "3".to_string(),
        "four" => "4".to_string(),
        "five" => "5".to_string(),
        "six" => "6".to_string(),
        "seven" => "7".to_string(),
        "eight" => "8".to_string(),
        "nine" => "9".to_string(),
        &_ => inp,
    }
}

pub fn get_calibration_number(inp: String, with_words: bool) -> Option<u32> {
    let digits = get_numbers(inp, with_words).unwrap_or("".to_string());

    let first_number = find_first_number(digits.clone());
    let last_number = find_last_number(digits);

    if first_number.is_none() || last_number.is_none() {
        return None;
    }

    let combined = format!("{}{}", first_number.unwrap(), last_number.unwrap());

    match combined.parse::<u32>() {
        Ok(c) => Some(c),
        Err(_) => None,
    }
}

pub fn part_one(lines: Vec<String>) -> u32 {
    lines
        .iter()
        .map(|l| get_calibration_number(l.to_string(), false))
        .filter(|cn| cn.is_some())
        .fold(0, |mut sum, cn| {
            sum += cn.unwrap();
            sum
        })
}

pub fn part_two(lines: Vec<String>) -> u32 {
    let mut sum = 0;
    for line in lines.iter() {
        let calibration_number = get_calibration_number(line.to_string(), true);

        if calibration_number.is_none() {
            continue;
        }

        let calibration_number = calibration_number.unwrap();

        sum += calibration_number;
    }

    sum
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
        let res = get_calibration_number("asd".to_string(), false);
        assert_eq!(res, None);

        let res = get_calibration_number("as12d".to_string(), false);
        assert_eq!(res, Some(12));
    }

    #[test]
    fn test_get_numbers() {
        let res = get_numbers("asd".to_string(), false);
        assert_eq!(res, None);

        let res = get_numbers("as12threed".to_string(), false);
        assert_eq!(res, Some("12".to_string()));

        let res = get_numbers("as12threed".to_string(), true);
        assert_eq!(res, Some("123".to_string()));

        let res = get_numbers("twone".to_string(), true);
        assert_eq!(res, Some("21".to_string()));
    }

    #[test]
    fn test_part_2_logic() {
        let input = Vec::from([
            "two1nine".to_string(),
            "eightwothree".to_string(),
            "abcone2threexyz".to_string(),
            "xtwone3four".to_string(),
            "4nineeightseven2".to_string(),
            "zoneight234".to_string(),
            "7pqrstsixteen".to_string(),
        ]);

        let res = part_two(input);
        assert_eq!(res, 281);
    }
}
