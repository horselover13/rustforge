// Session state: passives, packages, the co-op session, game state, the mod
// list, notices, and how this mod was loaded.

#![allow(dead_code)]

use super::HellForgeApi;
use crate::rust_ffi::{HfGenerationCapacity, HfModInfo, HfNotice, HfPassives, HfSession};

impl HellForgeApi {
    /// Overrides the local player's armor and second passives (solo only).
    ///
    /// Returns `true` when the override is held (or released), `false` for an
    /// id the game has no passive for.
    pub fn set_passives(&self, armor: u32, second: u32) -> bool {
        if !self.supports(6) {
            return false;
        }

        let Some(set_passives) = self.raw_api().set_passives else {
            return false;
        };

        unsafe { set_passives(self.mod_handle(), armor, second) != 0 }
    }

    /// The local player's passives now.
    ///
    /// Returns `None` when the record is not readable.
    pub fn get_passives(&self) -> Option<HfPassives> {
        if !self.supports(6) {
            return None;
        }

        let get_passives = self.raw_api().get_passives?;
        let mut out: HfPassives = unsafe { std::mem::zeroed() };

        (unsafe { get_passives(self.mod_handle(), &mut out) } != 0).then_some(out)
    }

    /// Loads a package by its resource hash and holds it for the session.
    ///
    /// Returns `HF_PACKAGE_RESIDENT` once the engine has all of it,
    /// `HF_PACKAGE_LOADING` until then (call again to check), or 0 when refused
    /// or failed.
    pub fn load_package(&self, package: u64) -> u32 {
        if !self.supports(6) {
            return 0;
        }

        let Some(load_package) = self.raw_api().load_package else {
            return 0;
        };

        unsafe { load_package(self.mod_handle(), package) as u32 }
    }

    /// The session's co-op role and host name.
    ///
    /// Returns `None` only when the loader does not expose `session`.
    pub fn session(&self) -> Option<HfSession> {
        if !self.supports(6) {
            return None;
        }

        let session = self.raw_api().session?;
        let mut out: HfSession = unsafe { std::mem::zeroed() };
        unsafe { session(self.mod_handle(), &mut out) };

        Some(out)
    }

    /// The game's state now (`HF_STATE_*`), or `HF_STATE_NONE`.
    pub fn game_state(&self) -> u32 {
        if !self.supports(7) {
            return 0;
        }

        let Some(game_state) = self.raw_api().game_state else {
            return 0;
        };

        unsafe { game_state(self.mod_handle()) }
    }

    /// The `index`-th mod of the session, in load order.
    ///
    /// Returns `None` past the last mod, or when the loader is too old.
    pub fn mods(&self, index: u32) -> Option<HfModInfo> {
        if !self.supports(10) {
            return None;
        }

        let mods = self.raw_api().mods?;
        let mut out: HfModInfo = unsafe { std::mem::zeroed() };
        out.size = std::mem::size_of::<HfModInfo>() as u32;

        (unsafe { mods(self.mod_handle(), index, &mut out) } != 0).then_some(out)
    }

    /// HellForge's latest message for the player.
    ///
    /// `version` is 0 before the first notice. Returns `None` only when the
    /// loader does not expose `notice`.
    pub fn notice(&self) -> Option<HfNotice> {
        if !self.supports(12) {
            return None;
        }

        let notice = self.raw_api().notice?;
        let mut out: HfNotice = unsafe { std::mem::zeroed() };
        unsafe { notice(self.mod_handle(), &mut out) };

        Some(out)
    }

    /// How this mod was set up: `HF_LOAD_BOOT` or `HF_LOAD_LIVE`.
    ///
    /// Older loaders report `HF_LOAD_BOOT`.
    pub fn load_phase(&self) -> u32 {
        if !self.supports(16) {
            return 0;
        }

        let Some(load_phase) = self.raw_api().load_phase else {
            return 0;
        };

        unsafe { load_phase(self.mod_handle()) }
    }

    /// Asks the map generator for room in its arrays, or reads the planned
    /// capacities.
    ///
    /// `want` asks for at least those capacities; `None` only reads. `have`
    /// is filled with the capacities planned and where each region lies in
    /// the block. Returns `true` on success, `false` when refused (too late,
    /// or a field over its limit) or when the loader is too old.
    pub fn generation_capacity(
        &self,
        want: Option<&HfGenerationCapacity>,
        have: Option<&mut HfGenerationCapacity>,
    ) -> bool {
        if !self.supports(17) {
            return false;
        }

        let Some(generation_capacity) = self.raw_api().generation_capacity else {
            return false;
        };

        let want = want.map_or(std::ptr::null(), |want| want as *const HfGenerationCapacity);
        let have = match have {
            Some(have) => {
                have.size = std::mem::size_of::<HfGenerationCapacity>() as u32;
                have as *mut HfGenerationCapacity
            }
            None => std::ptr::null_mut(),
        };

        unsafe { generation_capacity(self.mod_handle(), want, have) == 0 }
    }
}
