work day part:
    cargo run -p {{day}} --bin {{part}}

test day:
    cargo watch -x "nextest run -p {{day}}"
