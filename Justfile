test:
    cargo test

build:
    cargo build

run:
    cargo run

fmt:
    cargo fmt

check:
    cargo check

example $example_name:
    cargo run --release --example $example_name
