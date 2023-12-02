use day_01::*;

fn main() {
    let lines = read_lines();

    let mut sum = 0;
    for line in lines.iter() {
        let calibration_number = get_calibration_number(line.to_string());

        if calibration_number.is_none() {
            continue;
        }

        let calibration_number = calibration_number.unwrap();

        sum += calibration_number;
    }

    println!("part one :: {}", sum);
}
