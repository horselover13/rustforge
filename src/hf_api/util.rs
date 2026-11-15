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

/// A Wwise event's id from its name (FNV-1 of the lower-case name).
///
/// This is the C SDK's `hf_wwise_id`: an event of the mod's data spec
/// "sounds", for the record fields that name sound events.
pub fn wwise_id(name: &str) -> u32 {
    let mut hash = 2_166_136_261u32;

    for byte in name.bytes() {
        if byte == 0 {
            break;
        }

        hash = hash.wrapping_mul(16_777_619) ^ u32::from(byte.to_ascii_lowercase());
    }

    hash
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

#[cfg(test)]
mod tests {
    #[test]
    fn wwise_id_is_fnv_1_of_the_lower_case_name() {
        assert_eq!(super::wwise_id("yourname_rifle_fire"), 0xEDBF_9865);
        assert_eq!(super::wwise_id("ABC"), super::wwise_id("abc"));
        assert_eq!(super::wwise_id(""), 2_166_136_261);
    }
}
