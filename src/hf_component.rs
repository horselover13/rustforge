// The game's entity component records, from
// `native/mod-sdk/hellforge_components.h`.
//
// The header also generates the settings-table rows (`HfDifficultySettings`,
// `HfDamageInfo`, `HfProjectileSettings`, ...). They are re-exported here but
// are not `Component`s: the game finds them by table and row id, not as an
// entity's record.

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

/// An entity component record generated from `hellforge_components.h`.
///
/// This is implemented only for the 46 component records, not for the
/// settings-table rows the header also generates.
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
    HfHiveMind => "HiveMindComponentData",
    HfOrbitalAbility => "OrbitalAbilityComponentData",
    HfRotation => "RotationComponentData",
    HfDisplacement => "DisplacementComponentData",
    HfSeekingMissile => "SeekingMissileComponentData",
    HfLoadoutEntry => "LoadoutEntryComponentData",
    HfHellpodRack => "HellpodRackComponentData",
    HfAvatar => "AvatarComponentData",
    HfInventory => "InventoryComponentData",
    HfMeleeWeapon => "MeleeWeaponComponentData",
    HfAbilityWeapon => "AbilityWeaponComponentData",
    HfUnit => "UnitComponentData",
    HfSquadTransportFormation => "SquadTransportFormationComponentData",
    HfSquadTransport => "SquadTransportComponentData",
    HfGore => "GoreComponentData",
    HfAiSpawner => "AiSpawnerComponentData",
    HfStratagemBlocker => "StratagemBlockerComponentData",
    HfBughole => "BugholeComponentData",
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
    /// This is [`component_record`](Self::component_record) with the type
    /// applied: it returns the record and its stride, or `None` when the
    /// entity owns no such record, owns several, or shares it with another
    /// entity.
    ///
    /// Unlike [`component`](Self::component), this lookup has no layout or
    /// size check, so `T` must match the record the resource actually owns.
    pub fn shared_component<T: Component>(&self, resource: u64) -> Option<(&T, u32)> {
        let (address, stride) = self.component_record(T::NAME, resource)?;

        Some((unsafe { &*(address as *const T) }, stride))
    }
}
