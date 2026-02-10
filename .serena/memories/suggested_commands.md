# Suggested Commands

## Development Environment
```bash
nix develop              # Enter dev shell (installs all tools, runs lefthook install)
mise install             # Alternative: install tools via mise
```

## Building
```bash
swift build              # Debug build
swift build -c release   # Release build
nix build                # Nix build (reproducible)
nix run                  # Build and run directly
```

## Running
```bash
./.build/debug/darwin-ism list              # List all input sources
./.build/debug/darwin-ism list --enabled    # List enabled only
./.build/debug/darwin-ism enable <id>       # Enable input source
./.build/debug/darwin-ism disable <id>      # Disable input source
```

## Linting
```bash
swiftlint lint --config .swiftlint.yaml --strict    # Lint Swift files (strict)
nix run .#lint -- --config .swiftlint.yaml --strict  # Lint via Nix
```

## Formatting
```bash
treefmt                          # Format all files
treefmt --fail-on-change         # Check formatting without changing
swiftformat .                    # Format Swift files only
nix run .#format                 # Format via Nix
nix run .#format -- --fail-on-change  # Check via Nix
```

## Testing
No test targets currently defined. Manual testing via CLI.

## System Utilities (Darwin/macOS)
```bash
ls, cd, grep, find               # Standard Unix utilities (BSD variants on macOS)
```
