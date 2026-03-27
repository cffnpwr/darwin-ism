//! Safe wrapper for macOS Text Input Sources (TIS) APIs.
//!
//! This crate provides a small Rust interface for:
//! - listing input sources
//! - reading the current keyboard input source
//! - selecting an input source
//! - enabling an input source
//! - disabling an input source
//!
//! The main entry point is [`TisManager`]. Values returned from the system are
//! represented by [`InputSource`].
//!
//! This crate is intended for macOS. When used from GUI applications, call it
//! in the way required by Apple's documentation for `TextInputSources`.
//!
//! # Example
//!
//! ```no_run
//! use text_input_source::TisManager;
//!
//! let manager = TisManager::new();
//! let sources = manager.list_keyboard_input_sources(false)?;
//!
//! for source in &sources {
//!     println!(
//!         "{} ({}) enabled={}",
//!         source.localized_name()?.unwrap_or_else(|| "<unnamed>".into()),
//!         source.id()?.unwrap_or_else(|| "<unknown>".into()),
//!         source.is_enabled()?,
//!     );
//! }
//!
//! if let Some(us) = sources
//!     .iter()
//!     .find(|source| source.id().ok().flatten().as_deref() == Some("com.apple.keylayout.US"))
//! {
//!     us.select()?;
//! }
//! # Ok::<(), text_input_source::TisError>(())
//! ```
#[cfg(not(target_os = "macos"))]
compile_error!("text-input-source-rs supports macOS targets only");

mod ffi;
mod input_source;
mod tis_manager;

use core::fmt;
use std::fmt::Display;
use std::sync::{Mutex, OnceLock};

use core_foundation::base::{OSStatus, TCFType as _};
use core_foundation::string::{CFString, CFStringRef};

pub use crate::input_source::InputSource;
pub use crate::tis_manager::TisManager;

/// Errors returned by this crate.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum TisError {
    #[error("`{0}` returned NULL")]
    NullResult(OperationKind),

    #[error("`{0}` failed with OSStatus {1}")]
    Status(OperationKind, OSStatus),

    #[error("missing property: {0}")]
    MissingProperty(PropertyKind),

    #[error("unexpected Core Foundation type for property: {0}")]
    UnexpectedPropertyType(PropertyKind),
}

/// Convenient result type used by this crate.
pub type Result<T> = std::result::Result<T, TisError>;

/// Identifies a Text Input Sources operation exposed by this crate.
///
/// These values are surfaced through [`TisError`] to indicate which underlying
/// TIS API call returned `NULL` or failed with a non-zero `OSStatus`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OperationKind {
    /// `TISSelectInputSource`
    SelectInputSource,
    /// `TISEnableInputSource`
    EnableInputSource,
    /// `TISDisableInputSource`
    DisableInputSource,
    /// `TISCopyCurrentKeyboardInputSource`
    CopyCurrentKeyboardInputSource,
    /// `TISCreateInputSourceList`
    CreateInputSourceList,
}

impl OperationKind {
    const TIS_SELECT_INPUT_SOURCE_OP: &str = "TISSelectInputSource";
    const TIS_ENABLE_INPUT_SOURCE_OP: &str = "TISEnableInputSource";
    const TIS_DISABLE_INPUT_SOURCE_OP: &str = "TISDisableInputSource";
    const TIS_COPY_CURRENT_KEYBOARD_INPUT_SOURCE_OP: &str = "TISCopyCurrentKeyboardInputSource";
    const TIS_CREATE_INPUT_SOURCE_LIST_OP: &str = "TISCreateInputSourceList";

    fn name(self) -> &'static str {
        match self {
            OperationKind::SelectInputSource => Self::TIS_SELECT_INPUT_SOURCE_OP,
            OperationKind::EnableInputSource => Self::TIS_ENABLE_INPUT_SOURCE_OP,
            OperationKind::DisableInputSource => Self::TIS_DISABLE_INPUT_SOURCE_OP,
            OperationKind::CopyCurrentKeyboardInputSource => {
                Self::TIS_COPY_CURRENT_KEYBOARD_INPUT_SOURCE_OP
            }
            OperationKind::CreateInputSourceList => Self::TIS_CREATE_INPUT_SOURCE_LIST_OP,
        }
    }
}

