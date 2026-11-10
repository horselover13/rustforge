// Network field metadata and network/entity identity.

#![allow(dead_code)]

use super::{HellForgeApi, util::cstring};
use crate::rust_ffi::HfFieldType;

impl HellForgeApi {
    /// Looks up a network field's type by name (after `HF_EVENT_NETWORK_READY`).
    ///
    /// Returns `None` when the field is unknown or the loader is too old.
    pub fn network_field_type(&self, name: &str) -> Option<HfFieldType> {
        let name = cstring(name)?;
        let network_field_type = self.raw_api().network_field_type?;

        let mut out: HfFieldType = unsafe { std::mem::zeroed() };
        let status = unsafe { network_field_type(name.as_ptr(), &mut out) };

        (status == 0).then_some(out)
    }

    /// Widens an integer network field to `bits`, if it is exactly `expect`.
    ///
    /// Returns `true` when the type was widened (or already was); `false` when
    /// it was left alone or the field is unknown.
    pub fn set_network_field_bits(&self, name: &str, expect: HfFieldType, bits: i32) -> bool {
        let Some(name) = cstring(name) else {
            return false;
        };
        let Some(set_bits) = self.raw_api().set_network_field_bits else {
            return false;
        };

        unsafe { set_bits(self.mod_handle(), name.as_ptr(), expect, bits) == 0 }
    }

    /// Whether this machine simulates `entity`.
    ///
    /// `None` means there is no such entity; the loader is too old returns
    /// `Some(false)`.
    pub fn is_authority(&self, entity: u32) -> Option<bool> {
        if !self.supports(16) {
            return Some(false);
        }

        let is_authority = self.raw_api().is_authority?;

        match unsafe { is_authority(self.mod_handle(), entity) } {
            1 => Some(true),
            0 => Some(false),
            _ => None,
        }
    }

    /// The entity a network id names on this machine now.
    ///
    /// Returns `None` when the id names no entity, or the loader is too old.
    pub fn entity_of_network(&self, network: u32) -> Option<u32> {
        if !self.supports(16) {
            return None;
        }

        let entity_of_network = self.raw_api().entity_of_network?;
        let entity = unsafe { entity_of_network(self.mod_handle(), network) };

        (entity != u32::MAX).then_some(entity)
    }
}
