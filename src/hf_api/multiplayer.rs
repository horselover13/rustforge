// Multiplayer: mod sync and the lobby browser.
//
// The mod message and shared-table channel (send, share_set, share_get,
// share_row, on_message, on_share) is in `crate::hf_net`.

#![allow(dead_code)]

use super::{HellForgeApi, util::cstring};
use crate::rust_ffi::{HfBrowseLobby, HfBrowseState, HfSyncMod, HfSyncState};

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
}
