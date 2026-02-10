# darwin-ism - Project Overview

## Purpose
A macOS CLI tool for managing input sources (keyboard layouts, input methods).
It uses the Carbon Framework's TIS (Text Input Services) API to programmatically
list, enable, and disable input sources that macOS restricts via standard settings.

## Tech Stack
- **Language:** Swift 5.10.1
- **Platform:** macOS 14+ only (arm64-darwin, x86_64-darwin)
- **Build System:** Nix Flakes (primary), Swift Package Manager (fallback)
- **CLI Framework:** swift-argument-parser (1.3.0 ..< 1.6.0)
- **System Frameworks:** Carbon (TIS API), Foundation
- **Dev Tools:** mise (tool version manager)
- **Linting:** SwiftLint (strict mode)
- **Formatting:** SwiftFormat + nixfmt via treefmt
- **Pre-commit:** lefthook
- **Release:** release-please (conventional commits)
- **CI/CD:** GitHub Actions

## Key Architectural Decisions
- Carbon Framework for low-level TIS API access (not available via standard Swift APIs)
- Wide character display width handling for CJK/Japanese output
- Special workaround for disabling ABC keyboard layout (enables Japanese Romaji temporarily)
- Nix as primary build system with strict reproducibility
- flake.lock is the authoritative Swift version source

## License
MIT
