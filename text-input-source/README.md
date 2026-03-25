# text-input-source

[![GitHub License](https://img.shields.io/github/license/cffnpwr/darwin-ism?style=flat)](./LICENSE)

A Rust library for macOS Text Input Sources (TIS) API.

[日本語のREADMEはこちら](./README-ja.md)

## Overview

**macOS only.**

`text-input-source` is a Rust library for managing keyboard layouts and input methods on macOS via the Carbon Framework's TIS API.

## Features

- List all installed input sources
- List keyboard input sources
- Get the current keyboard input source
- Select an input source
- Enable an input source
- Disable an input source

## Requirements

- macOS 10.5 or later

## Usage

Add this to your `Cargo.toml`.

```toml
[dependencies]
text-input-source = "0.1"
```

### Example

```rust
use text_input_source::TisManager;

fn main() -> Result<(), text_input_source::TisError> {
    let manager = TisManager::new();

    // List all enabled keyboard input sources
    let sources = manager.list_keyboard_input_sources(false)?;
    for source in &sources {
        println!(
            "{} ({}) enabled={}",
            source.localized_name()?.unwrap_or_else(|| "<unnamed>".into()),
            source.id()?.unwrap_or_else(|| "<unknown>".into()),
            source.is_enabled()?,
        );
    }

    // Select the US keyboard layout
    if let Some(us) = sources
        .iter()
        .find(|s| s.id().ok().flatten().as_deref() == Some("com.apple.keylayout.US"))
    {
        us.select()?;
    }

    Ok(())
}
```

## API

### `TisManager`

| Method | Description |
|--------|-------------|
| `TisManager::new()` | Create a new manager |
| `list_input_sources(include_all_installed: bool)` | List all input sources |
| `list_keyboard_input_sources(include_all_installed: bool)` | List keyboard input sources |
| `current_keyboard_input_source()` | Get the current keyboard input source |

When `include_all_installed` is `false`, only enabled sources are returned.
When `true`, all installed sources, including disabled ones, are returned.

### `InputSource`

| Method | Description |
|--------|-------------|
| `id()` | Get the input source identifier |
| `localized_name()` | Get the localized display name |
| `is_enabled()` | Get whether this input source is enabled |
| `select()` | Select this input source |
| `enable()` | Enable this input source |
| `disable()` | Disable this input source |

### `TisError`

| Variant | Description |
|---------|-------------|
| `NullResult(OperationKind)` | A TIS API call returned NULL |
| `Status(OperationKind, OSStatus)` | A TIS API call failed with a non-zero OSStatus |
| `MissingProperty(PropertyKind)` | A required property was not found |
| `UnexpectedPropertyType(PropertyKind)` | A property had an unexpected Core Foundation type |

## Thread Safety

Because the TIS API is not thread-safe, `TisManager` and `InputSource` are `!Send + !Sync`.

## License

[MIT License](./LICENSE)
