use day_13::{part_one, part_two, read_lines};

fn main() {
    divan::main();
}

#[divan::bench]
fn bench_part_1() {
    let lines = read_lines();
    part_one(lines);
}

#[divan::bench]
fn bench_part_2() {
    let lines = read_lines();
    part_two(lines);
}
