// Function hooking and named game symbols.

#![allow(dead_code)]

use super::{HellForgeApi, util::cstring};
use std::ffi::c_void;

impl HellForgeApi {
    /// Detours the function at `target` to `detour`.
    ///
    /// `original` receives a pointer that calls the next hook in the chain (or
    /// the function's own code). Hooks stay for the session. This is
    /// [`hook_ex`](Self::hook_ex) with priority 0.
    ///
    /// # Safety
    /// `detour` must be a valid function pointer with the target's calling
    /// convention and signature. `original`, when non-null, must point to
    /// storage the loader may write a function pointer into. Hooking the wrong
    /// address, or using a mismatched signature, is undefined behaviour.
    pub unsafe fn hook(
        &self,
        target: usize,
        detour: *mut c_void,
        original: *mut *mut c_void,
    ) -> bool {
        let Some(hook) = self.raw_api().hook else {
            return false;
        };

        unsafe { hook(self.mod_handle(), target, detour, original) == 0 }
    }

    /// Like [`hook`](Self::hook), with a chain priority.
    ///
    /// Mods' hooks run first, highest priority first; HellForge's own hooks run
    /// last, nearest the game's code.
    ///
    /// # Safety
    /// See [`hook`](Self::hook).
    pub unsafe fn hook_ex(
        &self,
        target: usize,
        detour: *mut c_void,
        original: *mut *mut c_void,
        priority: i32,
    ) -> bool {
        if !self.supports(13) {
            return false;
        }

        let Some(hook_ex) = self.raw_api().hook_ex else {
            return false;
        };

        unsafe { hook_ex(self.mod_handle(), target, detour, original, priority) == 0 }
    }

    /// Removes this mod's hook of `target` from its chain.
    ///
    /// Returns `true` when a hook was removed, `false` when this mod had no hook
    /// there (or the loader is too old).
    pub fn unhook(&self, target: usize) -> bool {
        if !self.supports(16) {
            return false;
        }

        let Some(unhook) = self.raw_api().unhook else {
            return false;
        };

        unsafe { unhook(self.mod_handle(), target) == 0 }
    }

    /// The address of a game function named in `mappings/functions.json`.
    ///
    /// Returns 0 when the name is not mapped, the game build is not the
    /// reviewed one, the code is not loaded yet, or the code differs.
    pub fn function(&self, name: &str) -> usize {
        if !self.supports(16) {
            return 0;
        }

        let Some(name) = cstring(name) else {
            return 0;
        };
        let Some(function) = self.raw_api().function else {
            return 0;
        };

        unsafe { function(self.mod_handle(), name.as_ptr()) }
    }

    /// Hooks a game function by its mapped name.
    ///
    /// Returns `true` when the hook is in, or will be installed once the game's
    /// code is decoded and verified.
    ///
    /// # Safety
    /// See [`hook`](Self::hook).
    pub unsafe fn hook_function(
        &self,
        name: &str,
        detour: *mut c_void,
        original: *mut *mut c_void,
        priority: i32,
    ) -> bool {
        if !self.supports(16) {
            return false;
        }

        let Some(name) = cstring(name) else {
            return false;
        };
        let Some(hook_function) = self.raw_api().hook_function else {
            return false;
        };

        unsafe { hook_function(self.mod_handle(), name.as_ptr(), detour, original, priority) == 0 }
    }

    /// The address of a game global's cell, named in `mappings/globals.json`.
    ///
    /// The cell usually holds a pointer to the game object. Returns 0 when the
    /// name is not mapped, the build is not the reviewed one, the code is not
    /// loaded yet, or its witness does not match.
    pub fn global(&self, name: &str) -> usize {
        if !self.supports(16) {
            return 0;
        }

        let Some(name) = cstring(name) else {
            return 0;
        };
        let Some(global) = self.raw_api().global else {
            return 0;
        };

        unsafe { global(self.mod_handle(), name.as_ptr()) }
    }
}
