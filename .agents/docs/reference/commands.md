# Command Reference

## Development Environment

All commands below (`cargo`, `treefmt`, etc.) must be run inside the Nix dev shell.
Enter it first with `nix develop`, or prefix any command with `nix develop --command <cmd>`.

| Command | Description |
|---------|-------------|
| `nix develop` | Enter dev shell with all tools (runs lefthook install) |
| `mise install` | Install tools via mise (alternative to Nix) |

## Building

| Command | Description |
|---------|-------------|
| `cargo build` | Debug build |
| `cargo build --release` | Release build |
| `nix build` | Reproducible Nix build |
| `nix run` | Build and run via Nix |

## Running the CLI

| Command | Description |
|---------|-------------|
| `./target/debug/darwin-ism list` | List all input sources |
| `./target/debug/darwin-ism list --enabled` | List enabled sources only |
| `./target/debug/darwin-ism list --bundle-id <id>` | Filter by bundle ID |
| `./target/debug/darwin-ism enable <id>` | Enable an input source |
| `./target/debug/darwin-ism disable <id>` | Disable an input source |

## Quality Checks

| Command | Description |
|---------|-------------|
| `cargo clippy -- -D warnings` | Lint Rust (deny warnings) |
| `cargo test` | Run tests |
| `treefmt` | Format all files |
| `treefmt --fail-on-change` | Check formatting (CI mode) |

## Rust Dependency Management

After changing dependencies in `Cargo.toml`:
1. Run `cargo update` to update `Cargo.lock`
2. Commit both `Cargo.toml` and `Cargo.lock`
