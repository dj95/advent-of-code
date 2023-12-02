use day_02::*;

fn main() {
    let lines = read_lines();

    let res = part_one(
        lines,
        Set {
            blue: 14,
            green: 13,
            red: 12,
        },
    );

    println!("part 1 :: {}", res);
}
