// Multiplayer: mod sync, the lobby browser, and the mod message/shared table
// channel.

#![allow(dead_code)]

use super::{HellForgeApi, util::cstring};
use crate::rust_ffi::{HfBrowseLobby, HfBrowseState, HfSyncMod, HfSyncState};
use std::ffi::c_void;

const SHARE_VALUE_MAX: usize = 64;
const MESSAGE_MAX: usize = 1024;

impl HellForgeApi {
    /// The current mod-sync offer.
    ///
    /// Returns `None` when there is no offer (solo, or no backend connected).
    pub fn sync_state(&self) -> Option<HfSyncState> {
        if !self.supports(14) {
            return None;
        }

        let sync_state = self.raw_api().sync_state?;
        let mut out: HfSyncState = unsafe { std::mem::zeroed() };
        out.size = std::mem::size_of::<HfSyncState>() as u32;

        (unsafe { sync_state(self.mod_handle(), &mut out) } != 0).then_some(out)
    }

    /// The `index`-th mod of the current sync offer.
    ///
    /// Returns `None` past the last mod.
    pub fn sync_mod(&self, index: u32) -> Option<HfSyncMod> {
        if !self.supports(14) {
            return None;
        }

        let sync_mod = self.raw_api().sync_mod?;
        let mut out: HfSyncMod = unsafe { std::mem::zeroed() };
        out.size = std::mem::size_of::<HfSyncMod>() as u32;

        (unsafe { sync_mod(self.mod_handle(), index, &mut out) } != 0).then_some(out)
    }

    /// Sends the player's answer to a sync offer.
    ///
    /// `mod_id` names the mod for `HF_SYNC_BLOCK`, `HF_SYNC_TRUST` and
    /// `HF_SYNC_OPEN`; pass `None` for the other answers.
    ///
    /// Returns `true` when the answer was sent.
    pub fn sync_answer(&self, version: u32, answer: u32, mod_id: Option<&str>) -> bool {
        if !self.supports(14) {
            return false;
        }

        let Some(sync_answer) = self.raw_api().sync_answer else {
            return false;
        };

        let mod_id = match mod_id {
            Some(mod_id) => match cstring(mod_id) {
                Some(mod_id) => Some(mod_id),
                None => return false,
            },
            None => None,
        };
        let mod_id = mod_id
            .as_ref()
            .map_or(std::ptr::null(), |mod_id| mod_id.as_ptr());

        unsafe { sync_answer(self.mod_handle(), version, answer, mod_id) != 0 }
    }

    /// The lobby browser's state.
    ///
    /// Returns `None` only when the loader does not expose the browser. With
    /// no backend connected the state is still filled (`HF_BROWSE_NONE`).
    pub fn browse_state(&self) -> Option<HfBrowseState> {
        if !self.supports(16) {
            return None;
        }

        let browse_state = self.raw_api().browse_state?;
        let mut out: HfBrowseState = unsafe { std::mem::zeroed() };
        out.size = std::mem::size_of::<HfBrowseState>() as u32;

        unsafe { browse_state(self.mod_handle(), &mut out) };
        Some(out)
    }

    /// The `index`-th lobby of the browser's list.
    ///
    /// Returns `None` past the last lobby.
    pub fn browse_lobby(&self, index: u32) -> Option<HfBrowseLobby> {
        if !self.supports(16) {
            return None;
        }

        let browse_lobby = self.raw_api().browse_lobby?;
        let mut out: HfBrowseLobby = unsafe { std::mem::zeroed() };
        out.size = std::mem::size_of::<HfBrowseLobby>() as u32;

        (unsafe { browse_lobby(self.mod_handle(), index, &mut out) } != 0).then_some(out)
    }

    /// Asks the backend for a new list, or to join a lobby.
    ///
    /// `lobby_id` is used by `HF_BROWSE_JOIN` and is `None` for the other asks.
    ///
    /// Returns `true` when the ask was sent.
    pub fn browse_ask(&self, ask: u32, lobby_id: Option<&str>) -> bool {
        if !self.supports(16) {
            return false;
        }

        let Some(browse_ask) = self.raw_api().browse_ask else {
            return false;
        };

        let lobby_id = match lobby_id {
            Some(lobby_id) => match cstring(lobby_id) {
                Some(lobby_id) => Some(lobby_id),
                None => return false,
            },
            None => None,
        };
        let lobby_id = lobby_id
            .as_ref()
            .map_or(std::ptr::null(), |lobby_id| lobby_id.as_ptr());

        unsafe { browse_ask(self.mod_handle(), ask, lobby_id) != 0 }
    }

    /// Sends a message to the same mod on other machines of the lobby.
    ///
    /// `to` is `HF_TO_ALL`, `HF_TO_HOST`, or a peer id from an
    /// `HF_ON_MESSAGE` event. Returns `true` when the message was queued.
    pub fn send(&self, to: u64, data: &[u8]) -> bool {
        if !self.supports(16) || data.len() > MESSAGE_MAX {
            return false;
        }

        let Some(send) = self.raw_api().send else {
            return false;
        };
        let Ok(bytes) = u32::try_from(data.len()) else {
            return false;
        };

        unsafe { send(self.mod_handle(), to, data.as_ptr() as *const c_void, bytes) == 0 }
    }

    /// Sets a row of this mod's shared table (host only).
    ///
    /// An empty `value` removes the row. Returns `true` when set.
    pub fn share_set(&self, key: u64, value: &[u8]) -> bool {
        if !self.supports(16) || value.len() > SHARE_VALUE_MAX {
            return false;
        }

        let Some(share_set) = self.raw_api().share_set else {
            return false;
        };
        let Ok(bytes) = u32::try_from(value.len()) else {
            return false;
        };

        unsafe {
            share_set(
                self.mod_handle(),
                key,
                value.as_ptr() as *const c_void,
                bytes,
            ) == 0
        }
    }

    /// One row of this mod's shared table on this machine.
    ///
    /// Returns `None` when the row is absent.
    pub fn share_get(&self, key: u64) -> Option<Vec<u8>> {
        if !self.supports(16) {
            return None;
        }

        let share_get = self.raw_api().share_get?;
        let mut value = [0u8; SHARE_VALUE_MAX];
        let size = unsafe {
            share_get(
                self.mod_handle(),
                key,
                value.as_mut_ptr() as *mut c_void,
                SHARE_VALUE_MAX as u32,
            )
        } as usize;

        (size != 0).then(|| value[..size.min(SHARE_VALUE_MAX)].to_vec())
    }

    /// The `index`-th row of this mod's shared table on this machine.
    ///
    /// Returns the key and value, or `None` past the last row.
    pub fn share_row(&self, index: u32) -> Option<(u64, Vec<u8>)> {
        if !self.supports(16) {
            return None;
        }

        let share_row = self.raw_api().share_row?;
        let mut key = 0u64;
        let mut value = [0u8; SHARE_VALUE_MAX];
        let mut bytes = 0u32;
        let status = unsafe {
            share_row(
                self.mod_handle(),
                index,
                &mut key,
                value.as_mut_ptr() as *mut c_void,
                SHARE_VALUE_MAX as u32,
                &mut bytes,
            )
        };

        (status != 0).then(|| {
            let bytes = (bytes as usize).min(SHARE_VALUE_MAX);
            (key, value[..bytes].to_vec())
        })
    }
}
