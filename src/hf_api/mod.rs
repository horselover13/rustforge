#![allow(dead_code)]

use crate::rust_ffi::{HfApi, HfMod};
use std::os::raw::c_int;
use std::panic::{AssertUnwindSafe, PanicHookInfo, catch_unwind};
use std::sync::OnceLock;

pub mod actions;
pub mod callback;
pub mod content;
pub mod entity;
pub mod hook;
pub mod listeners;
pub mod log;
pub mod mem;
pub mod multiplayer;
pub mod network;
pub mod session;
pub mod stats;
pub mod status;
pub(crate) mod util;

pub static HF_API: OnceLock<HellForgeApi> = OnceLock::new();

/// The function shape `#[hf_entry]` registers.
pub type HfEntryFn = fn(&'static HellForgeApi);

static HF_ENTRY: OnceLock<HfEntryFn> = OnceLock::new();

/// Stores the mod's entry point. `#[hf_entry]`-generated initializers call
/// this while the DLL is loading, before the loader calls
/// [`hellforge_mod_init`].
#[doc(hidden)]
pub fn __register_entry(entry: HfEntryFn) {
    let _ = HF_ENTRY.set(entry);
}

fn setup_panic_hook() {
    std::panic::set_hook(Box::new(|panic_info: &PanicHookInfo<'_>| {
        let reason = match panic_info.payload().downcast_ref::<&str>() {
            Some(reason) => *reason,
            None => panic_info
                .payload()
                .downcast_ref::<String>()
                .map(String::as_str)
                .unwrap_or("unknown panic payload"),
        };

        let location = panic_info
            .location()
            .map(|l| format!("{}:{}:{}", l.file(), l.line(), l.column()))
            .unwrap_or_else(|| "unknown location".to_string());

        let message = format!("rustforge panicked at {location}: {reason}");

        // Logging is unavailable before `hellforge_mod_init` installs the API.
        match HF_API.get() {
            Some(api) => api.log(&message),
            None => eprintln!("{message}"),
        }
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
    size: u32,        // sizeof(HfApi) of the loader, for the same reason
    hf_mod: HfModPtr, // passed down to every C function
    c_api: HfApiPtr,
}

impl HellForgeApi {
    fn new(api_ptr: *const HfApi, mod_ptr: *mut HfMod) -> Self {
        unsafe {
            let api_ref = &*api_ptr;
            HellForgeApi {
                version: api_ref.version,
                size: api_ref.size,
                hf_mod: HfModPtr::new(mod_ptr),
                c_api: HfApiPtr::new(api_ptr),
            }
        }
    }

    /// The SDK version the loader was built against.
    pub fn version(&self) -> u32 {
        self.version
    }

    /// The loader's `sizeof(HfApi)`, which grows as functions are added.
    pub fn api_size(&self) -> u32 {
        self.size
    }

    /// Whether the loader is new enough for a function introduced in `sdk`.
    pub(crate) fn supports(&self, sdk: u32) -> bool {
        self.version >= sdk
    }

    /// The raw C API. Only call this on a version that still has the fields you use.
    pub(crate) fn raw_api(&self) -> &HfApi {
        unsafe { &*self.c_api.ptr() }
    }

    /// The opaque `HfMod*` every API call expects.
    pub(crate) fn mod_handle(&self) -> *mut HfMod {
        self.hf_mod.ptr() as *mut HfMod
    }
}

#[unsafe(no_mangle)]
extern "C" fn hellforge_mod_init(api: *const HfApi, mod_ptr: *mut HfMod) -> c_int {
    // Live mods may run set-up more than once for the same DLL. Install the
    // API and panic hook once; later calls reuse them and still invoke the
    // entry point.
    if HF_API.get().is_none() && HF_API.set(HellForgeApi::new(api, mod_ptr)).is_ok() {
        setup_panic_hook();
    }

    let Some(api) = HF_API.get() else {
        return 1;
    };

    run_entry(api)
}

fn run_entry(api: &'static HellForgeApi) -> c_int {
    let Some(entry) = HF_ENTRY.get().copied() else {
        // No `#[hf_entry]` function: the mod loads but has nothing to set up.
        api.log("rustforge: no #[hf_entry] function was registered");
        return 0;
    };

    match catch_unwind(AssertUnwindSafe(|| entry(api))) {
        Ok(()) => 0,
        // The panic hook has already logged the reason. Refuse the mod so the
        // loader does not keep a half-initialized set-up.
        Err(_) => 1,
    }
}

#[cfg(test)]
mod tests {
    use super::{HF_ENTRY, HellForgeApi, run_entry};
    use crate::rust_ffi::HfApi;
    use std::sync::atomic::{AtomicBool, Ordering};

    static CALLED: AtomicBool = AtomicBool::new(false);

    #[crate::hf_entry]
    fn test_entry(api: &HellForgeApi) {
        assert!(api.version() >= 1);
        CALLED.store(true, Ordering::SeqCst);
    }

    #[test]
    fn registered_entry_runs() {
        let mut raw: Box<HfApi> = Box::new(unsafe { std::mem::zeroed() });
        raw.version = 16;
        raw.size = std::mem::size_of::<HfApi>() as u32;
        let raw = Box::leak(raw);
        let api = Box::leak(Box::new(HellForgeApi::new(raw, std::ptr::null_mut())));

        assert!(HF_ENTRY.get().is_some());
        assert_eq!(run_entry(api), 0);
        assert!(CALLED.load(Ordering::SeqCst));
    }
}
