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
    HfCombatQueues, HfDestructibles, HfExplosionQueue, HfGame, HfGameMode, HfHealthManager,
    HfInventoryManager, HfLocalUser, HfMotionManager, HfPlayerManager, HfProjectileSystem,
    HfWielderManager,
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
}

global! {
    /// The entities of this machine's world (`game.dll+0x346BF98`).
    ///
    /// The header leaves the type incomplete. The documented layout is: at
    /// `+0xF22EC8` a hash map of network ids to slots (8-byte keys, 4-byte
    /// values), at `+0xF32F20` the network objects by slot (24 bytes each,
    /// `+0` the entity id, u32), at `+0xF1AEB0` the map of entity ids, and at
    /// `+0xF32F18` their descriptors.
    EntityManager
}

impl<'a> EntityManager<'a> {
    const NETWORK_OBJECTS: usize = 0xF32F20;
    const NETWORK_OBJECT_SIZE: usize = 24;

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
    /// The header leaves the type incomplete. The documented layout is:
    /// records of 12 bytes at `+0xC5F78` by world index.
    DestructionRecords
}

impl<'a> DestructionRecords<'a> {
    const RECORDS: usize = 0xC5F78;
    const RECORD_SIZE: usize = 12;

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
