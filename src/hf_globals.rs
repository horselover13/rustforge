// The game's objects by name, from `native/mod-sdk/hellforge_globals.h`.
//
// Every global is returned as its own handle (for example [`Game`]) rather
// than as a reference to the C struct. The handle reads the game's memory
// through the loader, so a field that cannot be read becomes `None` instead of
// an invalid value. Handles are only valid on the game thread, and the game
// remakes some objects: fetch the global again each time you use it.

#![allow(dead_code)]

use crate::hf_api::{HellForgeApi, util::cstring};
use crate::rust_ffi::{
    HfAiSpawnerManager, HfAiSquadManager, HfArcWeapons, HfArmory, HfAvatarManager,
    HfBehaviourManager, HfBombardmentManager, HfBugholeManager, HfCareerStats, HfClock,
    HfCollisionFilterA, HfCollisionFilterB, HfCombatQueues, HfCustomizationManager,
    HfDepositManager, HfDestructibles, HfDestructionRecords, HfDisplacementManager, HfDropAnchors,
    HfEagleManager, HfEncounterManager, HfEngineApi, HfEngineCore, HfEngineEntityIndex,
    HfEngineUnitApi, HfEntityManager, HfEquipmentManager, HfExplosionQueue, HfFactionManager,
    HfGame, HfGameMode, HfGameModeMission, HfGoreManager, HfGuardForces, HfHealthManager,
    HfHellpodPayloadManager, HfHiveMindManager, HfInventoryManager, HfJumppackManager,
    HfLaserDesignatorManager, HfLocalUser, HfLocomotionManager, HfMenuStack, HfMenuStates,
    HfMotionManager, HfMountManager, HfNetworkConfigOwner, HfNetworkManager, HfObjectiveManager,
    HfOrbitalAbilityManager, HfPackageSystem, HfPerceptionManager, HfPhysicsProperties,
    HfPlayerHistory, HfPlayerManager, HfProfile, HfProgression, HfProjectileSystem,
    HfRotationManager, HfSeekingMissileManager, HfSensorEyeManager, HfShieldManager,
    HfSquadFormationManager, HfSquadTransportManager, HfStratagemBlockerManager, HfTagManager,
    HfThumbnailManager, HfTurretManager, HfUnitManager, HfUnitRegistry, HfVehicleManager,
    HfVehicleMotionManager, HfWeaponHeats, HfWeaponLinkerManager, HfWeaponMagazines,
    HfWeaponRoundsManager, HfWeaponTargeting, HfWieldableManager, HfWielderManager, HfWwiseApi,
};
use std::ffi::c_void;
use std::fmt;
use std::mem::{MaybeUninit, offset_of, size_of};
use std::ptr::NonNull;

/// A readable window into one of the game's objects.
#[derive(Clone, Copy)]
struct Memory<'a> {
    api: &'a HellForgeApi,
    address: usize,
}

impl<'a> Memory<'a> {
    fn new(api: &'a HellForgeApi, address: usize) -> Self {
        Self { api, address }
    }

    fn at(self, offset: usize) -> Self {
        Self {
            api: self.api,
            address: self.address.wrapping_add(offset),
        }
    }

    fn read<T: Copy>(self, offset: usize) -> Option<T> {
        let mut value = MaybeUninit::<T>::uninit();
        let bytes = unsafe {
            std::slice::from_raw_parts_mut(value.as_mut_ptr().cast::<u8>(), size_of::<T>())
        };
        if !self.api.read(self.address.wrapping_add(offset), bytes) {
            return None;
        }

        Some(unsafe { value.assume_init() })
    }

    fn read_u8(self, offset: usize) -> Option<u8> {
        self.read(offset)
    }

    fn read_u32(self, offset: usize) -> Option<u32> {
        self.read(offset)
    }

    fn read_i32(self, offset: usize) -> Option<i32> {
        self.read(offset)
    }

    fn read_u64(self, offset: usize) -> Option<u64> {
        self.read(offset)
    }

    fn read_pointer(self, offset: usize) -> Option<usize> {
        let pointer = self.read::<usize>(offset)?;

        (pointer != 0).then_some(pointer)
    }

    fn read_object<T>(self, offset: usize) -> Option<NonNull<T>> {
        NonNull::new(self.read_pointer(offset)? as *mut T)
    }

    fn as_ptr(self) -> *mut c_void {
        self.address as *mut c_void
    }
}

impl fmt::Debug for Memory<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Memory")
            .field("address", &format_args!("{:#x}", self.address))
            .finish()
    }
}

macro_rules! global {
    ($(#[$meta:meta])* $name:ident) => {
        $(#[$meta])*
        #[derive(Clone, Copy, Debug)]
        pub struct $name<'a>(Memory<'a>);

        impl<'a> $name<'a> {
            fn new(api: &'a HellForgeApi, object: NonNull<c_void>) -> Self {
                Self(Memory::new(api, object.as_ptr() as usize))
            }

            /// The game-owned object this handle reads.
            ///
            /// It is the loader's pointer, not a copy: it is live only on the
            /// game thread, and only until the game remakes the object.
            pub fn as_ptr(&self) -> *mut c_void {
                self.0.as_ptr()
            }
        }
    };
}

/// The game's state (`HF_STATE_*`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GameState {
    None,
    Splash,
    Title,
    Ship,
    Mission,
    PrepareShip,
    PrepareMission,
    Other(u32),
}

impl GameState {
    fn from_raw(value: u32) -> Self {
        match value {
            0 => Self::None,
            1 => Self::Splash,
            2 => Self::Title,
            3 => Self::Ship,
            4 => Self::Mission,
            5 => Self::PrepareShip,
            6 => Self::PrepareMission,
            other => Self::Other(other),
        }
    }
}

global! {
    /// The session's top object (`game.dll+0x3326340`).
    ///
    /// Its state is the game's state, which the game's `ChangeState` sets.
    Game
}

impl<'a> Game<'a> {
    /// The game's state now.
    pub fn state(&self) -> Option<GameState> {
        self.0
            .read_u32(offset_of!(HfGame, state))
            .map(GameState::from_raw)
    }

    /// The game's Wwise world, or `None`.
    pub fn wwise_world(&self) -> Option<NonNull<c_void>> {
        self.0.read_object(offset_of!(HfGame, wwise_world))
    }

    /// The mission's level (the level generator), or `None` outside a mission.
    pub fn level(&self) -> Option<NonNull<c_void>> {
        self.0.read_object(offset_of!(HfGame, level))
    }
}

global! {
    /// The game mode (`game.dll+0x33266A0`).
    GameMode
}

impl<'a> GameMode<'a> {
    /// The mode: 1 in a mission.
    pub fn mode(&self) -> Option<u32> {
        self.0.read_u32(offset_of!(HfGameMode, mode))
    }

    /// Whether the game is in a mission.
    pub fn in_mission(&self) -> Option<bool> {
        self.mode().map(|mode| mode == 1)
    }

    /// The mission's data, or `None` when there is no mission.
    pub fn mission(&self) -> Option<Mission<'a>> {
        let address = self.0.read_pointer(offset_of!(HfGameMode, mission))?;

        Some(Mission(Memory::new(self.0.api, address)))
    }
}

/// The mission's data, from [`GameMode::mission`].
#[derive(Clone, Copy, Debug)]
pub struct Mission<'a>(Memory<'a>);

impl<'a> Mission<'a> {
    /// Whether this machine hosts the mission.
    ///
    /// Bit 0 of the mission's `+0x14` is set on the host.
    pub fn is_host(&self) -> Option<bool> {
        self.0.read_u32(0x14).map(|flags| flags & 1 != 0)
    }

    /// The game mode's entity.
    pub fn entity(&self) -> Option<u32> {
        self.0.read_u32(0x8)
    }

    /// The game-owned mission data this handle reads.
    pub fn as_ptr(&self) -> *mut c_void {
        self.0.as_ptr()
    }
}

/// A player's lifecycle in [`PlayerManager`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PlayerLifecycle {
    WaitingToRespawn,
    Spawned,
    Other(u32),
}

impl PlayerLifecycle {
    fn from_raw(value: u32) -> Self {
        match value {
            2 => Self::WaitingToRespawn,
            3 => Self::Spawned,
            other => Self::Other(other),
        }
    }
}

global! {
    /// The players of the session (`game.dll+0x3326468`).
    ///
    /// Slots move as players leave: tell players apart by peer id.
    PlayerManager
}

impl<'a> PlayerManager<'a> {
    /// The number of players in the session (at most 4).
    pub fn count(&self) -> Option<u32> {
        self.0
            .read_u32(offset_of!(HfPlayerManager, count))
            .map(|count| count.min(4))
    }

    /// The player in slot `index`, or `None` when the slot is empty.
    pub fn slot(&self, index: u32) -> Option<Player<'a>> {
        (index < self.count()?).then_some(Player {
            manager: self.0,
            index,
        })
    }

    /// The players in the session, in slot order.
    pub fn players(&self) -> impl Iterator<Item = Player<'a>> + use<'a> {
        let manager = *self;

        (0..manager.count().unwrap_or(0)).filter_map(move |index| manager.slot(index))
    }
}

/// One slot of [`PlayerManager`].
#[derive(Clone, Copy, Debug)]
pub struct Player<'a> {
    manager: Memory<'a>,
    index: u32,
}

impl<'a> Player<'a> {
    const PEER: usize = 0x2C8;
    const LIFECYCLE: usize = 0x2E0;
    const ENTITY_POINTERS: usize = 0xE8;
    const NETWORK_ID: usize = 0x3A8;
    const SLOT_STRIDE: usize = 0x38;
    const NETWORK_ID_STRIDE: usize = 0x20;

    fn slot(&self) -> usize {
        Self::SLOT_STRIDE * self.index as usize
    }

    /// The player's peer id, the stable way to tell players apart.
    pub fn peer(&self) -> Option<u64> {
        self.manager.read_u64(Self::PEER + self.slot())
    }

    /// The player's lifecycle.
    pub fn lifecycle(&self) -> Option<PlayerLifecycle> {
        self.manager
            .read_u32(Self::LIFECYCLE + self.slot())
            .map(PlayerLifecycle::from_raw)
    }

    /// The player's player entity.
    pub fn entity(&self) -> Option<u32> {
        let pointer = self
            .manager
            .read_pointer(Self::ENTITY_POINTERS + size_of::<usize>() * self.index as usize)?;

        Memory::new(self.manager.api, pointer).read_u32(0x8)
    }

    /// The player's Helldiver's network id.
    pub fn network_id(&self) -> Option<u32> {
        self.manager
            .read_u32(Self::NETWORK_ID + Self::NETWORK_ID_STRIDE * self.index as usize)
    }
}

global! {
    /// This machine's user (`game.dll+0x347CEF0`).
    LocalUser
}

impl LocalUser<'_> {
    /// This machine's player's peer id, as in [`Player::peer`].
    pub fn peer(&self) -> Option<u64> {
        self.0.read_u64(offset_of!(HfLocalUser, peer))
    }

    /// The game's host's peer id (it changes with a host migration).
    pub fn host_peer(&self) -> Option<u64> {
        self.0.read_u64(offset_of!(HfLocalUser, host_peer))
    }
}

global! {
    /// The entities of this machine's world (`game.dll+0x346BF98`).
    ///
    /// At `+0xF22EC8` a hash map of network ids to slots (8-byte keys, 4-byte
    /// values), at `+0xF32F20` the network objects by slot (24 bytes each,
    /// `+0` the entity id, u32), at `+0xF1AEB0` the map of entity ids, and at
    /// `+0xF32F18` their descriptors. It also holds the component tables,
    /// entity deltas, entity map and short ids.
    EntityManager
}

impl<'a> EntityManager<'a> {
    const NETWORK_OBJECTS: usize = 0xF32F20;
    const NETWORK_OBJECT_SIZE: usize = 24;

    /// Component table 0; table `c`'s pointer is at `+8 c`.
    pub fn component_tables(&self) -> Option<NonNull<c_void>> {
        self.0
            .read_object(offset_of!(HfEntityManager, component_tables))
    }

    /// The entity deltas a weapon's chosen attachments key into.
    pub fn entity_deltas(&self) -> Option<NonNull<c_void>> {
        self.0
            .read_object(offset_of!(HfEntityManager, entity_deltas))
    }

    /// The entity map: 4096 rows, from entity % 4096.
    pub fn entity_map(&self) -> Option<NonNull<c_void>> {
        self.0.read_object(offset_of!(HfEntityManager, entity_map))
    }

    /// The short ids: sorted `{u32 short id, u32 map row}`.
    pub fn short_ids(&self) -> Option<NonNull<c_void>> {
        self.0.read_object(offset_of!(HfEntityManager, short_ids))
    }

    /// The network object in `slot`, whose `slot` comes from the manager's
    /// network-id map.
    pub fn network_object(&self, slot: u32) -> NetworkObject<'a> {
        NetworkObject(
            self.0
                .at(Self::NETWORK_OBJECTS + Self::NETWORK_OBJECT_SIZE * slot as usize),
        )
    }
}

/// One entry of [`EntityManager`]'s network objects.
#[derive(Clone, Copy, Debug)]
pub struct NetworkObject<'a>(Memory<'a>);

impl NetworkObject<'_> {
    /// The entity this network object names.
    pub fn entity(&self) -> Option<u32> {
        self.0.read_u32(0)
    }
}

global! {
    /// Every entity's health (`game.dll+0x3326688`).
    ///
    /// An index's descriptor names its entity; its health record holds the
    /// health and life, and its ext record the maximum health.
    HealthManager
}

impl<'a> HealthManager<'a> {
    const HEALTH_RECORD_SIZE: usize = 0x1B8;
    const HEALTH: usize = 0x14;
    const LIFE: usize = 0x19C;
    const EXT_RECORD_SIZE: usize = 0x1C;
    const MAX_HEALTH: usize = 0x14;

    /// The number of indexes in use.
    pub fn count(&self) -> Option<u32> {
        self.0.read_u32(offset_of!(HfHealthManager, count))
    }

    fn descriptors(&self) -> Option<Memory<'a>> {
        let address = self
            .0
            .read_pointer(offset_of!(HfHealthManager, descriptors))?;

        Some(Memory::new(self.0.api, address))
    }

    fn records(&self) -> Option<Memory<'a>> {
        let address = self.0.read_pointer(offset_of!(HfHealthManager, records))?;

        Some(Memory::new(self.0.api, address))
    }

    fn ext(&self) -> Option<Memory<'a>> {
        let address = self.0.read_pointer(offset_of!(HfHealthManager, ext))?;

        Some(Memory::new(self.0.api, address))
    }

    fn valid(&self, index: u32) -> Option<()> {
        (index < self.count()?).then_some(())
    }

    /// The descriptor of index `index`, which names its entity.
    pub fn descriptor(&self, index: u32) -> Option<HealthDescriptor<'a>> {
        self.valid(index)?;
        let pointer = self
            .descriptors()?
            .read_pointer(size_of::<usize>() * index as usize)?;

        Some(HealthDescriptor(Memory::new(self.0.api, pointer)))
    }

    /// The health of index `index`.
    pub fn health(&self, index: u32) -> Option<i32> {
        self.valid(index)?;

        self.records()?
            .read_i32(Self::HEALTH_RECORD_SIZE * index as usize + Self::HEALTH)
    }

    /// The life of index `index`.
    pub fn life(&self, index: u32) -> Option<HealthLife> {
        self.valid(index)?;

        self.records()?
            .read_u32(Self::HEALTH_RECORD_SIZE * index as usize + Self::LIFE)
            .map(HealthLife::from_raw)
    }

    /// The maximum health of index `index`.
    pub fn max_health(&self, index: u32) -> Option<i32> {
        self.valid(index)?;

        self.ext()?
            .read_i32(Self::EXT_RECORD_SIZE * index as usize + Self::MAX_HEALTH)
    }
}

/// An entity named by a [`HealthManager`] descriptor.
#[derive(Clone, Copy, Debug)]
pub struct HealthDescriptor<'a>(Memory<'a>);

impl HealthDescriptor<'_> {
    /// The entity's resource id.
    pub fn resource(&self) -> Option<u64> {
        self.0.read_u64(0x0)
    }

    /// The entity.
    pub fn entity(&self) -> Option<u32> {
        self.0.read_u32(0x8)
    }

    /// The entity's unit.
    pub fn unit(&self) -> Option<u32> {
        self.0.read_u32(0xC)
    }

    /// The entity's network id.
    pub fn network_id(&self) -> Option<u32> {
        self.0.read_u32(0x10)
    }

    /// Whether this machine owns the entity (bit 0 of `+0x14`).
    pub fn owned_here(&self) -> Option<bool> {
        self.0.read_u32(0x14).map(|flags| flags & 1 != 0)
    }
}

