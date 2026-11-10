// Gameplay actions: healing, damage, statuses, movement, explosions, shots and
// the transport Pelican.

#![allow(dead_code)]

use super::HellForgeApi;
use crate::rust_ffi::{HfHoming, HfLimbState, HfPelicanCall, HfShotRule};

impl HellForgeApi {
    /// Heals the local Helldiver by `amount`, up to its maximum.
    ///
    /// Returns `HF_ACT_*`, or 0 when refused.
    pub fn heal(&self, amount: f32) -> u32 {
        if !self.supports(11) {
            return 0;
        }

        let Some(heal) = self.raw_api().heal else {
            return 0;
        };

        unsafe { heal(self.mod_handle(), amount) as u32 }
    }

    /// Heals one of the local Helldiver's limbs in full.
    ///
    /// Returns `HF_ACT_*`, or 0 when refused.
    pub fn heal_limb(&self, limb: u32) -> u32 {
        if !self.supports(11) {
            return 0;
        }

        let Some(heal_limb) = self.raw_api().heal_limb else {
            return 0;
        };

        unsafe { heal_limb(self.mod_handle(), limb) as u32 }
    }

    /// Damages one of the local Helldiver's limbs.
    ///
    /// Returns `HF_ACT_*`, or 0 when refused.
    pub fn injure(&self, limb: u32, damage: i32) -> u32 {
        if !self.supports(11) {
            return 0;
        }

        let Some(injure) = self.raw_api().injure else {
            return 0;
        };

        unsafe { injure(self.mod_handle(), limb, damage) as u32 }
    }

    /// A Helldiver's limb now.
    ///
    /// Returns `None` when the state cannot be read.
    pub fn limb_state(&self, entity: u32, limb: u32) -> Option<HfLimbState> {
        if !self.supports(11) {
            return None;
        }

        let limb_state = self.raw_api().limb_state?;
        let mut out: HfLimbState = unsafe { std::mem::zeroed() };

        (unsafe { limb_state(self.mod_handle(), entity, limb, &mut out) } != 0).then_some(out)
    }

    /// Applies a status (`HF_STATUS_*`) to an entity.
    ///
    /// `buildup` is 0 .. 1000 (100 is a full hit). Returns `HF_ACT_*`, or 0
    /// when refused.
    pub fn apply_status(&self, entity: u32, status: u32, buildup: f32) -> u32 {
        if !self.supports(11) {
            return 0;
        }

        let Some(apply_status) = self.raw_api().apply_status else {
            return 0;
        };

        unsafe { apply_status(self.mod_handle(), entity, status, buildup) as u32 }
    }

    /// Adds to the local Helldiver's velocity (m/s, world space, Z up).
    ///
    /// Each part is capped at 25 and the result at 50. Returns `HF_ACT_*`, or
    /// 0 when refused.
    pub fn add_velocity(&self, delta: [f32; 3]) -> u32 {
        if !self.supports(11) {
            return 0;
        }

        let Some(add_velocity) = self.raw_api().add_velocity else {
            return 0;
        };

        unsafe { add_velocity(self.mod_handle(), delta.as_ptr()) as u32 }
    }

    /// Makes the local Helldiver use its B-1 Supply Pack on itself.
    ///
    /// Returns `HF_ACT_*`, or 0 when refused.
    pub fn resupply_from_pack(&self) -> u32 {
        if !self.supports(11) {
            return 0;
        }

        let Some(resupply) = self.raw_api().resupply_from_pack else {
            return 0;
        };

        unsafe { resupply(self.mod_handle()) as u32 }
    }

    /// Spawns an explosion at `position`.
    ///
    /// Returns `HF_ACT_*`, or 0 when refused.
    pub fn spawn_explosion(&self, explosion_type: u32, position: [f32; 3]) -> u32 {
        if !self.supports(11) {
            return 0;
        }

        let Some(spawn_explosion) = self.raw_api().spawn_explosion else {
            return 0;
        };

        unsafe { spawn_explosion(self.mod_handle(), explosion_type, position.as_ptr()) as u32 }
    }

    /// Fires a projectile from `position` along `direction`.
    ///
    /// Returns `HF_ACT_*`, or 0 when refused.
    pub fn spawn_projectile(
        &self,
        projectile_type: u32,
        position: [f32; 3],
        direction: [f32; 3],
    ) -> u32 {
        if !self.supports(11) {
            return 0;
        }

        let Some(spawn_projectile) = self.raw_api().spawn_projectile else {
            return 0;
        };

        unsafe {
            spawn_projectile(
                self.mod_handle(),
                projectile_type,
                position.as_ptr(),
                direction.as_ptr(),
            ) as u32
        }
    }

    /// Credits an entity's kills to its owner, this machine's player.
    ///
    /// Returns `HF_ACT_*`, or 0 when refused.
    pub fn credit_to_player(&self, entity: u32) -> u32 {
        if !self.supports(11) {
            return 0;
        }

        let Some(credit_to_player) = self.raw_api().credit_to_player else {
            return 0;
        };

        unsafe { credit_to_player(self.mod_handle(), entity) as u32 }
    }

    /// Pauses (`true`) or resumes (`false`) the mission timer.
    ///
    /// Returns `HF_ACT_*`, or 0 when refused.
    pub fn freeze_mission_timer(&self, frozen: bool) -> u32 {
        if !self.supports(11) {
            return 0;
        }

        let Some(freeze) = self.raw_api().freeze_mission_timer else {
            return 0;
        };

        unsafe { freeze(self.mod_handle(), frozen as i32) as u32 }
    }

    /// Calls in a transport Pelican.
    ///
    /// Returns its entity, or `None` when the call was refused or is still
    /// waiting (try again on the next click).
    pub fn spawn_pelican(&self, call: &HfPelicanCall) -> Option<u32> {
        if !self.supports(11) {
            return None;
        }

        let spawn_pelican = self.raw_api().spawn_pelican?;
        let mut entity = 0u32;
        let status = unsafe { spawn_pelican(self.mod_handle(), call, &mut entity) };

        (status != 0).then_some(entity)
    }

    /// Adds a shot rule for the local player's shots of one projectile type.
    ///
    /// Returns the rule id (1 and up), or 0 when refused.
    pub fn modify_shots(&self, rule: &HfShotRule) -> u32 {
        if !self.supports(11) {
            return 0;
        }

        let Some(modify_shots) = self.raw_api().modify_shots else {
            return 0;
        };

        unsafe { modify_shots(self.mod_handle(), rule) }
    }

    /// Makes the local player's shots of one projectile type home in.
    ///
    /// Returns the rule id (1 and up), or 0 when refused.
    pub fn homing(&self, rule: &HfHoming) -> u32 {
        if !self.supports(11) {
            return 0;
        }

        let Some(homing) = self.raw_api().homing else {
            return 0;
        };

        unsafe { homing(self.mod_handle(), rule) }
    }

    /// Ends a shot or homing rule.
    ///
    /// Returns `true` when the rule was stopped, `false` when there was no such
    /// rule.
    pub fn stop_shots(&self, id: u32) -> bool {
        if !self.supports(11) {
            return false;
        }

        let Some(stop_shots) = self.raw_api().stop_shots else {
            return false;
        };

        unsafe { stop_shots(self.mod_handle(), id) == 0 }
    }
}
