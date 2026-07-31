#!/usr/bin/env bash
set -euo pipefail

MODE="${1:-run}"

if ! command -v cargo-watch >/dev/null 2>&1; then
    echo "cargo-watch is not installed."
    echo "Install it with:"
    echo "  cargo install cargo-watch"
    exit 1
fi

WATCH_ARGS=(
    -c
    -w Cargo.toml
    -w Cargo.lock
    -w rust-toolchain.toml
    -w crates
    -w campaigns
)

case "$MODE" in
    run)
        cargo watch "${WATCH_ARGS[@]}" -x "run -p storyforge-tui"
        ;;
    check)
        cargo watch "${WATCH_ARGS[@]}" -x "check --workspace --locked"
        ;;
    clippy)
        cargo watch "${WATCH_ARGS[@]}" -x "clippy --workspace --all-targets --all-features --locked -- -D warnings"
        ;;
    test)
        cargo watch "${WATCH_ARGS[@]}" -x "test --workspace --all-features --locked"
        ;;
    *)
        echo "Usage: $0 {run|check|clippy|test}"
        exit 1
        ;;
esac
