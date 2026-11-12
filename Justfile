[private]
default:
    @just --list

check:
    cargo clippy --workspace --all-targets -- -D warnings
    cargo fmt --check
    cargo test --workspace
    RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps
    ! rg TODO . -g '!Justfile'
    echo "Finished Successfully"
