use day_01::*;

use std::collections::HashMap;

fn replace_written_digits(inp: String, reversed: bool) -> String {
    let patterns = HashMap::from([
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ]);

    let mut pattern_key = "".to_string();
    let mut index = inp.len() + 1;
    for key in patterns.keys() {
        let mut k = key.to_string();
        if reversed {
            k = key.chars().rev().collect();
        }
        match inp.find(&k) {
            Some(ind) => {
                if ind >= index {
                    continue;
                }

                index = ind;
                pattern_key = key.to_string();
            }
            None => {
                continue;
            }
        }
    }

    if pattern_key.is_empty() {
        return inp;
    }

    let mut repl = pattern_key.clone();
    if reversed {
        repl = pattern_key.chars().rev().collect();
    }

    let res = inp.replace(
        &repl,
        patterns.get(pattern_key.as_str()).unwrap().to_owned(),
    );

    replace_written_digits(res, reversed)
}

fn get_corrected_calibration_number(inp: String) -> Option<u32> {
    let inp_first = replace_written_digits(inp.clone(), false);

    let inp_rev: String = inp.chars().rev().collect();
    let inp_last = replace_written_digits(inp_rev, true);

    let first_number = find_first_number(inp_first.clone());
    let last_number = find_first_number(inp_last);

    if first_number.is_none() || last_number.is_none() {
        return None;
    }

    let combined = format!("{}{}", first_number.unwrap(), last_number.unwrap());

    match combined.parse::<u32>() {
        Ok(c) => Some(c),
        Err(_) => None,
    }
}

fn part_two_logic(lines: Vec<String>) -> u32 {
    let mut sum = 0;
    for line in lines.iter() {
        let calibration_number = get_corrected_calibration_number(line.to_string());

        if calibration_number.is_none() {
            continue;
        }

        let calibration_number = calibration_number.unwrap();

        sum += calibration_number;
    }

    sum
}

fn main() {
    let lines = read_lines();

    let sum = part_two_logic(lines);

    println!("part two :: {}", sum);
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn test_replace_written_digits() {
        let res = replace_written_digits("a".to_string(), false);
        assert_eq!(res, "a");

        let res = replace_written_digits("aone".to_string(), false);
        assert_eq!(res, "a1");

        let res = replace_written_digits("atwone".to_string(), false);
        assert_eq!(res, "a2ne");
    }

    #[test]
    fn test_get_corrected_callibration_number() {
        let tests = HashMap::from([
            ("two1nine", Some(29)),
            ("eightwothree", Some(83)),
            ("abcone2threexyz", Some(13)),
            ("xtwone3four", Some(24)),
            ("4nineeightseven2", Some(42)),
            ("zoneight234", Some(14)),
            ("7pqrstsixteen", Some(76)),
        ]);

        for (inp, exp) in tests {
            let res = get_corrected_calibration_number(inp.to_string());
            assert_eq!(res, exp);
        }
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

        let res = part_two_logic(input);
        assert_eq!(res, 281);
    }
}