/// The life of an entity in [`HealthManager`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HealthLife {
    Alive,
    Downed,
    Dead,
}

impl HealthLife {
    fn from_raw(value: u32) -> Self {
        match value {
            0 => Self::Alive,
            1 => Self::Downed,
            _ => Self::Dead,
        }
    }
}

global! {
    /// The entities that move (`game.dll+0x3326558`).
    MotionManager
}

impl<'a> MotionManager<'a> {
    const DESCRIPTOR_ENTITY: usize = 0x8;
    const DESCRIPTOR_FLAGS: usize = 0x14;
    const RECORD_SIZE: usize = 0x84;

    /// The number of indexes in use.
    pub fn count(&self) -> Option<u32> {
        self.0.read_u32(offset_of!(HfMotionManager, count))
    }

    fn descriptors(&self) -> Option<Memory<'a>> {
        let address = self
            .0
            .read_pointer(offset_of!(HfMotionManager, descriptors))?;

        Some(Memory::new(self.0.api, address))
    }

    fn records(&self) -> Option<Memory<'a>> {
        let address = self.0.read_pointer(offset_of!(HfMotionManager, records))?;

        Some(Memory::new(self.0.api, address))
    }

    /// The positions `MoveObject` writes, 0xA4 bytes per index (three floats
    /// at the start).
    pub fn positions(&self) -> Option<NonNull<c_void>> {
        self.0.read_object(offset_of!(HfMotionManager, positions))
    }

    /// The positions motion starts from each frame, 0x2C bytes per index
    /// (three floats).
    pub fn sources(&self) -> Option<NonNull<c_void>> {
        self.0.read_object(offset_of!(HfMotionManager, sources))
    }

    fn descriptor(&self, index: u32) -> Option<Memory<'a>> {
        (index < self.count()?).then_some(())?;
        let pointer = self
            .descriptors()?
            .read_pointer(size_of::<usize>() * index as usize)?;

        Some(Memory::new(self.0.api, pointer))
    }

    /// The entity of index `index`.
    pub fn entity(&self, index: u32) -> Option<u32> {
        self.descriptor(index)?.read_u32(Self::DESCRIPTOR_ENTITY)
    }

    /// Whether index `index` is live (bit 0 of its descriptor's `+0x14`).
    pub fn is_live(&self, index: u32) -> Option<bool> {
        self.descriptor(index)?
            .read_u32(Self::DESCRIPTOR_FLAGS)
            .map(|flags| flags & 1 != 0)
    }

    /// The velocity of index `index`, in metres per second.
    pub fn velocity(&self, index: u32) -> Option<[f32; 3]> {
        self.records()?
            .read::<[f32; 3]>(Self::RECORD_SIZE * index as usize)
    }
}

global! {
    /// The explosion requests of this frame (`game.dll+0x346D558`):
    /// `RequestExplosion`'s queue.
    ExplosionQueue
}

impl ExplosionQueue<'_> {
    /// The number of requests queued (256 at most: `RequestExplosion` refuses
    /// more).
    pub fn count(&self) -> Option<u32> {
        self.0.read_u32(offset_of!(HfExplosionQueue, count))
    }
}

global! {
    /// The projectiles in flight (`game.dll+0x347CEA8`): `FireProjectile`'s
    /// system and the projectile pool.
    ProjectileSystem
}

impl ProjectileSystem<'_> {
    /// Whether projectiles run now (in a mission).
    pub fn is_running(&self) -> Option<bool> {
        self.0
            .read_u8(offset_of!(HfProjectileSystem, running))
            .map(|running| running != 0)
    }
}

global! {
    /// This frame's damage and status requests (`game.dll+0x347CF38`), which
    /// the frame's drain applies (`QueueDamage`, `QueueStatusRequest`).
    CombatQueues
}

impl CombatQueues<'_> {
    /// The damage requests queued (4096 at most).
    pub fn damage_count(&self) -> Option<u32> {
        self.0.read_u32(offset_of!(HfCombatQueues, damage_count))
    }

    /// The status requests queued (4096 at most).
    pub fn status_count(&self) -> Option<u32> {
        self.0.read_u32(offset_of!(HfCombatQueues, status_count))
    }
}

global! {
    /// The players' mission stats (`game.dll+0x3326AE0`).
    ///
    /// The header leaves the type incomplete. The documented layout is: at
    /// `+0x37D58` a map of player entities to indexes, and the index's record
    /// at `+0x37D98 + 0x28 index` holds a stat table and 64 source blocks of
    /// `0x148` bytes at `[record + 0x18]`, each a resource and 16 stat entries
    /// (`HF_STAT_*`).
    ScoreManager
}

impl<'a> ScoreManager<'a> {
    const RECORD_BASE: usize = 0x37D98;
    const RECORD_SIZE: usize = 0x28;

    /// The stat record of `index`, whose `index` comes from the manager's map
    /// of player entities to indexes.
    pub fn record(&self, index: u32) -> ScoreRecord<'a> {
        ScoreRecord(
            self.0
                .at(Self::RECORD_BASE + Self::RECORD_SIZE * index as usize),
        )
    }
}

/// One player's stat record in [`ScoreManager`].
#[derive(Clone, Copy, Debug)]
pub struct ScoreRecord<'a>(Memory<'a>);

impl<'a> ScoreRecord<'a> {
    const SOURCES: usize = 0x18;
    const SOURCE_SIZE: usize = 0x148;

    /// The record's 64 source blocks, or `None` before the game fills them.
    pub fn sources(&self) -> Option<ScoreSources<'a>> {
        let address = self.0.read_pointer(Self::SOURCES)?;

        Some(ScoreSources(Memory::new(self.0.api, address)))
    }

    /// The record's data, as the game owns it.
    pub fn as_ptr(&self) -> *mut c_void {
        self.0.as_ptr()
    }
}

/// The source blocks of a [`ScoreRecord`].
#[derive(Clone, Copy, Debug)]
pub struct ScoreSources<'a>(Memory<'a>);

impl<'a> ScoreSources<'a> {
    /// The number of source blocks.
    pub const COUNT: usize = 64;

    /// Source block `index` (fewer than [`COUNT`](Self::COUNT)).
    pub fn get(&self, index: usize) -> Option<ScoreSource<'a>> {
        (index < Self::COUNT).then(|| ScoreSource(self.0.at(ScoreRecord::SOURCE_SIZE * index)))
    }

    /// The blocks' data, as the game owns it.
    pub fn as_ptr(&self) -> *mut c_void {
        self.0.as_ptr()
    }
}

/// One source block of a [`ScoreRecord`]: a resource and 16 stat entries
/// (`HF_STAT_*`).
#[derive(Clone, Copy, Debug)]
pub struct ScoreSource<'a>(Memory<'a>);

impl ScoreSource<'_> {
    /// The block's data, as the game owns it.
    pub fn as_ptr(&self) -> *mut c_void {
        self.0.as_ptr()
    }
}

global! {
    /// What entities hold (`game.dll+0x3326420`).
    WielderManager
}

impl WielderManager<'_> {
    const INSTANCE_SIZE: usize = 0x1D0;

    /// The instances' slots, or `None` before the game makes them.
    pub fn slots(&self) -> Option<NonNull<c_void>> {
        self.0.read_object(offset_of!(HfWielderManager, slots))
    }

    /// Slot 0 (the instance's hand) of `instance`.
    pub fn hand(&self, instance: u32) -> Option<NonNull<c_void>> {
        let slots = self.slots()?;

        NonNull::new(
            (slots.as_ptr() as usize).wrapping_add(Self::INSTANCE_SIZE * instance as usize)
                as *mut c_void,
        )
    }
}

/// A record's inventory selection in [`InventoryManager`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InventorySelection {
    Primary,
    Secondary,
    Support,
    Held,
    Other(u32),
}

impl InventorySelection {
    fn from_raw(value: u32) -> Self {
        match value {
            1 => Self::Primary,
            2 => Self::Secondary,
            3 => Self::Support,
            4 | 5 => Self::Held,
            other => Self::Other(other),
        }
    }
}

global! {
    /// The Helldivers' inventories (`game.dll+0x3326738`).
    InventoryManager
}

impl<'a> InventoryManager<'a> {
    const RECORD_SIZE: usize = 0x30;

    /// The inventory records, or `None` before the game makes them.
    pub fn records(&self) -> Option<NonNull<c_void>> {
        self.0.read_object(offset_of!(HfInventoryManager, records))
    }

    /// Record `index` of an entity, whose `index` comes from the manager's map
    /// of entities to records.
    pub fn record(&self, index: u32) -> Option<InventoryRecord<'a>> {
        let records = self.records()?;

        Some(InventoryRecord(
            Memory::new(self.0.api, records.as_ptr() as usize)
                .at(Self::RECORD_SIZE * index as usize),
        ))
    }
}

/// One inventory record in [`InventoryManager`].
#[derive(Clone, Copy, Debug)]
pub struct InventoryRecord<'a>(Memory<'a>);

impl InventoryRecord<'_> {
    const SELECTION: usize = 0x1C;

    /// The inventory selection.
    pub fn selection(&self) -> Option<InventorySelection> {
        self.0
            .read_u32(Self::SELECTION)
            .map(InventorySelection::from_raw)
    }
}

global! {
    /// The world's destructible objects (`game.dll+0x3326550`).
    Destructibles
}

impl<'a> Destructibles<'a> {
    const DESCRIPTORS: usize = 0x4030;
    const WORLD_INDEXES: usize = 0x6030;

    /// The destructible index's holder, or `None`.
    pub fn index(&self) -> Option<NonNull<c_void>> {
        self.0.read_object(offset_of!(HfDestructibles, index))
    }

    /// The number of destructibles.
    pub fn count(&self) -> Option<u32> {
        self.0.read_u32(offset_of!(HfDestructibles, count))
    }

    fn valid(&self, index: u32) -> Option<()> {
        (index < self.count()?).then_some(())
    }

    /// The descriptor pointer of destructible `index`.
    pub fn descriptor(&self, index: u32) -> Option<NonNull<c_void>> {
        self.valid(index)?;

        self.0
            .read_object(Self::DESCRIPTORS + size_of::<usize>() * index as usize)
    }

    /// The world index of destructible `index`.
    pub fn world_index(&self, index: u32) -> Option<u32> {
        self.valid(index)?;

        self.0
            .read_u32(Self::WORLD_INDEXES + size_of::<u32>() * index as usize)
    }
}

global! {
    /// The destruction state of the world's destructibles
    /// (`game.dll+0x346D528`).
    ///
    /// Records are 12 bytes each at `+0xC5F78`, by world index.
    DestructionRecords
}

impl<'a> DestructionRecords<'a> {
    const RECORDS: usize = 0xC5F78;
    const RECORD_SIZE: usize = 12;

    /// The record map's slots (0x8000 inline at `+0xC5F48`).
    pub fn slots(&self) -> Option<NonNull<c_void>> {
        self.0.read_object(offset_of!(HfDestructionRecords, slots))
    }

    /// The record map's slot count (a power of two; mask it with count - 1).
    pub fn slot_count(&self) -> Option<u32> {
        self.0
            .read_u32(offset_of!(HfDestructionRecords, slot_count))
    }

    /// The record map's empty key.
    pub fn empty_key(&self) -> Option<u32> {
        self.0.read_u32(offset_of!(HfDestructionRecords, empty_key))
    }

    /// The record of `world_index`, which must be one of
    /// [`Destructibles::world_index`]'s values.
    pub fn record(&self, world_index: u32) -> DestructionRecord<'a> {
        DestructionRecord(
            self.0
                .at(Self::RECORDS + Self::RECORD_SIZE * world_index as usize),
        )
    }
}

/// One destructible's state in [`DestructionRecords`].
#[derive(Clone, Copy, Debug)]
pub struct DestructionRecord<'a>(Memory<'a>);

impl DestructionRecord<'_> {
    const UNIT: usize = 0x0;
    const STAGE: usize = 0x7;
    const FLAGS: usize = 0x8;

    /// The destructible's unit.
    pub fn unit(&self) -> Option<u32> {
        self.0.read_u32(Self::UNIT)
    }

    /// The destructible's destruction stage.
    pub fn stage(&self) -> Option<u8> {
        self.0.read_u8(Self::STAGE)
    }

    /// Whether the destructible is registered (bit 0 of `+0x8`).
    pub fn registered(&self) -> Option<bool> {
        self.0.read_u8(Self::FLAGS).map(|flags| flags & 1 != 0)
    }
}

impl HellForgeApi {
    /// The object a global's cell points at now, or `None`.
    ///
    /// This is the equivalent of `hf_global_object_` in the C header: it asks
    /// the loader for the cell and reads the pointer the cell holds.
    fn global_object(&self, name: &str) -> Option<NonNull<c_void>> {
        if !self.supports(16) {
            return None;
        }

        let name = cstring(name)?;
        let global = self.raw_api().global?;
        let cell = unsafe { global(self.mod_handle(), name.as_ptr()) };
        if cell == 0 {
            return None;
        }

        let mut object: *mut c_void = std::ptr::null_mut();
        // Read through a byte slice rather than a pointer-sized `&mut`, so the
        // read matches the C `hf->read(cell, &object, sizeof object)` exactly.
        let bytes = unsafe {
            std::slice::from_raw_parts_mut(
                (&mut object as *mut *mut c_void).cast::<u8>(),
                size_of::<*mut c_void>(),
            )
        };
        if !self.read(cell, bytes) || object.is_null() {
            return None;
        }

        NonNull::new(object)
    }

    /// The Game (`game.dll+0x3326340`), or `None` when the game has not made
    /// it yet.
    pub fn game(&self) -> Option<Game<'_>> {
        Some(Game::new(self, self.global_object("Game")?))
    }

    /// The game mode (`game.dll+0x33266A0`), or `None` when there is none.
    pub fn game_mode(&self) -> Option<GameMode<'_>> {
        Some(GameMode::new(self, self.global_object("GameMode")?))
    }

    /// The players of the session (`game.dll+0x3326468`), or `None`.
    pub fn player_manager(&self) -> Option<PlayerManager<'_>> {
        Some(PlayerManager::new(
            self,
            self.global_object("PlayerManager")?,
        ))
    }

    /// This machine's user (`game.dll+0x347CEF0`), or `None`.
    pub fn local_user(&self) -> Option<LocalUser<'_>> {
        Some(LocalUser::new(self, self.global_object("LocalUser")?))
    }

    /// The entities of this machine's world (`game.dll+0x346BF98`), or `None`.
    pub fn entity_manager(&self) -> Option<EntityManager<'_>> {
        Some(EntityManager::new(
            self,
            self.global_object("EntityManager")?,
        ))
    }

    /// Every entity's health (`game.dll+0x3326688`), or `None`.
    pub fn health_manager(&self) -> Option<HealthManager<'_>> {
        Some(HealthManager::new(
            self,
            self.global_object("HealthManager")?,
        ))
    }

    /// The entities that move (`game.dll+0x3326558`), or `None`.
    pub fn motion_manager(&self) -> Option<MotionManager<'_>> {
        Some(MotionManager::new(
            self,
            self.global_object("MotionManager")?,
        ))
    }

    /// The explosion requests of this frame (`game.dll+0x346D558`), or `None`.
    pub fn explosion_queue(&self) -> Option<ExplosionQueue<'_>> {
        Some(ExplosionQueue::new(
            self,
            self.global_object("ExplosionQueue")?,
        ))
    }

    /// The projectiles in flight (`game.dll+0x347CEA8`), or `None`.
    pub fn projectile_system(&self) -> Option<ProjectileSystem<'_>> {
        Some(ProjectileSystem::new(
            self,
            self.global_object("ProjectileSystem")?,
        ))
    }

    /// This frame's damage and status requests (`game.dll+0x347CF38`), or
    /// `None`.
    pub fn combat_queues(&self) -> Option<CombatQueues<'_>> {
        Some(CombatQueues::new(self, self.global_object("CombatQueues")?))
    }

    /// The players' mission stats (`game.dll+0x3326AE0`), or `None`.
    pub fn score_manager(&self) -> Option<ScoreManager<'_>> {
        Some(ScoreManager::new(self, self.global_object("ScoreManager")?))
    }

    /// What entities hold (`game.dll+0x3326420`), or `None`.
    pub fn wielder_manager(&self) -> Option<WielderManager<'_>> {
        Some(WielderManager::new(
            self,
            self.global_object("WielderManager")?,
        ))
    }

    /// The Helldivers' inventories (`game.dll+0x3326738`), or `None`.
    pub fn inventory_manager(&self) -> Option<InventoryManager<'_>> {
        Some(InventoryManager::new(
            self,
            self.global_object("InventoryManager")?,
        ))
    }

    /// The world's destructible objects (`game.dll+0x3326550`), or `None`.
    pub fn destructibles(&self) -> Option<Destructibles<'_>> {
        Some(Destructibles::new(
            self,
            self.global_object("Destructibles")?,
        ))
    }

    /// The destruction state of the world's destructibles
    /// (`game.dll+0x346D528`), or `None`.
    pub fn destruction_records(&self) -> Option<DestructionRecords<'_>> {
        Some(DestructionRecords::new(
            self,
            self.global_object("DestructionRecords")?,
        ))
    }
}

