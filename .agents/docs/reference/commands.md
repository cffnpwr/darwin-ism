# Command Reference

## Development Environment

| Command | Description |
|---------|-------------|
| `nix develop` | Enter dev shell with all tools (runs lefthook install) |
| `mise install` | Install tools via mise (alternative to Nix) |

## Building

| Command | Description |
|---------|-------------|
| `swift build` | Debug build |
| `swift build -c release` | Release build |
| `nix build` | Reproducible Nix build |
| `nix run` | Build and run via Nix |

## Running the CLI

| Command | Description |
|---------|-------------|
| `./.build/debug/darwin-ism list` | List all input sources |
| `./.build/debug/darwin-ism list --enabled` | List enabled sources only |
| `./.build/debug/darwin-ism list --bundle-id <id>` | Filter by bundle ID |
| `./.build/debug/darwin-ism enable <id>` | Enable an input source |
| `./.build/debug/darwin-ism disable <id>` | Disable an input source |

## Quality Checks

| Command | Description |
|---------|-------------|
| `swiftlint lint --config .swiftlint.yaml --strict` | Lint Swift (strict mode) |
| `nix run .#lint -- --config .swiftlint.yaml --strict` | Lint via Nix |
| `treefmt` | Format all files |
| `treefmt --fail-on-change` | Check formatting |
| `nix run .#format` | Format via Nix |
| `nix run .#format -- --fail-on-change` | Check formatting via Nix |

## Version Checks

| Command | Description |
|---------|-------------|
| `scripts/check-swift-version.sh` | Verify Swift version consistency |
| `scripts/check-tool-versions.sh` | Check tool version compatibility |

## Nix Dependency Management

After changing Swift dependencies in `Package.swift`:
1. Run `swift package resolve` to update `Package.resolved`
2. Regenerate Nix files with swiftpm2nix
