run day part:
    cargo run -p {{day}} --bin {{part}}

test day:
    cargo watch -x "nextest run -p {{day}}"

create day:
    cargo generate --path ./daily-template --name {{day}}