global! {
    /// The `DepositManager` global (`game.dll+0x33265E8`), or `None`.
    ///
    /// Entities that hand out supplies (a Supply Pack). At +0x20 a map of entities to deposits; at
    /// +0x60 a map of entities to definition overrides
    DepositManager
}

impl<'a> DepositManager<'a> {
    /// `+0x50` 8 bytes per deposit: +0 the supplies left (u32)
    pub fn amounts(&self) -> Option<NonNull<c_void>> {
        self.0.read_object(offset_of!(HfDepositManager, amounts))
    }

    /// `+0xA0` the overriding definitions, 0x98 bytes each (+0x24 the self-use ability: 2629 for
    /// the B-1 Supply Pack)
    pub fn definitions(&self) -> Option<NonNull<c_void>> {
        self.0
            .read_object(offset_of!(HfDepositManager, definitions))
    }
}

global! {
    /// The `AvatarManager` global (`game.dll+0x3326D20`), or `None`.
    ///
    /// The Helldivers' avatars (the AvatarComponent manager, component world +0x956128): their
    /// movement, stamina and actions. At +0xF8 a map of entities to indexes; descriptor pointers at
    /// +0x110 + 8 index (+8 the entity); the avatars simulated here take the first indexes (their
    /// count at +0x70); each index's simulation block at +0x53E170 + 0x1238 index begins with its
    /// action context (+0 the entity, +8 the running action, TryStartAction's first argument), its
    /// state flags at +0x53E880 and +0x53E888 (u64 each) of the same stride; AvatarSettings'
    /// private copies inline from +0x547D24 (0x354 bytes each)
    AvatarManager
}

impl<'a> AvatarManager<'a> {
    /// `+0x70` avatars simulated on this machine: they take the first indexes, and AvatarUpdate
    /// runs only those
    pub fn simulated(&self) -> Option<u32> {
        self.0.read(offset_of!(HfAvatarManager, simulated))
    }
}

global! {
    /// The `ActionManager` global, or `None`.
    ///
    /// ActionManager: AvatarManager's former name, kept for mods written with it; use AvatarManager
    ActionManager
}

impl<'a> ActionManager<'a> {
    /// `+0x70` avatars simulated on this machine: they take the first indexes, and AvatarUpdate
    /// runs only those
    pub fn simulated(&self) -> Option<u32> {
        self.0.read(offset_of!(HfAvatarManager, simulated))
    }
}

global! {
    /// The `WieldableManager` global (`game.dll+0x3326730`), or `None`.
    ///
    /// Wieldable items (weapons). At +0x18 a map of entities to indexes
    WieldableManager
}

impl<'a> WieldableManager<'a> {
    /// `+0x38` the wielder of each index (u32 entity each)
    pub fn wielders(&self) -> Option<NonNull<c_void>> {
        self.0.read_object(offset_of!(HfWieldableManager, wielders))
    }
}

global! {
    /// The `FactionManager` global (`game.dll+0x3326CB8`), or `None`.
    ///
    /// The entities' factions. At +0x12050 a map of entities to indexes
    FactionManager
}

impl<'a> FactionManager<'a> {
    /// `+0x12078` the faction mask of each index (u32 each; bit 0 the Helldivers' side)
    pub fn masks(&self) -> Option<NonNull<c_void>> {
        self.0.read_object(offset_of!(HfFactionManager, masks))
    }
}

global! {
    /// The `BehaviourManager` global (`game.dll+0x3326740`), or `None`.
    ///
    /// The entities' behaviours. At +0x40 a map of entities to indexes
    BehaviourManager
}

impl<'a> BehaviourManager<'a> {
    /// `+0x58` a pointer per index: +8 the entity (u32), +0x10 its network handle (0x7FFF: none)
    pub fn handles(&self) -> Option<NonNull<c_void>> {
        self.0.read_object(offset_of!(HfBehaviourManager, handles))
    }

    /// `+0x60` the behaviour records, 0x1F8 bytes each; P (record + 8) is a behaviour's state, P+0
    /// its stage
    pub fn records(&self) -> Option<NonNull<c_void>> {
        self.0.read_object(offset_of!(HfBehaviourManager, records))
    }
}

global! {
    /// The `TagManager` global (`game.dll+0x3326430`), or `None`.
    ///
    /// The entities' tag words. At +0x18 a map of entities to indexes
    TagManager
}

impl<'a> TagManager<'a> {
    /// `+0x38` the tag word of each index (u64 each): 0x10000000 its kills credited to no one, bit
    /// 35 never targeted
    pub fn tags(&self) -> Option<NonNull<c_void>> {
        self.0.read_object(offset_of!(HfTagManager, tags))
    }
}

global! {
    /// The `Clock` global (`game.dll+0x3326348`), or `None`.
    ///
    /// The game's clock, which behaviours time themselves by
    Clock
}

impl<'a> Clock<'a> {
    /// `+0x18` now, in microseconds
    pub fn now(&self) -> Option<u64> {
        self.0.read(offset_of!(HfClock, now))
    }
}

global! {
    /// The `EngineApi` global (`game.dll+0x3326308`), or `None`.
    ///
    /// The engine's C API (helldivers2.exe's), which the game calls into: one pointer per domain
    /// (input, world, application, unit, camera, material, ...), each to that domain's table of
    /// functions
    EngineApi
}

impl<'a> EngineApi<'a> {
    /// `+0x10` the Application functions (helldivers2.exe+0x27C9E60): +0x3E8 the text of an id,
    /// const char* (u32 id), which HellForge points at its own to answer mod texts; +0x378 the
    /// resource manager's lock (ResourceOverride)
    pub fn application(&self) -> Option<NonNull<c_void>> {
        self.0.read_object(offset_of!(HfEngineApi, application))
    }

    /// `+0x18` the Unit functions (helldivers2.exe+0x27C8F10): +0x88 a node's world position (unit,
    /// node), +0x748 the unit object for a handle (jumps to UnitFromHandle)
    pub fn unit(&self) -> Option<NonNull<c_void>> {
        self.0.read_object(offset_of!(HfEngineApi, unit))
    }
}

global! {
    /// The `EngineUnitApi` global (`game.dll+0x3326310`), or `None`.
    ///
    /// An engine API table in helldivers2.exe (helldivers2.exe+0x27CD4C0 on this build)
    EngineUnitApi
}

impl<'a> EngineUnitApi<'a> {
    /// `+0x10` UnitActorByName (helldivers2.exe 0x799DE0): (unit, actor name hash, NULL) -> the
    /// actor, 0xFFFFFFFF none
    pub fn actor_by_name(&self) -> Option<NonNull<c_void>> {
        self.0
            .read_object(offset_of!(HfEngineUnitApi, actor_by_name))
    }
}

global! {
    /// The `WwiseApi` global (`game.dll+0x3326318`), or `None`.
    ///
    /// The Wwise plugin's API table; its functions take the WwiseWorld (hf_global_Game's
    /// wwise_world)
    WwiseApi
}

impl<'a> WwiseApi<'a> {
    /// `+0x338` posts a sound: (world, event, game object, flags)
    pub fn post(&self) -> Option<NonNull<c_void>> {
        self.0.read_object(offset_of!(HfWwiseApi, post))
    }

    /// `+0x590` the source on a unit's node: (world, unit, node)
    pub fn unit_source(&self) -> Option<NonNull<c_void>> {
        self.0.read_object(offset_of!(HfWwiseApi, unit_source))
    }

    /// `+0x5A0` a source's game object: (world, source), -1 when the source is gone
    pub fn source_object(&self) -> Option<NonNull<c_void>> {
        self.0.read_object(offset_of!(HfWwiseApi, source_object))
    }
}

global! {
    /// The `AbilityManager` global (`game.dll+0x3326640`), or `None`.
    ///
    /// The abilities running on entities: StartAbility's first argument. At +0x18 a map of entities
    /// to indexes; it ticks each ability at 60 a second (AbilityTick)
    AbilityManager
}

global! {
    /// The `WeaponLinkerManager` global (`game.dll+0x33267F8`), or `None`.
    ///
    /// WeaponLinker instances (a silo's targeting remote linked to its missile and rack). At +0x20
    /// a map of entities to indexes; its copies of the records of instances this machine has
    /// authority over (12 bytes each) are compacted when one goes
    WeaponLinkerManager
}

impl<'a> WeaponLinkerManager<'a> {
    /// `+0x38` a descriptor pointer per index (LinkerRecord's argument)
    pub fn descriptors(&self) -> Option<NonNull<c_void>> {
        self.0
            .read_object(offset_of!(HfWeaponLinkerManager, descriptors))
    }

    /// `+0x90` record copies in use
    pub fn copy_count(&self) -> Option<u32> {
        self.0.read(offset_of!(HfWeaponLinkerManager, copy_count))
    }

    /// `+0x98` the record copies, 12 bytes each
    pub fn copies(&self) -> Option<NonNull<c_void>> {
        self.0
            .read_object(offset_of!(HfWeaponLinkerManager, copies))
    }
}

global! {
    /// The `SeekingMissileManager` global (`game.dll+0x3326DF0`), or `None`.
    ///
    /// Seeking missiles (a silo's). At +0x20 a map of entities to indexes
    SeekingMissileManager
}

impl<'a> SeekingMissileManager<'a> {
    /// `+0x48` 0xC0 bytes per index: +0x48 the missile's position (3 floats)
    pub fn rows(&self) -> Option<NonNull<c_void>> {
        self.0
            .read_object(offset_of!(HfSeekingMissileManager, rows))
    }

    /// `+0x50` 0x38 bytes per index: +8 its remote's network handle, +0xC its guidance target,
    /// +0x35 engine byte, +0x36 launched byte
    pub fn instances(&self) -> Option<NonNull<c_void>> {
        self.0
            .read_object(offset_of!(HfSeekingMissileManager, instances))
    }
}

global! {
    /// The `LaserDesignatorManager` global (`game.dll+0x3326960`), or `None`.
    ///
    /// Laser designators (a silo remote's laser). At +0x30 a map of entities to indexes
    LaserDesignatorManager
}

impl<'a> LaserDesignatorManager<'a> {
    /// `+0x58` 0x38 bytes per index: +0x2C the marked point (3 floats)
    pub fn instances(&self) -> Option<NonNull<c_void>> {
        self.0
            .read_object(offset_of!(HfLaserDesignatorManager, instances))
    }

    /// `+0x60` a byte per index: the laser on
    pub fn on(&self) -> Option<NonNull<c_void>> {
        self.0.read_object(offset_of!(HfLaserDesignatorManager, on))
    }
}

global! {
    /// The `GuidanceTargets` global (`game.dll+0x3326BD8`), or `None`.
    ///
    /// Guided entities' targets (SetGuidanceTarget). At +0x840 a map of entities to indexes
    GuidanceTargets
}

global! {
    /// The `ExplosiveManager` global (`game.dll+0x3326728`), or `None`.
    ///
    /// Explosives (SetExplosiveOwner's first argument). At +0x38 a map of entities to indexes
    ExplosiveManager
}

global! {
    /// The `EquipmentManager` global (`game.dll+0x3326DC0`), or `None`.
    ///
    /// The Equipment component manager (component 233, component world +0xE9F8F0; how held items
    /// are wielded, holstered and dropped): EquipmentUpdate's first argument; EquipmentRecord looks
    /// in its private copies. At +0x20 a map of entities to indexes; a silo's targeting remote's
    /// owner is at +4 of its instance record
    EquipmentManager
}

impl<'a> EquipmentManager<'a> {
    /// `+0x40` 48 bytes per index: +4 the owner entity
    pub fn records(&self) -> Option<NonNull<c_void>> {
        self.0.read_object(offset_of!(HfEquipmentManager, records))
    }
}

global! {
    /// The `RemoteOwners` global, or `None`.
    ///
    /// RemoteOwners: EquipmentManager's former name, kept for mods written with it; use
    /// EquipmentManager
    RemoteOwners
}

impl<'a> RemoteOwners<'a> {
    /// `+0x40` 48 bytes per index: +4 the owner entity
    pub fn records(&self) -> Option<NonNull<c_void>> {
        self.0.read_object(offset_of!(HfEquipmentManager, records))
    }
}

global! {
    /// The `HellpodRackManager` global (`game.dll+0x3326A58`), or `None`.
    ///
    /// Hellpod racks (RackReleaseItem's first argument). At +0x28 a map of entities to indexes
    HellpodRackManager
}

global! {
    /// The `AttachableManager` global (`game.dll+0x3326698`), or `None`.
    ///
    /// Attachables (what sits in someone's hands). At +0x18 a map of entities to indexes; records
    /// of 0xAC bytes (+0 the holder's link)
    AttachableManager
}

global! {
    /// The `MountManager` global (`game.dll+0x3326438`), or `None`.
    ///
    /// Mounts (a Pelican's turret). At +0x20 a map of entities to indexes
    MountManager
}

impl<'a> MountManager<'a> {
    /// `+0x48` 6 u32 per index: the mounted children 0 to 4 first
    pub fn records(&self) -> Option<NonNull<c_void>> {
        self.0.read_object(offset_of!(HfMountManager, records))
    }
}

global! {
    /// The `DropAnchors` global (`game.dll+0x3326658`), or `None`.
    ///
    /// Drop positions (a transport Pelican's beacon). At +0x30 a map of entities to indexes
    DropAnchors
}

impl<'a> DropAnchors<'a> {
    /// `+0x60` 0x20 bytes per index: +0 the anchor
    pub fn records(&self) -> Option<NonNull<c_void>> {
        self.0.read_object(offset_of!(HfDropAnchors, records))
    }
}

global! {
    /// The `PerceptionManager` global (`game.dll+0x3326548`), or `None`.
    ///
    /// Perceivers and what they sense (FindPerceptionEntry's first argument). At +0x30 a map of
    /// entities to indexes
    PerceptionManager
}

impl<'a> PerceptionManager<'a> {
    /// `+0x50` the perceiver records, 0x13F8 bytes each
    pub fn records(&self) -> Option<NonNull<c_void>> {
        self.0.read_object(offset_of!(HfPerceptionManager, records))
    }
}

global! {
    /// The `WeaponManager` global (`game.dll+0x3326660`), or `None`.
    ///
    /// The weapon manager (component world +0xE9FC38): WeaponUpdate's, OwnSpareAmmo's, SpendRound's
    /// and MagazineRounds' first argument
    WeaponManager
}

global! {
    /// The `MagazineManager` global, or `None`.
    ///
    /// MagazineManager: WeaponManager's former name, kept for mods written with it; use
    /// WeaponManager
    MagazineManager
}

global! {
    /// The `WeaponMagazines` global (`game.dll+0x3326648`), or `None`.
    ///
    /// The WeaponMagazine component manager (component 5, component world +0x180): a magazine
    /// weapon's spare magazines and rounds. AddMagazineRounds refills it; WeaponMagazineRecord
    /// looks in its private copies
    WeaponMagazines
}

impl<'a> WeaponMagazines<'a> {
    /// `+0x20` the hash of the entities that have an instance (8-byte rows: entity, instance index)
    pub fn hash(&self) -> Option<NonNull<c_void>> {
        self.0.read_object(offset_of!(HfWeaponMagazines, hash))
    }

    /// `+0x38` each instance's entity descriptor (8 bytes each: +0 type, +8 entity)
    pub fn descriptors(&self) -> Option<NonNull<c_void>> {
        self.0
            .read_object(offset_of!(HfWeaponMagazines, descriptors))
    }

