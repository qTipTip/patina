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

# Render one example into renders/<name>.png: `just render 02_orthographic_sphere_depth`.
render name:
    cargo run --release --example {{ name }} -- renders/{{ name }}.png

# Renders are deterministic, so `git status` shows which ones changed.
render-all:
    for e in examples/*.rs; do just render "$(basename "$e" .rs)"; done
