use core::ffi::c_void;

use core_foundation::string::CFStringRef;
use core_foundation_sys::array::CFArrayRef;
use core_foundation_sys::base::{Boolean, OSStatus};
use core_foundation_sys::dictionary::CFDictionaryRef;

pub type TISInputSourceRef = *mut c_void;

#[link(name = "Carbon", kind = "framework")]
unsafe extern "C" {
    pub fn TISCreateInputSourceList(
        properties: CFDictionaryRef,
        include_all_installed: Boolean,
    ) -> CFArrayRef;
    pub fn TISCopyCurrentKeyboardInputSource() -> TISInputSourceRef;
    pub fn TISGetInputSourceProperty(
        input_source: TISInputSourceRef,
        property_key: CFStringRef,
    ) -> *const c_void;
    pub fn TISSelectInputSource(input_source: TISInputSourceRef) -> OSStatus;
    pub fn TISEnableInputSource(input_source: TISInputSourceRef) -> OSStatus;
    pub fn TISDisableInputSource(input_source: TISInputSourceRef) -> OSStatus;

    pub static kTISPropertyInputSourceCategory: CFStringRef;
    pub static kTISPropertyInputSourceIsEnabled: CFStringRef;
    pub static kTISPropertyInputSourceID: CFStringRef;
    pub static kTISPropertyLocalizedName: CFStringRef;
    pub static kTISCategoryKeyboardInputSource: CFStringRef;
}
