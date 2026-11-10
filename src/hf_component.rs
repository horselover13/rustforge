// The game's entity component records, from
// `native/mod-sdk/hellforge_components.h`.
//
// Each type below is the `#[repr(C, packed)]` record of one component: its
// named fields are at their offsets, and the bytes research has not split into
// fields are `_unknown_XX` padding. The header also has settings-table rows
// (`HfProjectileSettings`, `HfExplosionSettings`, ...), which are found by id
// rather than as an entity's record.
//
// The game owns these records. A vanilla entity's are the game's read-only
// data: read them, and change them with
// [`set_field`](crate::hf_api::HellForgeApi::set_field),
// [`edit_entity`](crate::hf_api::HellForgeApi::edit_entity) or
// [`set_live`](crate::hf_api::HellForgeApi::set_live). A record an entity a mod
// added owns is writable through [`component_mut`](crate::hf_api::HellForgeApi::component_mut).
//
// Read and write them on the game thread (an event listener's or
// `on_game_thread`'s call), after `HF_EVENT_ENTITIES_READY`. Some records are
// remade each mission, so fetch one each time you use it rather than keeping
// it.

#![allow(dead_code)]

use crate::hf_api::HellForgeApi;
use crate::rust_ffi::HfRef;
use std::mem::size_of;

pub use crate::rust_ffi::{
    HfAbilityWeapon, HfAiSpawner, HfArcSettings, HfArcWeapon, HfAvatar, HfBeamSettings,
    HfBeamWeapon, HfBehavior, HfBombardment, HfBoosterSettings, HfBughole, HfDamageInfo,
    HfDemolitionTemplate, HfDeposit, HfDifficultySettings, HfDisplacement, HfEagle,
    HfEnemyComposition, HfEnemyRoster, HfEnemyRosterLevel11, HfExplosionSettings, HfExplosive,
    HfGore, HfHealth, HfHellpodPayload, HfHellpodRack, HfHiveMind, HfInventory, HfJumppack,
    HfLoadoutEntry, HfLoadoutPackage, HfMeleeHitVolume, HfMeleeWeapon, HfMount, HfOrbitalAbility,
    HfPassiveModifier, HfPassiveStat, HfProjectileSettings, HfProjectileWeapon, HfRecharge,
    HfRotation, HfSeekingMissile, HfSensorEye, HfShield, HfSquadTransport,
    HfSquadTransportFormation, HfStatusEffectSettings, HfStratagemBlocker, HfStratagemSettings,
    HfTag, HfThrowable, HfThrower, HfTurret, HfUnit, HfVehicleMotion, HfWeaponCharge, HfWeaponData,
    HfWeaponHeat, HfWeaponMagazine, HfWeaponReload, HfWeaponRounds, HfWeaponWindUp,
};

/// A component record generated from `hellforge_components.h`.
///
/// # Safety
/// Implementors must be the `#[repr(C, packed)]` struct the header generated
/// for the component named by [`NAME`](Self::NAME), with that record's layout
/// and size. The loader checks the name and the size, then the record is
/// viewed as `Self`.
pub unsafe trait Component: Sized {
    /// The component's name, as the loader knows it
    /// (`"WeaponDataComponentData"`).
    const NAME: &'static str;

    /// The game build the layout was reviewed on.
    const LAYOUT: u32 = 25480438;
}

macro_rules! impl_components {
    ($($ty:ident => $name:literal,)*) => {
        $(
            unsafe impl Component for $ty {
                const NAME: &'static str = $name;
            }
        )*
    };
}

