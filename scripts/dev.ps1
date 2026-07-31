param(
    [ValidateSet("run", "check", "clippy", "test")]
    [string]$Mode = "run"
)

$ErrorActionPreference = "Stop"

if (-not (Get-Command cargo-watch -ErrorAction SilentlyContinue)) {
    Write-Host "cargo-watch is not installed."
    Write-Host "Install it with:"
    Write-Host "  cargo install cargo-watch"
    exit 1
}

$watchArgs = @(
    "-c",
    "-w", "Cargo.toml",
    "-w", "Cargo.lock",
    "-w", "rust-toolchain.toml",
    "-w", "crates",
    "-w", "campaigns"
)

switch ($Mode) {
    "run" {
        cargo watch @watchArgs -x "run -p storyforge-tui"
    }
    "check" {
        cargo watch @watchArgs -x "check --workspace --locked"
    }
    "clippy" {
        cargo watch @watchArgs -x "clippy --workspace --all-targets --all-features --locked -- -D warnings"
    }
    "test" {
        cargo watch @watchArgs -x "test --workspace --all-features --locked"
    }
}