    /// `+0x50` each instance's live row, 12 bytes: +0 spare magazines (network field 0xEC64918B),
    /// +4 rounds in the magazine (MagazineRounds returns it)
    pub fn live(&self) -> Option<NonNull<c_void>> {
        self.0.read_object(offset_of!(HfWeaponMagazines, live))
    }
}

global! {
    /// The `MagazineRefills` global, or `None`.
    ///
    /// MagazineRefills: WeaponMagazines's former name, kept for mods written with it; use
    /// WeaponMagazines
    MagazineRefills
}

impl<'a> MagazineRefills<'a> {
    /// `+0x20` the hash of the entities that have an instance (8-byte rows: entity, instance index)
    pub fn hash(&self) -> Option<NonNull<c_void>> {
        self.0.read_object(offset_of!(HfWeaponMagazines, hash))
    }

    /// `+0x38` each instance's entity descriptor (8 bytes each: +0 type, +8 entity)
    pub fn descriptors(&self) -> Option<NonNull<c_void>> {
        self.0
            .read_object(offset_of!(HfWeaponMagazines, descriptors))
    }

    /// `+0x50` each instance's live row, 12 bytes: +0 spare magazines (network field 0xEC64918B),
    /// +4 rounds in the magazine (MagazineRounds returns it)
    pub fn live(&self) -> Option<NonNull<c_void>> {
        self.0.read_object(offset_of!(HfWeaponMagazines, live))
    }
}

global! {
    /// The `WeaponTargeting` global (`game.dll+0x3326D30`), or `None`.
    ///
    /// Weapons' targeting. At +0x150 a map of entities to indexes
    WeaponTargeting
}

impl<'a> WeaponTargeting<'a> {
    /// `+0x178` the targeting records, 0xD0 bytes each
    pub fn records(&self) -> Option<NonNull<c_void>> {
        self.0.read_object(offset_of!(HfWeaponTargeting, records))
    }

    /// `+0x180` their network blocks, 0x18 bytes each
    pub fn network(&self) -> Option<NonNull<c_void>> {
        self.0.read_object(offset_of!(HfWeaponTargeting, network))
    }
}

global! {
    /// The `ShieldManager` global (`game.dll+0x3326630`), or `None`.
    ///
    /// The shields standing (relay bubbles, shield packs): a map of 8-byte entries {key, index}
    /// (index 0xFFFFFFFF: none) and, per index, a binding, a row and a live state
    ShieldManager
}

impl<'a> ShieldManager<'a> {
    /// `+0x30` the map's entries, 8 bytes each
    pub fn map(&self) -> Option<NonNull<c_void>> {
        self.0.read_object(offset_of!(HfShieldManager, map))
    }

    /// `+0x38` the map's entries
    pub fn capacity(&self) -> Option<u32> {
        self.0.read(offset_of!(HfShieldManager, capacity))
    }

    /// `+0x3C` the key of an empty entry
    pub fn empty(&self) -> Option<u32> {
        self.0.read(offset_of!(HfShieldManager, empty))
    }

    /// `+0x48` a binding pointer per index (ShieldRecord's argument)
    pub fn bindings(&self) -> Option<NonNull<c_void>> {
        self.0.read_object(offset_of!(HfShieldManager, bindings))
    }

    /// `+0x50` 0x88 bytes per index: +0 the physics handle
    pub fn rows(&self) -> Option<NonNull<c_void>> {
        self.0.read_object(offset_of!(HfShieldManager, rows))
    }

    /// `+0x58` 0x14 bytes per index: +2 broken (u8), +4 health
    pub fn live(&self) -> Option<NonNull<c_void>> {
        self.0.read_object(offset_of!(HfShieldManager, live))
    }
}

global! {
    /// The `CustomizationManager` global (`game.dll+0x33264F8`), or `None`.
    ///
    /// The players' armor and its passives. At +0x30 the passive ids' slots (u32 each, 42 ids); at
    /// +0x930 a map of entities to records, their descriptor pointers at +0x948; per record an
    /// applied block at +0x96C + 0x44 record (+0x38 the helmet passive, +0x3C the armor passive).
    /// Kit records: the kit id at +0 (u32), its type at +0x28 (u32: 0 armour, 1 helmet, 2 cape)
    CustomizationManager
}

impl<'a> CustomizationManager<'a> {
    /// `+0x0` the armor kits (a kit row's +0x1C its passive)
    pub fn kits(&self) -> Option<NonNull<c_void>> {
        self.0.read_object(offset_of!(HfCustomizationManager, kits))
    }

    /// `+0x8` the kits
    pub fn kit_count(&self) -> Option<u32> {
        self.0.read(offset_of!(HfCustomizationManager, kit_count))
    }

    /// `+0x20` pointers to the passive rows (0x34 bytes each), by slot
    pub fn passives(&self) -> Option<NonNull<c_void>> {
        self.0
            .read_object(offset_of!(HfCustomizationManager, passives))
    }

    /// `+0x28` the passive rows (32)
    pub fn passive_count(&self) -> Option<u32> {
        self.0
            .read(offset_of!(HfCustomizationManager, passive_count))
    }

    /// `+0x8E8` records in use (at most 4, one a player)
    pub fn record_count(&self) -> Option<u32> {
        self.0
            .read(offset_of!(HfCustomizationManager, record_count))
    }
}

global! {
    /// The `KitList` global, or `None`.
    ///
    /// KitList: CustomizationManager's former name, kept for mods written with it; use
    /// CustomizationManager
    KitList
}

impl<'a> KitList<'a> {
    /// `+0x0` the armor kits (a kit row's +0x1C its passive)
    pub fn kits(&self) -> Option<NonNull<c_void>> {
        self.0.read_object(offset_of!(HfCustomizationManager, kits))
    }

    /// `+0x8` the kits
    pub fn kit_count(&self) -> Option<u32> {
        self.0.read(offset_of!(HfCustomizationManager, kit_count))
    }

    /// `+0x20` pointers to the passive rows (0x34 bytes each), by slot
    pub fn passives(&self) -> Option<NonNull<c_void>> {
        self.0
            .read_object(offset_of!(HfCustomizationManager, passives))
    }

    /// `+0x28` the passive rows (32)
    pub fn passive_count(&self) -> Option<u32> {
        self.0
            .read(offset_of!(HfCustomizationManager, passive_count))
    }

    /// `+0x8E8` records in use (at most 4, one a player)
    pub fn record_count(&self) -> Option<u32> {
        self.0
            .read(offset_of!(HfCustomizationManager, record_count))
    }
}

global! {
    /// The `BombardmentManager` global (`game.dll+0x3326CE8`), or `None`.
    ///
    /// Bombardments (an orbital barrage). At +0x30 a map of entities to indexes, filled within the
    /// spawn
    BombardmentManager
}

impl<'a> BombardmentManager<'a> {
    /// `+0x60` 0x1C bytes per index: +0 shells a salvo, +4 salvos, +8 the target, +0x14 heading,
    /// +0x18 seed
    pub fn blocks(&self) -> Option<NonNull<c_void>> {
        self.0.read_object(offset_of!(HfBombardmentManager, blocks))
    }
}

global! {
    /// The `Progression` global (`game.dll+0x347CEF8`), or `None`.
    ///
    /// The progression manager: this machine's player's account (catalogue, warbonds, purchases).
    /// Its purchased warbond items from +0xD8A5C (u32 ids). HellForge moves its warbond list and
    /// catalogue elsewhere when mods add warbonds, so do not read those from it. The player's
    /// progression: the item catalogue and inventory in one table (a part of the game state, which
    /// HellForge enlarges: its catalogue moves past the state's old end, state_moves.c). As the
    /// game has it: a count at +0x1CE0, 0xB8-byte records from +0x1CE4 (+4 mixId, +8 itemId, +0xC
    /// category, +0xB0 amount)
    Progression
}

impl<'a> Progression<'a> {
    /// `+0xD8A58` warbond items purchased (5280 at most), their ids following
    pub fn bought_count(&self) -> Option<u32> {
        self.0.read(offset_of!(HfProgression, bought_count))
    }

    /// `+0xDDCF8` the login's state: 12 once logged in
    pub fn login(&self) -> Option<u32> {
        self.0.read(offset_of!(HfProgression, login))
    }

    /// `+0x17799D8` nonzero while a warbond buy waits for its reply
    pub fn pending_buy(&self) -> Option<u8> {
        self.0.read(offset_of!(HfProgression, pending_buy))
    }

    /// `+0x17799DC` the pending buy's seasonPassItemId
    pub fn pending_item(&self) -> Option<u32> {
        self.0.read(offset_of!(HfProgression, pending_item))
    }
}

global! {
    /// The `ProgressionManager` global, or `None`.
    ///
    /// ProgressionManager: Progression's former name, kept for mods written with it; use
    /// Progression
    ProgressionManager
}

impl<'a> ProgressionManager<'a> {
    /// `+0xD8A58` warbond items purchased (5280 at most), their ids following
    pub fn bought_count(&self) -> Option<u32> {
        self.0.read(offset_of!(HfProgression, bought_count))
    }

    /// `+0xDDCF8` the login's state: 12 once logged in
    pub fn login(&self) -> Option<u32> {
        self.0.read(offset_of!(HfProgression, login))
    }

    /// `+0x17799D8` nonzero while a warbond buy waits for its reply
    pub fn pending_buy(&self) -> Option<u8> {
        self.0.read(offset_of!(HfProgression, pending_buy))
    }

    /// `+0x17799DC` the pending buy's seasonPassItemId
    pub fn pending_item(&self) -> Option<u32> {
        self.0.read(offset_of!(HfProgression, pending_item))
    }
}

global! {
    /// The `PlayerHistory` global (`game.dll+0x347CE50`), or `None`.
    ///
    /// The players' loadouts. Records of 0x1690 bytes from +0: the peer id at +0 (u64), the loadout
    /// at +0x38 (stratagem slots of 0x30 bytes at +0x188 with the type at +0, their count at
    /// +0x788), its loaded copy at +0xA28 (load type +0x9E4: 2 for a mission). Loadouts of players
    /// who left at +0x2FD10 (0x9F0 each); the shared stratagems at +0x45410 (64 of 0x6C: {type,
    /// choice, state})
    PlayerHistory
}

impl<'a> PlayerHistory<'a> {
    /// `+0x2D200` records in use
    pub fn count(&self) -> Option<u32> {
        self.0.read(offset_of!(HfPlayerHistory, count))
    }

    /// `+0x2FD0C` loadouts kept for players who left
    pub fn left_count(&self) -> Option<u32> {
        self.0.read(offset_of!(HfPlayerHistory, left_count))
    }
}

global! {
    /// The `Profile` global (`game.dll+0x347CDD8`), or `None`.
    ///
    /// This machine's player's saved loadout. At +0x4C 32 {stratagem stable id, uses} pairs (u32
    /// each)
    Profile
}

impl<'a> Profile<'a> {
    /// `+0x40` the primary weapon's loadout id
    pub fn primary(&self) -> Option<u32> {
        self.0.read(offset_of!(HfProfile, primary))
    }

    /// `+0x44` the secondary weapon's loadout id
    pub fn secondary(&self) -> Option<u32> {
        self.0.read(offset_of!(HfProfile, secondary))
    }

    /// `+0x48` the throwable's loadout id
    pub fn throwable(&self) -> Option<u32> {
        self.0.read(offset_of!(HfProfile, throwable))
    }
}

global! {
    /// The `MenuStates` global (`game.dll+0x347CE28`), or `None`.
    ///
    /// The UI state machine. Its state stack: 5 u32 from +0x429C, the top at +0x429C + 4 (depth -
    /// 1) (5 the Armory, 14 the mission briefing's loadout). Its presenter (+0x4294) is 0 while no
    /// menu is open
    MenuStates
}

impl<'a> MenuStates<'a> {
    /// `+0x4294` the menu presenter open (14 the loadout screen)
    pub fn presenter(&self) -> Option<u32> {
        self.0.read(offset_of!(HfMenuStates, presenter))
    }

    /// `+0x42B0` states on the stack (1 to 5)
    pub fn depth(&self) -> Option<u32> {
        self.0.read(offset_of!(HfMenuStates, depth))
    }
}

global! {
    /// The `UiManager` global, or `None`.
    ///
    /// UiManager: MenuStates's former name, kept for mods written with it; use MenuStates
    UiManager
}

impl<'a> UiManager<'a> {
    /// `+0x4294` the menu presenter open (14 the loadout screen)
    pub fn presenter(&self) -> Option<u32> {
        self.0.read(offset_of!(HfMenuStates, presenter))
    }

    /// `+0x42B0` states on the stack (1 to 5)
    pub fn depth(&self) -> Option<u32> {
        self.0.read(offset_of!(HfMenuStates, depth))
    }
}

global! {
    /// The `MenuStack` global (`game.dll+0x347CE38`), or `None`.
    ///
    /// The menu stack
    MenuStack
}

impl<'a> MenuStack<'a> {
    /// `+0x0` the menu on top (11 the loadout screen)
    pub fn top(&self) -> Option<u32> {
        self.0.read(offset_of!(HfMenuStack, top))
    }
}

global! {
    /// The `Armory` global (`game.dll+0x33269E8`), or `None`.
    ///
    /// The armory's list (BuildArmoryList's, or HellForge's larger copy when mods add weapons):
    /// 32-byte entries, the loadout id at +0 and the entity at +8 (u64). Type t's entries start at
    /// entry [+0x1B24 + 4 t] and number [+0x1B40 + 4 t] (u32; 7 types, 1 primary, 2 support, 3
    /// secondary)
    Armory
}

impl<'a> Armory<'a> {
    /// `+0x1B24` type 0's first entry; type t's at +4 t
    pub fn starts(&self) -> Option<u32> {
        self.0.read(offset_of!(HfArmory, starts))
    }

    /// `+0x1B40` type 0's entries; type t's at +4 t
    pub fn counts(&self) -> Option<u32> {
        self.0.read(offset_of!(HfArmory, counts))
    }
}

global! {
    /// The `ThumbnailManager` global (`game.dll+0x347CD80`), or `None`.
    ///
    /// The Armory's thumbnail manager: six cards of up to 15 icon requests rendered into one atlas.
    /// Card c's state at +1832 + 1816 c (u32, 8 complete), its request count at +1836 + 1816 c, its
    /// requests of 120 bytes from +32 + 1816 c (+24 the item's id, +100 its kind 0 weapon to 4
    /// cape, +104 its stage: 1 waiting, 2 submitted, 3 unit available, 4 done)
    ThumbnailManager
}

impl<'a> ThumbnailManager<'a> {
    /// `+0x2B38` the card being generated (0xFFFFFFFF none)
    pub fn active_card(&self) -> Option<u32> {
        self.0.read(offset_of!(HfThumbnailManager, active_card))
    }

    /// `+0x2B58` the active card's phase
    pub fn phase(&self) -> Option<u32> {
        self.0.read(offset_of!(HfThumbnailManager, phase))
    }
}

global! {
    /// The `PackageSystem` global (`game.dll+0x347CEB0`), or `None`.
    ///
    /// The RefcountedPackageSystem: RequestPackages' system. Its reference map has 16-byte entries
    /// keyed by package id (u64, 0 empty) and no overflow path
    PackageSystem
}

impl<'a> PackageSystem<'a> {
    /// `+0x0` the reference map's entries
    pub fn map(&self) -> Option<NonNull<c_void>> {
        self.0.read_object(offset_of!(HfPackageSystem, map))
    }

    /// `+0x8` the map's entries
    pub fn capacity(&self) -> Option<u32> {
        self.0.read(offset_of!(HfPackageSystem, capacity))
    }
}

global! {
    /// The `GameConfig` global (`game.dll+0x347CDF8`), or `None`.
    ///
    /// The game's config settings: FindConfigSetting's config. Two open-addressing tables of
    /// 48-byte entries, the online overrides at +0x12078 and the defaults at +0xC050 (each
    /// {entries, capacity +8, empty key +0xC, multiplier +0x10}; an entry's type at +0x14, its
    /// value at +0x18)
    GameConfig
}

global! {
    /// The `NavigationSystem` global (`game.dll+0x33263C8`), or `None`.
    ///
    /// The engine's navigation interface, a table of functions: its +8 sets up a nav world's
    /// navmesh grid (ecx the nav world's id), called for each mission's level (setup_navmeshgrid,
    /// pass 0x68) and for the ship
    NavigationSystem
}

global! {
    /// The `PhysicsPlugin` global (`game.dll+0x3326380`), or `None`.
    ///
    /// Set by the physics plugin setup's first call (SetupPhysicsConfig, once per process): 0 until
    /// the engine's physics pools are set up
    PhysicsPlugin
}

