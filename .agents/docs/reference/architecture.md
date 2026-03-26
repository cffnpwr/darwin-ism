# Architecture

## Overview

darwin-ism is a Rust CLI application that wraps the macOS Carbon Framework's TIS (Text Input Services) API. The project is a Cargo workspace with two crates.

```
CLI (clap derive)                     — src/cli.rs
  └── High-level API                  — src/lib.rs
        └── TisManager (query methods) — text-input-source/src/tis_manager.rs
              └── InputSource (TISInputSourceRef wrapper) — text-input-source/src/input_source.rs
                    └── Carbon Framework TIS API (hand-written FFI) — text-input-source/src/ffi.rs
```

## Workspace Structure

### `darwin-ism` (root crate)

The main binary crate.

#### `src/main.rs`

Minimal entry point. Parses CLI args and calls `cli::run()`.

#### `src/cli.rs`

CLI definition and command implementations using clap derive:

- **`Cli`**: Root struct with optional `--version` flag and subcommand
- **`Commands`**: Enum of subcommands (`List`, `Enable`, `Disable`)
- **`ListArgs`**: `--enabled` flag and `--bundle-id` filter
- **`EnableArgs`**: positional `<id>` argument
- **`DisableArgs`**: positional `<id>` argument

Also contains wide character display utilities (`is_wide_char`, `display_width`, `pad_to_width`) for proper CJK output alignment.

Build-time version information is embedded via `build.rs`:
```rust
pub const VERSION: &str = concat!(
    env!("CARGO_PKG_VERSION"),
    "\ncommit: ", env!("DARWIN_ISM_GIT_HASH"),
    "\nbuilt-at: ", env!("DARWIN_ISM_BUILT_AT"),
);
```

#### `src/lib.rs`

High-level API that wraps `text-input-source`:

```rust
pub fn list(include_all_installed: bool) -> Result<Vec<InputSource>>
pub fn list_enabled() -> Result<Vec<InputSource>>
pub fn find_by_id(id: &str) -> Result<Option<InputSource>>
pub fn enable(id: &str) -> Result<bool>
pub fn disable(id: &str) -> Result<bool>
```

### `text-input-source` (library crate)

TIS API Rust library, published as a reusable workspace member.

#### `text-input-source/src/ffi.rs`

Hand-written `unsafe extern "C"` FFI bindings to the Carbon Framework TIS API:
- `TISCreateInputSourceList`, `TISEnableInputSource`, `TISDisableInputSource`, etc.
- `kTISPropertyInputSourceID`, `kTISPropertyInputSourceIsEnabled`, etc.
- Linked via `#[link(name = "Carbon", kind = "framework")]`

#### `text-input-source/src/input_source.rs`

`InputSource` type wrapping `TISInputSourceRef`:
- Memory managed with `CFRetain`/`CFRelease` via `Clone`/`Drop`
- Marked `!Send + !Sync` via `PhantomData<Rc<()>>` — TIS API is not thread-safe
- Methods: `id()`, `bundle_id()`, `localized_name()`, `input_source_type()`, `is_enabled()`, `is_enable_capable()`, `enable()`, `disable()`, `select()`

#### `text-input-source/src/tis_manager.rs`

`TisManager` type with query methods:
- `list_input_sources(include_all_installed)` — all input sources
- `list_keyboard_input_sources(include_all_installed)` — keyboard category filter
- `list_input_sources_with_bundle_id(bundle_id, include_all_installed)` — bundle ID filter
- `current_keyboard_input_source()` — currently active source

#### `text-input-source/src/lib.rs`

Public API surface, error types, and global mutex:

```rust
pub enum TisError {
    NullResult(OperationKind),
    Status(OperationKind, OSStatus),
    MissingProperty(PropertyKind),
    UnexpectedPropertyType(PropertyKind),
}
```

All TIS operations are wrapped in `with_tis_lock()` — a global `Mutex` guard — because the TIS API is not thread-safe.

## Key Design Patterns

- **`!Send + !Sync`**: `PhantomData<Rc<()>>` enforces single-thread use of `InputSource` and `TisManager`
- **Global mutex**: `with_tis_lock()` serializes all TIS API calls
- **CF memory management**: `from_create_rule()` for owned pointers, `from_get_rule()` for borrowed; `Clone` CFRetains, `Drop` CFReleases
- **Build-time metadata**: `build.rs` embeds git hash and build timestamp via env vars

## ABC Keyboard Workaround

When disabling the ABC keyboard layout, macOS may silently refuse. The `disable()` function in `src/lib.rs` implements a multi-step workaround:

1. Attempt direct disable
2. If the source ID matches `com.apple.keylayout.ABC*` and it remains enabled:
   - Enable Kotoeri (Japanese Romaji) if not already enabled
   - Enable Kotoeri Roman mode (ASCII input)
   - Retry the disable
   - Clean up temporarily enabled Kotoeri sources
