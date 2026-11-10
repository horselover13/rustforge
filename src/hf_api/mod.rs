#![allow(dead_code)]

use crate::rust_ffi::{HfApi, HfMod};
use std::panic::PanicHookInfo;
use std::sync::OnceLock;

pub mod callback;
pub mod log;
pub mod mem;

pub static HF_API: OnceLock<HellForgeApi> = OnceLock::new();

// alias for HF_API.get().unwrap()
macro_rules! hf_api {
    () => {
        *crate::hf_api::HF_API.get().unwrap()
    };
}

// rust-analyzer false positive, please ignore
fn setup_panic_hook() {
    std::panic::set_hook(Box::new(move |panic_info: &PanicHookInfo<'_>| {
        let msg = match panic_info.payload().downcast_ref::<&str>() {
            Some(s) => *s,
            None => panic_info
                .payload()
                .downcast_ref::<String>()
                .map(String::as_str)
                .unwrap_or("Unknown panic payload"),
        };

        let location = panic_info
            .location()
            .map(|l| format!("{}:{}:{}", l.file(), l.line(), l.column()))
            .unwrap_or_else(|| "Unknown location".to_string());

        hf_api!().log(format!("Rustforge panicked at {}! {}\n", location, msg).as_str());
    }));
}

// is passed down to LITERALLY every function of HfApi, just a wrapper for the raw pointer to HfMod
#[derive(Debug)]
pub struct HfModPtr(*mut HfMod);

unsafe impl Send for HfModPtr {}

unsafe impl Sync for HfModPtr {}

impl HfModPtr {
    /// # Safety
    /// That pointer probably should exist
    pub unsafe fn new(ptr: *mut HfMod) -> HfModPtr {
        Self(ptr)
    }

    pub fn ptr(&self) -> *const HfMod {
        self.0 as *const _
    }
}

#[derive(Debug)]
pub struct HfApiPtr(*const HfApi);

unsafe impl Send for HfApiPtr {}

unsafe impl Sync for HfApiPtr {}

impl HfApiPtr {
    /// # Safety
    /// That pointer also, preferrably, should exist
    pub unsafe fn new(ptr: *const HfApi) -> HfApiPtr {
        Self(ptr)
    }

    pub fn ptr(&self) -> *const HfApi {
        self.0 as *const _
    }
}

#[derive(Debug)]
pub struct HellForgeApi {
    version: u32,     // API version, specifically u32, directly taken from C bindgen'd value
    hf_mod: HfModPtr, // passed down to every C function
    c_api: HfApiPtr,
}

impl HellForgeApi {
    fn new(api_ptr: *const HfApi, mod_ptr: *mut HfMod) -> Self {
        unsafe {
            let api_ref = &*api_ptr;
            HellForgeApi {
                version: api_ref.version,
                hf_mod: HfModPtr::new(mod_ptr),
                c_api: HfApiPtr::new(api_ptr),
            }
        }
    }
}

#[unsafe(no_mangle)]
extern "C" fn hellforge_mod_init(api: *const HfApi, mod_ptr: *mut HfMod) {
    let hf_api = HellForgeApi::new(api, mod_ptr);
    HF_API.set(hf_api).unwrap();
    setup_panic_hook();
}