global! {
    /// The `ObjectiveManager` global (`game.dll+0x3326DA0`), or `None`.
    ///
    /// The mission's objective instances, which the HUD's objective updater (UpdateHudObjectives)
    /// numbers
    ObjectiveManager
}

impl<'a> ObjectiveManager<'a> {
    /// `+0x24` objective instances in the mission
    pub fn instance_count(&self) -> Option<u32> {
        self.0.read(offset_of!(HfObjectiveManager, instance_count))
    }
}

global! {
    /// The `HiveMindManager` global (`game.dll+0x3326D10`), or `None`.
    ///
    /// The mission's HiveMind: the enemy director and the mission's countdowns. The mission's
    /// countdowns. With one timer, its slot at +0x51948: +4 a byte set once it has run out, +6 its
    /// pause byte (1 paused), which the countdown's update honours; hf->freeze_mission_timer writes
    /// it, host only
    HiveMindManager
}

impl<'a> HiveMindManager<'a> {
    /// `+0x650` the mission's level (lg): its difficulty level at +0xF9 (u8)
    pub fn level(&self) -> Option<NonNull<c_void>> {
        self.0.read_object(offset_of!(HfHiveMindManager, level))
    }

    /// `+0x67C` the mission's timers (hf->freeze_mission_timer needs 1)
    pub fn timer_count(&self) -> Option<u32> {
        self.0.read(offset_of!(HfHiveMindManager, timer_count))
    }

    /// `+0x6B0` the timer's director: +8 the game mode's entity (u32)
    pub fn director(&self) -> Option<NonNull<c_void>> {
        self.0.read_object(offset_of!(HfHiveMindManager, director))
    }

    /// `+0x51884` the compiled map guard (outpost) compositions, family 3; their rows inline from
    /// +0x41138
    pub fn guard_compositions(&self) -> Option<u32> {
        self.0
            .read(offset_of!(HfHiveMindManager, guard_compositions))
    }

    /// `+0x5188C` the compiled reinforcement compositions, family 4; their rows inline from
    /// +0x432F8 (PickReinforcementComposition reads them)
    pub fn reinforcement_compositions(&self) -> Option<u32> {
        self.0
            .read(offset_of!(HfHiveMindManager, reinforcement_compositions))
    }

    /// `+0x518B4` the outposts' guard budget, which the map guards spend
    pub fn guard_budget(&self) -> Option<f32> {
        self.0.read(offset_of!(HfHiveMindManager, guard_budget))
    }

    /// `+0x51948` the mission countdown left, s
    pub fn countdown_left(&self) -> Option<f32> {
        self.0.read(offset_of!(HfHiveMindManager, countdown_left))
    }

    /// `+0x5194C` set once the timer has run out
    pub fn expired(&self) -> Option<u8> {
        self.0.read(offset_of!(HfHiveMindManager, expired))
    }

    /// `+0x5194E` 1 while the timer is paused
    pub fn paused(&self) -> Option<u8> {
        self.0.read(offset_of!(HfHiveMindManager, paused))
    }
}

global! {
    /// The `MissionTimers` global, or `None`.
    ///
    /// MissionTimers: HiveMindManager's former name, kept for mods written with it; use
    /// HiveMindManager
    MissionTimers
}

impl<'a> MissionTimers<'a> {
    /// `+0x650` the mission's level (lg): its difficulty level at +0xF9 (u8)
    pub fn level(&self) -> Option<NonNull<c_void>> {
        self.0.read_object(offset_of!(HfHiveMindManager, level))
    }

    /// `+0x67C` the mission's timers (hf->freeze_mission_timer needs 1)
    pub fn timer_count(&self) -> Option<u32> {
        self.0.read(offset_of!(HfHiveMindManager, timer_count))
    }

    /// `+0x6B0` the timer's director: +8 the game mode's entity (u32)
    pub fn director(&self) -> Option<NonNull<c_void>> {
        self.0.read_object(offset_of!(HfHiveMindManager, director))
    }

    /// `+0x51884` the compiled map guard (outpost) compositions, family 3; their rows inline from
    /// +0x41138
    pub fn guard_compositions(&self) -> Option<u32> {
        self.0
            .read(offset_of!(HfHiveMindManager, guard_compositions))
    }

    /// `+0x5188C` the compiled reinforcement compositions, family 4; their rows inline from
    /// +0x432F8 (PickReinforcementComposition reads them)
    pub fn reinforcement_compositions(&self) -> Option<u32> {
        self.0
            .read(offset_of!(HfHiveMindManager, reinforcement_compositions))
    }

    /// `+0x518B4` the outposts' guard budget, which the map guards spend
    pub fn guard_budget(&self) -> Option<f32> {
        self.0.read(offset_of!(HfHiveMindManager, guard_budget))
    }

    /// `+0x51948` the mission countdown left, s
    pub fn countdown_left(&self) -> Option<f32> {
        self.0.read(offset_of!(HfHiveMindManager, countdown_left))
    }

    /// `+0x5194C` set once the timer has run out
    pub fn expired(&self) -> Option<u8> {
        self.0.read(offset_of!(HfHiveMindManager, expired))
    }

    /// `+0x5194E` 1 while the timer is paused
    pub fn paused(&self) -> Option<u8> {
        self.0.read(offset_of!(HfHiveMindManager, paused))
    }
}

global! {
    /// The `Telemetry` global (`game.dll+0x347CE18`), or `None`.
    ///
    /// The game's telemetry: the mission-end telemetry record (TelemetryMissionEnd) at +0x31820
    /// keeps 16 objective instances
    Telemetry
}

global! {
    /// The `GuardForces` global (`game.dll+0x33265C0`), or `None`.
    ///
    /// The mission's outposts (GuardForce records). A record (0x2B8 bytes): +0x2A8 its size (1
    /// small, 2 medium, 3 large, 4 stronghold), +0x2AB whether it is active
    GuardForces
}

impl<'a> GuardForces<'a> {
    /// `+0xC` outposts
    pub fn count(&self) -> Option<u32> {
        self.0.read(offset_of!(HfGuardForces, count))
    }

    /// `+0x48` the GuardForce records, 0x2B8 bytes each
    pub fn records(&self) -> Option<NonNull<c_void>> {
        self.0.read_object(offset_of!(HfGuardForces, records))
    }
}

global! {
    /// The `UnitRegistry` global (`helldivers2.exe+0x1A100F0`), or `None`.
    ///
    /// The engine's unit registry: every unit by its handle's index (UnitFromHandle reads it under
    /// its lock at +0xD0)
    UnitRegistry
}

impl<'a> UnitRegistry<'a> {
    /// `+0x88` the units, a pointer each by index
    pub fn objects(&self) -> Option<NonNull<c_void>> {
        self.0.read_object(offset_of!(HfUnitRegistry, objects))
    }

    /// `+0x98` how many indexes
    pub fn count(&self) -> Option<u32> {
        self.0.read(offset_of!(HfUnitRegistry, count))
    }

    /// `+0xA0` a byte per index: its generation (handle >> 22)
    pub fn generations(&self) -> Option<NonNull<c_void>> {
        self.0.read_object(offset_of!(HfUnitRegistry, generations))
    }
}

global! {
    /// The `EngineCore` global (`helldivers2.exe+0x1A10208`), or `None`.
    ///
    /// The engine's core object: its worlds, its resource manager and its resource overrides
    EngineCore
}

impl<'a> EngineCore<'a> {
    /// `+0x3F8` the resource override store (SetResourceOverride)
    pub fn overrides(&self) -> Option<NonNull<c_void>> {
        self.0.read_object(offset_of!(HfEngineCore, overrides))
    }

    /// `+0x400` the resource manager: +0x208 its package store (+0x80 count, +0x88 packages),
    /// +0x218 its load queue (24-byte entries, head +0x321C, tail +0x3220)
    pub fn resource_manager(&self) -> Option<NonNull<c_void>> {
        self.0
            .read_object(offset_of!(HfEngineCore, resource_manager))
    }

    /// `+0x590` how many worlds
    pub fn world_count(&self) -> Option<u32> {
        self.0.read(offset_of!(HfEngineCore, world_count))
    }

    /// `+0x598` the worlds, a pointer each
    pub fn worlds(&self) -> Option<NonNull<c_void>> {
        self.0.read_object(offset_of!(HfEngineCore, worlds))
    }
}

global! {
    /// The `NetworkManager` global (`helldivers2.exe+0x1A10278`), or `None`.
    ///
    /// The engine's network manager (NetworkManagerUpdate)
    NetworkManager
}

impl<'a> NetworkManager<'a> {
    /// `+0x0` the network configuration: +0x0C the type width, +0x10 field type count, +0x18 field
    /// types, +0x28/+0x40/+0x70 the entity type arrays
    pub fn config(&self) -> Option<NonNull<c_void>> {
        self.0.read_object(offset_of!(HfNetworkManager, config))
    }

    /// `+0x98` the transport: +0xA0 its connection count, +0xA8 the connections
    pub fn transport(&self) -> Option<NonNull<c_void>> {
        self.0.read_object(offset_of!(HfNetworkManager, transport))
    }

    /// `+0xA8` the game session (+0x18 its configuration, +0x4870 its copy of the type width)
    pub fn session(&self) -> Option<NonNull<c_void>> {
        self.0.read_object(offset_of!(HfNetworkManager, session))
    }

    /// `+0xD0` how many peers the game asked to keep
    pub fn kept_count(&self) -> Option<u32> {
        self.0.read(offset_of!(HfNetworkManager, kept_count))
    }

    /// `+0xD8` those peers' ids (u64)
    pub fn kept(&self) -> Option<NonNull<c_void>> {
        self.0.read_object(offset_of!(HfNetworkManager, kept))
    }
}

global! {
    /// The `NetworkConfigOwner` global (`helldivers2.exe+0x1A10268`), or `None`.
    ///
    /// The object whose +0 is the engine's network configuration (the plan's network_config,
    /// [[cell]]); in the decoded image it holds the same pointer as NetworkManager
    NetworkConfigOwner
}

impl<'a> NetworkConfigOwner<'a> {
    /// `+0x0` the network configuration (see NetworkManager's config)
    pub fn config(&self) -> Option<NonNull<c_void>> {
        self.0.read_object(offset_of!(HfNetworkConfigOwner, config))
    }
}

global! {
    /// The `EngineEntityIndex` global (`helldivers2.exe+0x1B135B8`), or `None`.
    ///
    /// The engine's entity index: whether an entity handle is alive (its generation)
    EngineEntityIndex
}

impl<'a> EngineEntityIndex<'a> {
    /// `+0x80` how many indexes
    pub fn size(&self) -> Option<u32> {
        self.0.read(offset_of!(HfEngineEntityIndex, size))
    }

    /// `+0x88` a byte per index: its generation (handle >> 22 & 0xFF)
    pub fn generations(&self) -> Option<NonNull<c_void>> {
        self.0
            .read_object(offset_of!(HfEngineEntityIndex, generations))
    }
}

global! {
    /// The `NavWorlds` global (`helldivers2.exe+0x27C30B8`), or `None`.
    ///
    /// The navigation worlds, a pointer each by id (their count is the u32 at 0x27C30C0). A nav
    /// world: +0x08 min x, y, +0x14 max x, y, +0x6710 its grid (2 x u16), +0x6718 the tile size,
    /// +0x671C..+0x6728 the grid's min and max
    NavWorlds
}

global! {
    /// The `PhysicsProperties` global (`helldivers2.exe+0x27C5E48`), or `None`.
    ///
    /// The engine's physics properties: the collision filters by name
    PhysicsProperties
}

impl<'a> PhysicsProperties<'a> {
    /// `+0xF8` how many collision filters
    pub fn filter_count(&self) -> Option<u32> {
        self.0.read(offset_of!(HfPhysicsProperties, filter_count))
    }

    /// `+0x100` their name hashes (u32 each); a filter's row is its place here
    pub fn filter_names(&self) -> Option<NonNull<c_void>> {
        self.0
            .read_object(offset_of!(HfPhysicsProperties, filter_names))
    }
}

global! {
    /// The `CollisionFilterA` global (`helldivers2.exe+0x27C5E40`), or `None`.
    ///
    /// One of the two collision filters (its slot +0x18 is CollisionFilterPairs)
    CollisionFilterA
}

impl<'a> CollisionFilterA<'a> {
    /// `+0x28` the collision matrix's first word: 16 bytes per filter row, a bit per other filter
    pub fn matrix(&self) -> Option<u32> {
        self.0.read(offset_of!(HfCollisionFilterA, matrix))
    }
}

global! {
    /// The `CollisionFilterB` global (`helldivers2.exe+0x27C5B90`), or `None`.
    ///
    /// The other collision filter (its slot +0x18 is CollisionFilterPairs too)
    CollisionFilterB
}

impl<'a> CollisionFilterB<'a> {
    /// `+0x28` the collision matrix's first word: 16 bytes per filter row, a bit per other filter
    pub fn matrix(&self) -> Option<u32> {
        self.0.read(offset_of!(HfCollisionFilterB, matrix))
    }
}

global! {
    /// The `StatusReceivers` global (`game.dll+0x3326620`), or `None`.
    ///
    /// The status effect receivers: ApplyStatus's, HasStatus's and RemoveStatus's first argument.
    /// At +0x12C0 a map of entity ids to indexes (8-byte entries; capacity +0x12C8, empty key
    /// +0x12CC, hash multiplier +0x12D0)
    StatusReceivers
}

global! {
    /// The `ComponentWorld` global (`game.dll+0x346BFC8`), or `None`.
    ///
    /// The component world, EntityManager + 0x40: the component manager globals point inside it
    /// (HealthManager at +0x7C6230, ScoreManager at +0x7C7368, PlayerManager at +0xEA0818).
    /// Re-pointed together with EntityManager
    ComponentWorld
}

global! {
    /// The `GameModeMission` global (`game.dll+0x3326D60`), or `None`.
    ///
    /// The game_mode_mission component (capacity 1), the mission's game-mode state beside GameMode
    GameModeMission
}

impl<'a> GameModeMission<'a> {
    /// `+0x48` the most instances seen
    pub fn max(&self) -> Option<u32> {
        self.0.read(offset_of!(HfGameModeMission, max))
    }

    /// `+0x50` instances (0 or 1)
    pub fn count(&self) -> Option<u32> {
        self.0.read(offset_of!(HfGameModeMission, count))
    }

    /// `+0x80` the instance's entity descriptor
    pub fn descriptor(&self) -> Option<NonNull<c_void>> {
        self.0
            .read_object(offset_of!(HfGameModeMission, descriptor))
    }
}

global! {
    /// The `CareerStats` global (`game.dll+0x346D570`), or `None`.
    ///
    /// The player's career stats from the backend (the UI's "Kills: %u" line reads them)
    CareerStats
}

impl<'a> CareerStats<'a> {
    /// `+0x14` career kills, the larger of the local and server values
    pub fn kills(&self) -> Option<u32> {
        self.0.read(offset_of!(HfCareerStats, kills))
    }
}

global! {
    /// The `ProjectileWeapons` global (`game.dll+0x33266D8`), or `None`.
    ///
    /// The ProjectileWeapon component: ProjectileFire's first argument; ProjectileWeaponRecord
    /// looks in its private copies
    ProjectileWeapons
}

global! {
    /// The `WeaponDatas` global (`game.dll+0x3326CE0`), or `None`.
    ///
    /// The WeaponData component (weapon_data): WeaponDataRecord looks in its private copies
    WeaponDatas
}

global! {
    /// The `BeamWeapons` global (`game.dll+0x3326A20`), or `None`.
    ///
    /// The BeamWeapon component (0x70-byte instances): BeamWeaponUpdate's first argument;
    /// BeamWeaponRecord looks in its private copies
    BeamWeapons
}

global! {
    /// The `LinkedAmmo` global (`game.dll+0x3326AA8`), or `None`.
    ///
    /// The WeaponLinkedAmmo component: WeaponProjectileType reads its round type for weapons with
    /// linked ammunition
    LinkedAmmo
}

global! {
    /// The `Corpses` global (`game.dll+0x3326920`), or `None`.
    ///
    /// The corpse component: per corpse the link to the entity it came from (CorpseSource). At
    /// +0x28 a map of entity ids to indexes (capacity +0x30, empty key +0x34, hash multiplier
    /// +0x38); per index a 0x38-byte record at +0x50 (+0 the source network id) and a 0x48-byte
    /// spawn record at +0x48 (+0x3C the source entity)
    Corpses
}

