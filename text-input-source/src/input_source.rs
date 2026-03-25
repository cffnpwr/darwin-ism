use std::ffi::c_void;
use std::fmt;
use std::marker::PhantomData;
use std::rc::Rc;

use core_foundation::base::{CFGetTypeID, CFRelease, CFRetain, CFTypeRef, TCFType as _};
use core_foundation::number::{CFBooleanGetTypeID, CFBooleanGetValue, CFBooleanRef};
use core_foundation::string::{CFString, CFStringGetTypeID, CFStringRef};
use core_foundation_sys::base::OSStatus;

use crate::{OperationKind, PropertyKind, Result, TisError, ffi, with_tis_lock};


/// A macOS text input source.
///
/// Use [`crate::TisManager`] to obtain values of this type.
///
/// # Example
///
/// ```no_run
/// use text_input_source::TisManager;
///
/// let manager = TisManager::new();
/// let source = manager.current_keyboard_input_source()?;
///
/// println!("{:?}", source.id()?);
/// println!("{:?}", source.localized_name()?);
/// # Ok::<(), text_input_source::TisError>(())
/// ```
pub struct InputSource {
    raw: ffi::TISInputSourceRef,
    _not_send_sync: PhantomData<Rc<()>>,
}

impl InputSource {
    /// Returns the stable input source identifier, if available.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use text_input_source::TisManager;
    ///
    /// let manager = TisManager::new();
    /// let source = manager.current_keyboard_input_source()?;
    /// println!("{:?}", source.id()?);
    /// # Ok::<(), text_input_source::TisError>(())
    /// ```
    pub fn id(&self) -> Result<Option<String>> {
        self.get_string_property(PropertyKind::InputSourceId)
    }

    /// Returns the localized display name, if available.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use text_input_source::TisManager;
    ///
    /// let manager = TisManager::new();
    /// let source = manager.current_keyboard_input_source()?;
    /// println!("{:?}", source.localized_name()?);
    /// # Ok::<(), text_input_source::TisError>(())
    /// ```
    pub fn localized_name(&self) -> Result<Option<String>> {
        self.get_string_property(PropertyKind::LocalizedName)
    }

    /// Returns whether this input source is currently enabled.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use text_input_source::TisManager;
    ///
    /// let manager = TisManager::new();
    /// let source = manager.current_keyboard_input_source()?;
    /// println!("{}", source.is_enabled()?);
    /// # Ok::<(), text_input_source::TisError>(())
    /// ```
    pub fn is_enabled(&self) -> Result<bool> {
        self.get_required_bool_property(PropertyKind::IsEnabled)
    }

    /// Selects this input source.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use text_input_source::TisManager;
    ///
    /// let manager = TisManager::new();
    /// let sources = manager.list_keyboard_input_sources(false)?;
    ///
    /// if let Some(source) = sources.first() {
    ///     source.select()?;
    /// }
    /// # Ok::<(), text_input_source::TisError>(())
    /// ```
    pub fn select(&self) -> Result<()> {
        with_tis_lock(|| {
            status_to_result(OperationKind::SelectInputSource, unsafe {
                ffi::TISSelectInputSource(self.raw)
            })
        })
    }

    /// Enables this input source.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use text_input_source::TisManager;
    ///
    /// let manager = TisManager::new();
    /// let sources = manager.list_keyboard_input_sources(true)?;
    ///
    /// if let Some(source) = sources.first() {
    ///     source.enable()?;
    /// }
    /// # Ok::<(), text_input_source::TisError>(())
    /// ```
    pub fn enable(&self) -> Result<()> {
        with_tis_lock(|| {
            status_to_result(OperationKind::EnableInputSource, unsafe {
                ffi::TISEnableInputSource(self.raw)
            })
        })
    }

    /// Disables this input source.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use text_input_source::TisManager;
    ///
    /// let manager = TisManager::new();
    /// let sources = manager.list_keyboard_input_sources(false)?;
    ///
    /// if let Some(source) = sources.first() {
    ///     source.disable()?;
    /// }
    /// # Ok::<(), text_input_source::TisError>(())
    /// ```
    pub fn disable(&self) -> Result<()> {
        with_tis_lock(|| {
            status_to_result(OperationKind::DisableInputSource, unsafe {
                ffi::TISDisableInputSource(self.raw)
            })
        })
    }

    fn get_string_property(&self, property: PropertyKind) -> Result<Option<String>> {
        with_tis_lock(|| {
            let value = self.property_value(property);
            if value.is_null() {
                return Ok(None);
            }

            let type_id = unsafe { CFGetTypeID(value as CFTypeRef) };
            if type_id != unsafe { CFStringGetTypeID() } {
                return Err(TisError::UnexpectedPropertyType(property));
            }

            let string = unsafe { CFString::wrap_under_get_rule(value as CFStringRef) };
            Ok(Some(string.to_string()))
        })
    }

    fn get_required_bool_property(&self, property: PropertyKind) -> Result<bool> {
        with_tis_lock(|| {
            let value = self.property_value(property);
            if value.is_null() {
                return Err(TisError::MissingProperty(property));
            }

            let type_id = unsafe { CFGetTypeID(value as CFTypeRef) };
            if type_id != unsafe { CFBooleanGetTypeID() } {
                return Err(TisError::UnexpectedPropertyType(property));
            }

            Ok(unsafe { CFBooleanGetValue(value as CFBooleanRef) })
        })
    }

    pub(crate) fn property_value(&self, property: PropertyKind) -> *const c_void {
        unsafe { ffi::TISGetInputSourceProperty(self.raw, property.key()) }
    }

    pub(crate) unsafe fn from_create_rule(raw: ffi::TISInputSourceRef) -> Self {
        Self {
            raw,
            _not_send_sync: PhantomData,
        }
    }

    pub(crate) unsafe fn from_get_rule(raw: ffi::TISInputSourceRef) -> Self {
        let retained = unsafe { CFRetain(raw as CFTypeRef) as ffi::TISInputSourceRef };
        unsafe { Self::from_create_rule(retained) }
    }
}

impl Clone for InputSource {
    fn clone(&self) -> Self {
        unsafe { Self::from_get_rule(self.raw) }
    }
}

impl Drop for InputSource {
    fn drop(&mut self) {
        unsafe {
            CFRelease(self.raw as CFTypeRef);
        }
    }
}

impl fmt::Debug for InputSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("InputSource")
            .field("raw", &self.raw)
            .finish()
    }
}

fn status_to_result(operation: OperationKind, status: OSStatus) -> Result<()> {
    match status {
        0 => Ok(()),
        status => Err(TisError::Status(operation, status)),
    }
}
