use text_input_source::{InputSource, TisError, TisManager};

pub use text_input_source::InputSource as InputSourceInfo;

/// Errors returned by this crate.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("input source not found: {0}")]
    NotFound(String),

    #[error("input source cannot be enabled: {0}")]
    NotEnableCapable(String),

    #[error("operation failed: {0}")]
    OperationFailed(#[from] TisError),
}

pub type Result<T> = std::result::Result<T, Error>;

const KOTOERI_ROMAJI_ID: &str = "com.apple.inputmethod.Kotoeri.RomajiTyping";
const KOTOERI_ROMAN_MODE_ID: &str = "com.apple.inputmethod.Kotoeri.RomajiTyping.Roman";

/// Returns all installed input sources.
///
/// When `include_all_installed` is `false`, only currently enabled sources are returned.
pub fn list(include_all_installed: bool) -> Result<Vec<InputSource>> {
    let manager = TisManager::new();
    Ok(manager.list_input_sources(include_all_installed)?)
}

/// Returns all installed input sources matching the given bundle ID.
pub fn list_with_bundle_id(
    bundle_id: &str,
    include_all_installed: bool,
) -> Result<Vec<InputSource>> {
    let manager = TisManager::new();
    Ok(manager.list_input_sources_with_bundle_id(bundle_id, include_all_installed)?)
}

/// Returns only the currently enabled input sources.
pub fn list_enabled() -> Result<Vec<InputSource>> {
    let sources = list(false)?;
    let mut enabled = Vec::new();
    for source in sources {
        if source.is_enabled()? {
            enabled.push(source);
        }
    }
    Ok(enabled)
}

/// Finds an input source by its ID.
pub fn find_by_id(id: &str) -> Result<Option<InputSource>> {
    let sources = list(true)?;
    for source in sources {
        if source.id()?.as_deref() == Some(id) {
            return Ok(Some(source));
        }
    }
    Ok(None)
}

/// Enables the input source with the given ID.
///
/// Returns `Ok(false)` if the source was already enabled, `Ok(true)` if it was
/// successfully enabled.
pub fn enable(id: &str) -> Result<bool> {
    let source = find_by_id(id)?.ok_or_else(|| Error::NotFound(id.to_owned()))?;

    if source.is_enabled()? {
        return Ok(false);
    }

    if !source.is_enable_capable()? {
        return Err(Error::NotEnableCapable(id.to_owned()));
    }

    source.enable()?;
    Ok(true)
}

/// Disables the input source with the given ID.
///
/// Returns `Ok(false)` if the source was already disabled, `Ok(true)` if it was
/// successfully disabled.
///
/// For `com.apple.keylayout.ABC*` sources that cannot be disabled directly,
/// a workaround via Kotoeri (Japanese Romaji input) is attempted automatically.
pub fn disable(id: &str) -> Result<bool> {
    let source = find_by_id(id)?.ok_or_else(|| Error::NotFound(id.to_owned()))?;

    if !source.is_enabled()? {
        return Ok(false);
    }

    let _ = source.disable();

    // Verify the source was actually disabled (TISDisableInputSource may return
    // noErr but not actually disable in some cases)
    let actually_disabled = match find_by_id(id)? {
        Some(updated) => !updated.is_enabled()?,
        None => true,
    };

    if actually_disabled {
        return Ok(true);
    }

    // If direct disable failed and target is ABC keyboard layout, try workaround
    if id.starts_with("com.apple.keylayout.ABC") {
        disable_abc_with_workaround(id, &source)
    } else {
        Err(Error::OperationFailed(TisError::Status(
            text_input_source::OperationKind::DisableInputSource,
            -1,
        )))
    }
}

/// Enables Kotoeri (Japanese Romaji input) if not already enabled.
/// Returns the source and whether it was already enabled.
fn enable_kotoeri_if_needed() -> Result<(InputSource, bool)> {
    let kotoeri = find_by_id(KOTOERI_ROMAJI_ID)?
        .ok_or_else(|| Error::NotFound(KOTOERI_ROMAJI_ID.to_owned()))?;

    let was_enabled = kotoeri.is_enabled()?;

    if !was_enabled {
        kotoeri.enable()?;
    }

    Ok((kotoeri, was_enabled))
}

/// Enables Roman mode (English input) in Kotoeri if not already enabled.
fn enable_roman_mode_if_needed() -> Result<()> {
    let roman = find_by_id(KOTOERI_ROMAN_MODE_ID)?
        .ok_or_else(|| Error::NotFound(KOTOERI_ROMAN_MODE_ID.to_owned()))?;

    if !roman.is_enabled()? {
        roman.enable()?;
    }

    Ok(())
}

/// Disables Kotoeri if it was temporarily enabled and other input methods exist.
fn disable_kotoeri_if_temporary(kotoeri: &InputSource, was_enabled: bool) -> Result<()> {
    if was_enabled {
        return Ok(());
    }

    let enabled_sources = list_enabled()?;
    let has_other = enabled_sources.iter().any(|src| {
        let id = src.id().ok().flatten().unwrap_or_default();
        let type_str = src.input_source_type().ok().flatten().unwrap_or_default();
        id != KOTOERI_ROMAJI_ID
            && !id.starts_with("com.apple.inputmethod.Kotoeri.")
            && (type_str.contains("KeyboardInputMethod") || type_str.contains("KeyboardLayout"))
    });

    if has_other {
        kotoeri.disable()?;
    }

    Ok(())
}

fn disable_abc_with_workaround(id: &str, source: &InputSource) -> Result<bool> {
    // Step 1: Enable Kotoeri as alternative
    let (kotoeri, kotoeri_was_enabled) = enable_kotoeri_if_needed()?;

    // Step 2: Enable Roman mode in Kotoeri
    enable_roman_mode_if_needed()?;

    // Step 3: Try to disable ABC again
    source.disable()?;

    // Verify actually disabled
    let actually_disabled = match find_by_id(id)? {
        Some(updated) => !updated.is_enabled()?,
        None => true,
    };

    if !actually_disabled {
        return Err(Error::OperationFailed(TisError::Status(
            text_input_source::OperationKind::DisableInputSource,
            -1,
        )));
    }

    // Step 4: Disable Kotoeri if it was temporarily enabled
    disable_kotoeri_if_temporary(&kotoeri, kotoeri_was_enabled)?;

    Ok(true)
}