global! {
    /// The `WeaponWindUps` global (`game.dll+0x33267A0`), or `None`.
    ///
    /// The WeaponWindUp component manager (component 70, component world +0x1B29B8): WindUpUpdate's
    /// first argument; WeaponWindUpRecord looks in its private copies
    WeaponWindUps
}

global! {
    /// The `Recharges` global (`game.dll+0x3326768`), or `None`.
    ///
    /// The Recharge component manager (component 99, component world +0x1C6F50; jump and hover
    /// packs' recharge): RechargeRecord looks in its private copies
    Recharges
}

global! {
    /// The `WeaponReloads` global (`game.dll+0x3326A70`), or `None`.
    ///
    /// The WeaponReload component manager (component 113, component world +0x7571C8). At +0x20 the
    /// hash of the entities that have an instance (the out-of-ammo handler's test);
    /// WeaponReloadRecord looks in its private copies
    WeaponReloads
}

global! {
    /// The `WeaponRoundsManager` global (`game.dll+0x3326CF0`), or `None`.
    ///
    /// The WeaponRounds component manager (component 117, component world +0x757398; weapons loaded
    /// round by round). WeaponRoundsRecord looks in its private copies
    WeaponRoundsManager
}

impl<'a> WeaponRoundsManager<'a> {
    /// `+0x18` instances (the world update's loop bound, 0x57201C)
    pub fn count(&self) -> Option<u32> {
        self.0.read(offset_of!(HfWeaponRoundsManager, count))
    }

    /// `+0x40` each instance's entity descriptor (8 bytes each: +0 type, +8 entity)
    pub fn descriptors(&self) -> Option<NonNull<c_void>> {
        self.0
            .read_object(offset_of!(HfWeaponRoundsManager, descriptors))
    }

    /// `+0x58` each instance's live row, 20 bytes: +0 its own spare rounds (network field
    /// 0x9A34C336)
    pub fn live(&self) -> Option<NonNull<c_void>> {
        self.0.read_object(offset_of!(HfWeaponRoundsManager, live))
    }
}

global! {
    /// The `WeaponCharges` global (`game.dll+0x3326C20`), or `None`.
    ///
    /// The WeaponCharge component manager (component 204, component world +0x75F1D8):
    /// WeaponChargeUpdate's first argument; WeaponChargeRecord looks in its private copies
    WeaponCharges
}

global! {
    /// The `Throwers` global (`game.dll+0x3326880`), or `None`.
    ///
    /// The Thrower component manager (component 258, component world +0xEB5CF0): ThrowerUpdate's
    /// first argument; ThrowerRecord looks in its private copies
    Throwers
}

global! {
    /// The `Throwables` global (`game.dll+0x33264C0`), or `None`.
    ///
    /// The Throwable component manager (component 259, component world +0xEB5DA8; grenades and
    /// thrown knives): ThrowableUpdate's first argument. Its records come from the type lookup
    /// ThrowableTypeRecord only (no private copies)
    Throwables
}

global! {
    /// The `ArcWeapons` global (`game.dll+0x3326C10`), or `None`.
    ///
    /// The ArcWeapon component manager (component 265, component world +0xEB64A0; the Arc Thrower's
    /// arcs): ArcWeaponUpdate's first argument; ArcWeaponRecord looks in its private copies
    ArcWeapons
}

impl<'a> ArcWeapons<'a> {
    /// `+0x38` instances (the world update's loop bound, 0x573737)
    pub fn count(&self) -> Option<u32> {
        self.0.read(offset_of!(HfArcWeapons, count))
    }
}

global! {
    /// The `WeaponHeats` global (`game.dll+0x3326D48`), or `None`.
    ///
    /// The WeaponHeat component manager (component 266, component world +0xEB6570):
    /// WeaponHeatUpdate's first argument; WeaponHeatRecord looks in its private copies
    WeaponHeats
}

impl<'a> WeaponHeats<'a> {
    /// `+0x18` instances (WeaponHeatUpdate's loop bound, 0x764988)
    pub fn count(&self) -> Option<u32> {
        self.0.read(offset_of!(HfWeaponHeats, count))
    }

    /// `+0x40` each instance's entity descriptor (8 bytes each: +8 entity, +0x14 bit 1 owned by
    /// this machine; WeaponHeatUpdate steps only those)
    pub fn descriptors(&self) -> Option<NonNull<c_void>> {
        self.0.read_object(offset_of!(HfWeaponHeats, descriptors))
    }
}

global! {
    /// The `AbilityWeapons` global (`game.dll+0x3326A10`), or `None`.
    ///
    /// The AbilityWeapon component manager (component 267, component world +0xEB6620; weapons whose
    /// fire starts an ability: the C4 detonator, the silo's targeting remote):
    /// AbilityWeaponUpdate's first argument. Its records come from the type lookup
    /// AbilityWeaponTypeRecord
    AbilityWeapons
}

global! {
    /// The `MeleeWeapons` global (`game.dll+0x3326B78`), or `None`.
    ///
    /// The MeleeWeapon component manager (component 268, component world +0xEB6670). Its records
    /// come from the type lookup at game.dll+0x50E360 (not mapped: it follows a jmp, so it does not
    /// pass the start check)
    MeleeWeapons
}

global! {
    /// The `WeaponCustomizations` global (`game.dll+0x3326A38`), or `None`.
    ///
    /// The WeaponCustomization component manager (component 271, component world +0xEB6810; a
    /// weapon's attachment slots and units): WeaponCustomizationUpdate's first argument;
    /// WeaponCustomizationRecord looks in its private copies
    WeaponCustomizations
}

global! {
    /// The `AiSpawnerManager` global (`game.dll+0x33265C8`), or `None`.
    ///
    /// The AiSpawnerComponent manager (component world +0x75EA20): The AI spawners (the director's
    /// spawn points and breaches). At +0x28 a map of entities to indexes
    AiSpawnerManager
}

impl<'a> AiSpawnerManager<'a> {
    /// `+0x28` the entity map's entries, 8 bytes each (entity u32, index u32); the map's capacity
    /// (u32) is 8 bytes after this field, its empty key 0xC after it, its hash multiplier 0x10
    /// after it
    pub fn map(&self) -> Option<NonNull<c_void>> {
        self.0.read_object(offset_of!(HfAiSpawnerManager, map))
    }
}

global! {
    /// The `BugholeManager` global (`game.dll+0x33267B0`), or `None`.
    ///
    /// The BugholeComponent manager (component world +0x7C56B0): Bug holes (Terminid nests' holes).
    /// At +0x28 a map of entities to indexes
    BugholeManager
}

impl<'a> BugholeManager<'a> {
    /// `+0x28` the entity map's entries, 8 bytes each (entity u32, index u32); the map's capacity
    /// (u32) is 8 bytes after this field, its empty key 0xC after it, its hash multiplier 0x10
    /// after it
    pub fn map(&self) -> Option<NonNull<c_void>> {
        self.0.read_object(offset_of!(HfBugholeManager, map))
    }
}

global! {
    /// The `VehicleMotionManager` global (`game.dll+0x3326458`), or `None`.
    ///
    /// The VehicleMotionComponent manager (component world +0x7C72E8): Vehicle motion (vehicles'
    /// and mechs' driving physics). At +0x40 a map of entities to indexes
    VehicleMotionManager
}

impl<'a> VehicleMotionManager<'a> {
    /// `+0x40` the entity map's entries, 8 bytes each (entity u32, index u32); the map's capacity
    /// (u32) is 8 bytes after this field, its empty key 0xC after it, its hash multiplier 0x10
    /// after it
    pub fn map(&self) -> Option<NonNull<c_void>> {
        self.0.read_object(offset_of!(HfVehicleMotionManager, map))
    }
}

global! {
    /// The `VehicleManager` global (`game.dll+0x3326668`), or `None`.
    ///
    /// The VehicleComponent manager (component world +0x1C6348): Vehicles and mechs
    /// (VehicleSettings resolves their records; private copies at +0xC0, copy map at +0x80). At
    /// +0x38 a map of entities to indexes
    VehicleManager
}

impl<'a> VehicleManager<'a> {
    /// `+0x38` the entity map's entries, 8 bytes each (entity u32, index u32); the map's capacity
    /// (u32) is 8 bytes after this field, its empty key 0xC after it, its hash multiplier 0x10
    /// after it
    pub fn map(&self) -> Option<NonNull<c_void>> {
        self.0.read_object(offset_of!(HfVehicleManager, map))
    }
}

global! {
    /// The `SquadTransportManager` global (`game.dll+0x3326CC0`), or `None`.
    ///
    /// The SquadTransportComponent manager (component world +0x1C6EB0): Squad transports (enemy
    /// dropships carrying a squad). At +0x20 a map of entities to indexes
    SquadTransportManager
}

impl<'a> SquadTransportManager<'a> {
    /// `+0x20` the entity map's entries, 8 bytes each (entity u32, index u32); the map's capacity
    /// (u32) is 8 bytes after this field, its empty key 0xC after it, its hash multiplier 0x10
    /// after it
    pub fn map(&self) -> Option<NonNull<c_void>> {
        self.0.read_object(offset_of!(HfSquadTransportManager, map))
    }
}

global! {
    /// The `SquadFormationManager` global (`game.dll+0x3326A88`), or `None`.
    ///
    /// The SquadTransportFormationComponent manager (component world +0x757568): Squad transport
    /// formations (the squad slots of a dropship). At +0x18 a map of entities to indexes
    SquadFormationManager
}

impl<'a> SquadFormationManager<'a> {
    /// `+0x18` the entity map's entries, 8 bytes each (entity u32, index u32); the map's capacity
    /// (u32) is 8 bytes after this field, its empty key 0xC after it, its hash multiplier 0x10
    /// after it
    pub fn map(&self) -> Option<NonNull<c_void>> {
        self.0.read_object(offset_of!(HfSquadFormationManager, map))
    }
}

global! {
    /// The `GoreManager` global (`game.dll+0x33264D0`), or `None`.
    ///
    /// The GoreComponent manager (component world +0x7FF2F8): Gore (dismemberment state). At
    /// +0x156DA0 a map of entities to indexes
    GoreManager
}

impl<'a> GoreManager<'a> {
    /// `+0x156DA0` the entity map's entries, 8 bytes each (entity u32, index u32); the map's
    /// capacity (u32) is 8 bytes after this field, its empty key 0xC after it, its hash multiplier
    /// 0x10 after it
    pub fn map(&self) -> Option<NonNull<c_void>> {
        self.0.read_object(offset_of!(HfGoreManager, map))
    }
}

global! {
    /// The `StratagemBlockerManager` global (`game.dll+0x3326568`), or `None`.
    ///
    /// The StratagemBlockerComponent manager (component world +0x1E60): Stratagem blockers (what
    /// jams stratagems in an area). At +0x18 a map of entities to indexes
    StratagemBlockerManager
}

impl<'a> StratagemBlockerManager<'a> {
    /// `+0x18` the entity map's entries, 8 bytes each (entity u32, index u32); the map's capacity
    /// (u32) is 8 bytes after this field, its empty key 0xC after it, its hash multiplier 0x10
    /// after it
    pub fn map(&self) -> Option<NonNull<c_void>> {
        self.0
            .read_object(offset_of!(HfStratagemBlockerManager, map))
    }
}

global! {
    /// The `OrbitalAbilityManager` global (`game.dll+0x33267F0`), or `None`.
    ///
    /// The OrbitalAbilityComponent manager (component world +0xF064A8): Orbital abilities
    /// (OrbitalAbilitySettings resolves their records; private copies at +0xD8, copy map at +0x98).
    /// At +0x58 a map of entities to indexes
    OrbitalAbilityManager
}

impl<'a> OrbitalAbilityManager<'a> {
    /// `+0x58` the entity map's entries, 8 bytes each (entity u32, index u32); the map's capacity
    /// (u32) is 8 bytes after this field, its empty key 0xC after it, its hash multiplier 0x10
    /// after it
    pub fn map(&self) -> Option<NonNull<c_void>> {
        self.0.read_object(offset_of!(HfOrbitalAbilityManager, map))
    }
}

global! {
    /// The `HellpodPayloadManager` global (`game.dll+0x3326560`), or `None`.
    ///
    /// The HellpodPayloadComponent manager (component world +0x75C1B8): Hellpod payloads (what a
    /// hellpod delivers; HellpodPayloadSettings resolves their records, private copies at +0xA0,
    /// copy map at +0x60). At +0x28 a map of entities to indexes
    HellpodPayloadManager
}

impl<'a> HellpodPayloadManager<'a> {
    /// `+0x28` the entity map's entries, 8 bytes each (entity u32, index u32); the map's capacity
    /// (u32) is 8 bytes after this field, its empty key 0xC after it, its hash multiplier 0x10
    /// after it
    pub fn map(&self) -> Option<NonNull<c_void>> {
        self.0.read_object(offset_of!(HfHellpodPayloadManager, map))
    }
}

global! {
    /// The `UnitManager` global (`game.dll+0x3326508`), or `None`.
    ///
    /// The UnitComponent manager (component world +0x1B4170): The entities' units (their scene
    /// units; UnitSettings resolves their records, private copies at +0xC0, copy map at +0x80). At
    /// +0x40 a map of entities to indexes
    UnitManager
}

impl<'a> UnitManager<'a> {
    /// `+0x40` the entity map's entries, 8 bytes each (entity u32, index u32); the map's capacity
    /// (u32) is 8 bytes after this field, its empty key 0xC after it, its hash multiplier 0x10
    /// after it
    pub fn map(&self) -> Option<NonNull<c_void>> {
        self.0.read_object(offset_of!(HfUnitManager, map))
    }
}

global! {
    /// The `RotationManager` global (`game.dll+0x33264A0`), or `None`.
    ///
    /// The RotationComponent manager (component world +0xF065D8): Rotation (how entities turn;
    /// RotationSettings resolves their records, private copies at +0xB8, copy map at +0x78). At
    /// +0x38 a map of entities to indexes
    RotationManager
}

impl<'a> RotationManager<'a> {
    /// `+0x38` the entity map's entries, 8 bytes each (entity u32, index u32); the map's capacity
    /// (u32) is 8 bytes after this field, its empty key 0xC after it, its hash multiplier 0x10
    /// after it
    pub fn map(&self) -> Option<NonNull<c_void>> {
        self.0.read_object(offset_of!(HfRotationManager, map))
    }
}

global! {
    /// The `DisplacementManager` global (`game.dll+0x3326AC8`), or `None`.
    ///
    /// The DisplacementComponent manager (component world +0xEB71D8): Displacement
    /// (DisplacementSettings resolves their records, private copies at +0xA0, copy map at +0x60).
    /// At +0x20 a map of entities to indexes
    DisplacementManager
}

impl<'a> DisplacementManager<'a> {
    /// `+0x20` the entity map's entries, 8 bytes each (entity u32, index u32); the map's capacity
    /// (u32) is 8 bytes after this field, its empty key 0xC after it, its hash multiplier 0x10
    /// after it
    pub fn map(&self) -> Option<NonNull<c_void>> {
        self.0.read_object(offset_of!(HfDisplacementManager, map))
    }
}

global! {
    /// The `JumppackManager` global (`game.dll+0x3326BB8`), or `None`.
    ///
    /// The JumppackComponent manager (component world +0xEB63F8): Jump packs and hover packs
    /// (JumppackSettings resolves their records, private copies at +0xA0, copy map at +0x60). At
    /// +0x20 a map of entities to indexes
    JumppackManager
}

impl<'a> JumppackManager<'a> {
    /// `+0x20` the entity map's entries, 8 bytes each (entity u32, index u32); the map's capacity
    /// (u32) is 8 bytes after this field, its empty key 0xC after it, its hash multiplier 0x10
    /// after it
    pub fn map(&self) -> Option<NonNull<c_void>> {
        self.0.read_object(offset_of!(HfJumppackManager, map))
    }

    /// `+0x40` 12 bytes per index: the replicated hover input vector
    pub fn hover_input(&self) -> Option<NonNull<c_void>> {
        self.0
            .read_object(offset_of!(HfJumppackManager, hover_input))
    }

    /// `+0x48` 76 bytes per index: +0 launch latch, +1 sustain latch, +4 launch elapsed (f32 s), +8
    /// sustain elapsed (f32 s), +0xC launch window (f32 s)
    pub fn flight(&self) -> Option<NonNull<c_void>> {
        self.0.read_object(offset_of!(HfJumppackManager, flight))
    }

