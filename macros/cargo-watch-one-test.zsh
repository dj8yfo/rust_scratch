#!/usr/bin/env zsh

# https://stackoverflow.com/a/47743269/2085356

# 1. Make sure to install cargo-watch via `cargo install cargo-watch`.
# More info about cargo-watch: https://crates.io/crates/cargo-watch

# 2. Make sure to install cargo-limit via `cargo install cargo-limit`.
# More info about carg-limit: https://crates.io/crates/cargo-limit

# More info about cargo test: https://doc.rust-lang.org/book/ch11-02-running-tests.html
# 2 sets of options:
# 1. cargo test --help      => these go to the cargo test command
# 2. cargo test -- --help   => these go to the binary that is being tested

# cargo watch -x check -x "ltest $argv --color always -q --message-format short  -- --nocapture" -c -q
cargo watch -x check -x "test --test $argv" -c -q
