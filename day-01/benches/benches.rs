use day_01::{part_one, read_lines, part_two};

fn main() {
    divan::main();
}

#[divan::bench]
fn part_1() {
    let lines = read_lines();

    part_one(lines);
}

#[divan::bench]
fn part_2() {
    let lines = read_lines();

    part_two(lines);
}

