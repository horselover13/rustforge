// Stats fields the live-edit API cannot reach, and whose stats a lobby uses.

#![allow(dead_code)]

use super::HellForgeApi;
use crate::rust_ffi::{HfLiveValue, HfStat, HfValue};

impl HellForgeApi {
    /// A stats field's value now and its base.
    ///
    /// Covers the fields [`get_live`](Self::get_live) cannot (entity deltas,
    /// for example). Returns `None` when the field is not in the snapshot or
    /// has not been read yet.
    pub fn get_stat(&self, stat: &HfStat) -> Option<HfLiveValue> {
        if !self.supports(6) {
            return None;
        }

        let get_stat = self.raw_api().get_stat?;
        let mut out: HfLiveValue = unsafe { std::mem::zeroed() };

        (unsafe { get_stat(self.mod_handle(), stat, &mut out) } != 0).then_some(out)
    }

    /// Changes a stats field live, where a stats file would write it at boot.
    ///
    /// `value` is `None` to restore the value from before this session's live
    /// change (or, with `HF_LIVE_BASE`, the base value). Returns
    /// `HF_APPLIES_RESPAWN`, or 0 when refused.
    pub fn set_stat(&self, stat: &HfStat, value: Option<&HfValue>, flags: u32) -> u32 {
        if !self.supports(9) {
            return 0;
        }

        let Some(set_stat) = self.raw_api().set_stat else {
            return 0;
        };

        let value = value.map_or(std::ptr::null(), |value| value as *const HfValue);

        unsafe { set_stat(self.mod_handle(), stat, value, flags) as u32 }
    }

    /// Whose stats the lobby plays with: `HF_STATS_HOST` or `HF_STATS_OWN`.
    pub fn stats_source(&self) -> u32 {
        if !self.supports(15) {
            return 0;
        }

        let Some(stats_source) = self.raw_api().stats_source else {
            return 0;
        };

        unsafe { stats_source(self.mod_handle()) }
    }
}