    /// `+0x50` 5 bytes per index, replicated: +0 active, +1 sustain, +2 hover launch, +3 mode
    /// toggle, +4 thrusting
    pub fn flags(&self) -> Option<NonNull<c_void>> {
        self.0.read_object(offset_of!(HfJumppackManager, flags))
    }
}

global! {
    /// The `EagleManager` global (`game.dll+0x3326650`), or `None`.
    ///
    /// The EagleComponent manager (component world +0xF0B300): The Eagle jets in flight
    /// (EagleSettings resolves their records, private copies at +0xB0, copy map at +0x70). At +0x30
    /// a map of entities to indexes
    EagleManager
}

impl<'a> EagleManager<'a> {
    /// `+0x30` the entity map's entries, 8 bytes each (entity u32, index u32); the map's capacity
    /// (u32) is 8 bytes after this field, its empty key 0xC after it, its hash multiplier 0x10
    /// after it
    pub fn map(&self) -> Option<NonNull<c_void>> {
        self.0.read_object(offset_of!(HfEagleManager, map))
    }

    /// `+0x58` 0xFC bytes per index, the jet: +0xC its target index (9 none), +0x10 targets tracked
    /// (<= 8), +0x14 8 target entities, +0x54 aim point (3 f32), +0x68 releases so far, +0x70
    /// attacking (u8), +0x78 jet position (3 f32), +0xD4 velocity (3 f32), +0xE0 speed (f32)
    pub fn records(&self) -> Option<NonNull<c_void>> {
        self.0.read_object(offset_of!(HfEagleManager, records))
    }

    /// `+0x60` 28 bytes per index: +0 the approach start position, +0xC the target point at the
    /// call (3 f32 each)
    pub fn flight(&self) -> Option<NonNull<c_void>> {
        self.0.read_object(offset_of!(HfEagleManager, flight))
    }
}

global! {
    /// The `EncounterManager` global (`game.dll+0x3326618`), or `None`.
    ///
    /// The EncounterComponent manager (component world +0x7C4D30): The director's encounters: its
    /// reinforcements (EncounterUpdate picks a composition and sends it by transport or breach). At
    /// +0x948 a map of entities to indexes
    EncounterManager
}

impl<'a> EncounterManager<'a> {
    /// `+0x948` the entity map's entries, 8 bytes each (entity u32, index u32); the map's capacity
    /// (u32) is 8 bytes after this field, its empty key 0xC after it, its hash multiplier 0x10
    /// after it
    pub fn map(&self) -> Option<NonNull<c_void>> {
        self.0.read_object(offset_of!(HfEncounterManager, map))
    }
}

global! {
    /// The `AiSquadManager` global (`game.dll+0x33265A0`), or `None`.
    ///
    /// The AiSquadComponent manager (component world +0xF05068): AI squads. A squad's record
    /// (0x2510 bytes) queues the enemies it is to spawn: up to 32 entries of 0xC8 bytes from
    /// +0xC08, their count at +0x2508 (a 33rd entry overruns the count and crashes the game). At
    /// +0x20 a map of entities to indexes
    AiSquadManager
}

impl<'a> AiSquadManager<'a> {
    /// `+0xC` squads in use
    pub fn count(&self) -> Option<u32> {
        self.0.read(offset_of!(HfAiSquadManager, count))
    }

    /// `+0x20` the entity map's entries, 8 bytes each (entity u32, index u32); the map's capacity
    /// (u32) is 8 bytes after this field, its empty key 0xC after it, its hash multiplier 0x10
    /// after it
    pub fn map(&self) -> Option<NonNull<c_void>> {
        self.0.read_object(offset_of!(HfAiSquadManager, map))
    }

    /// `+0x38` descriptor pointers by index (+8 the entity)
    pub fn descriptors(&self) -> Option<NonNull<c_void>> {
        self.0
            .read_object(offset_of!(HfAiSquadManager, descriptors))
    }

    /// `+0x40` the squad records, 0x2510 bytes each, by index
    pub fn records(&self) -> Option<NonNull<c_void>> {
        self.0.read_object(offset_of!(HfAiSquadManager, records))
    }
}

global! {
    /// The `SensorEyeManager` global (`game.dll+0x3326520`), or `None`.
    ///
    /// The SensorEyeComponent manager (component world +0x75ED20): Sensor eyes (what enemies and
    /// sentries see with: SensorEyeRanges' first argument; SensorEyeSettings resolves their
    /// records, private copies at +0xB0). At +0x30 a map of entities to indexes
    SensorEyeManager
}

impl<'a> SensorEyeManager<'a> {
    /// `+0x30` the entity map's entries, 8 bytes each (entity u32, index u32); the map's capacity
    /// (u32) is 8 bytes after this field, its empty key 0xC after it, its hash multiplier 0x10
    /// after it
    pub fn map(&self) -> Option<NonNull<c_void>> {
        self.0.read_object(offset_of!(HfSensorEyeManager, map))
    }
}

global! {
    /// The `TurretManager` global (`game.dll+0x3326D70`), or `None`.
    ///
    /// The TurretComponent manager (component world +0xEB5060): Turrets (sentries, mounted guns:
    /// TurretUpdate's first argument; TurretSettings resolves their records, private copies at
    /// +0xA8). At +0x28 a map of entities to indexes
    TurretManager
}

impl<'a> TurretManager<'a> {
    /// `+0x28` the entity map's entries, 8 bytes each (entity u32, index u32); the map's capacity
    /// (u32) is 8 bytes after this field, its empty key 0xC after it, its hash multiplier 0x10
    /// after it
    pub fn map(&self) -> Option<NonNull<c_void>> {
        self.0.read_object(offset_of!(HfTurretManager, map))
    }
}

global! {
    /// The `LocomotionManager` global (`game.dll+0x3326CF8`), or `None`.
    ///
    /// The LocomotionComponent manager (component world +0xF06698): Locomotion (how characters
    /// move; private copies at +0xA8, copy map at +0x68). At +0x20 a map of entities to indexes
    LocomotionManager
}

impl<'a> LocomotionManager<'a> {
    /// `+0x20` the entity map's entries, 8 bytes each (entity u32, index u32); the map's capacity
    /// (u32) is 8 bytes after this field, its empty key 0xC after it, its hash multiplier 0x10
    /// after it
    pub fn map(&self) -> Option<NonNull<c_void>> {
        self.0.read_object(offset_of!(HfLocomotionManager, map))
    }
}

