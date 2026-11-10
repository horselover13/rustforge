// Shared conversions for the C <-> Rust boundary.

#![allow(dead_code)]

use std::ffi::{CStr, CString};
use std::os::raw::c_char;

/// Converts a Rust string into a NUL-terminated C string.
///
/// Returns `None` when the string contains an interior NUL byte, which the C
/// API could not represent.
pub(crate) fn cstring(value: &str) -> Option<CString> {
    CString::new(value).ok()
}

/// Copies a C string the loader owns. Returns `None` for a null pointer.
///
/// # Safety
/// `ptr` must be null or point to a valid, NUL-terminated string.
pub(crate) unsafe fn string_from_ptr(ptr: *const c_char) -> Option<String> {
    if ptr.is_null() {
        None
    } else {
        Some(
            unsafe { CStr::from_ptr(ptr) }
                .to_string_lossy()
                .into_owned(),
        )
    }
}

/// Like [`string_from_ptr`], but a null pointer becomes `""`.
///
/// # Safety
/// `ptr` must be null or point to a valid, NUL-terminated string.
pub(crate) unsafe fn string_from_ptr_or_empty(ptr: *const c_char) -> String {
    unsafe { string_from_ptr(ptr) }.unwrap_or_default()
}

/// Reads a fixed-size C character array (for example `HfSession::host`).
pub(crate) fn string_from_array<const N: usize>(array: &[c_char; N]) -> String {
    let bytes: Vec<u8> = array
        .iter()
        .map(|value| *value as u8)
        .take_while(|value| *value != 0)
        .collect();

    String::from_utf8_lossy(&bytes).into_owned()
}
