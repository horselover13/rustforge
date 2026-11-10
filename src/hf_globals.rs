// The game's objects (its managers and queues: what the functions of
// `hellforge_functions.h` take) by name, from
// `native/mod-sdk/hellforge_globals.h`.
//
// Each accessor returns the game's own object, or `None`: before
// `HF_EVENT_GAME_LOADED`, on another game build, when the code that witnesses
// it is not the reviewed one, or while the game has not made it (out of a
// mission, some are not).
//
// The game owns the object and keeps it alive; some globals are made again
// each mission. Ask for a global each time you use it rather than keeping it.
// Read it, and write it only where its field says so, on the game thread (an
// event listener's or `on_game_thread`'s call).

#![allow(dead_code)]

use crate::hf_api::{HellForgeApi, util::cstring};
use crate::rust_ffi::{
    HfCombatQueues, HfDestructibles, HfDestructionRecords, HfEntityManager, HfExplosionQueue,
    HfGame, HfGameMode, HfHealthManager, HfInventoryManager, HfLocalUser, HfMotionManager,
    HfPlayerManager, HfProjectileSystem, HfScoreManager, HfWielderManager,
};
use std::ffi::c_void;
use std::ptr::NonNull;

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
                std::mem::size_of::<*mut c_void>(),
            )
        };
        if !self.read(cell, bytes) || object.is_null() {
            return None;
        }

        NonNull::new(object)
    }

    /// The global `name` as a shared reference to `T`.
    ///
    /// The globals are `packed`, so `T` has alignment 1 and any object address
    /// the loader returns is aligned for it.
    fn global_ref<T>(&self, name: &str) -> Option<&T> {
        let object = self.global_object(name)?;

        Some(unsafe { object.cast::<T>().as_ref() })
    }

    /// The global `name` as a non-null pointer to `T`, for globals the header
    /// leaves incomplete.
    fn global_ptr<T>(&self, name: &str) -> Option<NonNull<T>> {
        self.global_object(name).map(NonNull::cast::<T>)
    }

    // The session and its mode.

    /// The Game (`game.dll+0x3326340`): the session's top object, whose state
    /// the game's `ChangeState` sets.
    ///
    /// Its `state` is the game's state (`HF_STATE_*`);
    /// [`game_state`](HellForgeApi::game_state) reads it for you.
    ///
    /// Returns `None` when the game has not made it yet.
    pub fn game(&self) -> Option<&HfGame> {
        self.global_ref("Game")
    }

    /// The game mode (`game.dll+0x33266A0`).
    ///
    /// `mode` is 1 in a mission. `mission` is the mission's data: bit 0 of its
    /// `+0x14` is set on the host, and its `+8` is the game mode's entity.
    pub fn game_mode(&self) -> Option<&HfGameMode> {
        self.global_ref("GameMode")
    }

    // The session's players.

    /// The players of the session (`game.dll+0x3326468`).
    ///
    /// `count` is at most 4. Per slot `i` (0 to `count - 1`): the peer id at
    /// `+0x2C8 + 0x38 i` (u64), its lifecycle at `+0x2E0 + 0x38 i` (u32:
    /// 2 waiting to respawn, 3 spawned), its player entity at
    /// `[+0xE8 + 8 i] + 8` (u32), and its Helldiver's network id at
    /// `+0x3A8 + 0x20 i`. Slots move as players leave: tell players apart by
    /// peer id.
    pub fn player_manager(&self) -> Option<&HfPlayerManager> {
        self.global_ref("PlayerManager")
    }

    /// This machine's user (`game.dll+0x347CEF0`).
    ///
    /// `peer` is this machine's player's peer id, as in `HfPlayerEvent.peer`.
    pub fn local_user(&self) -> Option<&HfLocalUser> {
        self.global_ref("LocalUser")
    }

    // The entities of this machine's world.

    /// The entities of this machine's world (`game.dll+0x346BF98`).
    ///
    /// The header leaves the type incomplete; the documented layout is:
    /// at `+0xF22EC8` a hash map of network ids to slots (8-byte keys,
    /// 4-byte values), at `+0xF32F20` the network objects by slot (24 bytes
    /// each, `+0` the entity id, u32), at `+0xF1AEB0` the map of entity ids,
    /// and at `+0xF32F18` their descriptors.
    pub fn entity_manager(&self) -> Option<NonNull<HfEntityManager>> {
        self.global_ptr("EntityManager")
    }

    // Every entity's health.

    /// Every entity's health (`game.dll+0x3326688`).
    ///
    /// At `+0x1030` a hash map of entity ids to indexes (8-byte keys,
    /// 4-byte values). An index's descriptor names its entity (`+0` resource
    /// u64, `+8` entity u32, `+12` unit, `+16` network id, `+20` bit 0 owned
    /// here); its record holds health (i32) at `+0x14` and its life (u32:
    /// 0 alive, 1 downed, 2 or more dead) at `+0x19C`; its ext record holds
    /// the maximum health (i32) at `+0x14`.
    pub fn health_manager(&self) -> Option<&HfHealthManager> {
        self.global_ref("HealthManager")
    }

    // The entities that move.

    /// The entities that move (`game.dll+0x3326558`).
    ///
    /// At `+0x48A0` a hash map of entity ids to indexes (8-byte keys,
    /// 4-byte values). An index's descriptor names its entity at `+8` (u32)
    /// and has bit 0 of `+0x14` set while it is live; its record starts with
    /// its velocity (3 floats, m/s).
    pub fn motion_manager(&self) -> Option<&HfMotionManager> {
        self.global_ref("MotionManager")
    }

    // This frame's combat requests and the projectiles in flight.

    /// The explosion requests of this frame (`game.dll+0x346D558`):
    /// `RequestExplosion`'s queue.
    ///
    /// `count` is the number queued, 256 at most (`RequestExplosion` refuses
    /// more).
    pub fn explosion_queue(&self) -> Option<&HfExplosionQueue> {
        self.global_ref("ExplosionQueue")
    }

    /// The projectiles in flight (`game.dll+0x347CEA8`): `FireProjectile`'s
    /// system and the projectile pool.
    ///
    /// `running` is 1 while projectiles run (in a mission).
    pub fn projectile_system(&self) -> Option<&HfProjectileSystem> {
        self.global_ref("ProjectileSystem")
    }

    /// This frame's damage and status requests (`game.dll+0x347CF38`), which
    /// the frame's drain applies (`QueueDamage`, `QueueStatusRequest`).
    ///
    /// `damage_count` and `status_count` are the requests queued, 4096 at most
    /// each.
    pub fn combat_queues(&self) -> Option<&HfCombatQueues> {
        self.global_ref("CombatQueues")
    }

    // The players' mission stats.

    /// The players' mission stats (`game.dll+0x3326AE0`).
    ///
    /// The header leaves the type incomplete; the documented layout is: at
    /// `+0x37D58` a map of player entities to indexes, and the index's record
    /// at `+0x37D98 + 0x28 index` holds a stat table and 64 source blocks of
    /// `0x148` bytes at `[record + 0x18]`, each a resource and 16 stat entries
    /// (`HF_STAT_*`).
    pub fn score_manager(&self) -> Option<NonNull<HfScoreManager>> {
        self.global_ptr("ScoreManager")
    }

    // What entities hold.

    /// What entities hold (`game.dll+0x3326420`).
    ///
    /// At `+0x30` a map of entities to instances; an instance's slot 0 (its
    /// hand) is at `[+0x60] + 0x1D0 instance`.
    pub fn wielder_manager(&self) -> Option<&HfWielderManager> {
        self.global_ref("WielderManager")
    }

    /// The Helldivers' inventories (`game.dll+0x3326738`).
    ///
    /// At `+0x28` a map of entities to records; a record's inventory selection
    /// is at `+0x1C` (u32: 1 primary, 2 secondary, 3 support, 4 or 5 a held
    /// item).
    pub fn inventory_manager(&self) -> Option<&HfInventoryManager> {
        self.global_ref("InventoryManager")
    }

    // The world's destructible objects and their destruction state.

    /// The world's destructible objects (`game.dll+0x3326550`).
    ///
    /// Their descriptor pointers are at `+0x4030` and world indexes at
    /// `+0x6030`, one per destructible. `count` is the number of them.
    pub fn destructibles(&self) -> Option<&HfDestructibles> {
        self.global_ref("Destructibles")
    }

    /// The destruction state of the world's destructibles
    /// (`game.dll+0x346D528`).
    ///
    /// The header leaves the type incomplete; the documented layout is:
    /// records of 12 bytes at `+0xC5F78` by world index (`+0` unit, `+7` stage
    /// u8, `+8` bit 0 registered).
    pub fn destruction_records(&self) -> Option<NonNull<HfDestructionRecords>> {
        self.global_ptr("DestructionRecords")
    }
}
