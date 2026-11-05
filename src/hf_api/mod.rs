use crate::rust_ffi::{HfApi, HfMod};
use std::sync::OnceLock;

pub mod log;
pub mod mem;

pub static HF_API: OnceLock<HellForgeApi> = OnceLock::new();

// is passed down to LITERALLY every function of HfApi, just a wrapper for the raw pointer to HfMod
#[derive(Debug)]
pub struct HfModPtr(*mut HfMod);

unsafe impl Send for HfModPtr {}

unsafe impl Sync for HfModPtr {}

impl HfModPtr {
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
}