impl_components! {
    HfProjectileWeapon => "ProjectileWeaponComponentData",
    HfWeaponData => "WeaponDataComponentData",
    HfWeaponMagazine => "WeaponMagazineComponentData",
    HfTag => "TagComponentData",
    HfMount => "MountComponentData",
    HfBehavior => "BehaviorComponentData",
    HfLoadoutPackage => "LoadoutPackageComponentData",
    HfDeposit => "DepositComponentData",
    HfDifficultySettings => "DifficultySettings",
    HfDamageInfo => "DamageInfo",
    HfProjectileSettings => "ProjectileSettings",
    HfStratagemSettings => "StratagemDefinition",
    HfExplosionSettings => "ExplosionSettings",
    HfArcSettings => "ArcSettings",
    HfBeamSettings => "BeamSettings",
    HfStatusEffectSettings => "StatusEffectSettings",
    HfHealth => "HealthComponentData",
    HfJumppack => "JumppackComponentData",
    HfRecharge => "RechargeComponentData",
    HfShield => "ShieldComponentData",
    HfHellpodPayload => "HellpodPayloadComponentData",
    HfSensorEye => "SensorEyeComponentData",
    HfTurret => "TurretComponentData",
    HfWeaponWindUp => "WeaponWindUpComponentData",
    HfBeamWeapon => "BeamWeaponComponentData",
    HfWeaponHeat => "WeaponHeatComponentData",
    HfEagle => "EagleComponentData",
    HfThrower => "ThrowerComponentData",
    HfBombardment => "BombardmentComponentData",
    HfExplosive => "ExplosiveComponentData",
    HfThrowable => "ThrowableComponentData",
    HfVehicleMotion => "VehicleMotionComponentData",
    HfWeaponCharge => "WeaponChargeComponentData",
    HfWeaponReload => "WeaponReloadComponentData",
    HfWeaponRounds => "WeaponRoundsComponentData",
    HfArcWeapon => "ArcWeaponComponentData",
    HfBoosterSettings => "BoosterSettings",
    HfPassiveModifier => "PassiveModifier",
    HfPassiveStat => "PassiveStat",
    HfHiveMind => "HiveMindComponentData",
    HfOrbitalAbility => "OrbitalAbilityComponentData",
    HfRotation => "RotationComponentData",
    HfDisplacement => "DisplacementComponentData",
    HfSeekingMissile => "SeekingMissileComponentData",
    HfLoadoutEntry => "LoadoutEntryComponentData",
    HfHellpodRack => "HellpodRackComponentData",
    HfAvatar => "AvatarComponentData",
    HfInventory => "InventoryComponentData",
    HfMeleeHitVolume => "MeleeHitVolume",
    HfMeleeWeapon => "MeleeWeaponComponentData",
    HfAbilityWeapon => "AbilityWeaponComponentData",
    HfEnemyRoster => "EnemyRoster",
    HfEnemyRosterLevel11 => "EnemyRosterLevel11",
    HfEnemyComposition => "EnemyComposition",
    HfUnit => "UnitComponentData",
    HfSquadTransportFormation => "SquadTransportFormationComponentData",
    HfSquadTransport => "SquadTransportComponentData",
    HfGore => "GoreComponentData",
    HfAiSpawner => "AiSpawnerComponentData",
    HfStratagemBlocker => "StratagemBlockerComponentData",
    HfBughole => "BugholeComponentData",
    HfDemolitionTemplate => "DemolitionTemplate",
}

impl HellForgeApi {
    /// An entity's own record of component `T`, after
    /// `HF_EVENT_ENTITIES_READY`.
    ///
    /// Returns `None` when the entity does not own the record, owns several,
    /// the loader is older than SDK 6, or its layout is unknown.
    pub fn component<T: Component>(&self, entity: HfRef) -> Option<&T> {
        let record = self.record(entity, T::NAME, T::LAYOUT, size_of::<T>() as u32)?;

        Some(unsafe { &*record.cast::<T>() })
    }

    /// An entity's own record of component `T`, for writing.
    ///
    /// # Safety
    /// Only write the record of an entity a mod added: its records are
    /// writable. A vanilla entity's record is the game's read-only data, and
    /// writing through it crashes the game. The caller must also make sure no
    /// other reference or game code touches the record at the same time, and
    /// must call on the game thread.
    pub unsafe fn component_mut<T: Component>(&self, entity: HfRef) -> Option<&mut T> {
        let record = self.record(entity, T::NAME, T::LAYOUT, size_of::<T>() as u32)?;

        Some(unsafe { &mut *record.cast::<T>() })
    }

    /// The record of component `T` that `resource` owns, after
    /// `HF_EVENT_ENTITIES_READY`.
    ///
    /// This is [`component_record`](Self::component_record) with the layout
    /// checked: it returns the record and its stride, or `None` when the
    /// entity owns no such record, owns several, or shares it with another
    /// entity.
    pub fn shared_component<T: Component>(&self, resource: u64) -> Option<(&T, u32)> {
        let (address, stride) = self.component_record(T::NAME, resource)?;

        Some((unsafe { &*(address as *const T) }, stride))
    }
}
