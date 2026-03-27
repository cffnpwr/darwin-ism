use std::marker::PhantomData;
use std::rc::Rc;

use core_foundation::array::{CFArrayGetCount, CFArrayGetValueAtIndex, CFArrayRef};
use core_foundation::base::{Boolean, CFRelease, CFTypeRef, TCFType};
use core_foundation::dictionary::{CFDictionary, CFDictionaryRef};
use core_foundation::string::CFString;

use crate::input_source::InputSource;
use crate::{
    InputSourceCategoryValue,
    OperationKind,
    PropertyKind,
    Result,
    TisError,
    ffi,
    tis_constant_string,
    with_tis_lock,
};

/// Entry point for querying and changing macOS text input sources.
///
/// Create one manager and use it to list available sources, inspect the
/// current keyboard input source, or obtain [`InputSource`] values to enable,
/// disable, and select.
///
/// # Example
///
/// ```no_run
/// use text_input_source::TisManager;
///
/// let manager = TisManager::new();
/// let current = manager.current_keyboard_input_source()?;
/// println!("{:?}", current.localized_name()?);
/// # Ok::<(), text_input_source::TisError>(())
/// ```
#[derive(Debug, Default)]
pub struct TisManager {
    _not_send_sync: PhantomData<Rc<()>>,
}

impl TisManager {
    /// Creates a new manager.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use text_input_source::TisManager;
    ///
    /// let manager = TisManager::new();
    /// let _ = manager.current_keyboard_input_source()?;
    /// # Ok::<(), text_input_source::TisError>(())
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self {
            _not_send_sync: PhantomData,
        }
    }

    /// Returns input sources that match the current system view.
    ///
    /// When `include_all_installed` is `false`, the returned list is limited to
    /// sources that are currently enabled. When `true`, installed sources that
    /// are not currently enabled may also be included.
    ///
    /// # Errors
    ///
    /// Returns [`TisError`] if the underlying TIS API call fails.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use text_input_source::TisManager;
    ///
    /// let manager = TisManager::new();
    /// let sources = manager.list_input_sources(false)?;
    /// println!("{}", sources.len());
    /// # Ok::<(), text_input_source::TisError>(())
    /// ```
    pub fn list_input_sources(&self, include_all_installed: bool) -> Result<Vec<InputSource>> {
        list_input_sources_impl(None, include_all_installed)
    }

    /// Returns keyboard input sources.
    ///
    /// When `include_all_installed` is `false`, the returned list is limited to
    /// sources that are currently enabled. When `true`, installed sources that
    /// are not currently enabled may also be included.
    ///
    /// # Errors
    ///
    /// Returns [`TisError`] if the underlying TIS API call fails.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use text_input_source::TisManager;
    ///
    /// let manager = TisManager::new();
    /// let sources = manager.list_keyboard_input_sources(false)?;
    ///
    /// for source in sources {
    ///     println!("{:?}", source.id()?);
    /// }
    /// # Ok::<(), text_input_source::TisError>(())
    /// ```
    pub fn list_keyboard_input_sources(
        &self,
        include_all_installed: bool,
    ) -> Result<Vec<InputSource>> {
        let category_key = tis_constant_string(PropertyKind::InputSourceCategory.key());
        let keyboard_category = InputSourceCategoryValue::KeyboardInputSource.as_cf_string();
        let filter = CFDictionary::from_CFType_pairs(&[(category_key, keyboard_category)]);
        list_input_sources_impl(Some(&filter), include_all_installed)
    }

    /// Returns input sources matching the given bundle identifier.
    ///
    /// When `include_all_installed` is `false`, the returned list is limited to
    /// sources that are currently enabled. When `true`, installed sources that
    /// are not currently enabled may also be included.
    ///
    /// # Errors
    ///
    /// Returns [`TisError`] if the underlying TIS API call fails.
    pub fn list_input_sources_with_bundle_id(
        &self,
        bundle_id: &str,
        include_all_installed: bool,
    ) -> Result<Vec<InputSource>> {
        let bundle_id_key = tis_constant_string(PropertyKind::BundleId.key());
        let bundle_id_value = CFString::new(bundle_id);
        let filter = CFDictionary::from_CFType_pairs(&[(bundle_id_key, bundle_id_value)]);
        list_input_sources_impl(Some(&filter), include_all_installed)
    }

    /// Returns the current keyboard input source.
    ///
    /// # Errors
    ///
    /// Returns [`TisError`] if the underlying TIS API call returns NULL.
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
    pub fn current_keyboard_input_source(&self) -> Result<InputSource> {
        with_tis_lock(|| {
            let raw = copy_current_keyboard_input_source()?;
            Ok(unsafe { InputSource::from_create_rule(raw) })
        })
    }
}

fn copy_current_keyboard_input_source() -> Result<ffi::TISInputSourceRef> {
    let raw = unsafe { ffi::TISCopyCurrentKeyboardInputSource() };
    if raw.is_null() {
        Err(TisError::NullResult(
            OperationKind::CopyCurrentKeyboardInputSource,
        ))
    } else {
        Ok(raw)
    }
}

fn list_input_sources_impl(
    filter: Option<&CFDictionary<CFString, CFString>>,
    include_all_installed: bool,
) -> Result<Vec<InputSource>> {
    with_tis_lock(|| {
        let filter_ref = filter.map_or(std::ptr::null(), TCFType::as_concrete_TypeRef);

        let array_ref = create_input_source_list(filter_ref, u8::from(include_all_installed))?;

        let result = input_sources_from_array(array_ref);
        unsafe {
            CFRelease(array_ref as CFTypeRef);
        }
        Ok(result)
    })
}

fn create_input_source_list(
    properties: CFDictionaryRef,
    include_all_installed: Boolean,
) -> Result<CFArrayRef> {
    let array_ref = unsafe { ffi::TISCreateInputSourceList(properties, include_all_installed) };
    if array_ref.is_null() {
        Err(TisError::NullResult(OperationKind::CreateInputSourceList))
    } else {
        Ok(array_ref)
    }
}

fn input_sources_from_array(array_ref: CFArrayRef) -> Vec<InputSource> {
    let len = unsafe { CFArrayGetCount(array_ref) };
    #[allow(
        clippy::cast_sign_loss,
        reason = "CFArrayGetCount always returns a non-negative value"
    )]
    let mut result = Vec::with_capacity(len as usize);

    for index in 0..len {
        let value = unsafe { CFArrayGetValueAtIndex(array_ref, index) };
        if !value.is_null() {
            result.push(unsafe { InputSource::from_get_rule(value.cast_mut()) });
        }
    }

    result
}
