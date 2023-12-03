use day_02::{part_one, read_lines, Set, part_two};

fn main() {
    divan::main();
}

#[divan::bench]
fn bench_part_1() {
    let lines = read_lines();
    part_one(
        lines,
        Set {
            blue: 14,
            green: 13,
            red: 12,
        },
    );
}

#[divan::bench]
fn bench_part_2() {
    let lines = read_lines();
    part_two(lines);
}
