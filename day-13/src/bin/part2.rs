use day_13::*;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

fn main() {
    let lines = read_lines();
    let stdout_log = tracing_subscriber::fmt::layer();

    tracing_subscriber::registry().with(stdout_log).init();
    let res = part_two(lines);

    println!("part 2 :: {}", res);
}
