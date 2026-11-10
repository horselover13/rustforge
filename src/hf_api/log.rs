// Logging, build identification, and the mod's own identity and log paths.

#![allow(dead_code)]

use super::HellForgeApi;
use super::util::string_from_ptr_or_empty;
use crate::rust_ffi::HfMod;
use std::ffi::CString;

impl HellForgeApi {
    pub fn log(&self, msg: &str) {
        self.log_line(msg);
    }

    pub fn log_fmt(&self, msg: &str) {
        self.log_line(msg);
    }

    fn log_line(&self, msg: &str) {
        // `log` is a printf-style function: pass the message as data so a `%`
        // in it is not read as a format directive, and keep it NUL-terminated.
        let Ok(c_msg) = CString::new(msg) else {
            eprintln!("[HellForge] {msg}");
            return;
        };

        unsafe {
            let api = &*self.c_api.0;
            if let Some(log_fn) = api.log.as_ref() {
                (log_fn)(
                    self.hf_mod.ptr() as *mut HfMod,
                    c"%s".as_ptr(),
                    c_msg.as_ptr(),
                );
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

    /// The mod's own id (UTF-8), or `""` when it is not known.
    pub fn mod_id(&self) -> String {
        let Some(mod_id) = self.raw_api().mod_id else {
            return String::new();
        };

        unsafe { string_from_ptr_or_empty(mod_id(self.mod_handle())) }
    }

    /// The mod's own folder (UTF-8), or `""` when it is not known.
    pub fn mod_folder(&self) -> String {
        let Some(mod_folder) = self.raw_api().mod_folder else {
            return String::new();
        };

        unsafe { string_from_ptr_or_empty(mod_folder(self.mod_handle())) }
    }

    /// The mod's last log line, which often says why the call that just
    /// returned 0 was refused.
    pub fn last_log(&self) -> String {
        if !self.supports(6) {
            return String::new();
        }

        let Some(last_log) = self.raw_api().last_log else {
            return String::new();
        };

        unsafe { string_from_ptr_or_empty(last_log(self.mod_handle())) }
    }

    /// The session's logs folder (UTF-8: `modloader.log` and every
    /// `mod-<id>.log`), or `""` when it is not known.
    pub fn logs_folder(&self) -> String {
        if !self.supports(6) {
            return String::new();
        }

        let Some(logs_folder) = self.raw_api().logs_folder else {
            return String::new();
        };

        unsafe { string_from_ptr_or_empty(logs_folder(self.mod_handle())) }
    }
}
