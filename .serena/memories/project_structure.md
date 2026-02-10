# Project Structure

```
darwin-ism/
├── Sources/darwin-ism/              # Swift source code
│   ├── CLI.swift                    # Main entry point, subcommands (list, enable, disable)
│   ├── InputSource.swift            # InputSource struct wrapping TISInputSource
│   └── InputSourceManager.swift     # Static methods for listing/finding input sources
├── nix/                             # Nix build configuration
│   ├── darwin-ism/
│   │   ├── package.nix              # Nix package definition
│   │   └── generated/               # Auto-generated swiftpm2nix files
│   ├── swiftformat.nix              # SwiftFormat nix package
│   └── swiftlint.nix                # SwiftLint nix package
├── scripts/                         # Development scripts
│   ├── check-swift-version.sh       # Version consistency check
│   └── check-tool-versions.sh       # Tool version check
├── .github/
│   ├── workflows/                   # CI/CD workflows
│   │   ├── ci.yaml                  # Build, lint, format
│   │   ├── publish.yaml             # Release artifacts
│   │   ├── release-please.yaml      # Automated versioning
│   │   └── check-status.yaml        # PR status monitoring
│   └── release-please/              # Release-please config
├── Package.swift                    # SPM manifest
├── Package.resolved                 # Locked dependencies
├── flake.nix                        # Nix flake config
├── flake.lock                       # Locked nix inputs
├── lefthook.yaml                    # Pre-commit hooks
├── treefmt.toml                     # Formatter config
├── mise.toml                        # Tool version manager
├── .swift-version                   # Swift version (5.10.1)
├── .swiftformat                     # SwiftFormat rules
├── .swiftlint.yaml                  # SwiftLint rules
└── .editorconfig                    # Editor settings
```

## Key Source Files
- **CLI.swift**: ~270 lines. Contains DarwinISM main command, ListSources/Enable/Disable subcommands, wide character display utilities. Version is managed by release-please markers.
- **InputSource.swift**: ~60 lines. Wraps TISInputSource with Swift-friendly properties and enable/disable methods.
- **InputSourceManager.swift**: ~30 lines. Static enum providing list/find/listEnabled methods using TIS API.
