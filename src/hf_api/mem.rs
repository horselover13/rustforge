#![allow(dead_code)]

use super::HellForgeApi;
use crate::rust_ffi::HfMod;
use std::ffi::c_void;

impl HellForgeApi {
    // pub fn register_event_callback(&self, _event: u32) {
    //     unsafe {
    //         let hf_mod = self.hf_mod.ptr();
    //         let c_api = &*self.c_api.0;
    //
    //         if let Some(_on_fn) = c_api.on.as_ref() {
    //             (*_on_fn)(hf_mod, _event as u32, );
    //         }
    //     }
    // }

    pub fn module_base(&self, module: &str) -> usize {
        unsafe {
            let c_api = &*self.c_api.0;

            if let Some(mod_fn) = c_api.module_base.as_ref() {
                (*mod_fn)(
                    module.as_ptr() as *const ::std::os::raw::c_char,
                    std::ptr::null_mut(),
                );
            }
        }

        0
    }

    pub fn read(&self, address: usize, buffer: &mut [u8]) -> bool {
        unsafe {
            let c_api = &*self.c_api.0;
            let mut_ptr = buffer.as_mut_ptr() as *mut c_void;

            if let Some(read_fn) = c_api.read.as_ref() {
                (*read_fn)(address, mut_ptr, buffer.len());
            }
        }

        true
    }

    pub fn write(&self, address: usize, data: &[u8]) -> bool {
        unsafe {
            let c_api = &*self.c_api.0;
            let data_ptr = data.as_ptr() as *mut c_void;

            if let Some(write_fn) = c_api.write.as_ref() {
                (*write_fn)(address, data_ptr, data.len());
            }
        }

        true
    }

    pub fn write_u32_if(&self, address: usize, _expect: u32, _value: u32) {
        unsafe {
            let c_api = &*self.c_api.0;
            let hf_mod = self.hf_mod.ptr() as *mut HfMod;

            if let Some(write_fn) = c_api.write_u32_if.as_ref() {
                (*write_fn)(hf_mod, address, _expect, _value);
            }
        }
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
}