impl Display for OperationKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

/// Identifies a property exposed by an input source.
///
/// These values are mainly surfaced through [`TisError`] when a property is
/// missing or has an unexpected type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PropertyKind {
    InputSourceCategory,
    IsEnabled,
    IsEnableCapable,
    InputSourceId,
    BundleId,
    InputSourceType,
    LocalizedName,
}

impl PropertyKind {
    const K_TIS_PROPERTY_INPUT_SOURCE_CATEGORY_NAME: &str = "kTISPropertyInputSourceCategory";
    const K_TIS_PROPERTY_INPUT_SOURCE_IS_ENABLED_NAME: &str = "kTISPropertyInputSourceIsEnabled";
    const K_TIS_PROPERTY_INPUT_SOURCE_IS_ENABLE_CAPABLE_NAME: &str =
        "kTISPropertyInputSourceIsEnableCapable";
    const K_TIS_PROPERTY_INPUT_SOURCE_ID_NAME: &str = "kTISPropertyInputSourceID";
    const K_TIS_PROPERTY_BUNDLE_ID_NAME: &str = "kTISPropertyBundleID";
    const K_TIS_PROPERTY_INPUT_SOURCE_TYPE_NAME: &str = "kTISPropertyInputSourceType";
    const K_TIS_PROPERTY_LOCALIZED_NAME_NAME: &str = "kTISPropertyLocalizedName";

    fn key(self) -> CFStringRef {
        match self {
            Self::InputSourceCategory => unsafe { ffi::kTISPropertyInputSourceCategory },
            Self::IsEnabled => unsafe { ffi::kTISPropertyInputSourceIsEnabled },
            Self::IsEnableCapable => unsafe { ffi::kTISPropertyInputSourceIsEnableCapable },
            Self::InputSourceId => unsafe { ffi::kTISPropertyInputSourceID },
            Self::BundleId => unsafe { ffi::kTISPropertyBundleID },
            Self::InputSourceType => unsafe { ffi::kTISPropertyInputSourceType },
            Self::LocalizedName => unsafe { ffi::kTISPropertyLocalizedName },
        }
    }

    fn name(self) -> &'static str {
        match self {
            Self::InputSourceCategory => Self::K_TIS_PROPERTY_INPUT_SOURCE_CATEGORY_NAME,
            Self::IsEnabled => Self::K_TIS_PROPERTY_INPUT_SOURCE_IS_ENABLED_NAME,
            Self::IsEnableCapable => Self::K_TIS_PROPERTY_INPUT_SOURCE_IS_ENABLE_CAPABLE_NAME,
            Self::InputSourceId => Self::K_TIS_PROPERTY_INPUT_SOURCE_ID_NAME,
            Self::BundleId => Self::K_TIS_PROPERTY_BUNDLE_ID_NAME,
            Self::InputSourceType => Self::K_TIS_PROPERTY_INPUT_SOURCE_TYPE_NAME,
            Self::LocalizedName => Self::K_TIS_PROPERTY_LOCALIZED_NAME_NAME,
        }
    }
}

impl fmt::Display for PropertyKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum InputSourceCategoryValue {
    KeyboardInputSource,
}

impl InputSourceCategoryValue {
    fn as_cf_string(self) -> CFString {
        match self {
            Self::KeyboardInputSource => {
                tis_constant_string(unsafe { ffi::kTISCategoryKeyboardInputSource })
            }
        }
    }
}

pub(crate) fn tis_constant_string(value: CFStringRef) -> CFString {
    unsafe { CFString::wrap_under_get_rule(value) }
}

fn with_tis_lock<T>(f: impl FnOnce() -> Result<T>) -> Result<T> {
    let mutex = tis_api_mutex();
    let _guard = match mutex.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    };
    f()
}

fn tis_api_mutex() -> &'static Mutex<()> {
    static TIS_API_MUTEX: OnceLock<Mutex<()>> = OnceLock::new();
    TIS_API_MUTEX.get_or_init(|| Mutex::new(()))
}
