# darwin-ism

[![GitHub License](https://img.shields.io/github/license/cffnpwr/darwin-ism?style=flat)](./LICENSE)

A CLI tool for managing macOS input sources.

Allows you to list, enable, and disable input sources that macOS protects via `defaults write`, using the Carbon Framework's TIS API.

[日本語のREADMEはこちら](./README-ja.md)

## How to Install

### Nix (Flakes)

```bash
# Run directly
nix run github:cffnpwr/darwin-ism

# Install
nix profile install github:cffnpwr/darwin-ism
```

### Nix (non-Flakes)

```bash
nix-env -if https://github.com/cffnpwr/darwin-ism/archive/main.tar.gz
```

### GitHub Release

TBD

### Build from Source

#### Prerequisites

- macOS (aarch64-darwin or x86_64-darwin)

Prepare one of the following environments:

- [Nix](https://nixos.org/) - A Nix environment that supports Nix Flakes
- [mise](https://mise.jdx.dev/) - An environment with mise installed
- An environment with Swift 6.0 or later installed

If not using Nix, the following is also required:

- Xcode 16 or later (or Command Line Tools) - Apple SDK is required to use the Carbon Framework

#### How to build

1. Clone the repository

```bash
git clone https://github.com/cffnpwr/darwin-ism.git
cd darwin-ism
```

2. Set up the development environment

<details>
<summary>Using Nix</summary>

```bash
nix develop
```

The Nix environment automatically sets up Swift and the Apple SDK.

</details>

<details>
<summary>Using mise</summary>

```bash
mise install
```

</details>

<details>
<summary>Using Swift directly</summary>

Skip this step.

</details>

3. Build

```bash
swift build
```

Or, if using Nix:

```bash
nix build
```

4. Run

```bash
./darwin-ism --help

# If built with Nix
./result/bin/darwin-ism --help
```

## How to use

```
darwin-ism <COMMAND> [OPTIONS]
```

### List of commands

| Command | Description |
|---------|-------------|
| `list` | List all input sources |
| `enable <id>` | Enable an input source |
| `disable <id>` | Disable an input source |
| `help` | Show help |
| `version` | Show version |

### Options for `list` subcommand

| Option | Description |
|--------|-------------|
| `--enabled`, `-e` | Show only enabled input sources |
| `--bundle-id`, `-b <id>` | Filter by bundle ID |

### Examples

```bash
# List all input sources
darwin-ism list

# Show only enabled input sources
darwin-ism list --enabled

# Filter by specific bundle ID
darwin-ism list --bundle-id dev.ensan.inputmethod.azooKeyMac

# Enable an input source (check ID with list command)
darwin-ism enable <id>

# Disable an input source
darwin-ism disable <id>
```

## How to set up development environment

For setting up the development environment, refer to the [Prerequisites section under "Build from Source"](#prerequisites).

### Pre-commit Hook

This project uses [lefthook](https://github.com/evilmartians/lefthook) for pre-commit hooks.
In Nix environment, it is automatically installed when running `nix develop`.

### Linter / Formatter

| Tool | Purpose | Config File |
|------|---------|-------------|
| [SwiftFormat](https://github.com/nicklockwood/SwiftFormat) | Formatter | `.swiftformat` |
| [SwiftLint](https://github.com/realm/SwiftLint) | Linter | `.swiftlint.yaml` |
| [treefmt](https://github.com/numtide/treefmt) | Formatter multiplexer | `treefmt.toml` |

## License

[MIT License](./LICENSE)
