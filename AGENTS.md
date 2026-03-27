# AGENTS.md - darwin-ism

## Project Summary

darwin-ism is a macOS CLI tool for managing input sources (keyboard layouts, input methods) using the Carbon Framework's TIS API. It enables programmatic listing, enabling, and disabling of input sources that macOS restricts via standard settings. Built with Rust and Nix Flakes.

## Critical Rules

- Clippy runs in **deny warnings mode** — all warnings are errors. Run `cargo clippy -- -D warnings` before committing.
- Format with `treefmt` before committing. CI fails on unformatted code.
- The Rust toolchain version is specified in `rust-toolchain.toml`. Do not change it without updating `flake.nix` accordingly.
- The version string in `Cargo.toml` is managed by release-please markers (`x-release-please-start-version` / `x-release-please-end`). Do not manually edit the version.

→ Details: `.agents/docs/critical/rules.md`

## Quick Start

All commands below require the Nix dev shell. Run `nix develop` first, or use `nix develop --command <cmd>` to run a single command non-interactively.

```bash
# Enter dev shell (Nix) — required before all other commands
nix develop

# Build
cargo build

# Run
./target/debug/darwin-ism list
./target/debug/darwin-ism list --enabled
./target/debug/darwin-ism enable <id>
./target/debug/darwin-ism disable <id>

# Lint
cargo clippy -- -D warnings

# Format
treefmt
```

→ Full reference: `.agents/docs/reference/commands.md`

## Task Navigation

### Adding or Modifying CLI Commands

**Task**: Add new subcommands, flags, or options to the CLI

→ Read: `.agents/docs/reference/architecture.md`

Subcommands are defined in `src/cli.rs` using clap derive. Follow existing patterns (`ListArgs`, `EnableArgs`, `DisableArgs`).

### Working with Input Sources

**Task**: Modify how input sources are queried, enabled, or disabled

→ Read: `.agents/docs/reference/architecture.md`

Core types: `InputSource` (wrapper around `TISInputSourceRef`) in `text-input-source/src/input_source.rs` and `TisManager` (query methods) in `text-input-source/src/tis_manager.rs`.

### Nix Build Configuration

**Task**: Modify build settings, dependencies, or dev shell

→ Key files: `flake.nix`, `Cargo.toml`

After changing Rust dependencies, run `cargo update` and commit `Cargo.lock`.

### Release and Versioning

**Task**: Understand or modify the release process

→ Read: `.agents/docs/workflows/release.md`

### CI/CD

**Task**: Modify GitHub Actions workflows

→ Workflows in `.github/workflows/`: `build-check.yaml` (build/lint/format), `release-please.yaml` (versioning), `publish.yaml` (release artifacts).

## Project Structure

```
src/                          # Rust source (CLI entry point, high-level API)
  main.rs                     # Entry point
  cli.rs                      # CLI definition and command implementations (clap derive)
  lib.rs                      # High-level API over text-input-source
text-input-source/            # Rust library crate (TIS API wrapper)
  src/
    ffi.rs                    # Hand-written Carbon Framework FFI bindings
    input_source.rs           # InputSource type (TISInputSourceRef wrapper)
    tis_manager.rs            # TisManager query methods
    lib.rs                    # Public API, error types, global mutex
scripts/                      # Dev scripts
.github/workflows/            # CI/CD workflows
```

## Development Tools

| Tool           | Role                 | Config                    |
| -------------- | -------------------- | ------------------------- |
| Clippy         | Linting              | (rustup component)        |
| rustfmt        | Rust formatting      | (rustup component)        |
| treefmt        | Multi-formatter      | `treefmt.toml`            |
| lefthook       | Pre-commit hooks     | `lefthook.yaml`           |
| mise           | Tool version manager | `mise.toml`               |
| release-please | Automated releases   | `.github/release-please/` |

## Commit Convention

Conventional commits with gitmoji. Examples from this project:

- `feat: :sparkles: description` — new feature
- `fix: :bug: description` — bug fix
- `ci: :green_heart: description` — CI changes

Japanese descriptions are standard in this project.
