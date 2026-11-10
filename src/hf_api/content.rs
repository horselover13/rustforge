// Content registration and lookup: weapons, projectiles, damage, stratagems,
// entities, behaviours, difficulties, abilities, and the content catalogue.

#![allow(dead_code)]

use super::{HellForgeApi, util::cstring};
use crate::rust_ffi::{
    HfAbility, HfBehaviour, HfContent, HfDamage, HfDifficulty, HfEntity, HfProjectile, HfStratagem,
    HfStratagemCategory, HfWeapon,
};

impl HellForgeApi {
    /// The address of an entity's record of `component`, after
    /// `HF_EVENT_ENTITIES_READY`.
    ///
    /// Returns the record address and its stride, or `None` when the entity
    /// owns no such record, owns several, or shares it with another entity.
    pub fn component_record(&self, component: &str, resource: u64) -> Option<(usize, u32)> {
        let component = cstring(component)?;
        let component_record = self.raw_api().component_record?;

        let mut stride = 0u32;
        let address = unsafe { component_record(component.as_ptr(), resource, &mut stride) };

        (address != 0).then_some((address, stride))
    }

    /// Adds a new arsenal weapon (call from `hellforge_mod_init`).
    ///
    /// Returns the new loadout id, or 0 on failure.
    pub fn add_weapon(&self, weapon: &HfWeapon) -> u32 {
        if !self.supports(2) {
            return 0;
        }

        let Some(add_weapon) = self.raw_api().add_weapon else {
            return 0;
        };

        unsafe { add_weapon(self.mod_handle(), weapon) }
    }

    /// The entity resource of a weapon any mod added, for
    /// [`component_record`](Self::component_record); 0 if there is none.
    pub fn weapon_entity(&self, weapon: u32) -> u64 {
        if !self.supports(2) {
            return 0;
        }

        let Some(weapon_entity) = self.raw_api().weapon_entity else {
            return 0;
        };

        unsafe { weapon_entity(weapon) }
    }

    /// Adds a new projectile type (call from `hellforge_mod_init`).
    ///
    /// Returns its `ProjectileType` (351 and up), or 0 on failure.
    pub fn add_projectile(&self, projectile: &HfProjectile) -> u32 {
        if !self.supports(3) {
            return 0;
        }

        let Some(add_projectile) = self.raw_api().add_projectile else {
            return 0;
        };

        unsafe { add_projectile(self.mod_handle(), projectile) }
    }

    /// The settings row of a projectile a mod added, after
    /// `HF_EVENT_PROJECTILES_READY`; 0 if there is none.
    pub fn projectile_row(&self, projectile_type: u32) -> usize {
        if !self.supports(3) {
            return 0;
        }

        let Some(projectile_row) = self.raw_api().projectile_row else {
            return 0;
        };

        unsafe { projectile_row(projectile_type) }
    }

    /// Adds a new damage type (call from `hellforge_mod_init`, before the
    /// projectiles that deal it).
    ///
    /// Returns its id (650 and up), or 0 on failure.
    pub fn add_damage(&self, damage: &HfDamage) -> u32 {
        if !self.supports(3) {
            return 0;
        }

        let Some(add_damage) = self.raw_api().add_damage else {
            return 0;
        };

        unsafe { add_damage(self.mod_handle(), damage) }
    }

    /// The `DamageInfo` row of a damage type a mod added, after
    /// `HF_EVENT_PROJECTILES_READY`; 0 if there is none.
    pub fn damage_row(&self, id: u32) -> usize {
        if !self.supports(3) {
            return 0;
        }

        let Some(damage_row) = self.raw_api().damage_row else {
            return 0;
        };

        unsafe { damage_row(id) }
    }

    /// The game's 64-bit resource id of a resource name.
    pub fn resource_hash(&self, name: &str) -> u64 {
        if !self.supports(3) {
            return 0;
        }

        let Some(name) = cstring(name) else {
            return 0;
        };
        let Some(resource_hash) = self.raw_api().resource_hash else {
            return 0;
        };

        unsafe { resource_hash(name.as_ptr()) }
    }

    /// Adds a new stratagem (call from `hellforge_mod_init`).
    ///
    /// Returns its `StratagemType` (150 and up), or 0 on failure.
    pub fn add_stratagem(&self, stratagem: &HfStratagem) -> u32 {
        if !self.supports(4) {
            return 0;
        }

        let Some(add_stratagem) = self.raw_api().add_stratagem else {
            return 0;
        };

        unsafe { add_stratagem(self.mod_handle(), stratagem) }
    }

