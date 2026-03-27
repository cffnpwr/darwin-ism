# Critical Rules

## Development Environment

All build, lint, and format commands require the Nix dev shell. Run `nix develop` before any other command. Without it, `cargo`, `treefmt`, and other tools will not be available.

To run a single command without entering the shell interactively:

```bash
nix develop --command cargo build
nix develop --command cargo clippy -- -D warnings
nix develop --command treefmt --fail-on-change
```

## Build and Quality Gates

All of the following must pass before committing:

1. **Build**: `cargo build` must succeed without errors.
2. **Lint**: `cargo clippy -- -D warnings` — deny warnings mode treats all warnings as errors.
3. **Format**: `treefmt --fail-on-change` — code must match formatting standards.

Pre-commit hooks (lefthook) enforce format automatically.

## Rust Toolchain

The Rust toolchain version is specified in `rust-toolchain.toml` (nightly channel). Do not change it without updating `flake.nix` accordingly.

## Release-Please Markers

The version in `Cargo.toml` is managed by release-please:

```toml
# x-release-please-start-version
version = "0.1.0"
# x-release-please-end
```

Do not manually change the version string. It is updated automatically by the release-please workflow.

## CI Requirements

CI runs on every push to main and on PRs:
- **build**: Compiles on both arm64 (macOS-15) and amd64 (macOS-15-Intel)
- **clippy**: Deny warnings mode
- **format**: Fails on any formatting difference
