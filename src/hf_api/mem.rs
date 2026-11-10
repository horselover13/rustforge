#![allow(dead_code)]

use super::{HellForgeApi, util::cstring};
use std::ffi::c_void;

impl HellForgeApi {
    /// A loaded module's base address, or 0 when it is not loaded.
    pub fn module_base(&self, module: &str) -> usize {
        self.module_base_with_size(module)
            .map_or(0, |(base, _)| base)
    }

    /// A loaded module's base address and image size, or `None`.
    pub fn module_base_with_size(&self, module: &str) -> Option<(usize, usize)> {
        let module = cstring(module)?;
        let module_base = self.raw_api().module_base?;

        let mut size = 0usize;
        let base = unsafe { module_base(module.as_ptr(), &mut size) };

        (base != 0).then_some((base, size))
    }

    /// Reads `buffer.len()` bytes from `address`.
    ///
    /// Returns `true` on success.
    pub fn read(&self, address: usize, buffer: &mut [u8]) -> bool {
        let Some(read) = self.raw_api().read else {
            return false;
        };
        let buffer_ptr = buffer.as_mut_ptr() as *mut c_void;

        unsafe { read(address, buffer_ptr, buffer.len()) == 0 }
    }

    /// Writes `data` at `address`, making the memory writable for the write.
    ///
    /// Returns `true` on success.
    pub fn write(&self, address: usize, data: &[u8]) -> bool {
        let Some(write) = self.raw_api().write else {
            return false;
        };
        let data_ptr = data.as_ptr() as *const c_void;

        unsafe { write(address, data_ptr, data.len()) == 0 }
    }

    /// Writes `value` only if the u32 at `address` holds `expect` (or already
    /// `value`).
    ///
    /// Returns `true` when it was written or already set, `false` when it was
    /// left alone.
    pub fn write_u32_if(&self, address: usize, expect: u32, value: u32) -> bool {
        let Some(write_u32_if) = self.raw_api().write_u32_if else {
            return false;
        };

        unsafe { write_u32_if(self.mod_handle(), address, expect, value) == 0 }
    }

    pub fn read_u32(&self, addr: usize) -> Option<u32> {
        let mut buffer = [0u8; 4];
        if self.read(addr, &mut buffer) {
            return Some(u32::from_ne_bytes(buffer));
        }
        None
    }

    /// Reads a u64 value from memory address.
    pub fn read_u64(&self, addr: usize) -> Option<u64> {
        let mut buffer = [0u8; 8];
        if self.read(addr, &mut buffer) {
            return Some(u64::from_ne_bytes(buffer));
        }
        None
    }

    /// Finds the first match of an IDA-style byte pattern in a loaded module.
    ///
    /// The pattern reads the code as it is now, so a function another mod
    /// hooked starts with a jump. Use [`code_matches`](Self::code_matches) to
    /// check a function before hooking it.
    pub fn find_pattern(&self, module: &str, pattern: &str) -> usize {
        let Some(module) = cstring(module) else {
            return 0;
        };
        let Some(pattern) = cstring(pattern) else {
            return 0;
        };
        let Some(find_pattern) = self.raw_api().find_pattern else {
            return 0;
        };

        unsafe { find_pattern(module.as_ptr(), pattern.as_ptr()) }
    }

    /// Whether the bytes at `address` matched `pattern` before any hook
    /// HellForge placed.
    ///
    /// This is the check to make before hooking a function: another mod's hook
    /// does not fail it, where `find_pattern` and `read` would see the jump.
    pub fn code_matches(&self, address: usize, pattern: &str) -> bool {
        if !self.supports(13) {
            return false;
        }

        let Some(pattern) = cstring(pattern) else {
            return false;
        };
        let Some(code_matches) = self.raw_api().code_matches else {
            return false;
        };

        unsafe { code_matches(address, pattern.as_ptr()) != 0 }
    }

    /// Reads memory the way [`code_matches`](Self::code_matches) sees it: with
    /// any hook HellForge placed seen through.
    ///
    /// Returns `true` on success.
    pub fn read_code(&self, address: usize, buffer: &mut [u8]) -> bool {
        if !self.supports(13) {
            return false;
        }

        let Some(read_code) = self.raw_api().read_code else {
            return false;
        };

        unsafe { read_code(address, buffer.as_mut_ptr() as *mut c_void, buffer.len()) == 0 }
    }
}