    /// The settings row of a stratagem a mod added, once the game has loaded
    /// its stratagems; 0 if there is none.
    pub fn stratagem_row(&self, stratagem_type: u32) -> usize {
        if !self.supports(4) {
            return 0;
        }

        let Some(stratagem_row) = self.raw_api().stratagem_row else {
            return 0;
        };

        unsafe { stratagem_row(stratagem_type) }
    }

    /// Adds a new entity (call from `hellforge_mod_init`).
    ///
    /// Returns its resource id, or 0 on failure.
    pub fn add_entity(&self, entity: &HfEntity) -> u64 {
        if !self.supports(4) {
            return 0;
        }

        let Some(add_entity) = self.raw_api().add_entity else {
            return 0;
        };

        unsafe { add_entity(self.mod_handle(), entity) }
    }

    /// Adds a new behaviour (call from `hellforge_mod_init`).
    ///
    /// Returns its id (694 and up), or 0 on failure.
    pub fn add_behaviour(&self, behaviour: &HfBehaviour) -> u32 {
        if !self.supports(4) {
            return 0;
        }

        let Some(add_behaviour) = self.raw_api().add_behaviour else {
            return 0;
        };

        unsafe { add_behaviour(self.mod_handle(), behaviour) }
    }

    /// Adds difficulty level 11 (call from `hellforge_mod_init`).
    ///
    /// Returns 11, or 0 on failure.
    pub fn add_difficulty(&self, difficulty: &HfDifficulty) -> u32 {
        if !self.supports(5) {
            return 0;
        }

        let Some(add_difficulty) = self.raw_api().add_difficulty else {
            return 0;
        };

        unsafe { add_difficulty(self.mod_handle(), difficulty) }
    }

    /// The settings row of a difficulty level once HellForge has moved the
    /// table; 0 if there is none.
    pub fn difficulty_row(&self, level: u32) -> usize {
        if !self.supports(5) {
            return 0;
        }

        let Some(difficulty_row) = self.raw_api().difficulty_row else {
            return 0;
        };

        unsafe { difficulty_row(level) }
    }

    /// Adds a new ability (call from `hellforge_mod_init`).
    ///
    /// Returns its id (2868 and up), or 0 on failure.
    pub fn add_ability(&self, ability: &HfAbility) -> u32 {
        if !self.supports(7) {
            return 0;
        }

        let Some(add_ability) = self.raw_api().add_ability else {
            return 0;
        };

        unsafe { add_ability(self.mod_handle(), ability) }
    }

    /// Adds a stratagem selection-screen heading (call from
    /// `hellforge_mod_init`, before its stratagems).
    ///
    /// Returns the category handle, or 0 on failure.
    pub fn add_stratagem_category(&self, category: &HfStratagemCategory) -> u32 {
        if !self.supports(8) {
            return 0;
        }

        let Some(add_category) = self.raw_api().add_stratagem_category else {
            return 0;
        };

        unsafe { add_category(self.mod_handle(), category) }
    }

    /// Finds content by kind and name.
    ///
    /// Vanilla content is named by its catalogue name; content a mod added can
    /// be named by its identity (for example
    /// `"skyeshade.pelican-support:pelican_gatling"`). Returns 0 when nothing
    /// matches.
    pub fn find(&self, kind: u32, name: &str) -> u64 {
        if !self.supports(6) {
            return 0;
        }

        let Some(name) = cstring(name) else {
            return 0;
        };
        let Some(find) = self.raw_api().find else {
            return 0;
        };

        unsafe { find(kind, name.as_ptr()) }
    }

    /// The `index`-th piece of content mods added.
    ///
    /// Returns `None` past the last piece, or when the loader is too old.
    pub fn content(&self, index: u32) -> Option<HfContent> {
        if !self.supports(7) {
            return None;
        }

        let content = self.raw_api().content?;
        let mut out: HfContent = unsafe { std::mem::zeroed() };
        out.size = std::mem::size_of::<HfContent>() as u32;

        (unsafe { content(self.mod_handle(), index, &mut out) } != 0).then_some(out)
    }

    /// Replaces a settings row's bytes with another row's, live.
    ///
    /// Returns `HF_APPLIES_NOW`, or 0 on failure.
    pub fn copy_row(&self, table: &str, row: u64, from: u64, offset: u32, size: u32) -> u32 {
        if !self.supports(9) {
            return 0;
        }

        let Some(table) = cstring(table) else {
            return 0;
        };
        let Some(copy_row) = self.raw_api().copy_row else {
            return 0;
        };

        unsafe { copy_row(self.mod_handle(), table.as_ptr(), row, from, offset, size) as u32 }
    }
}
