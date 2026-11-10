// Entity definition blocks, deferred edits, records, and live edits.

#![allow(dead_code)]

use super::{HellForgeApi, util::cstring};
use crate::rust_ffi::{HfEditDef, HfEntityDef, HfField, HfLiveValue, HfRef, HfValue};
use std::ffi::c_void;

impl HellForgeApi {
    /// Registers a new entity from its definition block (call from
    /// `hellforge_mod_init`).
    ///
    /// Registering the same block again returns the same entity. Returns the
    /// entity, or 0 on failure.
    pub fn register_entity(&self, entity: &HfEntityDef) -> HfRef {
        if !self.supports(6) {
            return 0;
        }

        let Some(register_entity) = self.raw_api().register_entity else {
            return 0;
        };

        unsafe { register_entity(self.mod_handle(), entity) }
    }

    /// Queues a deferred copy of `field` from `from` onto `entity`.
    ///
    /// Returns `true` when queued, `false` when refused.
    pub fn copy_field(&self, entity: HfRef, from: HfRef, field: &HfField) -> bool {
        if !self.supports(6) {
            return false;
        }

        let Some(copy_field) = self.raw_api().copy_field else {
            return false;
        };

        unsafe { copy_field(self.mod_handle(), entity, from, field) == 0 }
    }

    /// Queues a deferred set of `field` on `entity`.
    ///
    /// Returns `true` when queued, `false` when refused.
    pub fn set_field(&self, entity: HfRef, field: &HfField, value: &HfValue) -> bool {
        if !self.supports(6) {
            return false;
        }

        let Some(set_field) = self.raw_api().set_field else {
            return false;
        };

        unsafe { set_field(self.mod_handle(), entity, field, value) == 0 }
    }

    /// Queues a deferred change to `entity`'s tags.
    ///
    /// Returns `true` when queued, `false` when refused.
    pub fn set_tag(&self, entity: HfRef, tag: u64, present: bool) -> bool {
        if !self.supports(6) {
            return false;
        }

        let Some(set_tag) = self.raw_api().set_tag else {
            return false;
        };

        unsafe { set_tag(self.mod_handle(), entity, tag, present as i32) == 0 }
    }

    /// Queues a deferred mount of `carried` in `entity`'s `slot`.
    ///
    /// Returns `true` when queued, `false` when refused.
    pub fn mount(&self, entity: HfRef, slot: u32, carried: HfRef) -> bool {
        if !self.supports(6) {
            return false;
        }

        let Some(mount) = self.raw_api().mount else {
            return false;
        };

        unsafe { mount(self.mod_handle(), entity, slot, carried) == 0 }
    }

    /// An entity's own record of `component`, checked against the layout the
    /// mod was built with.
    ///
    /// Returns `None` when the entity does not own the record, the layout is
    /// unknown, or the loader is too old. Reading or writing the returned
    /// pointer is the caller's responsibility.
    pub fn record(
        &self,
        entity: HfRef,
        component: &str,
        layout: u32,
        size: u32,
    ) -> Option<*mut c_void> {
        if !self.supports(6) {
            return None;
        }

        let component = cstring(component)?;
        let record = self.raw_api().record?;
        let address =
            unsafe { record(self.mod_handle(), entity, component.as_ptr(), layout, size) };

        (!address.is_null()).then_some(address)
    }

    /// Changes an existing entity from a definition block.
    ///
    /// Returns `true` when queued, `false` when refused.
    pub fn edit_entity(&self, edit: &HfEditDef) -> bool {
        if !self.supports(6) {
            return false;
        }

        let Some(edit_entity) = self.raw_api().edit_entity else {
            return false;
        };

        unsafe { edit_entity(self.mod_handle(), edit) == 0 }
    }

    /// Changes a field in the running game, kept for the rest of the session.
    ///
    /// `value` is `None` to undo this mod's live edit (or, with
    /// `HF_LIVE_BASE`, to restore the field's base value). Returns
    /// `HF_APPLIES_*`, or 0 when refused.
    pub fn set_live(
        &self,
        target: HfRef,
        field: &HfField,
        value: Option<&HfValue>,
        flags: u32,
    ) -> u32 {
        if !self.supports(6) {
            return 0;
        }

        let Some(set_live) = self.raw_api().set_live else {
            return 0;
        };

        let value = value.map_or(std::ptr::null(), |value| value as *const HfValue);

        unsafe { set_live(self.mod_handle(), target, field, value, flags) as u32 }
    }

    /// Reads a field of the running game, found the way
    /// [`set_live`](Self::set_live) finds it.
    ///
    /// Returns the value now and its base, or `None` when it cannot be read.
    pub fn get_live(&self, target: HfRef, field: &HfField) -> Option<HfLiveValue> {
        if !self.supports(6) {
            return None;
        }

        let get_live = self.raw_api().get_live?;
        let mut out: HfLiveValue = unsafe { std::mem::zeroed() };

        (unsafe { get_live(self.mod_handle(), target, field, &mut out) } != 0).then_some(out)
    }
}
