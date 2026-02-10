# Architecture

## Overview

darwin-ism is a single-target Swift CLI application that wraps the macOS Carbon Framework's TIS (Text Input Services) API.

```
CLI (swift-argument-parser)
  └── InputSourceManager (static query methods)
        └── InputSource (TISInputSource wrapper)
              └── Carbon Framework TIS API
```

## Source Files

### CLI.swift
Main entry point using `@main` attribute. Defines:

- **DarwinISM**: Root `ParsableCommand` with subcommands
- **ListSources**: Lists input sources with optional filters (`--enabled`, `--bundle-id`)
- **Enable**: Enables an input source by ID
- **Disable**: Disables an input source by ID, with special ABC keyboard workaround

Also contains wide character display utilities (`isWideCharacter`, `displayWidth`, `padToWidth`) for proper CJK output alignment.

### InputSource.swift
Value type wrapping `TISInputSource`. Provides Swift-friendly access to:
- `id`, `bundleID`, `localizedName`, `type` (string properties)
- `isEnabled`, `isEnableCapable` (boolean properties)
- `enable()`, `disable()` (returns `OSStatus`)

Properties are extracted via `TISGetInputSourceProperty()` with safe CF type casting.

### InputSourceManager.swift
Stateless `enum` with static methods:
- `list(includeAllInstalled:bundleID:)` — query input sources
- `find(byID:)` — find a specific input source
- `listEnabled()` — list only enabled sources

## Key Design Patterns

- **Enum as namespace**: `InputSourceManager` is an enum (not instantiable) used purely for static methods
- **Safe CF bridging**: All CF type conversions use fallback values ("Unknown") to handle missing properties
- **Exit codes**: Defined as `AppExitCode` enum (success=0, notFound=1, operationFailed=2, invalidArgument=3)

## ABC Keyboard Workaround

When disabling the ABC keyboard layout, macOS may silently refuse. The `Disable` command implements a multi-step workaround:

1. Attempt direct disable
2. If the source remains enabled, enable Japanese Romaji input (Kotoeri) temporarily
3. Enable Roman mode in Kotoeri
4. Retry the disable
5. Clean up temporary input sources if possible