impl HellForgeApi {
    /// The `DepositManager` global (`game.dll+0x33265E8`), or `None`.
    pub fn deposit_manager(&self) -> Option<DepositManager<'_>> {
        Some(DepositManager::new(
            self,
            self.global_object("DepositManager")?,
        ))
    }

    /// The `AvatarManager` global (`game.dll+0x3326D20`), or `None`.
    pub fn avatar_manager(&self) -> Option<AvatarManager<'_>> {
        Some(AvatarManager::new(
            self,
            self.global_object("AvatarManager")?,
        ))
    }

    /// The `ActionManager` global, or `None`.
    pub fn action_manager(&self) -> Option<ActionManager<'_>> {
        Some(ActionManager::new(
            self,
            self.global_object("AvatarManager")?,
        ))
    }

    /// The `WieldableManager` global (`game.dll+0x3326730`), or `None`.
    pub fn wieldable_manager(&self) -> Option<WieldableManager<'_>> {
        Some(WieldableManager::new(
            self,
            self.global_object("WieldableManager")?,
        ))
    }

    /// The `FactionManager` global (`game.dll+0x3326CB8`), or `None`.
    pub fn faction_manager(&self) -> Option<FactionManager<'_>> {
        Some(FactionManager::new(
            self,
            self.global_object("FactionManager")?,
        ))
    }

    /// The `BehaviourManager` global (`game.dll+0x3326740`), or `None`.
    pub fn behaviour_manager(&self) -> Option<BehaviourManager<'_>> {
        Some(BehaviourManager::new(
            self,
            self.global_object("BehaviourManager")?,
        ))
    }

    /// The `TagManager` global (`game.dll+0x3326430`), or `None`.
    pub fn tag_manager(&self) -> Option<TagManager<'_>> {
        Some(TagManager::new(self, self.global_object("TagManager")?))
    }

    /// The `Clock` global (`game.dll+0x3326348`), or `None`.
    pub fn clock(&self) -> Option<Clock<'_>> {
        Some(Clock::new(self, self.global_object("Clock")?))
    }

    /// The `EngineApi` global (`game.dll+0x3326308`), or `None`.
    pub fn engine_api(&self) -> Option<EngineApi<'_>> {
        Some(EngineApi::new(self, self.global_object("EngineApi")?))
    }

    /// The `EngineUnitApi` global (`game.dll+0x3326310`), or `None`.
    pub fn engine_unit_api(&self) -> Option<EngineUnitApi<'_>> {
        Some(EngineUnitApi::new(
            self,
            self.global_object("EngineUnitApi")?,
        ))
    }

    /// The `WwiseApi` global (`game.dll+0x3326318`), or `None`.
    pub fn wwise_api(&self) -> Option<WwiseApi<'_>> {
        Some(WwiseApi::new(self, self.global_object("WwiseApi")?))
    }

    /// The `AbilityManager` global (`game.dll+0x3326640`), or `None`.
    pub fn ability_manager(&self) -> Option<AbilityManager<'_>> {
        Some(AbilityManager::new(
            self,
            self.global_object("AbilityManager")?,
        ))
    }

    /// The `WeaponLinkerManager` global (`game.dll+0x33267F8`), or `None`.
    pub fn weapon_linker_manager(&self) -> Option<WeaponLinkerManager<'_>> {
        Some(WeaponLinkerManager::new(
            self,
            self.global_object("WeaponLinkerManager")?,
        ))
    }

    /// The `SeekingMissileManager` global (`game.dll+0x3326DF0`), or `None`.
    pub fn seeking_missile_manager(&self) -> Option<SeekingMissileManager<'_>> {
        Some(SeekingMissileManager::new(
            self,
            self.global_object("SeekingMissileManager")?,
        ))
    }

    /// The `LaserDesignatorManager` global (`game.dll+0x3326960`), or `None`.
    pub fn laser_designator_manager(&self) -> Option<LaserDesignatorManager<'_>> {
        Some(LaserDesignatorManager::new(
            self,
            self.global_object("LaserDesignatorManager")?,
        ))
    }

    /// The `GuidanceTargets` global (`game.dll+0x3326BD8`), or `None`.
    pub fn guidance_targets(&self) -> Option<GuidanceTargets<'_>> {
        Some(GuidanceTargets::new(
            self,
            self.global_object("GuidanceTargets")?,
        ))
    }

    /// The `ExplosiveManager` global (`game.dll+0x3326728`), or `None`.
    pub fn explosive_manager(&self) -> Option<ExplosiveManager<'_>> {
        Some(ExplosiveManager::new(
            self,
            self.global_object("ExplosiveManager")?,
        ))
    }

    /// The `EquipmentManager` global (`game.dll+0x3326DC0`), or `None`.
    pub fn equipment_manager(&self) -> Option<EquipmentManager<'_>> {
        Some(EquipmentManager::new(
            self,
            self.global_object("EquipmentManager")?,
        ))
    }

    /// The `RemoteOwners` global, or `None`.
    pub fn remote_owners(&self) -> Option<RemoteOwners<'_>> {
        Some(RemoteOwners::new(
            self,
            self.global_object("EquipmentManager")?,
        ))
    }

    /// The `HellpodRackManager` global (`game.dll+0x3326A58`), or `None`.
    pub fn hellpod_rack_manager(&self) -> Option<HellpodRackManager<'_>> {
        Some(HellpodRackManager::new(
            self,
            self.global_object("HellpodRackManager")?,
        ))
    }

    /// The `AttachableManager` global (`game.dll+0x3326698`), or `None`.
    pub fn attachable_manager(&self) -> Option<AttachableManager<'_>> {
        Some(AttachableManager::new(
            self,
            self.global_object("AttachableManager")?,
        ))
    }

    /// The `MountManager` global (`game.dll+0x3326438`), or `None`.
    pub fn mount_manager(&self) -> Option<MountManager<'_>> {
        Some(MountManager::new(self, self.global_object("MountManager")?))
    }

    /// The `DropAnchors` global (`game.dll+0x3326658`), or `None`.
    pub fn drop_anchors(&self) -> Option<DropAnchors<'_>> {
        Some(DropAnchors::new(self, self.global_object("DropAnchors")?))
    }

    /// The `PerceptionManager` global (`game.dll+0x3326548`), or `None`.
    pub fn perception_manager(&self) -> Option<PerceptionManager<'_>> {
        Some(PerceptionManager::new(
            self,
            self.global_object("PerceptionManager")?,
        ))
    }

    /// The `WeaponManager` global (`game.dll+0x3326660`), or `None`.
    pub fn weapon_manager(&self) -> Option<WeaponManager<'_>> {
        Some(WeaponManager::new(
            self,
            self.global_object("WeaponManager")?,
        ))
    }

    /// The `MagazineManager` global, or `None`.
    pub fn magazine_manager(&self) -> Option<MagazineManager<'_>> {
        Some(MagazineManager::new(
            self,
            self.global_object("WeaponManager")?,
        ))
    }

    /// The `WeaponMagazines` global (`game.dll+0x3326648`), or `None`.
    pub fn weapon_magazines(&self) -> Option<WeaponMagazines<'_>> {
        Some(WeaponMagazines::new(
            self,
            self.global_object("WeaponMagazines")?,
        ))
    }

    /// The `MagazineRefills` global, or `None`.
    pub fn magazine_refills(&self) -> Option<MagazineRefills<'_>> {
        Some(MagazineRefills::new(
            self,
            self.global_object("WeaponMagazines")?,
        ))
    }

    /// The `WeaponTargeting` global (`game.dll+0x3326D30`), or `None`.
    pub fn weapon_targeting(&self) -> Option<WeaponTargeting<'_>> {
        Some(WeaponTargeting::new(
            self,
            self.global_object("WeaponTargeting")?,
        ))
    }

    /// The `ShieldManager` global (`game.dll+0x3326630`), or `None`.
    pub fn shield_manager(&self) -> Option<ShieldManager<'_>> {
        Some(ShieldManager::new(
            self,
            self.global_object("ShieldManager")?,
        ))
    }

    /// The `CustomizationManager` global (`game.dll+0x33264F8`), or `None`.
    pub fn customization_manager(&self) -> Option<CustomizationManager<'_>> {
        Some(CustomizationManager::new(
            self,
            self.global_object("CustomizationManager")?,
        ))
    }

    /// The `KitList` global, or `None`.
    pub fn kit_list(&self) -> Option<KitList<'_>> {
        Some(KitList::new(
            self,
            self.global_object("CustomizationManager")?,
        ))
    }

    /// The `BombardmentManager` global (`game.dll+0x3326CE8`), or `None`.
    pub fn bombardment_manager(&self) -> Option<BombardmentManager<'_>> {
        Some(BombardmentManager::new(
            self,
            self.global_object("BombardmentManager")?,
        ))
    }

    /// The `Progression` global (`game.dll+0x347CEF8`), or `None`.
    pub fn progression(&self) -> Option<Progression<'_>> {
        Some(Progression::new(self, self.global_object("Progression")?))
    }

    /// The `ProgressionManager` global, or `None`.
    pub fn progression_manager(&self) -> Option<ProgressionManager<'_>> {
        Some(ProgressionManager::new(
            self,
            self.global_object("Progression")?,
        ))
    }

    /// The `PlayerHistory` global (`game.dll+0x347CE50`), or `None`.
    pub fn player_history(&self) -> Option<PlayerHistory<'_>> {
        Some(PlayerHistory::new(
            self,
            self.global_object("PlayerHistory")?,
        ))
    }

    /// The `Profile` global (`game.dll+0x347CDD8`), or `None`.
    pub fn profile(&self) -> Option<Profile<'_>> {
        Some(Profile::new(self, self.global_object("Profile")?))
    }

    /// The `MenuStates` global (`game.dll+0x347CE28`), or `None`.
    pub fn menu_states(&self) -> Option<MenuStates<'_>> {
        Some(MenuStates::new(self, self.global_object("MenuStates")?))
    }

    /// The `UiManager` global, or `None`.
    pub fn ui_manager(&self) -> Option<UiManager<'_>> {
        Some(UiManager::new(self, self.global_object("MenuStates")?))
    }

    /// The `MenuStack` global (`game.dll+0x347CE38`), or `None`.
    pub fn menu_stack(&self) -> Option<MenuStack<'_>> {
        Some(MenuStack::new(self, self.global_object("MenuStack")?))
    }

    /// The `Armory` global (`game.dll+0x33269E8`), or `None`.
    pub fn armory(&self) -> Option<Armory<'_>> {
        Some(Armory::new(self, self.global_object("Armory")?))
    }

    /// The `ThumbnailManager` global (`game.dll+0x347CD80`), or `None`.
    pub fn thumbnail_manager(&self) -> Option<ThumbnailManager<'_>> {
        Some(ThumbnailManager::new(
            self,
            self.global_object("ThumbnailManager")?,
        ))
    }

    /// The `PackageSystem` global (`game.dll+0x347CEB0`), or `None`.
    pub fn package_system(&self) -> Option<PackageSystem<'_>> {
        Some(PackageSystem::new(
            self,
            self.global_object("PackageSystem")?,
        ))
    }

    /// The `GameConfig` global (`game.dll+0x347CDF8`), or `None`.
    pub fn game_config(&self) -> Option<GameConfig<'_>> {
        Some(GameConfig::new(self, self.global_object("GameConfig")?))
    }

    /// The `NavigationSystem` global (`game.dll+0x33263C8`), or `None`.
    pub fn navigation_system(&self) -> Option<NavigationSystem<'_>> {
        Some(NavigationSystem::new(
            self,
            self.global_object("NavigationSystem")?,
        ))
    }

    /// The `PhysicsPlugin` global (`game.dll+0x3326380`), or `None`.
    pub fn physics_plugin(&self) -> Option<PhysicsPlugin<'_>> {
        Some(PhysicsPlugin::new(
            self,
            self.global_object("PhysicsPlugin")?,
        ))
    }

    /// The `ObjectiveManager` global (`game.dll+0x3326DA0`), or `None`.
    pub fn objective_manager(&self) -> Option<ObjectiveManager<'_>> {
        Some(ObjectiveManager::new(
            self,
            self.global_object("ObjectiveManager")?,
        ))
    }

    /// The `HiveMindManager` global (`game.dll+0x3326D10`), or `None`.
    pub fn hive_mind_manager(&self) -> Option<HiveMindManager<'_>> {
        Some(HiveMindManager::new(
            self,
            self.global_object("HiveMindManager")?,
        ))
    }

    /// The `MissionTimers` global, or `None`.
    pub fn mission_timers(&self) -> Option<MissionTimers<'_>> {
        Some(MissionTimers::new(
            self,
            self.global_object("HiveMindManager")?,
        ))
    }

    /// The `Telemetry` global (`game.dll+0x347CE18`), or `None`.
    pub fn telemetry(&self) -> Option<Telemetry<'_>> {
        Some(Telemetry::new(self, self.global_object("Telemetry")?))
    }

    /// The `GuardForces` global (`game.dll+0x33265C0`), or `None`.
    pub fn guard_forces(&self) -> Option<GuardForces<'_>> {
        Some(GuardForces::new(self, self.global_object("GuardForces")?))
    }

    /// The `UnitRegistry` global (`helldivers2.exe+0x1A100F0`), or `None`.
    pub fn unit_registry(&self) -> Option<UnitRegistry<'_>> {
        Some(UnitRegistry::new(self, self.global_object("UnitRegistry")?))
    }

    /// The `EngineCore` global (`helldivers2.exe+0x1A10208`), or `None`.
    pub fn engine_core(&self) -> Option<EngineCore<'_>> {
        Some(EngineCore::new(self, self.global_object("EngineCore")?))
    }

    /// The `NetworkManager` global (`helldivers2.exe+0x1A10278`), or `None`.
    pub fn network_manager(&self) -> Option<NetworkManager<'_>> {
        Some(NetworkManager::new(
            self,
            self.global_object("NetworkManager")?,
        ))
    }

    /// The `NetworkConfigOwner` global (`helldivers2.exe+0x1A10268`), or `None`.
    pub fn network_config_owner(&self) -> Option<NetworkConfigOwner<'_>> {
        Some(NetworkConfigOwner::new(
            self,
            self.global_object("NetworkConfigOwner")?,
        ))
    }

    /// The `EngineEntityIndex` global (`helldivers2.exe+0x1B135B8`), or `None`.
    pub fn engine_entity_index(&self) -> Option<EngineEntityIndex<'_>> {
        Some(EngineEntityIndex::new(
            self,
            self.global_object("EngineEntityIndex")?,
        ))
    }

    /// The `NavWorlds` global (`helldivers2.exe+0x27C30B8`), or `None`.
    pub fn nav_worlds(&self) -> Option<NavWorlds<'_>> {
        Some(NavWorlds::new(self, self.global_object("NavWorlds")?))
    }

    /// The `PhysicsProperties` global (`helldivers2.exe+0x27C5E48`), or `None`.
    pub fn physics_properties(&self) -> Option<PhysicsProperties<'_>> {
        Some(PhysicsProperties::new(
            self,
            self.global_object("PhysicsProperties")?,
        ))
    }

    /// The `CollisionFilterA` global (`helldivers2.exe+0x27C5E40`), or `None`.
    pub fn collision_filter_a(&self) -> Option<CollisionFilterA<'_>> {
        Some(CollisionFilterA::new(
            self,
            self.global_object("CollisionFilterA")?,
        ))
    }

    /// The `CollisionFilterB` global (`helldivers2.exe+0x27C5B90`), or `None`.
    pub fn collision_filter_b(&self) -> Option<CollisionFilterB<'_>> {
        Some(CollisionFilterB::new(
            self,
            self.global_object("CollisionFilterB")?,
        ))
    }

    /// The `StatusReceivers` global (`game.dll+0x3326620`), or `None`.
    pub fn status_receivers(&self) -> Option<StatusReceivers<'_>> {
        Some(StatusReceivers::new(
            self,
            self.global_object("StatusReceivers")?,
        ))
    }

    /// The `ComponentWorld` global (`game.dll+0x346BFC8`), or `None`.
    pub fn component_world(&self) -> Option<ComponentWorld<'_>> {
        Some(ComponentWorld::new(
            self,
            self.global_object("ComponentWorld")?,
        ))
    }

    /// The `GameModeMission` global (`game.dll+0x3326D60`), or `None`.
    pub fn game_mode_mission(&self) -> Option<GameModeMission<'_>> {
        Some(GameModeMission::new(
            self,
            self.global_object("GameModeMission")?,
        ))
    }

    /// The `CareerStats` global (`game.dll+0x346D570`), or `None`.
    pub fn career_stats(&self) -> Option<CareerStats<'_>> {
        Some(CareerStats::new(self, self.global_object("CareerStats")?))
    }

    /// The `ProjectileWeapons` global (`game.dll+0x33266D8`), or `None`.
    pub fn projectile_weapons(&self) -> Option<ProjectileWeapons<'_>> {
        Some(ProjectileWeapons::new(
            self,
            self.global_object("ProjectileWeapons")?,
        ))
    }

    /// The `WeaponDatas` global (`game.dll+0x3326CE0`), or `None`.
    pub fn weapon_datas(&self) -> Option<WeaponDatas<'_>> {
        Some(WeaponDatas::new(self, self.global_object("WeaponDatas")?))
    }

    /// The `BeamWeapons` global (`game.dll+0x3326A20`), or `None`.
    pub fn beam_weapons(&self) -> Option<BeamWeapons<'_>> {
        Some(BeamWeapons::new(self, self.global_object("BeamWeapons")?))
    }

    /// The `LinkedAmmo` global (`game.dll+0x3326AA8`), or `None`.
    pub fn linked_ammo(&self) -> Option<LinkedAmmo<'_>> {
        Some(LinkedAmmo::new(self, self.global_object("LinkedAmmo")?))
    }

    /// The `Corpses` global (`game.dll+0x3326920`), or `None`.
    pub fn corpses(&self) -> Option<Corpses<'_>> {
        Some(Corpses::new(self, self.global_object("Corpses")?))
    }

    /// The `WeaponWindUps` global (`game.dll+0x33267A0`), or `None`.
    pub fn weapon_wind_ups(&self) -> Option<WeaponWindUps<'_>> {
        Some(WeaponWindUps::new(
            self,
            self.global_object("WeaponWindUps")?,
        ))
    }

    /// The `Recharges` global (`game.dll+0x3326768`), or `None`.
    pub fn recharges(&self) -> Option<Recharges<'_>> {
        Some(Recharges::new(self, self.global_object("Recharges")?))
    }

    /// The `WeaponReloads` global (`game.dll+0x3326A70`), or `None`.
    pub fn weapon_reloads(&self) -> Option<WeaponReloads<'_>> {
        Some(WeaponReloads::new(
            self,
            self.global_object("WeaponReloads")?,
        ))
    }

    /// The `WeaponRoundsManager` global (`game.dll+0x3326CF0`), or `None`.
    pub fn weapon_rounds_manager(&self) -> Option<WeaponRoundsManager<'_>> {
        Some(WeaponRoundsManager::new(
            self,
            self.global_object("WeaponRoundsManager")?,
        ))
    }

    /// The `WeaponCharges` global (`game.dll+0x3326C20`), or `None`.
    pub fn weapon_charges(&self) -> Option<WeaponCharges<'_>> {
        Some(WeaponCharges::new(
            self,
            self.global_object("WeaponCharges")?,
        ))
    }

    /// The `Throwers` global (`game.dll+0x3326880`), or `None`.
    pub fn throwers(&self) -> Option<Throwers<'_>> {
        Some(Throwers::new(self, self.global_object("Throwers")?))
    }

    /// The `Throwables` global (`game.dll+0x33264C0`), or `None`.
    pub fn throwables(&self) -> Option<Throwables<'_>> {
        Some(Throwables::new(self, self.global_object("Throwables")?))
    }

    /// The `ArcWeapons` global (`game.dll+0x3326C10`), or `None`.
    pub fn arc_weapons(&self) -> Option<ArcWeapons<'_>> {
        Some(ArcWeapons::new(self, self.global_object("ArcWeapons")?))
    }

    /// The `WeaponHeats` global (`game.dll+0x3326D48`), or `None`.
    pub fn weapon_heats(&self) -> Option<WeaponHeats<'_>> {
        Some(WeaponHeats::new(self, self.global_object("WeaponHeats")?))
    }

    /// The `AbilityWeapons` global (`game.dll+0x3326A10`), or `None`.
    pub fn ability_weapons(&self) -> Option<AbilityWeapons<'_>> {
        Some(AbilityWeapons::new(
            self,
            self.global_object("AbilityWeapons")?,
        ))
    }

    /// The `MeleeWeapons` global (`game.dll+0x3326B78`), or `None`.
    pub fn melee_weapons(&self) -> Option<MeleeWeapons<'_>> {
        Some(MeleeWeapons::new(self, self.global_object("MeleeWeapons")?))
    }

    /// The `WeaponCustomizations` global (`game.dll+0x3326A38`), or `None`.
    pub fn weapon_customizations(&self) -> Option<WeaponCustomizations<'_>> {
        Some(WeaponCustomizations::new(
            self,
            self.global_object("WeaponCustomizations")?,
        ))
    }

    /// The `AiSpawnerManager` global (`game.dll+0x33265C8`), or `None`.
    pub fn ai_spawner_manager(&self) -> Option<AiSpawnerManager<'_>> {
        Some(AiSpawnerManager::new(
            self,
            self.global_object("AiSpawnerManager")?,
        ))
    }

    /// The `BugholeManager` global (`game.dll+0x33267B0`), or `None`.
    pub fn bughole_manager(&self) -> Option<BugholeManager<'_>> {
        Some(BugholeManager::new(
            self,
            self.global_object("BugholeManager")?,
        ))
    }

    /// The `VehicleMotionManager` global (`game.dll+0x3326458`), or `None`.
    pub fn vehicle_motion_manager(&self) -> Option<VehicleMotionManager<'_>> {
        Some(VehicleMotionManager::new(
            self,
            self.global_object("VehicleMotionManager")?,
        ))
    }

    /// The `VehicleManager` global (`game.dll+0x3326668`), or `None`.
    pub fn vehicle_manager(&self) -> Option<VehicleManager<'_>> {
        Some(VehicleManager::new(
            self,
            self.global_object("VehicleManager")?,
        ))
    }

    /// The `SquadTransportManager` global (`game.dll+0x3326CC0`), or `None`.
    pub fn squad_transport_manager(&self) -> Option<SquadTransportManager<'_>> {
        Some(SquadTransportManager::new(
            self,
            self.global_object("SquadTransportManager")?,
        ))
    }

    /// The `SquadFormationManager` global (`game.dll+0x3326A88`), or `None`.
    pub fn squad_formation_manager(&self) -> Option<SquadFormationManager<'_>> {
        Some(SquadFormationManager::new(
            self,
            self.global_object("SquadFormationManager")?,
        ))
    }

    /// The `GoreManager` global (`game.dll+0x33264D0`), or `None`.
    pub fn gore_manager(&self) -> Option<GoreManager<'_>> {
        Some(GoreManager::new(self, self.global_object("GoreManager")?))
    }

    /// The `StratagemBlockerManager` global (`game.dll+0x3326568`), or `None`.
    pub fn stratagem_blocker_manager(&self) -> Option<StratagemBlockerManager<'_>> {
        Some(StratagemBlockerManager::new(
            self,
            self.global_object("StratagemBlockerManager")?,
        ))
    }

    /// The `OrbitalAbilityManager` global (`game.dll+0x33267F0`), or `None`.
    pub fn orbital_ability_manager(&self) -> Option<OrbitalAbilityManager<'_>> {
        Some(OrbitalAbilityManager::new(
            self,
            self.global_object("OrbitalAbilityManager")?,
        ))
    }

    /// The `HellpodPayloadManager` global (`game.dll+0x3326560`), or `None`.
    pub fn hellpod_payload_manager(&self) -> Option<HellpodPayloadManager<'_>> {
        Some(HellpodPayloadManager::new(
            self,
            self.global_object("HellpodPayloadManager")?,
        ))
    }

    /// The `UnitManager` global (`game.dll+0x3326508`), or `None`.
    pub fn unit_manager(&self) -> Option<UnitManager<'_>> {
        Some(UnitManager::new(self, self.global_object("UnitManager")?))
    }

    /// The `RotationManager` global (`game.dll+0x33264A0`), or `None`.
    pub fn rotation_manager(&self) -> Option<RotationManager<'_>> {
        Some(RotationManager::new(
            self,
            self.global_object("RotationManager")?,
        ))
    }

    /// The `DisplacementManager` global (`game.dll+0x3326AC8`), or `None`.
    pub fn displacement_manager(&self) -> Option<DisplacementManager<'_>> {
        Some(DisplacementManager::new(
            self,
            self.global_object("DisplacementManager")?,
        ))
    }

    /// The `JumppackManager` global (`game.dll+0x3326BB8`), or `None`.
    pub fn jumppack_manager(&self) -> Option<JumppackManager<'_>> {
        Some(JumppackManager::new(
            self,
            self.global_object("JumppackManager")?,
        ))
    }

    /// The `EagleManager` global (`game.dll+0x3326650`), or `None`.
    pub fn eagle_manager(&self) -> Option<EagleManager<'_>> {
        Some(EagleManager::new(self, self.global_object("EagleManager")?))
    }

    /// The `EncounterManager` global (`game.dll+0x3326618`), or `None`.
    pub fn encounter_manager(&self) -> Option<EncounterManager<'_>> {
        Some(EncounterManager::new(
            self,
            self.global_object("EncounterManager")?,
        ))
    }

    /// The `AiSquadManager` global (`game.dll+0x33265A0`), or `None`.
    pub fn ai_squad_manager(&self) -> Option<AiSquadManager<'_>> {
        Some(AiSquadManager::new(
            self,
            self.global_object("AiSquadManager")?,
        ))
    }

    /// The `SensorEyeManager` global (`game.dll+0x3326520`), or `None`.
    pub fn sensor_eye_manager(&self) -> Option<SensorEyeManager<'_>> {
        Some(SensorEyeManager::new(
            self,
            self.global_object("SensorEyeManager")?,
        ))
    }

    /// The `TurretManager` global (`game.dll+0x3326D70`), or `None`.
    pub fn turret_manager(&self) -> Option<TurretManager<'_>> {
        Some(TurretManager::new(
            self,
            self.global_object("TurretManager")?,
        ))
    }

    /// The `LocomotionManager` global (`game.dll+0x3326CF8`), or `None`.
    pub fn locomotion_manager(&self) -> Option<LocomotionManager<'_>> {
        Some(LocomotionManager::new(
            self,
            self.global_object("LocomotionManager")?,
        ))
    }
}
