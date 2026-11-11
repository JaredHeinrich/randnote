[private]
default:
    @just --list

check:
    cargo clippy --workspace --all-targets -- -D warnings
    cargo fmt --check
    cargo test
    rg TODO .
