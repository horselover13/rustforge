// Logging and build identification functions for HellForgeApi

#![allow(dead_code)]

use super::HellForgeApi;
use crate::rustforge::rust_ffi::hellforge_mod::HfMod;

impl HellForgeApi {
    pub fn log(&self, msg: &str) {
        unsafe {
            let api = &*self.c_api.0;
            if let Some(log_fn) = api.log.as_ref() {
                // Call the C log function through FFI - msg string as raw pointer to const i8
                (log_fn)(self.hf_mod.ptr() as *mut HfMod, msg.as_ptr() as *const i8);
            } else {
                eprintln!("[HellForge] {}", msg);
            }
        }
    }

    pub fn log_fmt(&self, msg: &str) {
        unsafe {
            let api = &*self.c_api.0;
            if let Some(log_fn) = api.log.as_ref() {
                (log_fn)(self.hf_mod.ptr() as *mut HfMod, msg.as_ptr() as *const i8);
            } else {
                eprintln!("[HellForge] {}", msg);
            }
        }
    }

    pub fn get_build(&self) -> String {
        unsafe {
            let c_api = &*self.c_api.0;

            if let Some(build_fn) = c_api.build_id.as_ref() {
                let c_build_id = build_fn();
                use std::ffi::CStr;
                if c_build_id.is_null() {
                    String::new()
                } else {
                    CStr::from_ptr(c_build_id).to_string_lossy().to_string()
                }
            } else {
                String::new()
            }
        }
    }
}
