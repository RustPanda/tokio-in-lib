default:
    just -l

build:
    cargo build --all

run: build
    cargo run