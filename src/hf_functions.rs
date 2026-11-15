// Game functions by name, from `native/mod-sdk/hellforge_functions.h`.

#![allow(dead_code)]

use crate::hf_api::HellForgeApi;
use std::ffi::c_void;
use std::mem::{size_of, transmute_copy};
use std::os::raw::c_char;

/// `ConsumeDamageEvent`: one damage event from this machine's queue.
pub type ConsumeDamageEventFn = unsafe extern "C" fn(
    system: *mut c_void,
    event: *mut u8,
    r8: u64,
    r9: u64,
    s5: u64,
    s6: u64,
) -> u64;

/// `KillDispatch`: tells the game something was killed, on the victim's owner.
pub type KillDispatchFn = unsafe extern "C" fn(
    context: *mut c_void,
    creditor: u64,
    victim: u32,
    dealer: u32,
    kind: u32,
    element: u32,
    s7: u64,
    s8: u64,
) -> u64;

/// `ActivateStratagemBeacon`: activates a stratagem's beacon once it lands.
pub type ActivateStratagemBeaconFn = unsafe extern "C" fn(
    manager: *mut c_void,
    element: i32,
    thrower: u32,
    position: *const f32,
    record: *mut c_void,
    type_: u32,
    s7: u64,
    s8: u64,
    s9: u64,
    s10: u64,
) -> u64;

/// `SpawnEntity`: spawns an entity into this machine's world.
pub type SpawnEntityFn = unsafe extern "C" fn(
    manager: *mut c_void,
    entity: u32,
    resource: u64,
    info: *const u8,
    s5: u64,
    s6: u64,
) -> u64;

/// `RequestExplosion`: requests an explosion in this machine's world.
pub type RequestExplosionFn = unsafe extern "C" fn(
    queue: *mut c_void,
    position: *const f32,
    type_: u32,
    source: u32,
    owner: u32,
    creditor: u64,
    a7: u64,
    a8: u64,
    a9: u64,
    a10: u64,
    a11: u64,
    a12: u64,
    a13: u64,
    a14: u64,
    a15: u64,
) -> u64;

/// `AddHealthFraction`: heals an entity by a share of its maximum health.
pub type AddHealthFractionFn =
    unsafe extern "C" fn(manager: *mut c_void, entity: u32, fraction: f32);

/// `RestoreZone`: restores one damage zone (a limb) to full.
pub type RestoreZoneFn = unsafe extern "C" fn(unused: *mut c_void, entity: u32, zone: u32);

/// `QueueStatusRequest`: queues a status buildup on an entity.
pub type QueueStatusRequestFn = unsafe extern "C" fn(
    unused: *mut c_void,
    status: u32,
    target: u32,
    buildup: f32,
    instigator: u32,
    zero: u32,
);

/// `SetVelocity`: sets an entity's velocity.
pub type SetVelocityFn =
    unsafe extern "C" fn(unused: *mut c_void, entity: u32, velocity: *const f32);

/// `FireProjectile`: fires a projectile.
pub type FireProjectileFn = unsafe extern "C" fn(
    system: *mut c_void,
    type_: u32,
    position: *const f32,
    direction: *const f32,
    entity: u32,
    zero: u32,
    zero2: u64,
);

impl HellForgeApi {
    /// The `ConsumeDamageEvent` function (`game.dll+0x12A7D40`), or `None`.
    ///
    /// It takes one damage event from this machine's queue in the frame's
    /// drain, before `ApplyDamage`'s armour and multipliers.
    pub fn consume_damage_event(&self) -> Option<ConsumeDamageEventFn> {
        self.named("ConsumeDamageEvent")
    }

    /// The `KillDispatch` function (`game.dll+0xB96310`), or `None`.
    pub fn kill_dispatch(&self) -> Option<KillDispatchFn> {
        self.named("KillDispatch")
    }

    /// The `ActivateStratagemBeacon` function (`game.dll+0x6ABDB0`), or
    /// `None`.
    pub fn activate_stratagem_beacon(&self) -> Option<ActivateStratagemBeaconFn> {
        self.named("ActivateStratagemBeacon")
    }

    /// The `SpawnEntity` function (`game.dll+0xFDC140`), or `None`.
    pub fn spawn_entity(&self) -> Option<SpawnEntityFn> {
        self.named("SpawnEntity")
    }

    /// The `RequestExplosion` function (`game.dll+0x13C0A80`), or `None`.
    pub fn request_explosion(&self) -> Option<RequestExplosionFn> {
        self.named("RequestExplosion")
    }

    /// The `AddHealthFraction` function (`game.dll+0x91E920`), or `None`.
    pub fn add_health_fraction(&self) -> Option<AddHealthFractionFn> {
        self.named("AddHealthFraction")
    }

    /// The `RestoreZone` function (`game.dll+0x65B2D0`), or `None`.
    pub fn restore_zone(&self) -> Option<RestoreZoneFn> {
        self.named("RestoreZone")
    }

    /// The `QueueStatusRequest` function (`game.dll+0x129F170`), or `None`.
    pub fn queue_status_request(&self) -> Option<QueueStatusRequestFn> {
        self.named("QueueStatusRequest")
    }

    /// The `SetVelocity` function (`game.dll+0x4A7550`), or `None`.
    pub fn set_velocity(&self) -> Option<SetVelocityFn> {
        self.named("SetVelocity")
    }

    /// The `FireProjectile` function (`game.dll+0x13A8F50`), or `None`.
    pub fn fire_projectile(&self) -> Option<FireProjectileFn> {
        self.named("FireProjectile")
    }

    /// Hooks `ConsumeDamageEvent` with `detour`, at `priority`.
    ///
    /// Returns `true` when the hook is in, or will be installed once the
    /// game's code is decoded and verified.
    ///
    /// # Safety
    /// `detour` must have the exact signature of
    /// [`ConsumeDamageEventFn`]. `original`, when non-null, must point to
    /// storage the loader may write the original function pointer into.
    pub unsafe fn hook_consume_damage_event(
        &self,
        detour: ConsumeDamageEventFn,
        original: *mut ConsumeDamageEventFn,
        priority: i32,
    ) -> bool {
        unsafe {
            self.hook_named(
                "ConsumeDamageEvent",
                detour as *mut c_void,
                original.cast(),
                priority,
            )
        }
    }

    /// Hooks `KillDispatch` with `detour`, at `priority`.
    ///
    /// # Safety
    /// See [`hook_consume_damage_event`](Self::hook_consume_damage_event).
    pub unsafe fn hook_kill_dispatch(
        &self,
        detour: KillDispatchFn,
        original: *mut KillDispatchFn,
        priority: i32,
    ) -> bool {
        unsafe {
            self.hook_named(
                "KillDispatch",
                detour as *mut c_void,
                original.cast(),
                priority,
            )
        }
    }

    /// Hooks `ActivateStratagemBeacon` with `detour`, at `priority`.
    ///
    /// # Safety
    /// See [`hook_consume_damage_event`](Self::hook_consume_damage_event).
    pub unsafe fn hook_activate_stratagem_beacon(
        &self,
        detour: ActivateStratagemBeaconFn,
        original: *mut ActivateStratagemBeaconFn,
        priority: i32,
    ) -> bool {
        unsafe {
            self.hook_named(
                "ActivateStratagemBeacon",
                detour as *mut c_void,
                original.cast(),
                priority,
            )
        }
    }

    /// Hooks `SpawnEntity` with `detour`, at `priority`.
    ///
    /// # Safety
    /// See [`hook_consume_damage_event`](Self::hook_consume_damage_event).
    pub unsafe fn hook_spawn_entity(
        &self,
        detour: SpawnEntityFn,
        original: *mut SpawnEntityFn,
        priority: i32,
    ) -> bool {
        unsafe {
            self.hook_named(
                "SpawnEntity",
                detour as *mut c_void,
                original.cast(),
                priority,
            )
        }
    }

    /// Hooks `RequestExplosion` with `detour`, at `priority`.
    ///
    /// # Safety
    /// See [`hook_consume_damage_event`](Self::hook_consume_damage_event).
    pub unsafe fn hook_request_explosion(
        &self,
        detour: RequestExplosionFn,
        original: *mut RequestExplosionFn,
        priority: i32,
    ) -> bool {
        unsafe {
            self.hook_named(
                "RequestExplosion",
                detour as *mut c_void,
                original.cast(),
                priority,
            )
        }
    }

    /// Resolves a mapped function by name to a non-null typed pointer.
    fn named<T: Copy>(&self, name: &str) -> Option<T> {
        let address = self.function(name);
        if address == 0 {
            return None;
        }

        debug_assert_eq!(size_of::<T>(), size_of::<usize>());
        Some(unsafe { transmute_copy::<usize, T>(&address) })
    }

    /// Hooks a mapped function by name, as `hook_function` does.
    unsafe fn hook_named(
        &self,
        name: &str,
        detour: *mut c_void,
        original: *mut *mut c_void,
        priority: i32,
    ) -> bool {
        unsafe { self.hook_function(name, detour, original, priority) }
    }
}

// Game functions added in SDK 17 to 19. Each `game_fn!` invocation declares the
// typed function pointer, the resolver, and (where the header marks the function
// hookable) the matching `hook_*` method.

macro_rules! game_fn {
    (
        $(#[$meta:meta])*
        $alias:ident / $accessor:ident / $name:literal / - (
            $($arg:ident : $ty:ty),* $(,)?
        ) -> $ret:ty
    ) => {
        $(#[$meta])*
        pub type $alias = unsafe extern "C" fn($($arg: $ty),*) -> $ret;

        impl HellForgeApi {
            #[doc = concat!("The `", $name, "` game function, or `None`.")]
            pub fn $accessor(&self) -> Option<$alias> {
                self.named($name)
            }
        }
    };
    (
        $(#[$meta:meta])*
        $alias:ident / $accessor:ident / $name:literal / $hook:ident (
            $($arg:ident : $ty:ty),* $(,)?
        ) -> $ret:ty
    ) => {
        $(#[$meta])*
        pub type $alias = unsafe extern "C" fn($($arg: $ty),*) -> $ret;

        impl HellForgeApi {
            #[doc = concat!("The `", $name, "` game function, or `None`.")]
            pub fn $accessor(&self) -> Option<$alias> {
                self.named($name)
            }

            #[doc = concat!("Hooks `", $name, "` with `detour`, at `priority`.")]
            ///
            /// Returns `true` when the hook is in, or will be installed once
            /// the game's code is decoded and verified.
            ///
            /// # Safety
            /// `detour` must have the exact signature of the function pointer
            /// above. `original`, when non-null, must point to storage the
            /// loader may write the original function pointer into.
            pub unsafe fn $hook(
                &self,
                detour: $alias,
                original: *mut $alias,
                priority: i32,
            ) -> bool {
                unsafe {
                    self.hook_named($name, detour as *mut c_void, original.cast(), priority)
                }
            }
        }
    };
}

game_fn! {
    /// `game.dll+0x129F910` (call only).
    /// Queues a damage request (4096 a frame: hf_global_CombatQueues' damage_count) that the
    /// frame's drain applies. hf->injure is the checked way to call it, on a limb of this
    /// machine's Helldiver.
    QueueDamageFn / queue_damage / "QueueDamage" / - (unused: *mut c_void, kind: u32, a3: u32, target: u32, amount: u32, a6: u8, network: u32, network2: u32, creditor: u64, a10: u32, f11: f32, f12: f32, f13: f32, direction: *const f32, position: *const f32, actor: u32, a17: u32, a18: u32, scale: f32) -> ()
}

game_fn! {
    /// `game.dll+0x9AD7D0` (call only).
    /// Whether a Helldiver needs ammunition (1) or not (0), as the Supply Pack's self-use asks.
    /// hf->resupply_from_pack calls it before TryStartAction.
    NeedsAmmoFn / needs_ammo / "NeedsAmmo" / - (unused: *mut c_void, entity: u32) -> u8
}

game_fn! {
    /// `game.dll+0xA431D0` (call only).
    /// Starts an action on an entity if it is free to: 1 started, 0 not. hf->resupply_from_pack is
    /// the checked way to call it.
    TryStartActionFn / try_start_action / "TryStartAction" / - (context: *mut c_void, request: *const u32) -> u8
}

game_fn! {
    /// `game.dll+0xFD9710` (call only).
    /// The entity spawn request: makes an entity of a resource at a pose; returns out.
    /// hf->spawn_pelican is the checked way to call it (host only, in a mission, with
    /// shuttle_transport).
    RequestEntitySpawnFn / request_entity_spawn / "RequestEntitySpawn" / - (out: *mut u32, resource: u64, spawn: *mut c_void) -> *mut u32
}

game_fn! {
    /// `game.dll+0x11509E0` (call only).
    /// Runs one tick of an ability, switching on its id through a 2867-entry table: 1 when it is
    /// done, 0 while it runs (and for an unknown id). HellForge points its id check at a stub for
    /// abilities of its own (hf->add_ability).
    AbilityTickFn / ability_tick / "AbilityTick" / - (ctx: *mut c_void, id: u32, tick: u32) -> u8
}

game_fn! {
    /// `game.dll+0x110F270` (call only).
    /// Ability 2088, the Solo Silo missile's armed ability, which its launch starts: tick 0 the
    /// launch blast (explosion 172), tick 2 it leaves the rack, tick 120 it arms and is done (1).
    SiloMissileArmedAbilityFn / silo_missile_armed_ability / "SiloMissileArmedAbility" / - (ctx: *mut c_void, tick: u32) -> u8
}

game_fn! {
    /// `game.dll+0x76B410` (call only).
    /// The missile a silo's targeting remote is linked to; returns out.
    LinkedMissileFn / linked_missile / "LinkedMissile" / - (linkers: *mut c_void, out: *mut u32, remote: u32) -> *mut u32
}

game_fn! {
    /// `game.dll+0x4FD6F0` (call only).
    /// A WeaponLinker instance's record (12 bytes; +4 0x06212002 a silo remote's link, which
    /// launches its missile): this machine's copy for an instance it has authority over, else the
    /// resource's shared one. NULL for no descriptor.
    LinkerRecordFn / linker_record / "LinkerRecord" / - (descriptor: *mut c_void) -> *mut u8
}

game_fn! {
    /// `game.dll+0x76B4C0` (call only).
    /// The hellpod rack a silo's targeting remote is linked to; returns out.
    LinkedRackFn / linked_rack / "LinkedRack" / - (linkers: *mut c_void, out: *mut u32, remote: u32) -> *mut u32
}

game_fn! {
    /// `game.dll+0x645B80` (call only).
    /// Turns a seeking missile's engine on or off, as a silo remote's link does at a launch.
    SetMissileEngineFn / set_missile_engine / "SetMissileEngine" / - (missiles: *mut c_void, missile: u32, on: u8) -> ()
}

game_fn! {
    /// `game.dll+0x645940` (call only).
    /// Launches a seeking missile: sets off its explosive, which starts its armed ability (2088
    /// for the Solo Silo's), and marks it launched (its instance's +0x36).
    LaunchMissileFn / launch_missile / "LaunchMissile" / - (missiles: *mut c_void, missile: u32) -> ()
}

game_fn! {
    /// `game.dll+0x8CE620` (call only).
    /// Names an explosive's owner, as a silo remote's link does for its missile at a launch.
    SetExplosiveOwnerFn / set_explosive_owner / "SetExplosiveOwner" / - (explosives: *mut c_void, entity: u32, owner: u32) -> ()
}

game_fn! {
    /// `game.dll+0x919940` (call only).
    /// Sets an entity's guidance target, as a silo remote's link does with its laser's point.
    SetGuidanceTargetFn / set_guidance_target / "SetGuidanceTarget" / - (unused: *mut c_void, entity: u32, point: *const f32) -> ()
}

game_fn! {
    /// `game.dll+0x9374D0` (call only).
    /// Has a hellpod rack let go of an item, as a silo remote's link does at a launch.
    RackReleaseItemFn / rack_release_item / "RackReleaseItem" / - (racks: *mut c_void, rack: u32, item: u32) -> ()
}

game_fn! {
    /// `game.dll+0x7CAF40` (hook or call).
    /// Starts an ability on an entity; the ability manager then ticks it (AbilityTick).
    /// HellForge's silo countdown starts its ability on the missile with it, and its shield wall
    /// detours it to see enemies' charges.
    StartAbilityFn / start_ability / "StartAbility" / hook_start_ability (abilities: *mut c_void, entity: u32, ability: u32, mode: u32, speed: f32, s6: u64, s7: u64) -> u64
}

game_fn! {
    /// `game.dll+0x472F60` (call only).
    /// Updates an entity's behaviour, switching on its id through a 693-entry table, on the
    /// machine with authority over the entity. HellForge points its id check at a stub for
    /// behaviours of its own (hf->add_behaviour).
    BehaviourUpdateFn / behaviour_update / "BehaviourUpdate" / - (ctx: *mut c_void, id: u32, dt: f32) -> ()
}

game_fn! {
    /// `game.dll+0x48EE50` (call only).
    /// Moves an entity's behaviour to a stage, switching on its id through a 693-entry table.
    /// HellForge points its id check at a stub for behaviours of its own.
    BehaviourStageFn / behaviour_stage / "BehaviourStage" / - (ctx: *mut c_void, id: u32, stage: u32) -> ()
}

game_fn! {
    /// `game.dll+0x4966D0` (call only).
    /// Hands an event to an entity's behaviour, switching on its id through a 693-entry table.
    /// HellForge points its id check at a stub for behaviours of its own.
    BehaviourEventFn / behaviour_event / "BehaviourEvent" / - (ctx: *mut c_void, id: u32, hash: u32) -> ()
}

game_fn! {
    /// `game.dll+0x493BE0` (call only).
    /// Resumes a behaviour on a machine without authority, after its record arrives from the
    /// network: for the transport Pelican (667) it sends the mover to the replicated flight target
    /// (P+0x1BC). HellForge sends its own behaviours into its base's case.
    BehaviourResumeFn / behaviour_resume / "BehaviourResume" / - (ctx: *mut c_void, id: u32) -> ()
}

game_fn! {
    /// `game.dll+0x45C760` (call only).
    /// Behaviour 667's update, the transport Pelican: flies to the beacon (stages 1-3), hovers (6,
    /// entered with P+0x180 = now), releases its cargo after 6 s, departs and is removed.
    /// HellForge's Pelican behaviours (hf->add_behaviour) run it.
    PelicanTransportUpdateFn / pelican_transport_update / "PelicanTransportUpdate" / - (ctx: *mut c_void) -> ()
}

game_fn! {
    /// `game.dll+0x45BD00` (call only).
    /// Behaviour 667's stage change, the transport Pelican.
    PelicanTransportStageFn / pelican_transport_stage / "PelicanTransportStage" / - (ctx: *mut c_void, stage: i32) -> ()
}

game_fn! {
    /// `game.dll+0x45BBE0` (call only).
    /// Behaviour 667's event handler, the transport Pelican.
    PelicanTransportEventFn / pelican_transport_event / "PelicanTransportEvent" / - (ctx: *mut c_void, hash: u32) -> ()
}

game_fn! {
    /// `game.dll+0x4BC5D0` (call only).
    /// A behaviour's entity's own position: its unit's root node from the scene graph; returns
    /// out.
    BehaviourOwnPositionFn / behaviour_own_position / "BehaviourOwnPosition" / - (out: *mut f32, ctx: *mut c_void) -> *mut f32
}

game_fn! {
    /// `game.dll+0x4D5E60` (call only).
    /// The extraction Pelican's orbit step: writes the next point on its orbit to P+0x1BC (the
    /// replicated flight target).
    BehaviourOrbitStepFn / behaviour_orbit_step / "BehaviourOrbitStep" / - (ctx: *mut c_void, position: *const f32, unused: *mut c_void, height: f32, radius: f32) -> ()
}

game_fn! {
    /// `game.dll+0x4D1150` (call only).
    /// Sends a flying behaviour's mover to a point.
    BehaviourMoveToFn / behaviour_move_to / "BehaviourMoveTo" / - (ctx: *mut c_void, target: *const f32) -> ()
}

game_fn! {
    /// `game.dll+0x4C1530` (call only).
    /// Sets the hover controller's desired facing, turning a hovering behaviour's body.
    BehaviourFaceFn / behaviour_face / "BehaviourFace" / - (ctx: *mut c_void, direction: *const f32) -> ()
}

game_fn! {
    /// `game.dll+0x4B0C40` (call only).
    /// Where a behaviour's target is (a turret's aim point); returns out.
    BehaviourTargetPositionFn / behaviour_target_position / "BehaviourTargetPosition" / - (out: *mut f32, ctx: *mut c_void, self_: *const f32) -> *mut f32
}

game_fn! {
    /// `game.dll+0x44F310` (call only).
    /// Behaviour 645's stage 1 update (idle), the Pelican's chin turret.
    PelicanTurretIdleFn / pelican_turret_idle / "PelicanTurretIdle" / - (ctx: *mut c_void, dt: f32) -> ()
}

game_fn! {
    /// `game.dll+0x4B0810` (call only).
    /// Whether a behaviour's turret is active (behaviour 645's firing stage tests it).
    TurretActiveFn / turret_active / "TurretActive" / - (ctx: *mut c_void) -> u8
}

game_fn! {
    /// `game.dll+0x4BB080` (call only).
    /// Whether a behaviour's turret has ammunition (behaviour 645's firing stage tests it).
    TurretHasAmmoFn / turret_has_ammo / "TurretHasAmmo" / - (ctx: *mut c_void) -> u8
}

game_fn! {
    /// `game.dll+0x450AD0` (call only).
    /// Behaviour 645's stage change, the Pelican's chin turret.
    PelicanTurretStageFn / pelican_turret_stage / "PelicanTurretStage" / - (ctx: *mut c_void, stage: i32) -> ()
}

game_fn! {
    /// `game.dll+0x4AF4E0` (call only).
    /// Installs a perception entry as a behaviour's target (P+0x10), or clears it.
    BehaviourSetTargetFn / behaviour_set_target / "BehaviourSetTarget" / - (ctx: *mut c_void, entry: *const u8) -> ()
}

game_fn! {
    /// `game.dll+0x7582D0` (call only).
    /// The angle in degrees between a weapon's forward and a point.
    WeaponDegreesToFn / weapon_degrees_to / "WeaponDegreesTo" / - (unused: *mut c_void, entity: u32, position: *const f32) -> f32
}

game_fn! {
    /// `game.dll+0x8858A0` (call only).
    /// Whether an entity is a valid target: it exists, is alive and has no flag 25.
    IsValidTargetFn / is_valid_target / "IsValidTarget" / - (unused: *mut c_void, entity: u32, info: *mut c_void) -> u8
}

game_fn! {
    /// `game.dll+0x8891C0` (call only).
    /// A perceiver's entry for an entity (+4 its position), or NULL when it has none.
    FindPerceptionEntryFn / find_perception_entry / "FindPerceptionEntry" / - (perception: *mut c_void, perceiver: u32, entity: u32) -> *mut u8
}

game_fn! {
    /// `game.dll+0x742900` (call only).
    /// The rounds in a weapon's magazine.
    MagazineRoundsFn / magazine_rounds / "MagazineRounds" / - (weapons: *mut c_void, entity: u32) -> i32
}

game_fn! {
    /// `game.dll+0x76E080` (call only).
    /// Adds rounds to a weapon's magazine, on the machine with authority over it.
    AddMagazineRoundsFn / add_magazine_rounds / "AddMagazineRounds" / - (unused: *mut c_void, entity: u32, add: i32) -> ()
}

game_fn! {
    /// `game.dll+0x743410` (call only).
    /// A weapon's magazine capacity.
    MagazineCapacityFn / magazine_capacity / "MagazineCapacity" / - (unused: *mut c_void, entity: u32) -> i32
}

game_fn! {
    /// `game.dll+0x4B54D0` (call only).
    /// Whether a behaviour's weapon points within max_degrees of a point: behaviour 645's fire
    /// gate.
    WeaponWithinDegreesFn / weapon_within_degrees / "WeaponWithinDegrees" / - (ctx: *mut c_void, position: *const f32, max_degrees: f32) -> u8
}

game_fn! {
    /// `game.dll+0xFD9D40` (call only).
    /// An entity's descriptor in this machine's world: +0 its resource (u64), +0xC its unit's
    /// index, +0x14 bit 0 set where this machine has authority over it; NULL for none.
    EntityDescriptorFn / entity_descriptor / "EntityDescriptor" / - (entity: u32) -> *mut u8
}

game_fn! {
    /// `game.dll+0x8449B0` (call only).
    /// Marks fields of an entity's behaviour record replicated, so the other machines receive
    /// them; does nothing without authority.
    MarkReplicatedFn / mark_replicated / "MarkReplicated" / - (unused: *mut c_void, entity: u32, mask: u32) -> ()
}

game_fn! {
    /// `game.dll+0x4FB390` (call only).
    /// A shield's record (the stats' Shield record: +0 the radius, +0x78 the bubble unit), as the
    /// game's own readers get it; NULL for no binding.
    ShieldRecordFn / shield_record / "ShieldRecord" / - (binding: *mut c_void) -> *mut u8
}

game_fn! {
    /// `game.dll+0x59FED0` (hook or call).
    /// One object's motion for a frame, on the motion jobs, ending at the single write of its
    /// position (MotionManager's positions). HellForge's shield wall detours it to hold enemies
    /// outside a bubble.
    MoveObjectFn / move_object / "MoveObject" / hook_move_object (motion: *mut c_void, index: u32, dt: f32, r9: u64, s5: u64, s6: u64) -> ()
}

game_fn! {
    /// `game.dll+0x13A9830` (call only).
    /// Spawns a shot into the projectile pool, copying its flight and armour penetration from its
    /// rows; returns its slot (its hit record at pool + 0x4D040 + 0xC8 slot). HellForge redirects
    /// the ProjectileWeapon pellet call (0x616612) to give other players' shots their own rows.
    /// The artillery's shells fly from it, with their ProjectileSettings row's speed (+0x20), drag
    /// (+0x28) and gravity.
    SpawnProjectileFn / spawn_projectile_function / "SpawnProjectile" / - (pool: *mut c_void, descriptor: *mut u8, extra: *mut c_void) -> u32
}

game_fn! {
    /// `game.dll+0x136EC00` (call only).
    /// Appends a record to the item catalogue (hf_global_ProgressionManager), as the login's
    /// catalogue reply does. On the game thread; CatalogueSort after.
    CatalogueAddItemFn / catalogue_add_item / "CatalogueAddItem" / - (unused: u64, mix_id: u32, item_id: u32, parent: u32, field: i32, flag: u8, buy: *const c_void, buy_count: u32, sell: *const c_void, sell_count: u32, dwords: *const u32, dword_count: u32, required: *const u32, required_count: u32, category: u32, max_cap: u32) -> u64
}

game_fn! {
    /// `game.dll+0x136E6A0` (call only).
    /// Sorts the item catalogue again, into its categories' starts (it only adds to them: they are
    /// zeroed first). On the game thread. From KB hellforge-live-mod-sync-re-content; HellForge's
    /// live-added items (ownership.c).
    CatalogueSortFn / catalogue_sort / "CatalogueSort" / - () -> ()
}

game_fn! {
    /// `game.dll+0x136EF80` (call only).
    /// Adds to an owned item as the buy/v2 reply does: no popup, no backend request. On the game
    /// thread.
    AddItemAmountFn / add_item_amount / "AddItemAmount" / - (manager: *mut c_void, mix_id: u32, amount: u32, source: u32, text: *const u8, extra: i32) -> ()
}

game_fn! {
    /// `game.dll+0x136F5E0` (call only).
    /// Takes an amount of an owned item away. On the game thread.
    RemoveItemAmountFn / remove_item_amount / "RemoveItemAmount" / - (manager: *mut c_void, mix_id: u32, amount: u32) -> ()
}

game_fn! {
    /// `game.dll+0xFDB440` (hook or call).
    /// Loads the game's entity data (generated_entities.dl_bin) into the entity state: the entity
    /// map, short ids and component tables. HellForge's weapon and entity registry adds the mods'
    /// entities right after it returns.
    LoadEntitiesFn / load_entities / "LoadEntities" / hook_load_entities (state: *mut c_void) -> ()
}

game_fn! {
    /// `game.dll+0x9F1CC0` (hook or call).
    /// Builds the armory's list from the loadout entries (component 29) into a fixed 193-entry
    /// object: per-type starts at +0x1B24, counts at +0x1B40, 32-byte entries. HellForge's detour
    /// replaces it with a larger list of its own (hf_global_Armory points there).
    BuildArmoryListFn / build_armory_list / "BuildArmoryList" / hook_build_armory_list (list: *mut c_void) -> ()
}

game_fn! {
    /// `game.dll+0x11E7C60` (call only).
    /// The armory entry of a loadout id, found by walking the armory's per-type ranges
    /// (hf_global_Armory): 32 bytes, the loadout id at +0 and the entity at +8 (u64); NULL when
    /// the armory lists none.
    ArmoryEntryOfFn / armory_entry_of / "ArmoryEntryOf" / - (id: u32) -> *mut u8
}

game_fn! {
    /// `game.dll+0x17802E0` (call only).
    /// game.dll's wrapper of the engine's text function (the slot at +1000 of
    /// hf_global_EngineApi's table): the text of an id, as a UTF-8 string. HellForge points that
    /// slot at its own, so mod texts answer here too; the armory calls the slot directly, so hook
    /// the slot, not this, to change texts.
    EngineTextFn / engine_text / "EngineText" / - (id: u32) -> *const u8
}

game_fn! {
    /// `game.dll+0x14C0550` (hook or call).
    /// Opens a menu presenter on this machine's ship UI. HellForge's terminal detour turns
    /// presenter 6 (the information terminal) into 41 (the lobby browser) or opens nothing when an
    /// editor shows its own browser.
    OpenMenuPresenterFn / open_menu_presenter / "OpenMenuPresenter" / hook_open_menu_presenter (manager: *mut c_void, presenter: i32, arg: *mut c_void) -> u64
}

game_fn! {
    /// `game.dll+0x13270B0` (hook or call).
    /// The menu sound manager's update, once a frame on the game thread, in missions and on the
    /// ship. HellForge's detour runs package requests and hf->on_game_thread callbacks there,
    /// before the game's own update.
    MenuSoundUpdateFn / menu_sound_update / "MenuSoundUpdate" / hook_menu_sound_update (manager: *mut c_void, dt: f32) -> ()
}

game_fn! {
    /// `game.dll+0x12FE350` (call only).
    /// Takes a reference on packages so the engine loads and keeps them. Game thread only: it
    /// changes the system's reference map without a lock, and a full map writes to address 0.
    /// hf->load_package is the checked way to call it.
    RequestPackagesFn / request_packages / "RequestPackages" / - (system: *mut c_void, ids: *const u64, count: u32) -> ()
}

game_fn! {
    /// `game.dll+0xFD9BA0` (call only).
    /// The entity a network id names on this machine now, written to out (and out returned).
    /// hf->entity_of_network is the checked way to call it.
    EntityOfNetworkFn / entity_of_network_function / "EntityOfNetwork" / - (out: *mut u32, network: u32) -> *mut u32
}

game_fn! {
    /// EntityOfHandle: EntityOfNetwork's former name, kept for mods written with it; use
    /// EntityOfNetwork.
    EntityOfHandleFn / entity_of_handle / "EntityOfHandle" / - (out: *mut u32, network: u32) -> *mut u32
}

game_fn! {
    /// `game.dll+0x18E9D50` (hook or call).
    /// The loadout unit-preview query: the record a loadout slot's preview shows; a missing
    /// loadout row gives a zero record. HellForge's detour breaks its fallback's endless self-call
    /// (G-8 Immolation in the hellpod).
    QueryLoadoutPreviewFn / query_loadout_preview / "QueryLoadoutPreview" / hook_query_loadout_preview (out: *mut c_void, alternate: u8, slot: u32, mode: u32, context: *mut c_void) -> *mut c_void
}

game_fn! {
    /// `game.dll+0x11F5010` (call only).
    /// The armory's commit: pushes this machine's player's loadout from the profile to its
    /// PlayerHistory record (hf_global_PlayerHistory), the way a boot's validation applies a
    /// changed weapon. Game thread; its request is handled a few frames later. From HellForge's
    /// loadout scrub (loadout_scrub.c).
    PushArmoryLoadoutFn / push_armory_loadout / "PushArmoryLoadout" / - () -> ()
}

game_fn! {
    /// `game.dll+0xF3A800` (hook or call).
    /// Level generation's QueueRandomCacheLootRewardsPackages pass: rolls each random loot cache
    /// against the generation settings' cache loot tables. HellForge's detour sets the Super
    /// Credits tables' weight to the plan's Forge Credits chance before the roll.
    RollCacheLootFn / roll_cache_loot / "RollCacheLoot" / hook_roll_cache_loot (params: *mut c_void, rdx: u64, context: *mut u8, r9: u64) -> u64
}

game_fn! {
    /// `game.dll+0x13714F0` (call only).
    /// Builds the buy request for a warbond item and records it as the progression's pending buy;
    /// its callers are the warbond page view and the MVVM popup. HellForge patches its entry so
    /// the currency is the item's own. HellForge's warbond items replace its first bytes with a
    /// jump, so hf_fn_ gives NULL while they are installed.
    RequestWarbondPurchaseFn / request_warbond_purchase / "RequestWarbondPurchase" / - (rcx: u64, item: u32, cost: i32, currency: u32) -> ()
}

game_fn! {
    /// `game.dll+0x13751D0` (hook or call).
    /// Completes the reply to a warbond buy: charges the pending buy and appends its item, once,
    /// to the purchased list (count +0xD8A58, ids from +0xD8A5C). HellForge's detour takes a
    /// repeatable mod item out of the list again so it sells again.
    FinishWarbondPurchaseFn / finish_warbond_purchase / "FinishWarbondPurchase" / hook_finish_warbond_purchase (progression: *mut u8) -> ()
}

game_fn! {
    /// `game.dll+0x13050B0` (hook or call).
    /// Spawns the Helldiver shown beside a warbond's card from its character row (kit, pose and
    /// weapon), on a card change. HellForge's detour fills a mod warbond's row once the catalogue
    /// and armory are ready.
    SpawnWarbondShowcaseFn / spawn_warbond_showcase / "SpawnWarbondShowcase" / hook_spawn_warbond_showcase (unused: *mut c_void, character: u32) -> ()
}

game_fn! {
    /// `game.dll+0x194A700` (hook or call).
    /// Updates the "#COUNT Medal(s) spent" widget on the Acquisition Center's cards and a
    /// warbond's page header. HellForge's detour gives it the text of the warbond's own currency.
    UpdateWarbondSpentFn / update_warbond_spent / "UpdateWarbondSpent" / hook_update_warbond_spent (widget: *mut u8, a: f32, b: f32) -> ()
}

game_fn! {
    /// `game.dll+0x1947950` (call only).
    /// Lays out and lights the Acquisition Center pager's 32 dots for the card count and the
    /// current card. HellForge redirects its one call (0x194345A) to show pages of 32 past 32
    /// warbonds.
    UpdateWarbondPagerFn / update_warbond_pager / "UpdateWarbondPager" / - (pager: *mut u8, dt: f32) -> ()
}

game_fn! {
    /// `game.dll+0x1947730` (call only).
    /// The pager's input: a clicked dot becomes the card to select (+0x7B14); nonzero when that
    /// changed. HellForge redirects its one call (0x1943384).
    WarbondPagerInputFn / warbond_pager_input / "WarbondPagerInput" / - (pager: *mut u8, rdx: u64, r8: u64, r9: u64) -> u32
}

game_fn! {
    /// `game.dll+0x143BF90` (call only).
    /// Sets a UI text element's text id, which it then resolves through the engine's text function
    /// (mod texts included). The stratagem picker's headings and the warbond spent widget use it.
    SetElementTextFn / set_element_text / "SetElementText" / - (element: *mut c_void, text: u32) -> ()
}

game_fn! {
    /// `game.dll+0x1441720` (call only).
    /// SetElementText for text elements of kind 0x200000.
    SetElementTextAltFn / set_element_text_alt / "SetElementTextAlt" / - (element: *mut c_void, text: u32) -> ()
}

game_fn! {
    /// `game.dll+0x143CAD0` (call only).
    /// Fills a number variable in a UI text element's text.
    SetElementNumberFn / set_element_number / "SetElementNumber" / - (element: *mut c_void, name: u32, value: u32) -> ()
}

game_fn! {
    /// `game.dll+0x14508E0` (call only).
    /// Shows or hides a UI widget. HellForge's locked-warbond button calls it on the button's
    /// price.
    SetWidgetVisibleFn / set_widget_visible / "SetWidgetVisible" / - (widget: *mut c_void, visible: u8) -> ()
}

game_fn! {
    /// `game.dll+0x1448690` (call only).
    /// Sets a UI element's colour. HellForge's warbond prices call it to tint a sample price in
    /// the sample's colour.
    SetElementColourFn / set_element_colour / "SetElementColour" / - (element: *mut c_void, rgb: *const f32) -> ()
}

game_fn! {
    /// `game.dll+0x18CEC00` (call only).
    /// Fills a stratagem picker section heading (its title and its available and total counts).
    /// HellForge redirects its call in the list's realize pass (0x18D2E85) to title mods'
    /// selection categories (hf->add_stratagem_category).
    SetPickerHeadingFn / set_picker_heading / "SetPickerHeading" / - (widget: *mut u8, key: u32) -> ()
}

game_fn! {
    /// `game.dll+0x136FC20` (call only).
    /// Whether the player has a stratagem available to take, as the picker's heading counts it.
    StratagemAvailableFn / stratagem_available / "StratagemAvailable" / - (account: *mut c_void, type_: u32) -> u8
}

game_fn! {
    /// `game.dll+0x20BE230` (call only).
    /// Sorts an array with a comparator, qsort's arguments. The stratagem picker sorts its
    /// candidates with it (call 0x18D8C40), which HellForge redirects to group mods' categories.
    SortArrayFn / sort_array / "SortArray" / - (data: *mut c_void, count: u64, width: u64, compare: *const c_void) -> ()
}

game_fn! {
    /// `game.dll+0x11EBB40` (call only).
    /// Returns the highest difficulty level a player may choose: min(10, the online override
    /// "0xAF218CAC 0xFAABBEA9", which it finds with FindConfigSetting). Every read of the
    /// difficulty table clamps the level to it; once a mod's add_difficulty is installed its 10 is
    /// 11. From HellForge's level 11 (difficulty.c, DIF_MAX_FN and the live override).
    HighestDifficultyFn / highest_difficulty / "HighestDifficulty" / - () -> u32
}

game_fn! {
    /// `game.dll+0x12E7990` (call only).
    /// Finds a config setting by key in the config's open-addressing tables of 48-byte entries.
    /// Returns the setting (+0xC its type, 6 a u32; +0x10 its value) or NULL.
    FindConfigSettingFn / find_config_setting / "FindConfigSetting" / - (config: *mut c_void, online: u32, key: u32) -> *const u8
}

game_fn! {
    /// `game.dll+0x19A28D0` (call only).
    /// Shows the chosen widget of the war table's difficulty bar: its level's number, name and
    /// icon from the difficulty table, and its band sprite (a 10-entry array read by level - 1).
    ShowDifficultyWidgetFn / show_difficulty_widget / "ShowDifficultyWidget" / - (widget: *mut c_void, level: u32) -> ()
}

game_fn! {
    /// `game.dll+0x19C4E70` (call only).
    /// Rates the mission on the pause map and at mission end against a table of 10 rows of
    /// thresholds, indexed by the mission's level - 1.
    ComputeMissionRatingFn / compute_mission_rating / "ComputeMissionRating" / - (rcx: *mut c_void, rdx: *mut c_void, r8: u32, r9: u32) -> f32
}

game_fn! {
    /// `game.dll+0x94EB90` (hook or call).
    /// The enemy director's per-frame step: reads each faction's roaming patrol interval
    /// (difficulty row +0x2A0, f32 s) and most living patrol enemies (+0x2AC, u32) from the
    /// mission's difficulty row.
    EnemyDirectorTickFn / enemy_director_tick / "EnemyDirectorTick" / hook_enemy_director_tick (hivemind: *mut c_void, dt: f32) -> ()
}

game_fn! {
    /// `game.dll+0x173B8D0` (call only).
    /// Allocates from an arena; the level generator's passes take their scratch tables from the
    /// generation arena with it.
    ArenaAllocateFn / arena_allocate / "ArenaAllocate" / - (arena: *mut c_void, size: u32, alignment: u32) -> *mut c_void
}

game_fn! {
    /// `game.dll+0x208AAA0` (call only).
    /// game.dll's memset.
    GameMemsetFn / game_memset / "GameMemset" / - (dest: *mut c_void, value: i32, size: u64) -> *mut c_void
}

game_fn! {
    /// `game.dll+0xF6E710` (call only).
    /// Makes the level generator's generation block G (the level's locations, their types, the
    /// minimap's arrays, the mission stamps, the objective list, places and clusters), once per
    /// world (from 0xAB8B70).
    CreateGenerationBlockFn / create_generation_block / "CreateGenerationBlock" / - (level: *mut c_void, rdx: *mut c_void) -> ()
}

game_fn! {
    /// `game.dll+0xF71220` (call only).
    /// Level init: copies the generation settings' radii into the level (lg+0x2D78..+0x2D8C), sets
    /// the map type (lg+0x2DE0) and the view rect a war-table preview is drawn in (lg+0x2D94).
    InitLevelFn / init_level / "InitLevel" / - (level: *mut c_void, rdx: *mut c_void, r8: u32, r9: u64) -> ()
}

game_fn! {
    /// `game.dll+0xFD03E0` (call only).
    /// The level generator's route cells: a Poisson disc of sites at the generation settings'
    /// spacing (+4, 40 m) in the disk (+0xC, 680 m) around the world's centre, into fixed arrays
    /// (975 vertices, 1621 edges, 1024 cells) it does not check.
    GenerateRouteCellsFn / generate_route_cells / "GenerateRouteCells" / - (rcx: *mut c_void, rdx: u32, r8: u64, xmm3: f32) -> ()
}

game_fn! {
    /// `game.dll+0xF93800` (call only).
    /// Level generator pass op 3: fills each route cell with points at its cell type's size
    /// (+0x18C) / 2 (grid) or / 4 (Poisson) and builds their diagrams into the block's areas
    /// (13224) and vertices (26205), unchecked.
    GenerateRouteCellVoronoiFn / generate_route_cell_voronoi / "GenerateRouteCellVoronoi" / - (rcx: *mut c_void, rdx: *mut c_void, pass: *mut c_void) -> ()
}

game_fn! {
    /// `game.dll+0xF64E30` (call only).
    /// Level generator pass op 4: gives every area of a route cell the height of its cell type's
    /// curve at t = clamp(d / 64 m), d its distance to the nearest route point.
    GenerateSubregionHeightsFn / generate_subregion_heights / "GenerateSubregionHeights" / - (rcx: *mut c_void, unused: u64, pass: *mut c_void) -> ()
}

game_fn! {
    /// `game.dll+0xF3F8E0` (call only).
    /// Level generator pass op 0x25: lays a 16 m grid (generation settings +0) over the site
    /// square, keeps the points within the disk and runs jc_voronoi on them (arrays of 5680 sites,
    /// 5849 vertices, 11528 edges, unchecked).
    GenerateVoronoiFn / generate_voronoi / "GenerateVoronoi" / - (rcx: *mut c_void, rdx: *mut c_void, pass: *mut c_void) -> ()
}

game_fn! {
    /// `game.dll+0xF68050` (call only).
    /// Level generator pass op 0xA4 on map type 1: rebuilds the route cells from points laid over
    /// the whole site square at each cell type's size (+0x18C), grid or Poisson. A failure has the
    /// level generated again.
    FinalizeVoronoiFn / finalize_voronoi / "FinalizeVoronoi" / - (rcx: *mut c_void, rdx: *mut c_void, pass: *mut c_void) -> ()
}

game_fn! {
    /// `game.dll+0xF4C100` (call only).
    /// Level generator pass op 0x3E (force_flatten_zones): groups connected cells whose type sets
    /// a height (+0x208) and flattens every zone cell to its group's height, one plateau per town.
    ForceFlattenZonesFn / force_flatten_zones / "ForceFlattenZones" / - (rcx: *mut c_void, unused: u64, pass: *mut c_void) -> ()
}

game_fn! {
    /// `game.dll+0xF37500` (call only).
    /// Gives each route cell of a type 0/1 map a subregion by a noise value sampled at the cell
    /// (SampleSubregionNoise): its zone cells become the map's towns.
    SelectSubRegionCellsFromHeightGroupFn / select_sub_region_cells_from_height_group / "SelectSubRegionCellsFromHeightGroup" / - (rcx: *mut c_void, rdx: *mut c_void, pass: *mut c_void) -> ()
}

game_fn! {
    /// `game.dll+0xF3A2E0` (call only).
    /// The noise value at a point, which picks a route cell's subregion.
    SampleSubregionNoiseFn / sample_subregion_noise / "SampleSubregionNoise" / - (noise: *mut c_void, x: f32, y: f32) -> f32
}

game_fn! {
    /// `game.dll+0xF79520` (call only).
    /// Level generator pass op 0x18: groups the level's locations (bases, outposts, objectives)
    /// into clusters of 24 (G+0x896FB0, 8 x 0xE0: members +0x10, count +0xD0, kind +0xD8), a few
    /// per kind, their centres on a ring at 0.6 to 0.8 of the playable radius.
    GenerateLocationClustersFn / generate_location_clusters / "GenerateLocationClusters" / - (rcx: *mut c_void, rdx: *mut c_void, pass: *mut c_void) -> ()
}

game_fn! {
    /// `game.dll+0xF7C310` (call only).
    /// Level generator pass op 0x8B: places each location as near its cluster's centre as the gaps
    /// allow, sorted by distance from the nearest location of its kind. A failure has the level
    /// generated again.
    SpreadLocationsFn / spread_locations / "SpreadLocations" / - (rcx: *mut c_void, rdx: *mut c_void, pass: *mut c_void) -> ()
}

game_fn! {
    /// `game.dll+0xFB41B0` (call only).
    /// Level generator pass op 0x8C (SpreadLocations_Cities): spreads the important locations
    /// (G+0x11A715C of 256) on city maps, flagging each in a 128-entry stack array.
    SpreadLocationsCitiesFn / spread_locations_cities / "SpreadLocationsCities" / - (rcx: *mut c_void, unused: u64, pass: *mut c_void) -> ()
}

game_fn! {
    /// `game.dll+0xF7E250` (call only).
    /// Places the level's outposts from the location-group sets: an item is offered only if a
    /// class in its list (+0x198, count +0x1A0) has weight on the map's layout and its difficulty
    /// word (+0x60) admits the mission's level (lg+0xF9).
    GenerateOutpostLocationsFn / generate_outpost_locations / "GenerateOutpostLocations" / - (rcx: *mut c_void, rdx: *mut c_void, pass: *mut c_void) -> ()
}

game_fn! {
    /// `game.dll+0xF8F1D0` (call only).
    /// Places the mission's objectives on the map: one place per placement attempt, retries
    /// included (G+0x8ABCC0, 16 pointers, count G+0x11A7170), unchecked.
    GenerateObjectiveLocationsFn / generate_objective_locations / "GenerateObjectiveLocations" / - (rcx: *mut c_void, rdx: *mut c_void, pass: *mut c_void) -> ()
}

game_fn! {
    /// `game.dll+0xF90DC0` (call only).
    /// Adds the extraction points to the level's objective and extraction places (G+0x8ABCC0,
    /// count G+0x11A7170).
    AddImportantLocationsFn / add_important_locations / "AddImportantLocations" / - (rcx: *mut c_void, rdx: *mut c_void, pass: *mut c_void) -> ()
}

game_fn! {
    /// `game.dll+0xF8E530` (call only).
    /// Fills the mission stamp list (G+0x8BBD70, 128 x 0x10: set, group, item, objective id),
    /// stamps made per objective, without a check.
    GenerateMissionStampListFn / generate_mission_stamp_list / "GenerateMissionStampList" / - (rcx: *mut c_void, rdx: *mut c_void, pass: *mut c_void) -> ()
}

game_fn! {
    /// `game.dll+0xF67980` (call only).
    /// Opens pockets of route cells that have no walkable neighbour; when it cannot, the level is
    /// generated again (lg+0xF954 counts the restarts).
    FixUnreachableAreasFn / fix_unreachable_areas / "FixUnreachableAreas" / - (rcx: *mut c_void, rdx: *mut c_void, pass: *mut c_void) -> ()
}

game_fn! {
    /// `game.dll+0xF25A00` (call only).
    /// Level generator op 0x6D, per unit: keeps a level's units within the playable radius (1024 x
    /// lg+0x2D70) and strips the rest at random, fading out to the border (lg+0x2D74).
    CullUnitsOutsidePlayableAreaFn / cull_units_outside_playable_area / "CullUnitsOutsidePlayableArea" / - (level: *mut c_void, rdx: *mut c_void, r8: u32, xmm3: f32) -> u8
}

game_fn! {
    /// `game.dll+0xFA9E10` (call only).
    /// Level generator pass op 0xA7 (generate_city_roads): a footprint per road edge, then the
    /// city cells' buildings in cell order, stopping when the footprint list is full.
    GenerateCityRoadsFn / generate_city_roads / "GenerateCityRoads" / - (rcx: *mut c_void, rdx: *mut c_void, pass: *mut c_void) -> ()
}

game_fn! {
    /// `game.dll+0xFA8FE0` (call only).
    /// Gives the city pass record its footprint list: 0x800 x 0x60 from the generation arena, its
    /// capacity at record +0x284.
    CreateCityFootprintListFn / create_city_footprint_list / "CreateCityFootprintList" / - (rcx: *mut c_void, rdx: *mut c_void, r8: *mut c_void, s5: u64, s6: *mut c_void) -> u8
}

game_fn! {
    /// `game.dll+0xFAF130` (call only).
    /// Dresses each city street: road ribbons and junction discs, the pieces at an edge's ends
    /// (+0x240, +0x250) and spaced street props (+0x270) up to their reach past the playable
    /// radius.
    FlattenCityRoadsFn / flatten_city_roads / "FlattenCityRoads" / - (rcx: *mut c_void, rdx: *mut c_void, pass: *mut c_void) -> ()
}

game_fn! {
    /// `game.dll+0xC048E0` (call only).
    /// The size of the level generator's stamp index (10757 slots after a bitset).
    StampIndexSizeFn / stamp_index_size / "StampIndexSize" / - (xmm0: f32, xmm1: f32, xmm2: f32) -> u32
}

game_fn! {
    /// `game.dll+0xC04AB0` (call only).
    /// Checks the layout of the level generator's stamp index against its slot count.
    StampIndexCheckLayoutFn / stamp_index_check_layout / "StampIndexCheckLayout" / - (xmm0: f32, xmm1: f32, xmm2: f32) -> ()
}

game_fn! {
    /// `game.dll+0x172F790` (call only).
    /// The game's open-addressing hash insert (704 callers): probes every slot for the key or an
    /// empty one. HellForge's guard skips and counts an entry when the table is full.
    HashMapInsertFn / hash_map_insert / "HashMapInsert" / - (table: *mut c_void, rdx: u32, r8: u32) -> ()
}

game_fn! {
    /// `game.dll+0x13DF7D0` (call only).
    /// Sets up the destruction-record map of hf_global_DestructionRecords: its slot pointer
    /// (+0xC5F48) and count (+0xC5F50; 0x8000 inline slots at +0x5F48).
    SetupDestructionRecordMapFn / setup_destruction_record_map / "SetupDestructionRecordMap" / - (owner: *mut c_void) -> ()
}

game_fn! {
    /// `game.dll+0x13E0200` (call only).
    /// Registers the world's static destructibles in the destruction-record map, probing it
    /// inline.
    RegisterStaticDestructionRecordsFn / register_static_destruction_records / "RegisterStaticDestructionRecords" / - (rcx: *mut c_void, rdx: *mut c_void, r8: u32) -> ()
}

game_fn! {
    /// `game.dll+0x1756730` (call only).
    /// Selects a mission's objectives from its mission row and returns how many. Every caller uses
    /// it: war table, mission descriptor, preview, telemetry; HellForge's second primary runs it
    /// again for a partner mission.
    SelectObjectivesFn / select_objectives / "SelectObjectives" / - (inputs: *mut u8, ids: *mut u32, classes: *mut u32) -> u32
}

game_fn! {
    /// `game.dll+0xAD0010` (call only).
    /// Launches the mission: lists the entity deltas its HiveMind gets (the mission row's, the
    /// modifiers', up to 10 resource hashes) before the spawn applies them.
    LaunchMissionFn / launch_mission / "LaunchMission" / - (rcx: *mut c_void) -> ()
}

game_fn! {
    /// `game.dll+0x17BA940` (call only).
    /// The mission's timer widget: the countdown's total less what is left
    /// (hf_global_HiveMindManager +0x51948) is the elapsed time and its segments.
    MissionTimerWidgetFn / mission_timer_widget / "MissionTimerWidget" / - (widget: *mut c_void) -> ()
}

game_fn! {
    /// `game.dll+0x18B42F0` (call only).
    /// The in-mission HUD's objective updater: copies every objective instance's id
    /// (hf_global_ObjectiveManager) into a 16-entry stack list to number them, and gives each map
    /// marker one of the map's 16 marker slots.
    UpdateHudObjectivesFn / update_hud_objectives / "UpdateHudObjectives" / - (rcx: *mut c_void) -> ()
}

game_fn! {
    /// `game.dll+0x18A7C60` (call only).
    /// Runs every frame while the map is open, through the HUD's objective updater
    /// (UpdateHudObjectives).
    UpdateMapScreenFn / update_map_screen / "UpdateMapScreen" / - (map: *mut c_void, xmm1: f32, r8: u32) -> ()
}

game_fn! {
    /// `game.dll+0x18A5580` (call only).
    /// Builds the map object's 16 marker slots (+0xF958, 0x2B8 each; its tracker list follows at
    /// +0x12518).
    BuildMapMarkerSlotsFn / build_map_marker_slots / "BuildMapMarkerSlots" / - (map: *mut c_void, unused: u64, r8: *mut c_void, r9: *mut c_void) -> ()
}

game_fn! {
    /// `game.dll+0x17CF690` (call only).
    /// The HUD's list of the mission's class 0 and 1 objectives (main objectives and steps), 16 at
    /// most.
    HudMandatoryObjectivesFn / hud_mandatory_objectives / "HudMandatoryObjectives" / - (rcx: *mut c_void) -> ()
}

game_fn! {
    /// `game.dll+0x17CB8E0` (call only).
    /// The mission HUD's top bar: shows the difficulty row's level number and icon (row +0x10,
    /// through SetImageWithBase).
    MissionHudTopBarFn / mission_hud_top_bar / "MissionHudTopBar" / - (rcx: *mut c_void, xmm1: f32) -> ()
}

game_fn! {
    /// `game.dll+0x62EC20` (call only).
    /// Builds the mission result record: its badges (+0x14C8: {id, amount} x 32, count +0x15C8),
    /// up to 11 before its objective loop, one per objective instance with a badge, up to 5 after.
    BuildMissionResultFn / build_mission_result / "BuildMissionResult" / - (unused: u64, rdx: *mut c_void, r8: *mut c_void) -> ()
}

game_fn! {
    /// `game.dll+0x148F2A0` (call only).
    /// Writes the SelectedMission telemetry line when a mission is chosen on the war table:
    /// difficulty, race, game mode and objective names, into a 512-byte stack buffer.
    TelemetrySelectedMissionFn / telemetry_selected_mission / "TelemetrySelectedMission" / - (rcx: *mut c_void, rdx: u8) -> u8
}

game_fn! {
    /// `game.dll+0x1347A10` (call only).
    /// The mission-end telemetry: keeps up to 16 objective instances in hf_global_Telemetry's
    /// record (+0x31820).
    TelemetryMissionEndFn / telemetry_mission_end / "TelemetryMissionEnd" / - (telemetry: *mut c_void) -> ()
}

game_fn! {
    /// `game.dll+0x9F3250` (call only).
    /// A SEAF Artillery side objective's spawn point rolls its shell: each kind in the unit's
    /// script data table 0x3268FC21 ({kind, weight}) is matched to its ObjectiveShell record and
    /// one is picked by weight with the objective's seeded random numbers.
    RollArtilleryShellFn / roll_artillery_shell / "RollArtilleryShell" / - (unused: u64, rdx: *mut c_void, r8: u32, r9: u32) -> ()
}

game_fn! {
    /// `game.dll+0x617E50` (call only).
    /// The SEAF Artillery's aim: reads the shell's speed and gravity from its projectile row,
    /// through SolveBallisticAim.
    ArtilleryAimFn / artillery_aim / "ArtilleryAim" / - (rcx: *mut c_void, rdx: u32, r8: *mut c_void, r9: u8, s5: *mut c_void) -> u8
}

game_fn! {
    /// `game.dll+0x13B1BD0` (call only).
    /// The ballistic solver the artillery's aim uses, with the projectile row's speed and gravity.
    SolveBallisticAimFn / solve_ballistic_aim / "SolveBallisticAim" / - (rcx: *mut c_void, rdx: *mut c_void, r8: *mut c_void, r9: u8, s5: u8, s6: u32, s7: f32) -> f32
}

game_fn! {
    /// `game.dll+0x13B18B0` (call only).
    /// Predicts where a projectile lands, from its row's speed and gravity.
    PredictProjectileLandingFn / predict_projectile_landing / "PredictProjectileLanding" / - (rcx: *mut c_void, rdx: *mut c_void, xmm2: f32, r9: *mut c_void, s5: u32, s6: *mut f32) -> ()
}

game_fn! {
    /// `game.dll+0x11E44D0` (call only).
    /// Generates the war table mission board's region missions (Super Earth's mega cities: slots
    /// 30..109 of a planet's 110, 10 levels per region).
    GenerateRegionMissionsFn / generate_region_missions / "GenerateRegionMissions" / - (rcx: *mut c_void, rdx: *mut c_void, r8: *mut c_void) -> ()
}

game_fn! {
    /// `game.dll+0x11E3C10` (call only).
    /// Generates a campaign planet's missions on the war table's mission board (slots 0..29, 3 per
    /// level).
    GenerateCampaignPlanetMissionsFn / generate_campaign_planet_missions / "GenerateCampaignPlanetMissions" / - (rcx: *mut c_void, rdx: *mut c_void, r8: *mut c_void) -> ()
}

game_fn! {
    /// `game.dll+0x11E4060` (call only).
    /// Generates a defence planet's missions on the war table's mission board (slots 0..29, 3 per
    /// level).
    GenerateDefencePlanetMissionsFn / generate_defence_planet_missions / "GenerateDefencePlanetMissions" / - (rcx: *mut c_void, rdx: *mut c_void, r8: *mut c_void) -> ()
}

game_fn! {
    /// `game.dll+0x19CFB40` (call only).
    /// Builds the war table's difficulty bar: a carousel of one widget per level up to the highest
    /// (bar + 0x598 + i 0x910, room for 10).
    DifficultyBarBuildFn / difficulty_bar_build / "DifficultyBarBuild" / - (bar: *mut c_void, rdx: *mut c_void, r8: *mut c_void, r9: u32) -> ()
}

game_fn! {
    /// `game.dll+0x19D1090` (call only).
    /// Destroys the war table difficulty bar's widgets.
    DifficultyBarDestroyFn / difficulty_bar_destroy / "DifficultyBarDestroy" / - (bar: *mut c_void) -> ()
}

game_fn! {
    /// `game.dll+0x19D1110` (call only).
    /// Chooses a level on the war table's difficulty bar.
    DifficultyBarChooseFn / difficulty_bar_choose / "DifficultyBarChoose" / - (bar: *mut c_void, rdx: u32, r8: u8, r9: *mut c_void) -> ()
}

game_fn! {
    /// `game.dll+0x144F800` (call only).
    /// Sets an image element to an atlas sprite (atlas sprites only), which gives the element its
    /// resource (+0x148).
    SetImageSpriteFn / set_image_sprite / "SetImageSprite" / - (element: *mut c_void, sprite: u64, zero: u8) -> ()
}

game_fn! {
    /// `game.dll+0x1450160` (call only).
    /// The image setter the stratagem icons use: an atlas sprite, else a texture of that name
    /// (through SetElementResourceSlot).
    SetImageSpriteOrTextureFn / set_image_sprite_or_texture / "SetImageSpriteOrTexture" / - (element: *mut c_void, resource: u64) -> ()
}

game_fn! {
    /// `game.dll+0x1449A70` (call only).
    /// Puts a resource into a UI element's slot: what SetImageSpriteOrTexture calls for a texture.
    /// HellForge gives a level-11 icon its texture with it.
    SetElementResourceSlotFn / set_element_resource_slot / "SetElementResourceSlot" / - (element: *mut c_void, slot: u32, resource: u64, sprite: u8, zero: u8) -> ()
}

game_fn! {
    /// `game.dll+0x1450230` (call only).
    /// Sets an image element from a base sprite and an image: the mission HUD's top bar sets the
    /// difficulty icon with it.
    SetImageWithBaseFn / set_image_with_base / "SetImageWithBase" / - (element: *mut c_void, base: u64, image: u64, zero: u8) -> ()
}

game_fn! {
    /// `game.dll+0x143EEF0` (call only).
    /// A UI element's rect: the mission card's minimap calls it after setting its UV rect.
    UiElementRectFn / ui_element_rect / "UiElementRect" / - (element: *mut c_void, rdx: *mut c_void, r8: *mut c_void) -> ()
}

game_fn! {
    /// `game.dll+0x18760E0` (call only).
    /// The war table mission card's minimap (element panel+0x220): a fixed UV rect of the 2048 m
    /// texture (0.275 to 0.725) and its markers at fixed scales.
    MissionCardMinimapFn / mission_card_minimap / "MissionCardMinimap" / - (rcx: *mut c_void, xmm1: f32) -> ()
}

game_fn! {
    /// `game.dll+0x142D870` (call only).
    /// The war table's 3D hologram, per frame (a UI object): the mission's scale (+0x8D704, 6.6)
    /// and half the playable radius from the preview record (+0x8D70C).
    WarTableHologramUpdateFn / war_table_hologram_update / "WarTableHologramUpdate" / - (hologram: *mut c_void, xmm1: f32, level: *mut c_void, xmm3: f32) -> ()
}

game_fn! {
    /// `game.dll+0x14329B0` (call only).
    /// The war table hologram's zoom slider: blends the scale (+0x8D704) from 6.6 to 11.
    WarTableHologramZoomFn / war_table_hologram_zoom / "WarTableHologramZoom" / - (hologram: *mut c_void, xmm1: f32) -> ()
}

game_fn! {
    /// `game.dll+0x1432F10` (call only).
    /// The war table hologram's terrain height lookup.
    WarTableHologramHeightAtFn / war_table_hologram_height_at / "WarTableHologramHeightAt" / - (hologram: *mut c_void, rdx: u64) -> f32
}

game_fn! {
    /// `game.dll+0x1433460` (call only).
    /// The war table hologram's terrain height: the map's lowest point lifted above the table's
    /// plane by a lift that follows the scale (+0x8D704).
    WarTableHologramTerrainHeightFn / war_table_hologram_terrain_height / "WarTableHologramTerrainHeight" / - (hologram: *mut c_void, rdx: *mut c_void, xmm2: f32) -> u64
}

game_fn! {
    /// `helldivers2.exe+0x689BC0` (call only).
    /// The engine's script render step, once a frame: calls the boot script's render(), which
    /// renders the world and takes the camera unit's pose then, so a pose set before it is the one
    /// the frame renders with. HellForge hooks it; its return value is not reviewed (it ends in a
    /// tail call), so it is not marked for hooking by name.
    EngineRenderStepFn / engine_render_step / "EngineRenderStep" / - (game: *mut c_void) -> ()
}

game_fn! {
    /// `helldivers2.exe+0x1FCAD0` (hook or call).
    /// Sets a unit's scene graph node's local pose and marks it changed (the engine's
    /// set_local_pose, its scene graph table 0x27C8F10 +0x60); UpdateUnit applies it.
    UnitSetLocalPoseFn / unit_set_local_pose / "UnitSetLocalPose" / hook_unit_set_local_pose (unit: u32, node: i32, pose: *const f32) -> ()
}

game_fn! {
    /// `helldivers2.exe+0x1B2DC0` (hook or call).
    /// Updates a unit in its world after its pose was set: its world poses from the local ones
    /// (the engine's update_unit).
    UpdateUnitFn / update_unit / "UpdateUnit" / hook_update_unit (world: *mut c_void, unit: *mut c_void) -> ()
}

game_fn! {
    /// `helldivers2.exe+0x9D8C0` (hook or call).
    /// The unit object for a handle, through the unit registry (UnitRegistry) under its lock; NULL
    /// when the handle is stale.
    UnitFromHandleFn / unit_from_handle / "UnitFromHandle" / hook_unit_from_handle (unit: u32) -> *mut c_void
}

game_fn! {
    /// `helldivers2.exe+0x317DE0` (hook or call).
    /// The engine's main world, chosen from the engine's worlds (EngineCore +0x590 count, +0x598
    /// array; the API table 0x27C9E60 +0x18). A world: +0x2305B8 its unit count, +0x2305C0 the
    /// units, +0x230920 its last render pose (a 4x4). From HellForge's terminal camera
    /// (terminal_camera.c).
    MainWorldFn / main_world / "MainWorld" / hook_main_world () -> *mut c_void
}

game_fn! {
    /// `helldivers2.exe+0x1FD190` (hook or call).
    /// A unit node's world position (3 floats in its world pose): the Unit table's (EngineApi
    /// +0x18, 0x27C8F10) +0x88.
    UnitNodeWorldPositionFn / unit_node_world_position / "UnitNodeWorldPosition" / hook_unit_node_world_position (unit: u32, node: i32) -> *const f32
}

game_fn! {
    /// `helldivers2.exe+0x799DE0` (hook or call).
    /// The unit's physics actor of that name, 0xFFFFFFFF when it has none (EngineUnitApi +0x10).
    /// hf->injure passes the limb's to QueueDamage.
    UnitActorByNameFn / unit_actor_by_name / "UnitActorByName" / hook_unit_actor_by_name (unit: u32, name: u32, index: *mut u32) -> u32
}

game_fn! {
    /// UnitFindActor: UnitActorByName's former name, kept for mods written with it; use
    /// UnitActorByName.
    UnitFindActorFn / unit_find_actor / "UnitFindActor" / hook_unit_find_actor (unit: u32, name: u32, index: *mut u32) -> u32
}

game_fn! {
    /// `helldivers2.exe+0x3217C0` (hook or call).
    /// Points every later lookup of a resource name at another resource's name, under the resource
    /// manager's lock (the Application table's +0x378). Objects already made keep what they had.
    SetResourceOverrideFn / set_resource_override / "SetResourceOverride" / hook_set_resource_override (type_: u64, name: u64, target: u64) -> ()
}

game_fn! {
    /// `helldivers2.exe+0x321900` (hook or call).
    /// Takes a resource override out (the API table's +0x380).
    ClearResourceOverrideFn / clear_resource_override / "ClearResourceOverride" / hook_clear_resource_override (type_: u64, name: u64) -> ()
}

game_fn! {
    /// `helldivers2.exe+0x321930` (hook or call).
    /// Whether a resource is loaded and can be had now (the API table's +0x388).
    CanGetResourceFn / can_get_resource / "CanGetResource" / hook_can_get_resource (type_: u64, name: u64) -> u8
}

game_fn! {
    /// `helldivers2.exe+0x2ABE20` (hook or call).
    /// The network transport's cleanup (its vtable 0x1676270 +0xA0, called from
    /// NetworkManagerUpdate): destroys a dead connection (+0x1878C) only when no list (session,
    /// lobbies, the manager's own) wants its peer. HellForge hooks it to close a crashed friend's
    /// connection.
    TransportCleanupFn / transport_cleanup / "TransportCleanup" / hook_transport_cleanup (transport: *mut c_void, wanted: *mut c_void, removed: *mut c_void) -> ()
}

game_fn! {
    /// `helldivers2.exe+0x2AA8A0` (hook or call).
    /// Destroys one of the transport's connections and takes it out of the transport's array
    /// (which closes up).
    DestroyConnectionFn / destroy_connection / "DestroyConnection" / hook_destroy_connection (transport: *mut c_void, connection: *mut c_void, r8: u64) -> ()
}

game_fn! {
    /// `helldivers2.exe+0x292FC0` (hook or call).
    /// Tells the game session these peers' connections are gone.
    SessionPeersRemovedFn / session_peers_removed / "SessionPeersRemoved" / hook_session_peers_removed (session: *mut c_void, removed: *const c_void) -> ()
}

game_fn! {
    /// `helldivers2.exe+0x172080` (hook or call).
    /// The network manager's update, every frame: calls the transport's cleanup (TransportCleanup,
    /// at 0x172466).
    NetworkManagerUpdateFn / network_manager_update / "NetworkManagerUpdate" / hook_network_manager_update (manager: *mut c_void) -> ()
}

game_fn! {
    /// `helldivers2.exe+0xCE5A10` (hook or call).
    /// Both collision filters' pair check (their slot +0x18), on the physics threads for new pairs
    /// of nearby bodies: compacts the pairs the collision matrix lets through and returns how many
    /// are kept. HellForge's detour drops grenade and shield pairs.
    CollisionFilterPairsFn / collision_filter_pairs / "CollisionFilterPairs" / hook_collision_filter_pairs (filter: *mut c_void, world: *mut c_void, pairs: *mut u32, count: i32) -> i32
}

game_fn! {
    /// `helldivers2.exe+0x77D6F0` (hook or call).
    /// A physics handle's world index (4 when the handle is -1) and its body id.
    PhysicsHandleBodyFn / physics_handle_body / "PhysicsHandleBody" / hook_physics_handle_body (handle: u32, body: *mut u32) -> u32
}

game_fn! {
    /// `helldivers2.exe+0x812CC0` (hook or call).
    /// Makes a navmesh cut (a silhouette generator) in a world; 0 when it fails. HellForge calls
    /// it with denormals flushed (MXCSR) and only while the world's cut lock (0x27C6350 + 8 x
    /// world) is set.
    CreateNavmeshCutFn / create_navmesh_cut / "CreateNavmeshCut" / hook_create_navmesh_cut (args: *const c_void) -> u32
}

game_fn! {
    /// `helldivers2.exe+0x8127A0` (hook or call).
    /// Removes navmesh cuts (turning them off first through EnableNavmeshCuts).
    RemoveNavmeshCutsFn / remove_navmesh_cuts / "RemoveNavmeshCuts" / hook_remove_navmesh_cuts (cuts: *mut u32, count: u32) -> ()
}

game_fn! {
    /// `helldivers2.exe+0x812AE0` (hook or call).
    /// Turns navmesh cuts on or off.
    EnableNavmeshCutsFn / enable_navmesh_cuts / "EnableNavmeshCuts" / hook_enable_navmesh_cuts (cuts: *mut u32, count: u32, on: u32) -> ()
}

game_fn! {
    /// `helldivers2.exe+0x7B45C0` (hook or call).
    /// Sets up a navigation world once at startup: 8 x 8 tiles of 128 m, +-512 m around the
    /// centre, which every mission reuses (NavWorlds).
    NavWorldSetupFn / nav_world_setup / "NavWorldSetup" / hook_nav_world_setup (id: u32, min: *const f32, max: *const f32, r9: *const f32, s5: *const f32, s6: f32, s7: u32, s8: u32) -> ()
}

game_fn! {
    /// `helldivers2.exe+0x89A6C0` (hook or call).
    /// Builds a navmesh tile's terrain (Recast): sizes its heightfield from a hard-coded 128 m
    /// (0x89A85E) while it rasterises the tile's real bounds; HellForge points that read at 256.0
    /// for 256 m tiles.
    BuildNavTileFn / build_nav_tile / "BuildNavTile" / hook_build_nav_tile (result: *mut c_void, rdx: u32, r8: *const c_void, r9: u32, s5: u64, s6: u64) -> *mut c_void
}

game_fn! {
    /// `helldivers2.exe+0x795630` (hook or call).
    /// Builds a physics world's pools (bodies, actors, ragdolls, cutters) from the config in one
    /// block; HellForge raises the sizes before the setup runs.
    BuildPhysicsPoolsFn / build_physics_pools / "BuildPhysicsPools" / hook_build_physics_pools (config: *const c_void, block: *mut c_void, size: u32) -> ()
}

game_fn! {
    /// `helldivers2.exe+0x58EC0` (hook or call).
    /// Allocates from an engine allocator (through its slot +0x30); arrays from it are freed
    /// through its slot +0x40.
    EngineAllocatorAllocateFn / engine_allocator_allocate / "EngineAllocatorAllocate" / hook_engine_allocator_allocate (allocator: *mut c_void, size: u64, align: u32) -> *mut c_void
}

game_fn! {
    /// `helldivers2.exe+0x175B60` (hook or call).
    /// Frees a network configuration entry's two arrays ({count, capacity, data, allocator} at
    /// +0x18 and +0x30), only those with a capacity; run when the game quits.
    DestroyNetworkTypeEntryFn / destroy_network_type_entry / "DestroyNetworkTypeEntry" / hook_destroy_network_type_entry (entry: *mut c_void) -> ()
}

game_fn! {
    /// `helldivers2.exe+0x60DA80` (hook or call).
    /// The engine's package lookup: the package object with that id, or null.
    FindPackageFn / find_package / "FindPackage" / hook_find_package (resources: *mut c_void, id: u64) -> *mut c_void
}

game_fn! {
    /// `helldivers2.exe+0x321330` (hook or call).
    /// 1 when the engine knows the package and every part of it is loaded (part state 4), else 0.
    PackageHasLoadedFn / package_has_loaded / "PackageHasLoaded" / hook_package_has_loaded (id: u64) -> u32
}

game_fn! {
    /// `helldivers2.exe+0x8C6F30` (hook or call).
    /// A lobby member's property as the PlayFab SDK holds it, or null. An engine API slot (+0x118)
    /// that game.dll calls for its own member keys.
    LobbyMemberDataFn / lobby_member_data / "LobbyMemberData" / hook_lobby_member_data (lobby: *mut c_void, peer: u64, key: *const c_char) -> *const c_char
}

game_fn! {
    /// `helldivers2.exe+0x8C7010` (hook or call).
    /// Posts this machine's own lobby member properties (one PFLobbyPostUpdate) and returns the
    /// HRESULT (0: queued). An engine API slot (+0x120).
    LobbySetMemberDataFn / lobby_set_member_data / "LobbySetMemberData" / hook_lobby_set_member_data (lobby: *mut c_void, count: u32, keys: *const c_void, values: *const c_void) -> i32
}

game_fn! {
    /// `helldivers2.exe+0x5C2E10` (hook or call).
    /// The allocate of the text registry's allocator (its vtable +0x30): allocates through the
    /// backing allocator and returns out.
    RegistryAllocateFn / registry_allocate / "RegistryAllocate" / hook_registry_allocate (allocator: *mut c_void, out: *mut u64, size: u64, align: u64) -> *mut c_void
}

game_fn! {
    /// `game.dll+0x61AF10` (hook or call).
    /// Gives one entity its own per-instance ProjectileWeapon record: a copy of its type's record
    /// with the delta's patches. An entity that already has one is left as it is.
    ProjectileWeaponCopyFn / projectile_weapon_copy / "ProjectileWeaponCopy" / hook_projectile_weapon_copy (manager: *mut c_void, handle: *mut c_void, delta: *const u32) -> ()
}

game_fn! {
    /// `game.dll+0x770B10` (hook or call).
    /// Gives one entity its own per-instance magazine record: a copy of its type's record with the
    /// delta's patches. One that already has a copy is left as it is.
    MagazineCopyFn / magazine_copy / "MagazineCopy" / hook_magazine_copy (manager: *mut c_void, record: *mut c_void, delta: *const u32) -> ()
}

game_fn! {
    /// `game.dll+0x812540` (hook or call).
    /// Attaches a child entity to a node of a parent unit at an offset and rotation: the call the
    /// mount system makes for each child it attaches.
    AttachToUnitFn / attach_to_unit / "AttachToUnit" / hook_attach_to_unit (manager: *mut c_void, child: u32, link: u32, node: u32, offset: *const f32, rotation: *const f32, s7: f32) -> ()
}

game_fn! {
    /// `game.dll+0x843EA0` (hook or call).
    /// Changes an entity's AI behaviour the game's own way: leaves the old one through
    /// BehaviourStage with 0, writes and replicates the id (MarkReplicated) and enters the new one
    /// at stage 1. Nothing happens if it already runs that behaviour.
    SetBehaviourFn / set_behaviour / "SetBehaviour" / hook_set_behaviour (unused: *mut c_void, entity: u32, behaviour: u32) -> ()
}

game_fn! {
    /// `game.dll+0xB0B020` (hook or call).
    /// Removes a child's relation to its parent: the call the mount system makes before it removes
    /// a child. A child without that relation is left as it is.
    UnlinkRelationFn / unlink_relation / "UnlinkRelation" / hook_unlink_relation (child: u32, owner: *mut c_void, parent: u32) -> ()
}

game_fn! {
    /// `game.dll+0xFDC310` (hook or call).
    /// Asks for an entity to be removed. Deferred: on the host it is queued in the world's removal
    /// map, and the pending-removal pass destroys it.
    RemoveEntityFn / remove_entity / "RemoveEntity" / hook_remove_entity (world: *mut c_void, entity: u32) -> ()
}

game_fn! {
    /// `game.dll+0xFDC820` (hook or call).
    /// Destroys one entity now: what the pending-removal pass (0xFDC7E0) calls for each queued
    /// entity.
    DestroyEntityFn / destroy_entity / "DestroyEntity" / hook_destroy_entity (world: *mut c_void, record: *mut c_void, forced: u8) -> ()
}

game_fn! {
    /// `game.dll+0x146F3B0` (hook or call).
    /// Closes the open stratagem selection on the loadout screen: what Back on a focused stratagem
    /// slot calls.
    CloseStratagemSelectorFn / close_stratagem_selector / "CloseStratagemSelector" / hook_close_stratagem_selector (ui: *mut c_void) -> ()
}

game_fn! {
    /// `game.dll+0x18D1440` (hook or call).
    /// Greys or un-greys one stratagem card: finds the card with that key, writes its enabled byte
    /// and, for a realized card, its grey bit and redraw. The game's post-pick refresh calls it
    /// with 0.
    SetStratagemCardEnabledFn / set_stratagem_card_enabled / "SetStratagemCardEnabled" / hook_set_stratagem_card_enabled (list: *mut c_void, key: u32, enabled: u8) -> ()
}

game_fn! {
    /// `game.dll+0x11ED0B0` (call only).
    /// Sets one lobby setting the way the options menu does; for id 0 it writes settings +0x174
    /// and the lobby's PrivacyMode key. It dispatches on the id through a jump table.
    SetLobbySettingFn / set_lobby_setting / "SetLobbySetting" / - (settings: *mut c_void, id: u32, value: *const c_void) -> ()
}

game_fn! {
    /// `game.dll+0x133E6E0` (hook or call).
    /// Starts or stops Quickplay matchmaking. (matchmaker, 0, -1, 0x7FFFFFFF, 0, 0, 0, 0) is the
    /// stop that the galactic war map's cancel makes.
    SetQuickplayFn / set_quickplay / "SetQuickplay" / hook_set_quickplay (matchmaker: *mut c_void, on: u8, r8: i32, r9: i32, s5: u8, s6: u8, s7: u8, s8: u8) -> ()
}

game_fn! {
    /// `game.dll+0x12FE4E0` (hook or call).
    /// Drops a reference that RequestPackages took on packages. Same shape as RequestPackages, and
    /// the same game-thread-only rule.
    ReleasePackagesFn / release_packages / "ReleasePackages" / hook_release_packages (system: *mut c_void, ids: *const u64, count: u32) -> ()
}

game_fn! {
    /// `game.dll+0x11D8020` (hook or call).
    /// The BeamInfo row of a beam type, from the beam table at game.dll+0x37C8250 (31 slots of
    /// 0x70 bytes). A leaf with no .pdata entry.
    BeamInfoOfFn / beam_info_of / "BeamInfoOf" / hook_beam_info_of (type_: u32) -> *mut c_void
}

game_fn! {
    /// `game.dll+0x11F8EA0` (hook or call).
    /// The DamageInfo row of a damage id, from the table at game.dll+0x37C60C0 (650 slots of 76
    /// bytes). A leaf with no .pdata entry.
    DamageInfoOfFn / damage_info_of / "DamageInfoOf" / hook_damage_info_of (id: u32) -> *mut c_void
}

game_fn! {
    /// `game.dll+0x11EC7C0` (hook or call).
    /// The ProjectileInfo row of a projectile type, from the table at game.dll+0x37C7670 (351
    /// types). A leaf with no .pdata entry.
    ProjectileInfoOfFn / projectile_info_of / "ProjectileInfoOf" / hook_projectile_info_of (type_: u32) -> *mut c_void
}

game_fn! {
    /// `game.dll+0x4EE6C0` (hook or call).
    /// The game plugin's per-frame entry point, registered in the plugin API table (no direct
    /// callers): runs two engine callbacks, then GameUpdate with the Game and the frame time.
    PluginUpdateFn / plugin_update / "PluginUpdate" / hook_plugin_update (dt: f32) -> ()
}

game_fn! {
    /// `game.dll+0xAB5000` (hook or call).
    /// The game's world update, once a frame on the game thread on every machine: kicks the
    /// explosion queue, runs the entity and component updates, later gathers the explosions, and
    /// checks for a language change.
    GameUpdateFn / game_update / "GameUpdate" / hook_game_update (unused: *mut c_void, dt: f32) -> ()
}

game_fn! {
    /// `game.dll+0x1327F50` (hook or call).
    /// Posts a UI sound the way the loadout screen does: the key becomes a Wwise event id, which
    /// it posts through the WwiseWorld.
    PlayLoadoutSoundFn / play_loadout_sound / "PlayLoadoutSound" / hook_play_loadout_sound (unused: *mut c_void, key: u32) -> ()
}

game_fn! {
    /// `game.dll+0x129C690` (hook or call).
    /// The peer id that gets the credit for what an owner does: the peer owning the owner's
    /// network object.
    CreditorOfFn / creditor_of / "CreditorOf" / hook_creditor_of (unused: *mut c_void, owner: u32) -> u64
}

game_fn! {
    /// Creditor: CreditorOf's former name, kept for mods written with it; use CreditorOf.
    CreditorFn / creditor / "Creditor" / hook_creditor (unused: *mut c_void, owner: u32) -> u64
}

game_fn! {
    /// `game.dll+0x953150` (hook or call).
    /// Picks an enemy type from a spawn group, weighted by the mission difficulty (row +0x30 has
    /// ten weights). Every spawn that picks an enemy type calls it; it returns 0xFFFFFFFF when
    /// nothing qualifies.
    PickEnemyFn / pick_enemy / "PickEnemy" / hook_pick_enemy (manager: *mut c_void, group: u32, roster: *mut c_void, r9: u8) -> u32
}

game_fn! {
    /// `game.dll+0x11D9DF0` (hook or call).
    /// An entity's attribute record by key (a float at +8), or null. It is the only reader of the
    /// armor passive slots and maps an avatar to its kits' passives.
    EntityAttributeFn / entity_attribute / "EntityAttribute" / hook_entity_attribute (entity: u32, key: u32, flags: u32) -> *mut c_void
}

game_fn! {
    /// `game.dll+0x9235F0` (hook or call).
    /// Applies one damage to an entity's health: acceptance mask, element and Vitality
    /// multipliers, zone health, downing and death (KillDispatch). ConsumeDamageEvent calls it
    /// with the DamageEvent's fields, on the machine that drains the queue.
    ApplyDamageFn / apply_damage / "ApplyDamage" / hook_apply_damage (health: *mut c_void, target: u32, kind: u32, element: u32, dealer: u32, owner: u32, creditor: u64, amount: i32, s9: u32, s10: f32, position: *mut f32, s12: *mut f32, s13: u32, s14: u8, s15: u8, s16: u32, s17: u32, s18: u32, s19: u8) -> ()
}

game_fn! {
    /// `game.dll+0x927050` (hook or call).
    /// 1 when the entity's health record says dead (life state +0x19C >= 2); 0 when alive, downed
    /// or without a health record.
    IsDeadFn / is_dead / "IsDead" / hook_is_dead (health: *mut c_void, entity: u32) -> u8
}

game_fn! {
    /// `game.dll+0x4A4410` (call only).
    /// The entity's health record (0x1B8 bytes in the HealthManager's records at +0x1058). An
    /// entity with no record gives a bad pointer, not NULL: check first. No code calls it.
    HealthRecordFn / health_record / "HealthRecord" / - (unused: u64, entity: u32) -> *mut c_void
}

game_fn! {
    /// `game.dll+0x91E500` (hook or call).
    /// Sets an entity's health to fraction x its maximum and revives a downed one (life state 0);
    /// does nothing to the dead. The stim's status handler calls it.
    RestoreHealthFn / restore_health / "RestoreHealth" / hook_restore_health (health: *mut c_void, entity: u32, unused: u64, fraction: f32) -> ()
}

game_fn! {
    /// `game.dll+0x507920` (hook or call).
    /// The entity's HealthComponentData: its private copy if it has one, else the shared type
    /// record (0x5650 bytes each).
    HealthSettingsFn / health_settings / "HealthSettings" / hook_health_settings (descriptor: *mut c_void) -> *mut c_void
}

game_fn! {
    /// `game.dll+0x91C240` (hook or call).
    /// Raises a health record's life state (+0x19C; a lower state is ignored) and runs what
    /// follows the change. ApplyDamage calls it, and the update calls it when the replicated state
    /// differs.
    SetLifeStateFn / set_life_state / "SetLifeState" / hook_set_life_state (health: *mut c_void, index: u32, state: u32, kind: u32, element: u32) -> ()
}

game_fn! {
    /// `game.dll+0x920260` (hook or call).
    /// The HealthManager's frame update: regeneration, time since damage, damage-over-time deaths
    /// and the replicated life state.
    HealthManagerUpdateFn / health_manager_update / "HealthManagerUpdate" / hook_health_manager_update (health: *mut c_void, dt: f32) -> ()
}

game_fn! {
    /// `game.dll+0x12A6EF0` (hook or call).
    /// The frame's drain of the combat queues: up to 16 damage events per call through
    /// ConsumeDamageEvent, then the status and other queues.
    DrainCombatQueuesFn / drain_combat_queues / "DrainCombatQueues" / hook_drain_combat_queues (queues: *mut c_void) -> ()
}

game_fn! {
    /// `game.dll+0x129EF00` (call only).
    /// Copies a DamageEvent into this machine's damage queue (CombatQueues +0x1120, 4096 at most;
    /// a full queue drops it). No code calls it.
    EnqueueDamageEventFn / enqueue_damage_event / "EnqueueDamageEvent" / - (unused: u64, event: *mut c_void) -> ()
}

game_fn! {
    /// `game.dll+0x129DD30` (hook or call).
    /// Turns a hit into a queued DamageEvent (damage = trunc(values +0) times the target's
    /// attribute) and queues the hit's status effects when the armour factor is not 0. Every
    /// damage source's hit goes through it.
    ProduceDamageFn / produce_damage / "ProduceDamage" / hook_produce_damage (unused: u64, values: *mut f32, hit: *mut c_void, key: u32) -> ()
}

game_fn! {
    /// `game.dll+0x12A15E0` (hook or call).
    /// Fills a hit event from its source: target, hit zone and armour, and the DamageInfo row's
    /// element and damage.
    InitHitInfoFn / init_hit_info / "InitHitInfo" / hook_init_hit_info (rcx: u64, hit: *mut c_void, kind: u32, r9: u32, s5: u64, source: *mut c_void, s7: u64) -> ()
}

game_fn! {
    /// `game.dll+0xFD9AF0` (hook or call).
    /// The network id of an entity (EntityOfNetwork's inverse); 0x7FFF when it has none.
    EntityToNetworkFn / entity_to_network / "EntityToNetwork" / hook_entity_to_network (entity: u32) -> u32
}

game_fn! {
    /// `game.dll+0x922060` (hook or call).
    /// The health zone a hit node belongs to (zones at settings +0x208, 0x228 bytes each, 38 at
    /// most); the default zone (settings +0x40) when none lists it.
    HealthZoneOfNodeFn / health_zone_of_node / "HealthZoneOfNode" / hook_health_zone_of_node (unused: u64, settings: *mut c_void, node: u32) -> *mut c_void
}

game_fn! {
    /// `game.dll+0x129DC30` (hook or call).
    /// Builds a hit's DamageValues with the builder of its kind (+0 damage, -1 to kill; +4 armour
    /// factor) and returns values.
    BuildDamageValuesFn / build_damage_values / "BuildDamageValues" / hook_build_damage_values (rcx: u64, values: *mut f32, hit: *mut c_void) -> *mut f32
}

game_fn! {
    /// `game.dll+0x856D40` (hook or call).
    /// 1 when the mission has the booster (ApplyDamage asks it for Vitality, id 1).
    IsBoosterActiveFn / is_booster_active / "IsBoosterActive" / hook_is_booster_active (unused: u64, booster: u32) -> u8
}

game_fn! {
    /// `game.dll+0x699E40` (hook or call).
    /// Applies a status to an entity owned here: susceptibility and buildup decide, then the
    /// type's handler runs. Weather Insulated (56) refuses 53..55.
    ApplyStatusFn / apply_status_function / "ApplyStatus" / hook_apply_status (receivers: *mut c_void, entity: u32, type_: u32, r9: u32, buildup: f32, s6: u32, s7: u64) -> ()
}

game_fn! {
    /// `game.dll+0x69B1F0` (hook or call).
    /// 1 when the entity has the status.
    HasStatusFn / has_status / "HasStatus" / hook_has_status (receivers: *mut c_void, entity: u32, type_: i32) -> u8
}

game_fn! {
    /// `game.dll+0x69AB60` (hook or call).
    /// Ends a status on an entity: the type's handler runs with apply = 0.
    RemoveStatusFn / remove_status / "RemoveStatus" / hook_remove_status (receivers: *mut c_void, entity: u32, type_: u32) -> ()
}

game_fn! {
    /// `game.dll+0x62C930` (hook or call).
    /// Adds amount to one of a player's mission stats, overall and under its source. Kills,
    /// deaths, stims, shots and damage all count through it.
    AddStatFn / add_stat / "AddStat" / hook_add_stat (stats: *mut c_void, peer: u64, key: u32, amount: u32, source: u32, resource: u64) -> ()
}

game_fn! {
    /// `game.dll+0x62C710` (hook or call).
    /// The ScoreManager index of the player with this peer id (through PlayerManager +0x2C8);
    /// 0xFFFFFFFF when none.
    StatsPlayerIndexFn / stats_player_index / "StatsPlayerIndex" / hook_stats_player_index (stats: *mut c_void, peer: u64) -> u32
}

game_fn! {
    /// `game.dll+0x1731A60` (hook or call).
    /// The 20-byte entry of a u32-keyed map that holds key, or the empty slot that ends its probe;
    /// NULL when the map is full. The stat tables use it (an entry's +4 is the stat's value).
    StatMapLookupFn / stat_map_lookup / "StatMapLookup" / hook_stat_map_lookup (map: *mut c_void, key: u32) -> *mut c_void
}

game_fn! {
    /// `game.dll+0x630130` (hook or call).
    /// Fills a player's end-of-mission score from the stat keys.
    BuildScoreFn / build_score / "BuildScore" / hook_build_score (stats: *mut c_void, score: *mut u32, index: u32) -> ()
}

game_fn! {
    /// `game.dll+0x129FD00` (hook or call).
    /// KillDispatch's listener that counts a kill in the creditor's mission stats (dealt_kills,
    /// team kills, melee kills) when the victim's KillScore is above 0.
    KillCreditListenerFn / kill_credit_listener / "KillCreditListener" / hook_kill_credit_listener (listener: *mut c_void, context: *mut c_void, creditor: u64, victim: u32, dealer: u32, kind: u32, element: u32) -> ()
}

game_fn! {
    /// `game.dll+0x11DA090` (hook or call).
    /// A value after one passive's modifiers of a key: (the last Set, else base) plus every Add,
    /// times every Multiply; base when the passive has none.
    PassiveValueFn / passive_value / "PassiveValue" / hook_passive_value (passive: u32, base: f32, key: u32) -> f32
}

game_fn! {
    /// `game.dll+0xAB2FA0` (hook or call).
    /// Changes the game state: runs the old state's exit handler, writes Game +0xAC21C and runs
    /// the new state's enter handler (Mission zeroes the StateGame block). HF_ON_STATE polls the
    /// value it writes.
    ChangeStateFn / change_state / "ChangeState" / hook_change_state (game: *mut c_void, state: u32, data: *mut c_void) -> ()
}

game_fn! {
    /// `game.dll+0x607820` (hook or call).
    /// Sets a player's avatar (+0x108 + 0x70 per owned player, network id at +0x3A8 + 0x20 per
    /// player) and its state to Spawned (3).
    SetPlayerAvatarFn / set_player_avatar / "SetPlayerAvatar" / hook_set_player_avatar (players: *mut c_void, player: u32, avatar: u32) -> ()
}

game_fn! {
    /// `game.dll+0x608A90` (hook or call).
    /// Sets a player's lifecycle state (PlayerManager +0x2E0 + 0x38 per player).
    SetPlayerStateFn / set_player_state / "SetPlayerState" / hook_set_player_state (players: *mut c_void, player: u32, state: u32) -> ()
}

game_fn! {
    /// `game.dll+0x50B8B0` (hook or call).
    /// The entity's TurretComponentData (0x4C bytes: +8 vertical, +0xC horizontal turn speed): its
    /// own copy in the turret manager (game.dll+0x3326D70, copies +0xA8) if it has one, else the
    /// shared type record.
    TurretSettingsFn / turret_settings / "TurretSettings" / hook_turret_settings (descriptor: *mut c_void) -> *mut c_void
}

game_fn! {
    /// `game.dll+0x503BC0` (hook or call).
    /// The entity's SensorEyeComponentData (0x2C bytes: the vision ranges the sensor copies at
    /// spawn): its own copy if it has one, else the shared type record.
    SensorEyeSettingsFn / sensor_eye_settings / "SensorEyeSettings" / hook_sensor_eye_settings (descriptor: *mut c_void) -> *mut c_void
}

game_fn! {
    /// `game.dll+0x515100` (hook or call).
    /// The weapon's ProjectileWeaponComponentData (+0 the projectile type): its own copy in
    /// ProjectileWeapons if it has one, else the shared type record.
    ProjectileWeaponRecordFn / projectile_weapon_record / "ProjectileWeaponRecord" / hook_projectile_weapon_record (descriptor: *mut c_void) -> *mut c_void
}

game_fn! {
    /// `game.dll+0x509A40` (hook or call).
    /// The weapon's WeaponData record (0x4D0 bytes; fire modes, the input bindings): its own copy
    /// in WeaponDatas if it has one, else the shared type record.
    WeaponDataRecordFn / weapon_data_record / "WeaponDataRecord" / hook_weapon_data_record (descriptor: *mut c_void) -> *mut c_void
}

game_fn! {
    /// `game.dll+0x50ECC0` (hook or call).
    /// The weapon's BeamWeaponComponentData (+0 the beam type): its own copy in BeamWeapons if it
    /// has one, else the shared type record.
    BeamWeaponRecordFn / beam_weapon_record / "BeamWeaponRecord" / hook_beam_weapon_record (descriptor: *mut c_void) -> *mut c_void
}

game_fn! {
    /// `game.dll+0x7456D0` (hook or call).
    /// The projectile type the weapon fires now, by its components: the magazine's round type or
    /// pattern, WeaponRounds' primary or alternate magazine, linked ammunition, else
    /// ProjectileWeapon +0.
    WeaponProjectileTypeFn / weapon_projectile_type / "WeaponProjectileType" / hook_weapon_projectile_type (unused: u64, weapon: u32) -> u32
}

game_fn! {
    /// ResolveFiredType: WeaponProjectileType's former name, kept for mods written with it; use
    /// WeaponProjectileType.
    ResolveFiredTypeFn / resolve_fired_type / "ResolveFiredType" / hook_resolve_fired_type (unused: u64, weapon: u32) -> u32
}

game_fn! {
    /// `game.dll+0x6128B0` (hook or call).
    /// Fires one shot of a weapon's projectiles; type 0 resolved to 0 fires nothing. The
    /// ProjectileWeapon update, the heat step and GenericFire all shoot through it.
    ProjectileFireFn / projectile_fire / "ProjectileFire" / hook_projectile_fire (projectiles: *mut c_void, weapon: u32, r8: u8, r9: u64, s5: u32, type_: u32, s7: u64, s8: u32, s9: f32, s10: f32, s11: f32) -> ()
}

game_fn! {
    /// ProjectileWeaponFire: ProjectileFire's former name, kept for mods written with it; use
    /// ProjectileFire.
    ProjectileWeaponFireFn / projectile_weapon_fire / "ProjectileWeaponFire" / hook_projectile_weapon_fire (projectiles: *mut c_void, weapon: u32, r8: u8, r9: u64, s5: u32, type_: u32, s7: u64, s8: u32, s9: f32, s10: f32, s11: f32) -> ()
}

game_fn! {
    /// `game.dll+0x741DF0` (hook or call).
    /// The generic fire: hands the shot to ProjectileFire (its arguments 5 to 11 pass through)
    /// when the weapon has a ProjectileWeapon or ArcWeapon instance, else returns. Charged
    /// releases and the heat levels fire through it.
    GenericFireFn / generic_fire / "GenericFire" / hook_generic_fire (rcx: u64, unused: u64, weapon: u32, r9: u64, s5: u32, type_: u32, s7: u64, s8: u32, s9: f32, s10: f32, s11: f32, s12: u8) -> ()
}

game_fn! {
    /// `game.dll+0x7627B0` (hook or call).
    /// Adds one shot's heat to a weapon with a WeaponHeat instance and fires the projectile of the
    /// heat level reached (its own type per stage).
    WeaponHeatStepFn / weapon_heat_step / "WeaponHeatStep" / hook_weapon_heat_step (rcx: u64, weapon: u32, r8: u8) -> ()
}

game_fn! {
    /// `game.dll+0x744690` (hook or call).
    /// Spends one round of a weapon (magazine, rounds or linked ammunition), shared by every
    /// output family; the beam spends one per pulse.
    SpendRoundFn / spend_round / "SpendRound" / hook_spend_round (weapons: *mut c_void, weapon: u32) -> ()
}

game_fn! {
    /// `game.dll+0x616AC0` (hook or call).
    /// One ProjectileWeapon instance's frame: its fire timer and trigger, then the shot (through
    /// the heat step when the weapon has WeaponHeat).
    ProjectileWeaponUpdateFn / projectile_weapon_update / "ProjectileWeaponUpdate" / hook_projectile_weapon_update (projectiles: *mut c_void, index: u32, dt: f32) -> ()
}

game_fn! {
    /// `game.dll+0x83E200` (hook or call).
    /// One BeamWeapon instance's frame: the charge gate and the beam shot from the trigger byte.
    BeamWeaponUpdateFn / beam_weapon_update / "BeamWeaponUpdate" / hook_beam_weapon_update (beams: *mut c_void, index: u32, dt: f32) -> ()
}

game_fn! {
    /// `game.dll+0x83F9D0` (hook or call).
    /// One beam pulse: spends a round and runs the heat step of the weapon's own WeaponHeat; no
    /// authority test on the way.
    BeamShotFn / beam_shot / "BeamShot" / hook_beam_shot (rcx: u64, rdx: u64, r8: u8, r9: u64, s5: u64, s6: u64, s7: u64, s8: u32, unused: u64, s10: u64) -> ()
}

game_fn! {
    /// `game.dll+0x7768F0` (hook or call).
    /// The reload refill: spends one spare magazine (one unit from the parent's deposit when
    /// shared) and fills the magazine, queuing the network field writes.
    ReloadMagazineFn / reload_magazine / "ReloadMagazine" / hook_reload_magazine (rcx: u64, weapon: u32, r8: u32, r9: u8) -> ()
}

game_fn! {
    /// `game.dll+0x6DEDB0` (hook or call).
    /// The turret system's frame: turns every turret toward its targeting direction, yaw then
    /// pitch, with the instance's turn speeds and the settings' limits (TurretSettings, resolved
    /// every frame).
    TurretUpdateFn / turret_update / "TurretUpdate" / hook_turret_update (turrets: *mut c_void, dt: f32) -> ()
}

game_fn! {
    /// `game.dll+0x6DE820` (hook or call).
    /// A new turret's turn speed factor from the ship-module modifiers (1.0 by default); turret
    /// creation stores it in the instance at +4.
    TurretSpeedFactorFn / turret_speed_factor / "TurretSpeedFactor" / hook_turret_speed_factor (unused: u64, rdx: u64) -> f32
}

game_fn! {
    /// `game.dll+0x64C3C0` (hook or call).
    /// Sets a sensor's vision ranges in its instance (+0xC, +0x10, +0x14) to its SensorEyeSettings
    /// ranges times scale and keeps scale at +0x20.
    SensorEyeRangesFn / sensor_eye_ranges / "SensorEyeRanges" / hook_sensor_eye_ranges (sensors: *mut c_void, entity: u32, scale: f32) -> ()
}

game_fn! {
    /// `game.dll+0x887760` (hook or call).
    /// The perception frame: every perceiver's sensors mark what they sense (the proximity sensors
    /// without a line-of-sight ray).
    PerceptionUpdateFn / perception_update / "PerceptionUpdate" / hook_perception_update (perception: *mut c_void, dt: f32) -> ()
}

game_fn! {
    /// `game.dll+0x78A420` (hook or call).
    /// The weapon wind-up frame: spin progress rises by dt / settings +0 while the trigger is
    /// held; on release settings +4 <= 0 resets it, else it falls by dt / settings +0.
    WindUpUpdateFn / wind_up_update / "WindUpUpdate" / hook_wind_up_update (windups: *mut c_void, dt: f32) -> ()
}

game_fn! {
    /// `game.dll+0x87A3C0` (call only).
    /// Applies the entity's armour or helmet modifier of a key (EntityAttribute with flags 3) to
    /// value: Set stores, Add and Time add, Multiply multiplies; no modifier leaves it. No code
    /// calls it.
    ApplyAttributeFn / apply_attribute / "ApplyAttribute" / - (entity: u32, key: u32, value: *mut f32) -> ()
}

game_fn! {
    /// `game.dll+0x874520` (hook or call).
    /// Moves one player's pending customization to the applied record; when the new kit's package
    /// is ready it copies the kit ids and sets the applied armour and helmet passives (+0x3C,
    /// +0x38) from the kit.
    UpdateCustomizationFn / update_customization / "UpdateCustomization" / hook_update_customization (customizations: *mut c_void, index: u32) -> ()
}

game_fn! {
    /// `game.dll+0x874D80` (hook or call).
    /// The customization frame: UpdateCustomization for every record, then the 2 passive ids of
    /// every player slot, loading and unloading their packages when they change.
    UpdateCustomizationsFn / update_customizations / "UpdateCustomizations" / hook_update_customizations (customizations: *mut c_void, dt: f32) -> ()
}

game_fn! {
    /// `game.dll+0x8760B0` (hook or call).
    /// Sets a player's pending armour kit (pending record +0xC, network field 0xB7D3A2EC);
    /// UpdateCustomization applies it and its passive.
    SetArmorKitFn / set_armor_kit / "SetArmorKit" / hook_set_armor_kit (customizations: *mut c_void, entity: u32, kit: u32) -> ()
}

game_fn! {
    /// `game.dll+0x875CD0` (hook or call).
    /// Sets a player's pending helmet kit (pending record +4); UpdateCustomization applies it and
    /// its passive.
    SetHelmetKitFn / set_helmet_kit / "SetHelmetKit" / hook_set_helmet_kit (customizations: *mut c_void, entity: u32, kit: u32) -> ()
}

game_fn! {
    /// `game.dll+0x875EC0` (hook or call).
    /// Sets a player's pending cape kit (pending record +8).
    SetCapeKitFn / set_cape_kit / "SetCapeKit" / hook_set_cape_kit (customizations: *mut c_void, entity: u32, kit: u32) -> ()
}

game_fn! {
    /// `game.dll+0x606820` (hook or call).
    /// A player's slot (0 to 3), kept at PlayerManager +0x3B4 + 0x20 per player; the customization
    /// update reads each slot's passives with it.
    PlayerSlotFn / player_slot / "PlayerSlot" / hook_player_slot (players: *mut c_void, player: u32) -> u32
}

game_fn! {
    /// `game.dll+0x4C89C0` (hook or call).
    /// Requests an explosion at a node of the behaving entity's unit: into the ExplosionQueue
    /// (RequestExplosionAt) and to the peers. Enemy behaviours and abilities call it.
    BehaviourExplosionFn / behaviour_explosion / "BehaviourExplosion" / hook_behaviour_explosion (context: *mut c_void, type_: u32, node: u32, r9: u8) -> ()
}

game_fn! {
    /// `game.dll+0x13C6D30` (hook or call).
    /// Queues an explosion request at a position through RequestExplosion; BehaviourExplosion and
    /// the ability explosion wrapper use it.
    RequestExplosionAtFn / request_explosion_at / "RequestExplosionAt" / hook_request_explosion_at (queue: *mut c_void, unused: u64, position: *mut f32, type_: u32, s5: u32, s6: u32, s7: u64, s8: u32) -> ()
}

game_fn! {
    /// `game.dll+0x13C5420` (hook or call).
    /// Starts this frame's explosions: up to 8 queued requests (kicked count at +0x24), early in
    /// the world update.
    ExplosionKickFn / explosion_kick / "ExplosionKick" / hook_explosion_kick (explosions: *mut c_void) -> ()
}

game_fn! {
    /// `game.dll+0x13C6180` (hook or call).
    /// Finishes the kicked explosions later in the same world update (ProcessExplosion for each,
    /// in order) and moves the rest of the queue to the front.
    ExplosionGatherFn / explosion_gather / "ExplosionGather" / hook_explosion_gather (explosions: *mut c_void) -> ()
}

game_fn! {
    /// `game.dll+0x13C0D10` (hook or call).
    /// Processes one kicked explosion request: its hits, damage and effects.
    ProcessExplosionFn / process_explosion / "ProcessExplosion" / hook_process_explosion (explosions: *mut c_void, rdx: u64, offset: u64) -> ()
}

game_fn! {
    /// `game.dll+0x13AB0E0` (hook or call).
    /// The projectile step of the world update, after the explosion kick: its impacts request
    /// explosions after the kick.
    ProjectileKickFn / projectile_kick / "ProjectileKick" / hook_projectile_kick (projectiles: *mut c_void) -> ()
}

game_fn! {
    /// `game.dll+0x4C3210` (hook or call).
    /// Starts an ability on the behaving entity: StartAbility with the AbilityManager, mode 1 and
    /// speed 1.0, returning its result.
    BehaviourStartAbilityFn / behaviour_start_ability / "BehaviourStartAbility" / hook_behaviour_start_ability (context: *mut c_void, ability: u32) -> u64
}

game_fn! {
    /// `game.dll+0x870310` (hook or call).
    /// For a corpse (Corpses global), the descriptor of the entity it came from; NULL when the
    /// entity is no corpse. Entity removal consults it.
    CorpseSourceFn / corpse_source / "CorpseSource" / hook_corpse_source (unused: u64, entity: u32) -> *mut c_void
}

game_fn! {
    /// `game.dll+0x6994F0` (hook or call).
    /// 1 when the entity can receive the status (StatusReceivers' susceptibility rule, the same as
    /// ApplyStatus's); the status queue checks it first.
    StatusReceivableFn / status_receivable / "StatusReceivable" / hook_status_receivable (unused: u64, entity: u32, type_: i32, r9: u32) -> u8
}

game_fn! {
    /// `game.dll+0xFDC780` (hook or call).
    /// The sync point: flushes every batched network field write (clamped and sent), before and
    /// after the frame's update phases.
    FlushNetworkFieldsFn / flush_network_fields / "FlushNetworkFields" / hook_flush_network_fields (entities: *mut c_void, rdx: u8) -> ()
}

game_fn! {
    /// `game.dll+0x785DE0` (hook or call).
    /// Puts an item in a wielder's slot (slot record +0 holds the entity) and wields it. The
    /// weapon switch calls it.
    WieldFn / wield / "Wield" / hook_wield (wielders: *mut c_void, wielder: u32, slot: u32, item: u32, s5: u32) -> ()
}

game_fn! {
    /// `game.dll+0x9AAA50` (hook or call).
    /// Selects an inventory slot and wields its item (Wield).
    SwitchWeaponFn / switch_weapon / "SwitchWeapon" / hook_switch_weapon (inventory: *mut c_void, entity: u32, selection: u32) -> ()
}

game_fn! {
    /// `game.dll+0xFD97E0` (hook or call).
    /// Queues one replicated field write {network id, field hash, value pointer} in the batch
    /// (0x800 entries) that FlushNetworkFields clamps and sends; every replicated gameplay field
    /// change goes through it.
    QueueFieldWriteFn / queue_field_write / "QueueFieldWrite" / hook_queue_field_write (network: u32, field: u32, value: *mut c_void) -> ()
}

game_fn! {
    /// `game.dll+0x83FD50` (hook or call).
    /// The weapon's beam type: BeamWeaponRecord +0, re-read at every shot; 0xFFFFFFFF for the
    /// invalid entity.
    BeamTypeOfFn / beam_type_of / "BeamTypeOf" / hook_beam_type_of (beams: *mut c_void, weapon: u32) -> u32
}

game_fn! {
    /// `game.dll+0x753A10` (hook or call).
    /// The out-of-ammo handler: returns at once when the weapon has a WeaponReload instance;
    /// otherwise it uses the weapon's WeaponData (its AutoDropAbility, the expendables' drop).
    OutOfAmmoFn / out_of_ammo / "OutOfAmmo" / hook_out_of_ammo (weapondata: *mut c_void, weapon: u32) -> ()
}

game_fn! {
    /// `game.dll+0x741A00` (hook or call).
    /// Fires a charged release through GenericFire with its own type (the charge level's
    /// projectile). The charge release 0x73D8C0 calls it.
    ChargeFireFn / charge_fire / "ChargeFire" / hook_charge_fire (rcx: u64, rdx: u32, type_: u32, r9: u64, s5: u32) -> ()
}

game_fn! {
    /// `game.dll+0x4F8A50` (call only).
    /// The entity's EncyclopediaEntryComponentData record (component 76; +0 its name text id,
    /// +0x24 its designation text id), by open addressing over 1330 slots; NULL for 0 or an entity
    /// without one.
    EncyclopediaRecordFn / encyclopedia_record / "EncyclopediaRecord" / - (entity: u64) -> *mut u8
}

game_fn! {
    /// `game.dll+0x1372290` (call only).
    /// The player's catalogue item (24 bytes, at account + 0xB9CE4) for a stratagem type, or NULL
    /// when the catalogue has none: what the picker's heading counts as selectable before
    /// StratagemAvailable.
    PickerCatalogueItemFn / picker_catalogue_item / "PickerCatalogueItem" / - (account: *mut c_void, type_: u32) -> *mut u8
}

game_fn! {
    /// `game.dll+0x505B40` (call only).
    /// The resource's Navigation component record (+4 its navigation profile hash, +8 a radius),
    /// by open addressing over a static table; NULL for 0 or a resource without one.
    NavigationRecordFn / navigation_record / "NavigationRecord" / - (resource: u64) -> *mut u8
}

game_fn! {
    /// `game.dll+0x495F20` (call only).
    /// How many stages a behaviour has; 0 for an unknown id, and such an entity never enters stage
    /// 1. HellForge points its id check at a stub that answers for its own behaviours with their
    /// base's count.
    BehaviourStageCountFn / behaviour_stage_count / "BehaviourStageCount" / - (id: u32) -> u32
}

game_fn! {
    /// `game.dll+0x47A240` (call only).
    /// The network slot a behaviour sends an event in; 0x1F when the behaviour does not replicate
    /// it. Paired with BehaviourSlotEvent; HellForge's stub answers for its own ids with their
    /// base's slot.
    BehaviourEventSlotFn / behaviour_event_slot / "BehaviourEventSlot" / - (id: u32, event: u32) -> u32
}

game_fn! {
    /// `game.dll+0x484B60` (call only).
    /// The event hash a behaviour's network slot stands for, as a replicated event is turned back
    /// into one; 0 for none. The inverse of BehaviourEventSlot.
    BehaviourSlotEventFn / behaviour_slot_event / "BehaviourSlotEvent" / - (id: u32, slot: u32) -> u32
}

game_fn! {
    /// `game.dll+0xF7A430` (call only).
    /// Writes each candidate's squared 2D distance from a point into its third float, before
    /// SpreadLocations sorts them by it (comparator 0xF7A5A0). SpreadLocations read location -1
    /// here on level-11 city maps until HellForge gave it location 0.
    FillDistancesToFn / fill_distances_to / "FillDistancesTo" / - (candidates: *mut f32, count: u32, centre: *mut f32) -> ()
}

game_fn! {
    /// `game.dll+0x143F3C0` (call only).
    /// Recomputes an image element's UV rect (+0x124..+0x130) from its base UV and its sprite
    /// sub-rect; a sub-rect of 0, 0, 1, 1 copies the base. HellForge's difficulty icon stub resets
    /// the sub-rect and ends here.
    UpdateImageUvFn / update_image_uv / "UpdateImageUv" / - (image: *mut u8) -> ()
}

game_fn! {
    /// `game.dll+0x505F50` (call only).
    /// The resource's shared HiveMindComponent definition (0x43C bytes; +0x74 the mission
    /// countdown in seconds), by open addressing over 38 slots; NULL for none. The mission spawn
    /// and the war table's mission card copy it and apply deltas; the shared one is never written.
    HiveMindDefinitionFn / hive_mind_definition / "HiveMindDefinition" / - (resource: u64) -> *mut u8
}

game_fn! {
    /// `game.dll+0x11F8E30` (call only).
    /// Releases the damage list's resource and, when asked, clears the DamageInfo id list (called
    /// through a table, not directly). A reload never clears past slot 649, so HellForge's moved
    /// list keeps its added rows.
    UnloadDamageListFn / unload_damage_list / "UnloadDamageList" / - (clear: u8) -> ()
}

game_fn! {
    /// `game.dll+0x11E4CD0` (hook or call).
    /// Collects the operation settings rows (25 of 0x490, lowest and highest difficulty at +0xC /
    /// +0x10) that allow a difficulty and match the caller's kind, and returns how many: the war
    /// table's mission board only offers a level they allow, so HellForge lets every row allowing
    /// 10 allow 11.
    CollectOperationsForDifficultyFn / collect_operations_for_difficulty / "CollectOperationsForDifficulty" / hook_collect_operations_for_difficulty (unused: u64, rdx: u32, r8: u32, difficulty: u32, s5: u32, s6: u32, unused2: u64, rows: *mut u32) -> u32
}

game_fn! {
    /// `game.dll+0xFBBB20` (hook or call).
    /// FinalizeVoronoi on structured maps (map type 2: mega cities, factories): welds vertices
    /// through a 24 x 24 bucket grid whose blocks come from a pool of 512 (+0xBA60, 0xA08 each,
    /// count +0x14CA60); 0 on failure. HellForge gives the pool 576 blocks.
    FinalizeVoronoiStructuredFn / finalize_voronoi_structured / "FinalizeVoronoiStructured" / hook_finalize_voronoi_structured (pass: *mut c_void, unused: u64, r8: *mut c_void, r9: *mut c_void) -> u8
}

game_fn! {
    /// `game.dll+0xFD0E00` (hook or call).
    /// Level generator op 0xA3 (revoronoi) on structured maps: refills the land around the city
    /// with a Poisson disc at the nature cell size (+0x18C), kept apart from the city's sites;
    /// returns the new diagram or NULL. HellForge grows the cell size on larger maps.
    ReVoronoiFn / re_voronoi / "ReVoronoi" / hook_re_voronoi (rcx: *mut c_void, rdx: *mut c_void, r8: *mut c_void, pass: *mut c_void, s5: *mut c_void, count: *mut u32) -> *mut c_void
}

game_fn! {
    /// `game.dll+0xFB1DB0` (hook or call).
    /// Places one road prop of a city street (its caller runs it rand % type+0x230 + 1 times per
    /// edge), testing the position against its rule's reach (+0x54) past the playable radius.
    /// HellForge measures that reach from the border on larger maps.
    PlaceRoadPropsFn / place_road_props / "PlaceRoadProps" / hook_place_road_props (rcx: *mut c_void, rdx: *mut c_void, r8: *mut c_void, r9: *mut c_void, s5: u32, s6: u32, s7: u32, s8: u8, s9: *mut c_void, s10: *mut c_void, s11: *mut c_void, s12: *mut c_void) -> ()
}

game_fn! {
    /// `game.dll+0xC04B80` (call only).
    /// Sets up the level generator's stamp index over an area: placed stamps, locations and roads
    /// by cell, for overlap tests (10757 slots after a bitset; slots + 1 at +0x1C). HellForge
    /// gives it 16383 slots.
    StampIndexInitFn / stamp_index_init / "StampIndexInit" / - (index: *mut c_void, x0: f32, y0: f32, x1: f32, y1: f32, cell: f32, unused: u64, unused2: u64, unused3: u64, unused4: u64, memory: *mut c_void) -> ()
}

game_fn! {
    /// `game.dll+0xC04EF0` (hook or call).
    /// Takes a free slot of the level generator's stamp index for a rect and links it into every
    /// cell the rect covers; returns the slot's record (callers write the stamp's id at +0) or
    /// NULL when full (17 callers; a full index used to be written through that NULL).
    StampIndexInsertFn / stamp_index_insert / "StampIndexInsert" / hook_stamp_index_insert (index: *mut c_void, x0: f32, y0: f32, x1: f32, y1: f32, slot: *mut u64) -> *mut u32
}

game_fn! {
    /// `game.dll+0xF586A0` (hook or call).
    /// Level generator pass: captures the level's minimap texture with the level's radii.
    CreateMinimapTextureFn / create_minimap_texture / "CreateMinimapTexture" / hook_create_minimap_texture (rcx: *mut c_void, unused: u64, pass: *mut c_void) -> ()
}

game_fn! {
    /// `game.dll+0x1A12D70` (hook or call).
    /// The war table mission card's duration: the mode's HiveMind definition with only the mission
    /// row's delta (+0x360) applied in a copy, its seconds divided by 60 into the card's minutes
    /// (0x1A131CC). HellForge shows the added level's own countdown there.
    UpdateMissionCardDurationFn / update_mission_card_duration / "UpdateMissionCardDuration" / hook_update_mission_card_duration (card: *mut c_void, unused: u64, mission: *const u8) -> ()
}

game_fn! {
    /// `game.dll+0x4EE160` (hook or call).
    /// game.dll's plugin setup_game, once per process (from its plugin table, no direct caller):
    /// takes the engine's APIs, then fills the engine's physics config with its pool sizes, from
    /// which the exe builds every world's pools. HellForge raises those sizes before it runs.
    SetupPhysicsConfigFn / setup_physics_config / "SetupPhysicsConfig" / hook_setup_physics_config (get_api: *mut c_void) -> ()
}

game_fn! {
    /// `game.dll+0x76B580` (hook or call).
    /// A silo remote's link, which its laser update runs every frame: while the remote's linker
    /// record says a silo link (+4 0x06212002) and the missile is not launched yet, it launches it
    /// (engine, explosive set off), names the explosive's owner, sets its guidance target to the
    /// laser's point and has the rack let it go. HellForge's silo countdown repeats these steps in
    /// this order.
    SiloRemoteLinkFn / silo_remote_link / "SiloRemoteLink" / hook_silo_remote_link (unused: u64, remote: u32) -> ()
}

game_fn! {
    /// `game.dll+0x9BF6D0` (hook or call).
    /// Updates every laser designator for a frame (a silo remote's laser) and calls each remote's
    /// link (SiloRemoteLink), which launches the missile on the first frame the laser marks a
    /// point.
    LaserDesignatorUpdateFn / laser_designator_update / "LaserDesignatorUpdate" / hook_laser_designator_update (designators: *mut c_void) -> ()
}

game_fn! {
    /// `game.dll+0x76AAB0` (call only).
    /// Links a silo's targeting remote to its missile when it is picked up: the rack's items in
    /// the SeekingMissile manager, and the missile's +8 names the remote's network handle. Reached
    /// through a component callback thunk (0x528870).
    LinkRemoteOnPickupFn / link_remote_on_pickup / "LinkRemoteOnPickup" / - (unused: u64, event: *mut u8, r8: *mut c_void) -> ()
}

game_fn! {
    /// `game.dll+0x642570` (call only).
    /// A seeking missile's pre-destroy callback: on the machine with authority over its remote,
    /// takes the remote out of its holder's hands, looking it up among the attachables and reading
    /// the record unchecked. HellForge guards that read (engine 0.1.21, InstallLooseGuard) for a
    /// remote attached to nothing.
    MissilePreDestroyFn / missile_pre_destroy / "MissilePreDestroy" / - (unused: u64, event: *mut u8) -> ()
}

game_fn! {
    /// `game.dll+0x76D280` (call only).
    /// A WeaponLinker's destroy callback: drops the entity's linker copy and moves the last copy
    /// into the freed slot, so a record pointer kept across frames can name another remote's.
    /// Returns the event's +0x10.
    CompactLinkerCopiesFn / compact_linker_copies / "CompactLinkerCopies" / - (unused: u64, event: *mut u8) -> u32
}

game_fn! {
    /// `game.dll+0x76BC70` (hook or call).
    /// Removes a silo remote once it is dropped and its rack is gone (ability 2089); runs only for
    /// a silo link, so a remote whose linker record HellForge marked as no link stays until the
    /// launch.
    SiloRemoteCleanupFn / silo_remote_cleanup / "SiloRemoteCleanup" / hook_silo_remote_cleanup (linkers: *mut c_void, remote: u32, r8: u32, r9: *mut u8) -> ()
}

game_fn! {
    /// `game.dll+0x44FE20` (hook or call).
    /// Behaviour 645's (the Pelican's chin turret) stage 2, aim: at least 1.5 s, until within 2
    /// degrees of its target, re-scoring its perceiver's entries. HellForge's turret aims without
    /// the wait.
    PelicanTurretAimFn / pelican_turret_aim / "PelicanTurretAim" / hook_pelican_turret_aim (ctx: *mut c_void) -> ()
}

game_fn! {
    /// `game.dll+0x615940` (hook or call).
    /// Emits one pellet of a ProjectileWeapon shot: builds its descriptor and spawns it into the
    /// projectile pool (SpawnProjectile, at 0x616612), on every machine. HellForge swaps the
    /// programmable round per player at 0x615B23 and gives other players' shots their own rows at
    /// the pool call.
    RealizePelletFn / realize_pellet / "RealizePellet" / hook_realize_pellet (manager: *mut c_void, index: u32, type_: u32, transform: *const u8, owner: u32, s6: f32, s7: u32, s8: u8, s9: f32, s10: f32, s11: f32, s12: u8) -> ()
}

game_fn! {
    /// `game.dll+0xBC2D30` (hook or call).
    /// Applies the state another machine sent for one of its network objects, by the object's
    /// network type through a table of the 1,855 vanilla types (0xBC2DB3). HellForge moves the
    /// table and gives each added type its base type's case.
    NetworkApplyStateFn / network_apply_state / "NetworkApplyState" / hook_network_apply_state (source: *mut c_void, network_id: u32, full: u8) -> ()
}

game_fn! {
    /// `game.dll+0x842200` (call only).
    /// Sets up an entity's behaviour record at its spawn; without authority over the entity it
    /// leaves the record empty, to be filled from what arrives (BehaviourNetworkFill).
    BehaviourSpawnSetupFn / behaviour_spawn_setup / "BehaviourSpawnSetup" / - (manager: *mut c_void, event: *mut u8, r8: *mut c_void) -> ()
}

game_fn! {
    /// `game.dll+0x842690` (call only).
    /// Fills a behaviour record from what arrives from the machine with authority: the id, the
    /// stage and the fields, ending with BehaviourResume. So a new behaviour resumes as its base
    /// does.
    BehaviourNetworkFillFn / behaviour_network_fill / "BehaviourNetworkFill" / - (unused: u64, event: *mut u8) -> ()
}

game_fn! {
    /// `game.dll+0x11EC5A0` (hook or call).
    /// Loads the projectile settings file and fills the projectile list (game + 0x37C7670,
    /// ProjectileType -> 272-byte row): clears exactly 351 slots and fills 1..350 of whichever
    /// list its instructions name, so HellForge's moved list keeps its new types over a reload.
    /// From HellForge's projectile types (projectiles.c).
    LoadProjectileListFn / load_projectile_list / "LoadProjectileList" / hook_load_projectile_list () -> ()
}

game_fn! {
    /// `game.dll+0x11EC760` (hook or call).
    /// Releases the projectile settings file and, when asked, clears the projectile list.
    UnloadProjectileListFn / unload_projectile_list / "UnloadProjectileList" / hook_unload_projectile_list (clear: u8) -> ()
}

game_fn! {
    /// `game.dll+0x75A0D0` (hook or call).
    /// One of the two lookups that bound a projectile type (0x75A376: cmp type, 351; above it,
    /// nothing). HellForge raises the bound to 351 + the new types.
    ProjectileLookupAFn / projectile_lookup_a / "ProjectileLookupA" / hook_projectile_lookup_a (unused: u64, out: *mut c_void, kind: u32, value: u32, s5: *mut c_void) -> ()
}

game_fn! {
    /// `game.dll+0x1829110` (hook or call).
    /// The other lookup that bounds a projectile type (0x1829374: cmp r14, 351; above it, another
    /// branch); it calls ProjectileLookupA. HellForge raises the bound to 351 + the new types.
    ProjectileLookupBFn / projectile_lookup_b / "ProjectileLookupB" / hook_projectile_lookup_b (rcx: *mut u8, rdx: u32, r8: u32, r9: u8, s5: *mut c_void) -> ()
}

game_fn! {
    /// `game.dll+0x11F46F0` (call only).
    /// An attachment item (0x58 bytes, its id at +8) by id, walking every attachment group (u32
    /// group count at game + 0x348DA98, group pointers at game + 0x37C5B50, each {items , u32
    /// count}); NULL for none.
    AttachmentItemByIdFn / attachment_item_by_id / "AttachmentItemById" / - (id: u32) -> *mut u8
}

game_fn! {
    /// `game.dll+0x5154C0` (call only).
    /// A delta's entry ({component count, first}) in the game's entity delta table ({pointer,
    /// count} x 5 at [game + 0x346BF98] + 0xF12E98), by open addressing; NULL for none. The spawn
    /// dereferences it unchecked, so an id a machine lacks crashes it.
    EntityDeltaFn / entity_delta / "EntityDelta" / - (resource: u64) -> *mut u8
}

game_fn! {
    /// `game.dll+0x108D190` (hook or call).
    /// The host removes a peer from the game session. HellForge then destroys that peer's dead
    /// connection, so a friend who crashed can connect again.
    RemoveSessionPeerFn / remove_session_peer / "RemoveSessionPeer" / hook_remove_session_peer (session: *mut c_void, peer: u64, r8: u32, r9: u32) -> ()
}

game_fn! {
    /// `game.dll+0x13EFEA0` (hook or call).
    /// The friends-only join check: asks the platform's friend list about a peer id. HellForge
    /// keeps Steam ids as peer ids, so it answers for a Steam friend.
    IsSteamFriendFn / is_steam_friend / "IsSteamFriend" / hook_is_steam_friend (unused: u64, peer: u64) -> u8
}

game_fn! {
    /// `game.dll+0x5740B0` (call only).
    /// Applies a spawn's entity deltas to the new entity's components (the mission's HiveMind gets
    /// a private copy of its definition, HiveMind case 0x57525C): on the machine with authority at
    /// the create pass, elsewhere the replicated list after it. It reads EntityDelta's answer
    /// unchecked.
    ApplySpawnDeltasFn / apply_spawn_deltas / "ApplySpawnDeltas" / - (rcx: *mut c_void, descriptor: *mut u8, r8: *mut c_void, r9: *mut c_void) -> ()
}

game_fn! {
    /// `game.dll+0x506580` (call only).
    /// An entity's HiveMindComponent record (+0x74 the mission countdown, which the countdown
    /// readers take as the duration): its own in hf_global_HiveMindManager (+0x519AC, 0x43C each),
    /// else the shared definition (HiveMindDefinition); NULL for NULL.
    HiveMindOfFn / hive_mind_of / "HiveMindOf" / - (descriptor: *const u8) -> *mut u8
}

game_fn! {
    /// `game.dll+0x883200` (call only).
    /// Registers a unit for destruction from the destructible index (DestructibleIndexEntry): a
    /// unit missing from it logs "Could not find any destructible index data" and is never
    /// registered, which also skips some of its havok actors. Its holes take a record from a ring
    /// of 1024 (0x13E0760).
    RegisterDestructibleFn / register_destructible / "RegisterDestructible" / - (unused: u64, event: *mut u8, r8: *mut c_void) -> ()
}

game_fn! {
    /// `game.dll+0xF6EC10` (call only).
    /// Fills the destructible index ([[game + 0x3326550]] + 0x2C60) from its baked resource (type
    /// 0xB8FD4D2CEDE20ED7) when the level loads: 16-byte entries {u64 unit name, u64 the unit's
    /// destruction settings}.
    LoadDestructibleIndexFn / load_destructible_index / "LoadDestructibleIndex" / - (level: *mut c_void) -> ()
}

game_fn! {
    /// `game.dll+0xFC45C0` (call only).
    /// A unit's entry in the destructible index, probed from slot (i + 2 x name) & (capacity - 1)
    /// to the first empty name; NULL for none. HellForge enters its unit copies as their base.
    DestructibleIndexEntryFn / destructible_index_entry / "DestructibleIndexEntry" / - (index: *mut c_void, unit: u64) -> *mut u8
}

game_fn! {
    /// `game.dll+0x4B3B80` (hook or call).
    /// How near the nearest living player is to a point: 1 - (squared distance / radius squared)
    /// for the nearest within the radius, 0 when none is. The Pelican's turret uses it to spare
    /// targets near players.
    PlayerNearnessFn / player_nearness / "PlayerNearness" / hook_player_nearness (unused: u64, point: *const f32, radius: f32) -> f32
}

game_fn! {
    /// `game.dll+0x19430B0` (hook or call).
    /// Builds the Acquisition Center's warbond pager and its 32 dots over a {count, current} pair.
    /// HellForge pages it past 32 cards (right after it, 0x1968015).
    BuildWarbondPagerFn / build_warbond_pager / "BuildWarbondPager" / hook_build_warbond_pager (screen: *mut u8, pair: *mut u32, r8: *mut c_void, r9: u32) -> ()
}

game_fn! {
    /// `game.dll+0x1969F30` (hook or call).
    /// Refreshes the warbond buy button's look: per-variant border, highlight, fills and star (the
    /// premium variant's branch only), then its label (0x196A229). HellForge recolours it for
    /// Forge Credits.
    RefreshWarbondBuyButtonFn / refresh_warbond_buy_button / "RefreshWarbondBuyButton" / hook_refresh_warbond_buy_button (button: *mut u8) -> ()
}

game_fn! {
    /// `game.dll+0x1969E00` (hook or call).
    /// Updates the warbond buy button: refreshes it (RefreshWarbondBuyButton), then its state, 4
    /// (owned: no hover, no press, no price, label UNLOCKED) when the warbond's data says owned
    /// (+0x64), else idle, hover or press.
    UpdateWarbondBuyButtonFn / update_warbond_buy_button / "UpdateWarbondBuyButton" / hook_update_warbond_buy_button (button: *mut u8, dt: f32) -> ()
}

game_fn! {
    /// `game.dll+0x196CE10` (hook or call).
    /// Fills a warbond screen's data: its variant, whether it is owned and its price. A warbond
    /// with no token priced in Super Credits counts as owned (0x196CE86, the free-warbond rule),
    /// which HellForge narrows to free variants.
    WarbondScreenDataFn / warbond_screen_data / "WarbondScreenData" / hook_warbond_screen_data (ctx: *mut u8) -> ()
}

game_fn! {
    /// `game.dll+0x1963860` (hook or call).
    /// The Acquisition Center's warbond list builder: counts each warbond's tokens that have any
    /// price; with none it sets owned = 1, else owned = OwnsWarbond. HellForge makes a priced-less
    /// premium one stay locked.
    BuildWarbondListFn / build_warbond_list / "BuildWarbondList" / hook_build_warbond_list (rcx: i32, rdx: *mut u8, r8: *mut u8) -> ()
}

game_fn! {
    /// `game.dll+0x193A530` (hook or call).
    /// Updates a warbond page's item panel widget; for an item the player cannot afford it shows
    /// NOT ENOUGH MEDALS (0x193A6B6), which HellForge names in the item's own currency.
    UpdateWarbondPanelFn / update_warbond_panel / "UpdateWarbondPanel" / hook_update_warbond_panel (widget: *mut u8) -> ()
}

game_fn! {
    /// `game.dll+0x19302C0` (hook or call).
    /// Draws a warbond item's claim button; in state 3 (cannot afford) it reads NOT ENOUGH MEDALS
    /// (0x1930754), which HellForge names in the item's own currency.
    DrawWarbondClaimButtonFn / draw_warbond_claim_button / "DrawWarbondClaimButton" / hook_draw_warbond_claim_button (button: *mut u8) -> ()
}

game_fn! {
    /// `game.dll+0x1975060` (hook or call).
    /// Updates a warbond page, setting its lock text ("Spend #COUNT more medals to unlock",
    /// 0x19751AA / 0x19751B9) on every call. HellForge's stub gives a mod warbond its own
    /// currency's text.
    UpdateWarbondPageFn / update_warbond_page / "UpdateWarbondPage" / hook_update_warbond_page (page: *mut u8) -> ()
}

game_fn! {
    /// `game.dll+0x139CCD0` (hook or call).
    /// The preset path: finds an item's kit through the catalogue hierarchy {index, mixId, kit}
    /// (kit type +0x28 in hf_global_CustomizationManager's object). HellForge's warbond showcase
    /// finds a mod warbond item's kit the same way.
    ItemPresetKitFn / item_preset_kit / "ItemPresetKit" / hook_item_preset_kit (unused: u64, rdx: u32, r8: u32, xmm3: f32, s5: u32) -> u32
}

game_fn! {
    /// `game.dll+0x1750870` (call only).
    /// A boot's loadout validation: an unknown weapon becomes the default loadout's of its kind
    /// and is pushed through the armory's commit (PushArmoryLoadout). HellForge's loadout scrub
    /// does the same after a live change takes content out.
    ValidateBootLoadoutFn / validate_boot_loadout / "ValidateBootLoadout" / - (out: *mut c_void) -> ()
}

game_fn! {
    /// `game.dll+0x1753080` (hook or call).
    /// Acquires or releases a stratagem's packages from the row table[type] names at that moment
    /// (the package system counts holds per package id). A type another mod takes over in between
    /// would release that mod's row's packages.
    StratagemPackagesFn / stratagem_packages / "StratagemPackages" / hook_stratagem_packages (rcx: *mut c_void, type_: u32, unused: u64, r9: u32) -> u32
}

game_fn! {
    /// `game.dll+0x1367690` (call only).
    /// A per-update pass of the players' loadouts: releases each shared stratagem (PlayerHistory +
    /// 0x45410, 64 entries of 0x6C) no world object uses.
    ReleaseUnusedSharedStratagemsFn / release_unused_shared_stratagems / "ReleaseUnusedSharedStratagems" / - (rcx: *mut c_void) -> ()
}

game_fn! {
    /// `game.dll+0x11F2080` (hook or call).
    /// Loads generated_stratagem_settings.dl_bin into the stratagem list: clears exactly 150 slots
    /// (0x4B0 bytes), sets table[row +0] = row and counts the vanilla rows (game + 0x348E3FC). A
    /// reload never clears the moved list's new slots, but resets the count, which HellForge
    /// raises again. From HellForge's stratagems (stratagems.c kStratagemClearPin).
    LoadStratagemSettingsFn / load_stratagem_settings / "LoadStratagemSettings" / hook_load_stratagem_settings () -> ()
}

game_fn! {
    /// `game.dll+0x18D8710` (call only).
    /// The stratagem picker's list: walks the types (bound 0x18D8AA9 mov edi, 0x96) into 150 stack
    /// candidates {key, section, type, order, enabled, blocked, pad} and sorts them. HellForge
    /// grows the walk and its categories reorder the candidates.
    BuildStratagemPickerFn / build_stratagem_picker / "BuildStratagemPicker" / - (rcx: *mut u8, rdx: u32, r8: *mut c_void) -> ()
}

game_fn! {
    /// `game.dll+0x11F4F00` (call only).
    /// A weapon's mastery package, through its WeaponCustomizationComponentData record (component
    /// 271, package id32 at +0x12FC); NULL for none. HellForge gives each registered weapon a
    /// record naming its own package.
    MasteryPackageOfFn / mastery_package_of / "MasteryPackageOf" / - (id: u32) -> *mut u8
}

game_fn! {
    /// `game.dll+0x1493770` (hook or call).
    /// The armory's customization screen builder: lists the weapon's mastery package's level
    /// unlocks and rewards (no bound on its 64 entries) and sorts them; an unlock no attachment
    /// matched left an empty entry and crashed the sort.
    BuildCustomizationListFn / build_customization_list / "BuildCustomizationList" / hook_build_customization_list (screen: *mut u8, weapon: *const u32) -> ()
}

game_fn! {
    /// `game.dll+0xAB8B70` (call only).
    /// Creates a world for a level, with its level generator's block (CreateGenerationBlock). A
    /// block made before HellForge's larger-map install keeps the old layout.
    CreateWorldFn / create_world / "CreateWorld" / - (owner: *mut u8) -> ()
}

game_fn! {
    /// `game.dll+0xFB9610` (hook or call).
    /// Lays a structured map's (mega city, factory) fixed template of sites stretched over the
    /// city's rect: positions scale with the map, the count is fixed.
    StretchCityTemplateFn / stretch_city_template / "StretchCityTemplate" / hook_stretch_city_template (arena: *mut c_void, xmm1: f32, r8: u8, r9: *mut u8, s5: *mut c_void, s6: *mut c_void, s7: *mut c_void) -> ()
}

game_fn! {
    /// `game.dll+0xC126C0` (hook or call).
    /// Lays the level generator's route sites over an area, as a grid (map type 1, grid = 1) or a
    /// Poisson disc, and returns the site list (from 0xF2D370, which it ends in). A grid of
    /// inexact spacing splits its corners, so HellForge rounds a larger map's spacing to quarter
    /// metres.
    LaySitesFn / lay_sites / "LaySites" / hook_lay_sites (grid: u8, rdx: *mut c_void, spacing: f32, r9: *mut c_void, s5: *mut c_void, s6: *mut c_void) -> *mut c_void
}

game_fn! {
    /// `game.dll+0xF31BD0` (hook or call).
    /// For a zone town's wall rules (subregion stamps, op 0x2B): keeps a slot's area edges that
    /// border another type (the town's ring) within a reach of the slot; 0 when none is in reach.
    KeepTownEdgesFn / keep_town_edges / "KeepTownEdges" / hook_keep_town_edges (rcx: *mut c_void, rdx: *mut c_void, r8: *mut c_void, r9: *mut c_void, reach: f32, s6: u8, s7: u32, s8: *mut c_void, s9: *mut c_void) -> u8
}

game_fn! {
    /// `game.dll+0xFC9C10` (hook or call).
    /// Records a building a town or city placed: enters its rect in the stamp index
    /// (StampIndexInsert). HellForge counts the buildings placed through it (0xF99AAE).
    BuildingPlacedFn / building_placed / "BuildingPlaced" / hook_building_placed (rcx: *mut c_void, building: *mut u8, r8: u32) -> ()
}

game_fn! {
    /// `game.dll+0xF64A60` (hook or call).
    /// An area's rim height (its rule's +0x34), which GenerateSubregionHeights stores at the
    /// area's +0x88.
    AreaRimHeightFn / area_rim_height / "AreaRimHeight" / hook_area_rim_height (rcx: *mut c_void, xmm1: f32, r8: *mut c_void) -> f32
}

game_fn! {
    /// `game.dll+0xF66B80` (hook or call).
    /// FixUnreachableAreas' search for a walkable neighbour to open for a pocket of route cells; 0
    /// for none, and then the level is generated again. HellForge lets a larger map's level go on
    /// from the 7th try.
    FindPocketNeighbourFn / find_pocket_neighbour / "FindPocketNeighbour" / hook_find_pocket_neighbour (rcx: *mut c_void, rdx: u32, r8: *mut c_void, r9: u8, s5: *mut c_void, s6: *mut c_void, s7: *mut c_void) -> u8
}

game_fn! {
    /// `game.dll+0x13462C0` (hook or call).
    /// Reads the mission-end telemetry record (TelemetryMissionEnd's, hf_global_Telemetry +
    /// 0x31820) and sends it with an outcome: two 16-place stack arrays of objective instances,
    /// its count and loop stopping at 16.
    SendMissionEndTelemetryFn / send_mission_end_telemetry / "SendMissionEndTelemetry" / hook_send_mission_end_telemetry (unused: u64, outcome: *const u8, r8: u8, r9: u8) -> ()
}

game_fn! {
    /// `game.dll+0x1380840` (call only).
    /// Builds the mission result (BuildMissionResult, a record on its stack: badges at +0x14C8,
    /// {id, amount} x 32, count +0x15C8) and reports it.
    MissionResultReportFn / mission_result_report / "MissionResultReport" / - (rcx: *mut c_void, rdx: *mut c_void) -> ()
}

game_fn! {
    /// `game.dll+0x5DEF00` (call only).
    /// Sets up an objective's network record at its spawn: discovered_state (+0x44) starts at 1
    /// (unknown: not on the map) for optional objectives, by the objective's class (0x5DF35B).
    /// HellForge's radar station starts at 0.
    ObjectiveSpawnSetupFn / objective_spawn_setup / "ObjectiveSpawnSetup" / - (unused: u64, event: *mut u8, r8: *mut c_void, r9: *mut c_void, s5: *mut c_void, s6: *mut c_void) -> ()
}

game_fn! {
    /// `game.dll+0x617960` (hook or call).
    /// A rate-of-fire selector press: moves a ProjectileWeapon to its next rate and returns the
    /// new rate index. Reached from WeaponFunctionPress (function type 2).
    RateOfFireCycleFn / rate_of_fire_cycle / "RateOfFireCycle" / hook_rate_of_fire_cycle (unused: u64, weapon: u32) -> u32
}

game_fn! {
    /// `game.dll+0x7552D0` (call only).
    /// Dispatches a press of a weapon's bound function (fire mode, rate of fire, ...) by its type.
    WeaponFunctionPressFn / weapon_function_press / "WeaponFunctionPress" / - (weapons: *mut c_void, weapon: u32, function: u32) -> ()
}

game_fn! {
    /// `game.dll+0x6117C0` (call only).
    /// Seeds a ProjectileWeapon's rate struct, only on the machine with authority over it (the
    /// current RPM is networked from there).
    ProjectileWeaponRateSeedFn / projectile_weapon_rate_seed / "ProjectileWeaponRateSeed" / - (manager: *mut c_void, descriptor: *const u8) -> ()
}

game_fn! {
    /// `game.dll+0x6190C0` (hook or call).
    /// Applies a ProjectileWeapon's received network state (one of NetworkApplyState's cases).
    ProjectileWeaponNetApplyFn / projectile_weapon_net_apply / "ProjectileWeaponNetApply" / hook_projectile_weapon_net_apply (manager: *mut c_void, weapon: u32, buffer: *const u8, cursor: *mut u32) -> ()
}

game_fn! {
    /// `game.dll+0x617550` (hook or call).
    /// The ProjectileWeapon system's frame pass, from the world update: resets a weapon's rate
    /// when its fire-rate modifier changed, then runs the fire timers.
    ProjectileWeaponSystemPassFn / projectile_weapon_system_pass / "ProjectileWeaponSystemPass" / hook_projectile_weapon_system_pass (manager: *mut c_void, dt: f32) -> ()
}

game_fn! {
    /// `game.dll+0x755F90` (call only).
    /// Sets a weapon's fire mode and sends its RPM entry again.
    SetCurrentFireModeFn / set_current_fire_mode / "SetCurrentFireMode" / - (weapons: *mut c_void, weapon: u32, mode: u32) -> u32
}

game_fn! {
    /// `game.dll+0x755BA0` (call only).
    /// A weapon function's current value for an input: for rate of fire, the current rate index.
    WeaponFunctionValueFn / weapon_function_value / "WeaponFunctionValue" / - (weapons: *mut c_void, weapon: u32, input: u32) -> u32
}

game_fn! {
    /// `game.dll+0x746460` (hook or call).
    /// Applies another machine's trigger state to its weapon's copy here (the trigger byte into
    /// the manager's +0x58); it builds no projectile itself. One of NetworkApplyState's cases.
    WeaponApplyReplicatedTriggerFn / weapon_apply_replicated_trigger / "WeaponApplyReplicatedTrigger" / hook_weapon_apply_replicated_trigger (weapons: *mut c_void, weapon: u32, buffer: *const u8, cursor: *mut u32) -> ()
}

game_fn! {
    /// `game.dll+0xBBD5B0` (call only).
    /// The receiver of a simple fire message (rpc_fire_single_round): resolves the weapon by its
    /// network id and fires it through the generic dispatch into ProjectileWeaponFire with type 0.
    /// A table callback.
    RpcFireSingleRoundFn / rpc_fire_single_round / "RpcFireSingleRound" / - (unused: u64, args: *const u8) -> ()
}

game_fn! {
    /// `game.dll+0xBC1010` (call only).
    /// The receiver of an advanced fire message (rpc_fire_single_round_advanced): fires the weapon
    /// with the sent type (skipping ResolveFiredType) and multipliers. A table callback (pointer
    /// at game + 0x214D430).
    RpcFireSingleRoundAdvancedFn / rpc_fire_single_round_advanced / "RpcFireSingleRoundAdvanced" / - (unused: u64, args: *const u8) -> ()
}

game_fn! {
    /// `game.dll+0xBF2A10` (hook or call).
    /// Sends a shot to the other machines as an advanced fire message, from ProjectileWeaponFire
    /// and the charge fire.
    SendFireSingleRoundAdvancedFn / send_fire_single_round_advanced / "SendFireSingleRoundAdvanced" / hook_send_fire_single_round_advanced (unused: u64, weapon: u32, direction: *const f32, token: u32, type_: u32, s6: u64, s7: u32, speed: f32, damage: f32, penetration: f32) -> ()
}

game_fn! {
    /// `game.dll+0xBB9950` (call only).
    /// The receiver of rpc_add_projectile_to_manager (config index 428): spawns a projectile of
    /// the sent type through 0xB86F10, which reads the projectile list with no bound. A table
    /// callback.
    RpcAddProjectileToManagerFn / rpc_add_projectile_to_manager / "RpcAddProjectileToManager" / - (unused: u64, args: *const u8) -> ()
}

game_fn! {
    /// `game.dll+0x79B480` (hook or call).
    /// Sets who wields a weapon (hf_global_WieldableManager's), at a pickup and other transfers.
    SetWielderFn / set_wielder / "SetWielder" / hook_set_wielder (unused: u64, weapon: u32, wielder: u32) -> ()
}

game_fn! {
    /// `game.dll+0x618EB0` (hook or call).
    /// Builds a ProjectileWeapon's network state fields at its creation (from the create pass's
    /// init table).
    ProjectileWeaponNetInitFn / projectile_weapon_net_init / "ProjectileWeaponNetInit" / hook_projectile_weapon_net_init (unused: u64, descriptor: *const u8, unused2: u64, fields: *mut u8, count: *mut u32, allocator: *mut c_void) -> ()
}

game_fn! {
    /// `game.dll+0x752370` (call only).
    /// Builds a Weapon instance: copies its input bindings (WeaponData +0xB8 / +0xBC into the
    /// instance's +0x350 / +0x354). A component callback.
    WeaponBuildFn / weapon_build / "WeaponBuild" / - (unused: u64, descriptor: *const u8, r8: *mut c_void) -> ()
}

game_fn! {
    /// `game.dll+0x61A260` (call only).
    /// Moves a ProjectileWeapon into the owned partition when this machine gains authority over it
    /// (MigrateToThisMachine's table), zeroing its rate struct.
    ProjectileWeaponEnterOwnedFn / projectile_weapon_enter_owned / "ProjectileWeaponEnterOwned" / - (unused: u64, descriptor: *const u8) -> ()
}

game_fn! {
    /// `game.dll+0x136E470` (hook or call).
    /// Whether the player owns a warbond: walks the season-pass list (account + 0x17492D8, 0x64
    /// each) and answers 1 when the player holds any of its tokens (+0x14..+0x20,
    /// HasCatalogueItem).
    OwnsWarbondFn / owns_warbond / "OwnsWarbond" / hook_owns_warbond (unused: u64, warbond: u32) -> u8
}

game_fn! {
    /// `game.dll+0x136C270` (call only).
    /// The warbond token whose first price (buyPrice[0].mixId) is the currency type's, only when
    /// it has no second price; 0 otherwise. WarbondScreenData and the warbond list take a warbond
    /// with none as free (owned).
    WarbondCurrencyTokenFn / warbond_currency_token / "WarbondCurrencyToken" / - (unused: u64, out: *mut u32, warbond: u32, currency: u32) -> ()
}

game_fn! {
    /// `game.dll+0x136FB40` (call only).
    /// Whether the player holds a catalogue item.
    HasCatalogueItemFn / has_catalogue_item / "HasCatalogueItem" / - (account: *mut c_void, item: u32) -> u8
}

game_fn! {
    /// `game.dll+0x4F3740` (hook or call).
    /// The entity's WeaponMagazineComponentData (component 5): its own copy in the manager's
    /// private copies if it has one, else its type's shared record (WeaponMagazineTypeRecord);
    /// NULL when the type has none.
    WeaponMagazineRecordFn / weapon_magazine_record / "WeaponMagazineRecord" / hook_weapon_magazine_record (descriptor: *mut c_void) -> *mut c_void
}

game_fn! {
    /// `game.dll+0x4F8090` (hook or call).
    /// The entity's WeaponWindUpComponentData (component 70): its own copy in the manager's
    /// private copies if it has one, else its type's shared record (the type lookup at
    /// game.dll+0x4F7C70, reached only by this tail jump, so not mapped); NULL when the type has
    /// none.
    WeaponWindUpRecordFn / weapon_wind_up_record / "WeaponWindUpRecord" / hook_weapon_wind_up_record (descriptor: *mut c_void) -> *mut c_void
}

game_fn! {
    /// `game.dll+0x4FBC30` (hook or call).
    /// The entity's RechargeComponentData (component 99): its own copy in the manager's private
    /// copies if it has one, else its type's shared record (RechargeTypeRecord); NULL when the
    /// type has none.
    RechargeRecordFn / recharge_record / "RechargeRecord" / hook_recharge_record (descriptor: *mut c_void) -> *mut c_void
}

game_fn! {
    /// `game.dll+0x4FD220` (hook or call).
    /// The entity's WeaponReloadComponentData (component 113): its own copy in the manager's
    /// private copies if it has one, else its type's shared record (WeaponReloadTypeRecord); NULL
    /// when the type has none.
    WeaponReloadRecordFn / weapon_reload_record / "WeaponReloadRecord" / hook_weapon_reload_record (descriptor: *mut c_void) -> *mut c_void
}

game_fn! {
    /// `game.dll+0x4FDD20` (hook or call).
    /// The entity's WeaponRoundsComponentData (component 117): its own copy in the manager's
    /// private copies if it has one, else its type's shared record (WeaponRoundsTypeRecord); NULL
    /// when the type has none.
    WeaponRoundsRecordFn / weapon_rounds_record / "WeaponRoundsRecord" / hook_weapon_rounds_record (descriptor: *mut c_void) -> *mut c_void
}

game_fn! {
    /// `game.dll+0x5052C0` (hook or call).
    /// The entity's WeaponChargeComponentData (component 204): its own copy in the manager's
    /// private copies if it has one, else its type's shared record (the type lookup at
    /// game.dll+0x504D80, reached only by this tail jump, so not mapped); NULL when the type has
    /// none.
    WeaponChargeRecordFn / weapon_charge_record / "WeaponChargeRecord" / hook_weapon_charge_record (descriptor: *mut c_void) -> *mut c_void
}

game_fn! {
    /// `game.dll+0x5092B0` (hook or call).
    /// The entity's EquipmentComponentData (component 233): its own copy in the manager's private
    /// copies if it has one, else its type's shared record (EquipmentTypeRecord); NULL when the
    /// type has none.
    EquipmentRecordFn / equipment_record / "EquipmentRecord" / hook_equipment_record (descriptor: *mut c_void) -> *mut c_void
}

game_fn! {
    /// `game.dll+0x50CCC0` (hook or call).
    /// The entity's ThrowerComponentData (component 258): its own copy in the manager's private
    /// copies if it has one, else its type's shared record (ThrowerTypeRecord); NULL when the type
    /// has none.
    ThrowerRecordFn / thrower_record / "ThrowerRecord" / hook_thrower_record (descriptor: *mut c_void) -> *mut c_void
}

game_fn! {
    /// `game.dll+0x50DC50` (hook or call).
    /// The entity's ArcWeaponComponentData (component 265): its own copy in the manager's private
    /// copies if it has one, else its type's shared record (ArcWeaponTypeRecord); NULL when the
    /// type has none.
    ArcWeaponRecordFn / arc_weapon_record / "ArcWeaponRecord" / hook_arc_weapon_record (descriptor: *mut c_void) -> *mut c_void
}

game_fn! {
    /// `game.dll+0x50E1F0` (hook or call).
    /// The entity's WeaponHeatComponentData (component 266): its own copy in the manager's private
    /// copies if it has one, else its type's shared record (WeaponHeatTypeRecord); NULL when the
    /// type has none.
    WeaponHeatRecordFn / weapon_heat_record / "WeaponHeatRecord" / hook_weapon_heat_record (descriptor: *mut c_void) -> *mut c_void
}

game_fn! {
    /// `game.dll+0x50F150` (hook or call).
    /// The entity's WeaponCustomizationComponentData (component 271): its own copy in the
    /// manager's private copies if it has one, else its type's shared record
    /// (WeaponCustomizationTypeRecord); NULL when the type has none.
    WeaponCustomizationRecordFn / weapon_customization_record / "WeaponCustomizationRecord" / hook_weapon_customization_record (descriptor: *mut c_void) -> *mut c_void
}

game_fn! {
    /// `game.dll+0x4F32F0` (hook or call).
    /// An entity type's shared WeaponMagazineComponentData (component 5) from the loaded component
    /// table: an open-addressing probe of the table's (type, record) rows; NULL when the type has
    /// no row. A leaf.
    WeaponMagazineTypeRecordFn / weapon_magazine_type_record / "WeaponMagazineTypeRecord" / hook_weapon_magazine_type_record (type_: u64) -> *mut c_void
}

game_fn! {
    /// `game.dll+0x4FB850` (hook or call).
    /// An entity type's shared RechargeComponentData (component 99) from the loaded component
    /// table: an open-addressing probe of the table's (type, record) rows; NULL when the type has
    /// no row. A leaf.
    RechargeTypeRecordFn / recharge_type_record / "RechargeTypeRecord" / hook_recharge_type_record (type_: u64) -> *mut c_void
}

game_fn! {
    /// `game.dll+0x4FCE20` (hook or call).
    /// An entity type's shared WeaponReloadComponentData (component 113) from the loaded component
    /// table: an open-addressing probe of the table's (type, record) rows; NULL when the type has
    /// no row. A leaf.
    WeaponReloadTypeRecordFn / weapon_reload_type_record / "WeaponReloadTypeRecord" / hook_weapon_reload_type_record (type_: u64) -> *mut c_void
}

game_fn! {
    /// `game.dll+0x4FD8E0` (hook or call).
    /// An entity type's shared WeaponRoundsComponentData (component 117) from the loaded component
    /// table: an open-addressing probe of the table's (type, record) rows; NULL when the type has
    /// no row. A leaf.
    WeaponRoundsTypeRecordFn / weapon_rounds_type_record / "WeaponRoundsTypeRecord" / hook_weapon_rounds_type_record (type_: u64) -> *mut c_void
}

game_fn! {
    /// `game.dll+0x508E90` (hook or call).
    /// An entity type's shared EquipmentComponentData (component 233) from the loaded component
    /// table: an open-addressing probe of the table's (type, record) rows; NULL when the type has
    /// no row. A leaf.
    EquipmentTypeRecordFn / equipment_type_record / "EquipmentTypeRecord" / hook_equipment_type_record (type_: u64) -> *mut c_void
}

game_fn! {
    /// `game.dll+0x50C720` (hook or call).
    /// An entity type's shared ThrowerComponentData (component 258) from the loaded component
    /// table: an open-addressing probe of the table's (type, record) rows; NULL when the type has
    /// no row. A leaf.
    ThrowerTypeRecordFn / thrower_type_record / "ThrowerTypeRecord" / hook_thrower_type_record (type_: u64) -> *mut c_void
}

game_fn! {
    /// `game.dll+0x50CD90` (hook or call).
    /// An entity type's shared ThrowableComponentData (component 259) from the loaded component
    /// table: an open-addressing probe of the table's (type, record) rows; NULL when the type has
    /// no row. A leaf.
    ThrowableTypeRecordFn / throwable_type_record / "ThrowableTypeRecord" / hook_throwable_type_record (type_: u64) -> *mut c_void
}

game_fn! {
    /// `game.dll+0x50D900` (hook or call).
    /// An entity type's shared ArcWeaponComponentData (component 265) from the loaded component
    /// table: an open-addressing probe of the table's (type, record) rows; NULL when the type has
    /// no row. A leaf.
    ArcWeaponTypeRecordFn / arc_weapon_type_record / "ArcWeaponTypeRecord" / hook_arc_weapon_type_record (type_: u64) -> *mut c_void
}

game_fn! {
    /// `game.dll+0x50DD30` (hook or call).
    /// An entity type's shared WeaponHeatComponentData (component 266) from the loaded component
    /// table: an open-addressing probe of the table's (type, record) rows; NULL when the type has
    /// no row. A leaf.
    WeaponHeatTypeRecordFn / weapon_heat_type_record / "WeaponHeatTypeRecord" / hook_weapon_heat_type_record (type_: u64) -> *mut c_void
}

game_fn! {
    /// `game.dll+0x50E2C0` (hook or call).
    /// An entity type's shared AbilityWeaponComponentData (component 267) from the loaded
    /// component table: an open-addressing probe of the table's (type, record) rows; NULL when the
    /// type has no row. A leaf.
    AbilityWeaponTypeRecordFn / ability_weapon_type_record / "AbilityWeaponTypeRecord" / hook_ability_weapon_type_record (type_: u64) -> *mut c_void
}

game_fn! {
    /// `game.dll+0x50ED80` (hook or call).
    /// An entity type's shared WeaponCustomizationComponentData (component 271) from the loaded
    /// component table: an open-addressing probe of the table's (type, record) rows; NULL when the
    /// type has no row. A leaf.
    WeaponCustomizationTypeRecordFn / weapon_customization_type_record / "WeaponCustomizationTypeRecord" / hook_weapon_customization_type_record (type_: u64) -> *mut c_void
}

game_fn! {
    /// `game.dll+0x7406A0` (hook or call).
    /// The weapon manager's frame, right after the WeaponData frames: among its calls
    /// AbilityWeaponFire (an ability weapon's pending fire) and WeaponChargeRecord.
    WeaponUpdateFn / weapon_update / "WeaponUpdate" / hook_weapon_update (weapons: *mut c_void, dt: f32) -> ()
}

game_fn! {
    /// `game.dll+0x753DB0` (hook or call).
    /// One WeaponData instance's frame (its record from WeaponDataRecord, the current fire mode);
    /// the world update calls it for every instance.
    WeaponDataUpdateFn / weapon_data_update / "WeaponDataUpdate" / hook_weapon_data_update (weapondata: *mut c_void, index: u32, dt: f32) -> ()
}

game_fn! {
    /// `game.dll+0x764970` (hook or call).
    /// The WeaponHeat frame: WeaponHeatInstanceStep for every instance this machine owns
    /// (descriptor +0x14 bit 1), then each weapon's reload-dependent heat state
    /// (WeaponReloadBusy).
    WeaponHeatUpdateFn / weapon_heat_update / "WeaponHeatUpdate" / hook_weapon_heat_update (heats: *mut c_void, dt: f32) -> ()
}

game_fn! {
    /// `game.dll+0x762F60` (hook or call).
    /// One weapon's heat for a frame, on its owner: charging adds heat, else it cools at +128 per
    /// second (overheated or charging: +140, none when +144 is set) times the climate multiplier;
    /// heat at the capacity +96 sets the overheated flag when +80 is set, heat down to +100 clears
    /// it. The heat (live row +0x58, 12 bytes: +4 heat, +8 overheated) goes out as network field
    /// 0xA5023836.
    WeaponHeatInstanceStepFn / weapon_heat_instance_step / "WeaponHeatInstanceStep" / hook_weapon_heat_instance_step (heats: *mut c_void, index: u32, dt: f32) -> ()
}

game_fn! {
    /// `game.dll+0x73C8F0` (hook or call).
    /// The WeaponCharge frame: the charge time of every charging weapon rises while the trigger is
    /// held, and the charge levels decide the release (ChargeRelease) or, past the overcharge,
    /// ChargeOvercharge.
    WeaponChargeUpdateFn / weapon_charge_update / "WeaponChargeUpdate" / hook_weapon_charge_update (charges: *mut c_void, dt: f32) -> ()
}

game_fn! {
    /// `game.dll+0x73D8C0` (hook or call).
    /// Releases a weapon's charge: on the machine that owns the instance (descriptor +0x14 bit 1)
    /// it fires the charge level's projectile through ChargeFire, once per burst shot.
    ChargeReleaseFn / charge_release / "ChargeRelease" / hook_charge_release (charges: *mut c_void, descriptor: *mut c_void) -> ()
}

game_fn! {
    /// `game.dll+0x73DC60` (hook or call).
    /// A weapon held past its overcharge: on the machine that owns it the weapon is removed and
    /// its explosion set off.
    ChargeOverchargeFn / charge_overcharge / "ChargeOvercharge" / hook_charge_overcharge (unused: u64, descriptor: *mut c_void) -> ()
}

game_fn! {
    /// `game.dll+0x73E280` (hook or call).
    /// A charged shot's multiplier from the weapon's charge time (live row +0x40, 40 bytes, +8):
    /// below full charge (+0x18) from at_min to 1, above it from 1 to at_overcharge. The charge
    /// fire (0x73E725, 0x73E7E5) and the arc spawn (0x13B3F79, 0x13B401B) scale their shot with
    /// it, each passing a pair of charge record fields.
    ChargeMultiplierFn / charge_multiplier / "ChargeMultiplier" / hook_charge_multiplier (charges: *mut c_void, weapon: u32, at_min: f32, at_overcharge: f32) -> f32
}

game_fn! {
    /// `game.dll+0x8C16E0` (hook or call).
    /// The Equipment frame (the component of how held items are wielded, holstered and dropped).
    EquipmentUpdateFn / equipment_update / "EquipmentUpdate" / hook_equipment_update (equipment: *mut c_void, dt: f32) -> ()
}

game_fn! {
    /// `game.dll+0x6CB140` (hook or call).
    /// The Thrower frame: throws the salvos of a thrower (mine dispensers and the like) by
    /// spawning their projectiles (SpawnProjectile).
    ThrowerUpdateFn / thrower_update / "ThrowerUpdate" / hook_thrower_update (throwers: *mut c_void, dt: f32) -> ()
}

game_fn! {
    /// `game.dll+0x6C3AE0` (hook or call).
    /// The Throwable frame: thrown grenades and knives (records from ThrowableTypeRecord), queuing
    /// their network fields.
    ThrowableUpdateFn / throwable_update / "ThrowableUpdate" / hook_throwable_update (throwables: *mut c_void, dt: f32) -> ()
}

game_fn! {
    /// `game.dll+0x80DDD0` (hook or call).
    /// One ArcWeapon instance's frame (its record from ArcWeaponRecord).
    ArcWeaponUpdateFn / arc_weapon_update / "ArcWeaponUpdate" / hook_arc_weapon_update (arcs: *mut c_void, index: u32, dt: f32) -> ()
}

game_fn! {
    /// `game.dll+0x7D67B0` (hook or call).
    /// The AbilityWeapon frame: reads each instance's record (AbilityWeaponTypeRecord) and may
    /// start its ability (StartAbility).
    AbilityWeaponUpdateFn / ability_weapon_update / "AbilityWeaponUpdate" / hook_ability_weapon_update (abilityweapons: *mut c_void) -> ()
}

game_fn! {
    /// `game.dll+0x74ACE0` (hook or call).
    /// The WeaponCustomization frame: each weapon's attachments (records from
    /// WeaponCustomizationRecord).
    WeaponCustomizationUpdateFn / weapon_customization_update / "WeaponCustomizationUpdate" / hook_weapon_customization_update (customizations: *mut c_void, dt: f32) -> ()
}

game_fn! {
    /// `game.dll+0x774B60` (hook or call).
    /// Starts a weapon's reload: unless forced, CanReload must agree; then it starts the
    /// WeaponReload record's ability (+4) through StartAbility at the reload speed (player stats,
    /// the +56 duration override). A record with ability 0 has no reload.
    StartReloadFn / start_reload / "StartReload" / hook_start_reload (reloads: *mut c_void, weapon: u32, force: u8) -> ()
}

game_fn! {
    /// `game.dll+0x775580` (hook or call).
    /// The reload gate: 1 when the weapon may reload now: its own spare total (OwnSpareAmmo) above
    /// 0, or else a worn backpack that can feed it.
    CanReloadFn / can_reload / "CanReload" / hook_can_reload (reloads: *mut c_void, weapon: u32, r8: u8, r9: u8) -> u8
}

game_fn! {
    /// `game.dll+0x7776F0` (hook or call).
    /// Whether a weapon's reload state (WeaponReloads +0x50, a u32 per instance) has any of the
    /// bits in mask; 0 when the weapon has no WeaponReload instance.
    ReloadStateHasFn / reload_state_has / "ReloadStateHas" / hook_reload_state_has (reloads: *mut c_void, weapon: u32, mask: u32) -> u8
}

game_fn! {
    /// `game.dll+0x7777A0` (hook or call).
    /// Whether a weapon's reload is under way: a reload state bit 1, 8 or 0x20 without its pair 2,
    /// 0x10 or 0x40, or bit 4 (ReloadStateHas on WeaponReloads).
    WeaponReloadBusyFn / weapon_reload_busy / "WeaponReloadBusy" / hook_weapon_reload_busy (unused: u64, weapon: u32) -> u8
}

game_fn! {
    /// `game.dll+0x744180` (hook or call).
    /// A weapon's own spare total: by its type, the spare magazines, spare rounds or spare
    /// heatsinks, plus a mounted shared deposit.
    OwnSpareAmmoFn / own_spare_ammo / "OwnSpareAmmo" / hook_own_spare_ammo (weapons: *mut c_void, weapon: u32) -> u32
}

game_fn! {
    /// `game.dll+0x757F70` (hook or call).
    /// Refills one weapon from a resupply: MagazineRefill for a magazine weapon, else RoundsRefill
    /// for one loaded by rounds, else HeatsinkRefill. resupply_all calls it for every carried
    /// weapon.
    WeaponRefillFn / weapon_refill / "WeaponRefill" / hook_weapon_refill (unused: u64, weapon: u32, scale: f32) -> ()
}

game_fn! {
    /// `game.dll+0x76E6D0` (hook or call).
    /// Gives a weapon spare magazines from a resupply: min(spare + max(1, magazines_from_supply x
    /// scale), the maximum), sent as network field 0xEC64918B; on the weapon's owner.
    MagazineRefillFn / magazine_refill / "MagazineRefill" / hook_magazine_refill (unused: u64, weapon: u32, scale: f32) -> ()
}

game_fn! {
    /// `game.dll+0x77A430` (hook or call).
    /// Gives a weapon loaded by rounds spare rounds from a resupply (rounds_from_supply x scale,
    /// up to the maximum).
    RoundsRefillFn / rounds_refill / "RoundsRefill" / hook_rounds_refill (unused: u64, weapon: u32, scale: f32) -> ()
}

game_fn! {
    /// `game.dll+0x764FA0` (hook or call).
    /// Gives a heat weapon spare heatsinks from a resupply (its WeaponHeat record), sent as
    /// network field 0xEC64918B.
    HeatsinkRefillFn / heatsink_refill / "HeatsinkRefill" / hook_heatsink_refill (unused: u64, weapon: u32, scale: f32) -> ()
}

game_fn! {
    /// `game.dll+0x7D5FB0` (hook or call).
    /// An ability weapon's settings for its current fire mode: the AbilityWeapon record + 40 x
    /// mode (+0 the ability, +32 spends a round, +33 always); NULL without an AbilityWeapon
    /// instance.
    AbilityWeaponModeSettingsFn / ability_weapon_mode_settings / "AbilityWeaponModeSettings" / hook_ability_weapon_mode_settings (unused: u64, weapon: u32) -> *mut c_void
}

game_fn! {
    /// `game.dll+0x7D6080` (hook or call).
    /// An ability weapon fires: starts its fire mode's ability on the weapon (StartAbility) and,
    /// when the mode says so, spends a round (SpendRound).
    AbilityWeaponFireFn / ability_weapon_fire / "AbilityWeaponFire" / hook_ability_weapon_fire (unused: u64, weapon: u32) -> ()
}

game_fn! {
    /// `game.dll+0x7D38D0` (call only).
    /// Starts a melee hit volume: a melee ability's handler passes its template, which is copied
    /// into the entity's active attack record (the manager at game.dll+0x3326830, 0x4BD8 bytes per
    /// instance), so edits to a template reach later swings only.
    MeleeHitVolumeStartFn / melee_hit_volume_start / "MeleeHitVolumeStart" / - (unused: u64, entity: u32, volume: *const c_void) -> ()
}

game_fn! {
    /// `game.dll+0x4F9B60` (hook or call).
    /// The entity's UnitComponentData (0x88 bytes): its own copy in the UnitManager (private
    /// copies at +0xC0) if it has one, else the shared type record.
    UnitSettingsFn / unit_settings / "UnitSettings" / hook_unit_settings (descriptor: *mut c_void) -> *mut c_void
}

game_fn! {
    /// `game.dll+0x4FA7A0` (hook or call).
    /// The entity's VehicleComponentData (0xE78 bytes): its own copy in the VehicleManager
    /// (private copies at +0xC0) if it has one, else the shared type record.
    VehicleSettingsFn / vehicle_settings / "VehicleSettings" / hook_vehicle_settings (descriptor: *mut c_void) -> *mut c_void
}

game_fn! {
    /// `game.dll+0x5011F0` (hook or call).
    /// The entity's HellpodPayloadComponentData (0xC bytes): its own copy in the
    /// HellpodPayloadManager (private copies at +0xA0) if it has one, else the shared type record.
    HellpodPayloadSettingsFn / hellpod_payload_settings / "HellpodPayloadSettings" / hook_hellpod_payload_settings (descriptor: *mut c_void) -> *mut c_void
}

game_fn! {
    /// `game.dll+0x508DB0` (hook or call).
    /// The avatar's AvatarComponentData (0x354 bytes: the Helldiver's movement speeds, +0x10 jog,
    /// +0x14 sprint, and stamina values, read every frame): its own copy in the avatar manager
    /// hf_global_AvatarManager (inline from +0x547D24) if it has one, else the shared type record.
    AvatarSettingsFn / avatar_settings / "AvatarSettings" / hook_avatar_settings (descriptor: *mut c_void) -> *mut c_void
}

game_fn! {
    /// `game.dll+0x50D830` (hook or call).
    /// The entity's JumppackComponentData (0x118 bytes: the jump pack's and hover pack's flight
    /// values, read every frame, so a change applies on the next frame): its own copy in the
    /// JumppackManager (private copies at +0xA0) if it has one, else the shared type record.
    JumppackSettingsFn / jumppack_settings / "JumppackSettings" / hook_jumppack_settings (descriptor: *mut c_void) -> *mut c_void
}

game_fn! {
    /// `game.dll+0x50FA40` (hook or call).
    /// The entity's DisplacementComponentData (0x278 bytes): its own copy in the
    /// DisplacementManager (private copies at +0xA0) if it has one, else the shared type record.
    DisplacementSettingsFn / displacement_settings / "DisplacementSettings" / hook_displacement_settings (descriptor: *mut c_void) -> *mut c_void
}

game_fn! {
    /// `game.dll+0x512E20` (hook or call).
    /// The entity's OrbitalAbilityComponentData (0x228 bytes): its own copy in the
    /// OrbitalAbilityManager (private copies at +0xD8) if it has one, else the shared type record.
    OrbitalAbilitySettingsFn / orbital_ability_settings / "OrbitalAbilitySettings" / hook_orbital_ability_settings (descriptor: *mut c_void) -> *mut c_void
}

game_fn! {
    /// `game.dll+0x513690` (hook or call).
    /// The entity's RotationComponentData (0x138 bytes): its own copy in the RotationManager
    /// (private copies at +0xB8) if it has one, else the shared type record.
    RotationSettingsFn / rotation_settings / "RotationSettings" / hook_rotation_settings (descriptor: *mut c_void) -> *mut c_void
}

game_fn! {
    /// `game.dll+0x514B40` (hook or call).
    /// The Eagle jet's EagleComponentData (0x98 bytes: +0x10 its payload kind, +0x18 its strike
    /// projectile, +0x4C its speed, +0x78 its attack distance): its own copy in the EagleManager
    /// (private copies at +0xB0) if it has one, else the shared type record. The update, the
    /// strike and the jet's set-up resolve it every time.
    EagleSettingsFn / eagle_settings / "EagleSettings" / hook_eagle_settings (descriptor: *mut c_void) -> *mut c_void
}

game_fn! {
    /// `game.dll+0x6FEF80` (hook or call).
    /// The vehicle system's frame: vehicles and mechs. The world update (0x571250) calls it every
    /// frame.
    VehicleUpdateFn / vehicle_update / "VehicleUpdate" / hook_vehicle_update (manager: *mut c_void, dt: f32) -> ()
}

game_fn! {
    /// `game.dll+0x6864D0` (hook or call).
    /// The squad transport system's frame: enemy dropships and the squads they carry, which it
    /// spawns through SpawnSquad. The world update (0x571250) calls it every frame.
    SquadTransportUpdateFn / squad_transport_update / "SquadTransportUpdate" / hook_squad_transport_update (manager: *mut c_void, dt: f32) -> ()
}

game_fn! {
    /// `game.dll+0x9322D0` (hook or call).
    /// The hellpod payload system's frame. The world update (0x571250) calls it every frame.
    HellpodPayloadUpdateFn / hellpod_payload_update / "HellpodPayloadUpdate" / hook_hellpod_payload_update (manager: *mut c_void, dt: f32) -> ()
}

game_fn! {
    /// `game.dll+0x7F7820` (hook or call).
    /// The AI spawner system's frame: the spawners spawn their squads through SpawnSquad. The
    /// world update (0x571250) calls it every frame.
    AiSpawnerUpdateFn / ai_spawner_update / "AiSpawnerUpdate" / hook_ai_spawner_update (manager: *mut c_void, dt: f32) -> ()
}

game_fn! {
    /// `game.dll+0x857F90` (hook or call).
    /// The bug hole system's frame: holes spawn their bugs through SpawnSquad with the
    /// HiveMindManager. The world update (0x571250) calls it every frame.
    BugholeUpdateFn / bughole_update / "BugholeUpdate" / hook_bughole_update (manager: *mut c_void, dt: f32) -> ()
}

game_fn! {
    /// `game.dll+0x7152F0` (hook or call).
    /// The vehicle motion system's frame: the vehicles' and mechs' driving physics. The world
    /// update (0x571250) calls it every frame.
    VehicleMotionUpdateFn / vehicle_motion_update / "VehicleMotionUpdate" / hook_vehicle_motion_update (manager: *mut c_void, dt: f32) -> ()
}

game_fn! {
    /// `game.dll+0x829E90` (hook or call).
    /// The avatar system's frame, for the avatars simulated here (the manager's first +0x70
    /// indexes): movement speed from AvatarSettings and stamina regeneration. The world update
    /// (0x571250) calls it every frame.
    AvatarUpdateFn / avatar_update / "AvatarUpdate" / hook_avatar_update (manager: *mut c_void, dt: f32) -> ()
}

game_fn! {
    /// `game.dll+0x88A700` (hook or call).
    /// The displacement system's frame. The world update (0x571250) calls it every frame.
    DisplacementUpdateFn / displacement_update / "DisplacementUpdate" / hook_displacement_update (manager: *mut c_void, dt: f32) -> ()
}

game_fn! {
    /// `game.dll+0x5F35B0` (hook or call).
    /// The orbital ability system's frame. The world update (0x571250) calls it every frame.
    OrbitalAbilityUpdateFn / orbital_ability_update / "OrbitalAbilityUpdate" / hook_orbital_ability_update (manager: *mut c_void, dt: f32) -> ()
}

game_fn! {
    /// `game.dll+0x8A7A90` (hook or call).
    /// The Eagle system's frame: for each jet (indexes below the manager's +0x20) it resolves
    /// EagleSettings and runs EagleStage, EagleTrackAim, EagleStrike and EagleFlight, then the
    /// flyby sound. The world update (0x571250) calls it every frame.
    EagleUpdateFn / eagle_update / "EagleUpdate" / hook_eagle_update (manager: *mut c_void, dt: f32) -> ()
}

game_fn! {
    /// `game.dll+0x8B8A20` (hook or call).
    /// The director's reinforcements: an encounter picks a composition
    /// (PickReinforcementComposition) and sends it to the faction's transport or breach
    /// (DispatchReinforcement). The world update (0x571250) calls it every frame.
    EncounterUpdateFn / encounter_update / "EncounterUpdate" / hook_encounter_update (manager: *mut c_void, dt: f32) -> ()
}

game_fn! {
    /// `game.dll+0x9110E0` (hook or call).
    /// The outposts' frame: it fills an outpost with its guards (FillOutpost) when it needs them.
    /// The world update (0x571250) calls it every frame.
    GuardForceUpdateFn / guard_force_update / "GuardForceUpdate" / hook_guard_force_update (manager: *mut c_void, dt: f32) -> ()
}

game_fn! {
    /// `game.dll+0x8A2990` (hook or call).
    /// Moves an Eagle jet through its stages and sets its speed per stage (EagleSettings +0x50,
    /// +0x54). EagleUpdate calls it for each jet.
    EagleStageFn / eagle_stage / "EagleStage" / hook_eagle_stage (eagles: *mut c_void, index: u32) -> ()
}

game_fn! {
    /// `game.dll+0x8A37D0` (hook or call).
    /// Keeps an Eagle jet's aim point on its targets, leading moving ones by its strike
    /// projectile's speed. EagleUpdate calls it for each jet.
    EagleTrackAimFn / eagle_track_aim / "EagleTrackAim" / hook_eagle_track_aim (eagles: *mut c_void, index: u32, dt: f32) -> ()
}

game_fn! {
    /// `game.dll+0x8A4410` (hook or call).
    /// An Eagle jet's attack: by its payload kind (EagleSettings +0x10: strafe, rockets, bombs) it
    /// starts within its attack distance and releases its strike projectiles. EagleUpdate calls it
    /// for each jet.
    EagleStrikeFn / eagle_strike / "EagleStrike" / hook_eagle_strike (eagles: *mut c_void, index: u32, dt: f32) -> ()
}

game_fn! {
    /// `game.dll+0x8A59E0` (hook or call).
    /// An Eagle jet's flight model. EagleUpdate calls it for each jet.
    EagleFlightFn / eagle_flight / "EagleFlight" / hook_eagle_flight (eagles: *mut c_void, index: u32, dt: f32) -> ()
}

game_fn! {
    /// `game.dll+0x9B5EC0` (hook or call).
    /// A jump pack's or hover pack's timers, abilities and visual sway, every frame. The world
    /// update runs it for each index below the manager's +0x10.
    JumppackTimersFn / jumppack_timers / "JumppackTimers" / hook_jumppack_timers (jumppacks: *mut c_void, index: u32, dt: f32) -> ()
}

game_fn! {
    /// `game.dll+0x9B4790` (hook or call).
    /// A jump pack's or hover pack's flight physics, every frame: launch, sustain, air control,
    /// hover and fuel; it deactivates the pack (SetJumppackActive) when the meter runs out. The
    /// world update runs it for each index below the manager's +0x14.
    JumppackFlightFn / jumppack_flight / "JumppackFlight" / hook_jumppack_flight (jumppacks: *mut c_void, index: u32, dt: f32) -> ()
}

game_fn! {
    /// `game.dll+0x9B3420` (hook or call).
    /// Activates or deactivates an entity's jump pack or hover pack.
    SetJumppackActiveFn / set_jumppack_active / "SetJumppackActive" / hook_set_jumppack_active (jumppacks: *mut c_void, entity: u32, active: u8, r9: u8) -> ()
}

game_fn! {
    /// `game.dll+0xA63790` (hook or call).
    /// The jump pack's take-off, from the avatar's state: the hop's values, and
    /// SetJumppackActive(..., 1).
    JumppackTakeOffFn / jumppack_take_off / "JumppackTakeOff" / hook_jumppack_take_off (rcx: *mut c_void, dt: f32) -> ()
}

game_fn! {
    /// `game.dll+0x959950` (hook or call).
    /// Picks the composition of a scheduled patrol from the compiled patrol compositions (family
    /// 1, the HiveMind's +0x3A538): a 0xD8-byte entry, or NULL. EnemyDirectorTick spawns it with
    /// SpawnSquad.
    PickPatrolCompositionFn / pick_patrol_composition / "PickPatrolComposition" / hook_pick_patrol_composition (hivemind: *mut c_void, rdx: u32, r8: *mut c_void, r9: u32) -> *mut c_void
}

game_fn! {
    /// `game.dll+0x953580` (hook or call).
    /// Spawns a squad of enemies from a composition at a position: the director's patrols
    /// (EnemyDirectorTick), spawners (AiSpawnerUpdate), bug holes (BugholeUpdate), dropships
    /// (SquadTransportUpdate) and outposts (FillOutpost) all spawn through it. Returns the squad,
    /// or NULL.
    SpawnSquadFn / spawn_squad / "SpawnSquad" / hook_spawn_squad (hivemind: *mut c_void, composition: *mut c_void, position: *const f32, r9: u32, flags: u32, s6: u64, s7: u64, s8: u64, s9: f32, s10: u32) -> *mut c_void
}

game_fn! {
    /// `game.dll+0x959740` (hook or call).
    /// The patrol clock: the frame time scaled by the HiveMind's progress and player-count
    /// multipliers (HiveMindComponentData +0x1DC, +0x1F0, +0x204), which EnemyDirectorTick takes
    /// off the patrol timer. Higher is faster.
    PatrolClockFn / patrol_clock / "PatrolClock" / hook_patrol_clock (hivemind: *mut c_void, dt: f32, r8: *mut c_void) -> f32
}

game_fn! {
    /// `game.dll+0x957E10` (hook or call).
    /// Picks a reinforcement composition (family 4, the HiveMind's +0x432F8, count +0x5188C) for a
    /// position; EncounterUpdate then dispatches it.
    PickReinforcementCompositionFn / pick_reinforcement_composition / "PickReinforcementComposition" / hook_pick_reinforcement_composition (hivemind: *mut c_void, rdx: *mut c_void, position: *const f32, r9: u32) -> u32
}

game_fn! {
    /// `game.dll+0x8B7900` (hook or call).
    /// Sends a selected reinforcement to the faction's transport or breach; returns a time (s)
    /// that EncounterUpdate adds to the frame time and keeps in the encounter's record (+0x24).
    DispatchReinforcementFn / dispatch_reinforcement / "DispatchReinforcement" / hook_dispatch_reinforcement (encounters: *mut c_void, rdx: u32, r8: *mut c_void) -> f32
}

game_fn! {
    /// `game.dll+0x90E150` (hook or call).
    /// Fills an outpost with its guards from the map guard compositions (family 3, the HiveMind's
    /// +0x41138), spending the guard budget (HiveMind +0x518B4): it picks enemies with PickEnemy
    /// and spawns them through 0x90D2A0, which calls SpawnSquad. GuardForceUpdate calls it.
    FillOutpostFn / fill_outpost / "FillOutpost" / hook_fill_outpost (guardforces: *mut c_void, outpost: u32) -> ()
}

game_fn! {
    /// `helldivers2.exe+0x1FC650` (hook or call).
    /// Sets a unit node's local position (floats 9-11 of its 16-float local pose), marks the node
    /// changed and the unit moving (WorldSetUnitMoving, 5). Unit.set_local_position calls it..
    UnitSetLocalPositionFn / unit_set_local_position / "UnitSetLocalPosition" / hook_unit_set_local_position (unit: u32, node: i32, position: *const f32) -> ()
}

game_fn! {
    /// `helldivers2.exe+0x1FC7A0` (hook or call).
    /// Sets a unit node's local rotation from a quaternion (made a 3x3 by 0x12C780, keeping
    /// position and scale), marks the node changed and the unit moving. Unit.set_local_rotation
    /// calls it..
    UnitSetLocalRotationFn / unit_set_local_rotation / "UnitSetLocalRotation" / hook_unit_set_local_rotation (unit: u32, node: i32, rotation: *const f32) -> ()
}

game_fn! {
    /// `helldivers2.exe+0x1FC8F0` (hook or call).
    /// Sets a unit node's local scale (floats 12-14 of its local pose) through the unit's own
    /// set-local-pose (its vtable +0x278). Unit.set_local_scale calls it..
    UnitSetLocalScaleFn / unit_set_local_scale / "UnitSetLocalScale" / hook_unit_set_local_scale (unit: u32, node: i32, scale: *const f32) -> ()
}

game_fn! {
    /// `helldivers2.exe+0x1FD3F0` (hook or call).
    /// Unit.teleport_local_position's engine side: writes the node's local position like
    /// UnitSetLocalPosition, marks the node changed and the unit moving (WorldSetUnitMoving, 5)..
    UnitTeleportLocalPositionFn / unit_teleport_local_position / "UnitTeleportLocalPosition" / hook_unit_teleport_local_position (unit: u32, node: i32, position: *const f32) -> ()
}

game_fn! {
    /// `helldivers2.exe+0x1FD350` (hook or call).
    /// Writes a unit node's whole local pose, marks the node changed and the unit moving.
    /// Unit.teleport_local_pose calls it with the unit object..
    UnitTeleportLocalPoseFn / unit_teleport_local_pose / "UnitTeleportLocalPose" / hook_unit_teleport_local_pose (unit: *mut c_void, node: i32, pose: *const f32) -> ()
}

game_fn! {
    /// `helldivers2.exe+0x1AF180` (hook or call).
    /// Marks a unit moving: tells the unit (its vtable +0xB0) and, when its +0x38 is -1, appends
    /// it to the world's moving list (+0x230490) under the world's lock (+0x230460).
    /// Unit.set_moving and every Unit/Camera/Light/Mesh local pose setter reach it..
    WorldSetUnitMovingFn / world_set_unit_moving / "WorldSetUnitMoving" / hook_world_set_unit_moving (world: *mut c_void, unit: *mut c_void, frames: u32) -> ()
}

game_fn! {
    /// `helldivers2.exe+0x1FD7D0` (hook or call).
    /// How far a unit node moved: computed from its scene graph's last world poses (+0x30, copied
    /// from +0x28 when unset) and its current ones (+0x28); returns out. Unit.delta_position calls
    /// it..
    UnitDeltaPositionFn / unit_delta_position / "UnitDeltaPosition" / hook_unit_delta_position (out: *mut f32, unit: u32, node: i32) -> *mut f32
}

game_fn! {
    /// `helldivers2.exe+0x1FD9D0` (hook or call).
    /// How far a unit node turned between its last and current world pose, as a quaternion;
    /// returns out. Unit.delta_rotation calls it..
    UnitDeltaRotationFn / unit_delta_rotation / "UnitDeltaRotation" / hook_unit_delta_rotation (out: *mut f32, unit: u32, node: i32) -> *mut f32
}

game_fn! {
    /// `helldivers2.exe+0x1FDE40` (hook or call).
    /// A unit node's pose change between its last and current world pose (16 floats); returns out.
    /// Unit.delta_pose calls it..
    UnitDeltaPoseFn / unit_delta_pose / "UnitDeltaPose" / hook_unit_delta_pose (out: *mut f32, unit: u32, node: i32) -> *mut f32
}

game_fn! {
    /// `helldivers2.exe+0x1FD570` (hook or call).
    /// Unit.teleport_local_rotation's engine side: writes the node's local rotation from a
    /// quaternion (keeping position and scale), marks the node changed and the unit moving..
    UnitTeleportLocalRotationFn / unit_teleport_local_rotation / "UnitTeleportLocalRotation" / hook_unit_teleport_local_rotation (unit: u32, node: i32, rotation: *const f32) -> ()
}

game_fn! {
    /// `helldivers2.exe+0x1FD680` (hook or call).
    /// Unit.teleport_local_scale's engine side: writes the node's local scale, marks the node
    /// changed and the unit moving..
    UnitTeleportLocalScaleFn / unit_teleport_local_scale / "UnitTeleportLocalScale" / hook_unit_teleport_local_scale (unit: u32, node: i32, scale: *const f32) -> ()
}

game_fn! {
    /// `helldivers2.exe+0x1A7E50` (hook or call).
    /// Spawns a unit in a world from its resource and returns the unit object (+0x08 its handle,
    /// which World.spawn_unit hands to Lua). World.spawn_unit calls it; so do game.dll-facing
    /// paths (0x1AA5CE, 0x2D1EF6, 0x2E0DAA)..
    WorldSpawnUnitFn / world_spawn_unit / "WorldSpawnUnit" / hook_world_spawn_unit (world: *mut c_void, resource: *const u64, name: *const u64, pose: *const f32, s5: u32, s6: u64, s7: u8, s8: u64) -> *mut c_void
}

game_fn! {
    /// `helldivers2.exe+0x1AECC0` (hook or call).
    /// Destroys a unit now and frees it from its world. World.destroy_unit calls it when the
    /// unit's vtable +0x1B0 returns null (otherwise WorldQueueUnitsDestroy)..
    WorldDestroyUnitFn / world_destroy_unit / "WorldDestroyUnit" / hook_world_destroy_unit (world: *mut c_void, unit: *mut c_void) -> ()
}

game_fn! {
    /// `helldivers2.exe+0x12D2D0` (hook or call).
    /// Queues units for destruction: takes each handle out of the unit registry (bumps its
    /// generation, clears its slot) and adds it to the world's queue counted at +0x10030.
    /// World.destroy_unit uses it for units whose vtable +0x1B0 is set..
    WorldQueueUnitsDestroyFn / world_queue_units_destroy / "WorldQueueUnitsDestroy" / hook_world_queue_units_destroy (world: *mut c_void, units: *const u32, count: u32) -> ()
}

game_fn! {
    /// `helldivers2.exe+0x3361C0` (hook or call).
    /// Links one unit to another's node by handles: resolves both (UnitFromHandle) and tail-calls
    /// WorldLinkUnit. World.link_unit calls it..
    WorldLinkUnitHandlesFn / world_link_unit_handles / "WorldLinkUnitHandles" / hook_world_link_unit_handles (world: *mut c_void, child: u32, child_node: i32, parent: u32, s5: u32) -> ()
}

game_fn! {
    /// `helldivers2.exe+0x1AF240` (hook or call).
    /// Links a unit's node to a parent unit's node in a world: puts the child in the world's
    /// linked list (+0x2304B0), the parent at the child's +0x1D0 and the child in the parent's
    /// +0x1D8 list..
    WorldLinkUnitFn / world_link_unit / "WorldLinkUnit" / hook_world_link_unit (world: *mut c_void, child: *mut c_void, child_node: i32, parent: *mut c_void, s5: u16) -> ()
}

game_fn! {
    /// `helldivers2.exe+0x1AF780` (hook or call).
    /// Unlinks a unit from its parent and takes it out of the world's linked list (+0x2304B0).
    /// World.unlink_unit calls it..
    WorldUnlinkUnitFn / world_unlink_unit / "WorldUnlinkUnit" / hook_world_unlink_unit (world: *mut c_void, unit: *mut c_void) -> ()
}

game_fn! {
    /// `helldivers2.exe+0x3351C0` (hook or call).
    /// World.mend_unit's engine side: asks the unit (vtable +0x48 with the index) and when that
    /// succeeds marks it moving (WorldSetUnitMoving, 1); 1 when it did..
    WorldMendUnitFn / world_mend_unit / "WorldMendUnit" / hook_world_mend_unit (world: *mut c_void, unit: u32, index: i32) -> u8
}

game_fn! {
    /// `helldivers2.exe+0x333BD0` (hook or call).
    /// Updates a world by a time step. World.update calls it..
    WorldUpdateFn / world_update / "WorldUpdate" / hook_world_update (world: *mut c_void, dt: f32) -> ()
}

game_fn! {
    /// `helldivers2.exe+0x1B0B10` (hook or call).
    /// Updates a world's scene graph by a time step. World.update_scene and
    /// World.update_with_callback call it..
    WorldUpdateSceneFn / world_update_scene / "WorldUpdateScene" / hook_world_update_scene (world: *mut c_void, dt: f32, xmm2: f32) -> ()
}

game_fn! {
    /// `helldivers2.exe+0x1B3AD0` (hook or call).
    /// Makes a debug line object in a world and records it in the world's line object map
    /// (+0x230638); returns it. World.create_line_object calls it..
    WorldCreateLineObjectFn / world_create_line_object / "WorldCreateLineObject" / hook_world_create_line_object (world: *mut c_void, no_depth_test: u8, r8: u8) -> *mut c_void
}

game_fn! {
    /// `helldivers2.exe+0x4E1820` (hook or call).
    /// Adds one debug line to a line object; it shows once the object is dispatched
    /// (LineObjectDispatch). LineObject.add_line calls it..
    LineObjectAddLineFn / line_object_add_line / "LineObjectAddLine" / hook_line_object_add_line (line_object: *mut c_void, color: u32, from: *const f32, to: *const f32) -> ()
}

game_fn! {
    /// `helldivers2.exe+0x4E46D0` (hook or call).
    /// Adds a debug cone's lines to a line object. LineObject.add_cone calls it..
    LineObjectAddConeFn / line_object_add_cone / "LineObjectAddCone" / hook_line_object_add_cone (line_object: *mut c_void, color: u32, from: *const f32, to: *const f32, s5: f32, s6: i32, s7: i32) -> ()
}

game_fn! {
    /// `helldivers2.exe+0x4E1FD0` (hook or call).
    /// Adds a debug circle's lines to a line object. LineObject.add_circle calls it..
    LineObjectAddCircleFn / line_object_add_circle / "LineObjectAddCircle" / hook_line_object_add_circle (line_object: *mut c_void, color: u32, center: *const f32, radius: f32, s5: *const f32, s6: i32) -> ()
}

game_fn! {
    /// `helldivers2.exe+0x4E2FC0` (hook or call).
    /// Adds a debug sphere's lines to a line object. LineObject.add_sphere calls it..
    LineObjectAddSphereFn / line_object_add_sphere / "LineObjectAddSphere" / hook_line_object_add_sphere (line_object: *mut c_void, color: u32, center: *const f32, radius: f32, s5: i32, s6: i32) -> ()
}

game_fn! {
    /// `helldivers2.exe+0x4E3380` (hook or call).
    /// Adds a debug half sphere's lines to a line object. LineObject.add_half_sphere calls it..
    LineObjectAddHalfSphereFn / line_object_add_half_sphere / "LineObjectAddHalfSphere" / hook_line_object_add_half_sphere (line_object: *mut c_void, color: u32, center: *const f32, radius: f32, s5: *const f32, s6: i32, s7: i32) -> ()
}

game_fn! {
    /// `helldivers2.exe+0x4E3AC0` (hook or call).
    /// Adds a debug box's 12 edges to a line object. LineObject.add_box calls it..
    LineObjectAddBoxFn / line_object_add_box / "LineObjectAddBox" / hook_line_object_add_box (line_object: *mut c_void, color: u32, pose: *const f32, extents: *const f32) -> ()
}

game_fn! {
    /// `helldivers2.exe+0x4E4040` (hook or call).
    /// Adds a debug capsule's lines to a line object. LineObject.add_capsule calls it..
    LineObjectAddCapsuleFn / line_object_add_capsule / "LineObjectAddCapsule" / hook_line_object_add_capsule (line_object: *mut c_void, color: u32, from: *const f32, to: *const f32, s5: f32, s6: i32, unused: i32, s8: i32) -> ()
}

game_fn! {
    /// `helldivers2.exe+0x4E5EC0` (hook or call).
    /// Sends a line object's lines to the renderer for drawing (a packet of 28 bytes a line plus a
    /// header). LineObject.dispatch calls it..
    LineObjectDispatchFn / line_object_dispatch / "LineObjectDispatch" / hook_line_object_dispatch (renderer: *mut c_void, render: *mut c_void, line_object: *mut c_void) -> ()
}

game_fn! {
    /// `helldivers2.exe+0x5D5840` (hook or call).
    /// MurmurHash64A (multiplier 0xC6A4A7935BD1E995): the engine's name hash. With seed 0 it is
    /// the IdString64 every binding makes from a string; its high 32 bits are the IdString32
    /// (Material variable names). Called from everywhere (378 binding sites)..
    MurmurHash64AFn / murmur_hash64_a / "MurmurHash64A" / hook_murmur_hash64_a (key: *const c_char, len: i32, seed: u64) -> u64
}

game_fn! {
    /// `helldivers2.exe+0x4F33D0` (hook or call).
    /// Sets a material's scalar shader variable: queues a render command (7) with the material's
    /// id, the variable and the value; nothing when the material has no render id.
    /// Material.set_scalar calls it, as does the game's material variable code (0xCCD06)..
    MaterialSetScalarFn / material_set_scalar / "MaterialSetScalar" / hook_material_set_scalar (material: *mut c_void, variable: u32, value: f32) -> ()
}

game_fn! {
    /// `helldivers2.exe+0x4F3580` (hook or call).
    /// Sets a material's 2-float shader variable through a queued render command.
    /// Material.set_vector2 calls it..
    MaterialSetVector2Fn / material_set_vector2 / "MaterialSetVector2" / hook_material_set_vector2 (material: *mut c_void, variable: u32, value: *const f32) -> ()
}

game_fn! {
    /// `helldivers2.exe+0x4F3650` (hook or call).
    /// Sets a material's 3-float shader variable through a queued render command.
    /// Material.set_vector3 calls it..
    MaterialSetVector3Fn / material_set_vector3 / "MaterialSetVector3" / hook_material_set_vector3 (material: *mut c_void, variable: u32, value: *const f32) -> ()
}

game_fn! {
    /// `helldivers2.exe+0x4F3730` (hook or call).
    /// Sets a material's 4-float shader variable through a queued render command.
    /// Material.set_vector4 calls it..
    MaterialSetVector4Fn / material_set_vector4 / "MaterialSetVector4" / hook_material_set_vector4 (material: *mut c_void, variable: u32, value: *const f32) -> ()
}

game_fn! {
    /// `helldivers2.exe+0x30F330` (hook or call).
    /// Sets a material's 4x4 matrix shader variable through a queued render command.
    /// Material.set_matrix4x4 calls it..
    MaterialSetMatrix4x4Fn / material_set_matrix4x4 / "MaterialSetMatrix4x4" / hook_material_set_matrix4x4 (material: *mut c_void, variable: u32, value: *const f32) -> ()
}

game_fn! {
    /// `helldivers2.exe+0x30F1B0` (hook or call).
    /// Points a material's texture slot at a loaded texture resource. Material.set_texture calls
    /// it..
    MaterialSetTextureFn / material_set_texture / "MaterialSetTexture" / hook_material_set_texture (material: *mut c_void, slot: u32, texture: u64) -> ()
}

game_fn! {
    /// `helldivers2.exe+0x4F3230` (hook or call).
    /// Binds a resource object to a material slot (stored in the material's table at +0x20, count
    /// +0x18). Material.set_resource calls it..
    MaterialSetResourceFn / material_set_resource / "MaterialSetResource" / hook_material_set_resource (material: *mut c_void, slot: u32, resource: *mut c_void) -> ()
}

game_fn! {
    /// `helldivers2.exe+0x30F150` (hook or call).
    /// Turns a shader pass flag on or off for a material and sends the new bits (0x4F39F0).
    /// Material.set_shader_pass_flag calls it..
    MaterialSetShaderPassFlagFn / material_set_shader_pass_flag / "MaterialSetShaderPassFlag" / hook_material_set_shader_pass_flag (material: *mut c_void, flag: u32, enabled: u32) -> ()
}

game_fn! {
    /// `helldivers2.exe+0x30F420` (hook or call).
    /// Queues a render command that rebuilds a material's shader. Material.reinitialize_shader
    /// calls it..
    MaterialReinitializeShaderFn / material_reinitialize_shader / "MaterialReinitializeShader" / hook_material_reinitialize_shader (material: *mut c_void) -> ()
}

game_fn! {
    /// `helldivers2.exe+0x4D40F0` (hook or call).
    /// Projects a world point to screen pixels with a camera's world pose and projection.
    /// Camera.world_to_screen calls it with the window's size unless given a Vector2 size..
    CameraWorldToScreenFn / camera_world_to_screen / "CameraWorldToScreen" / hook_camera_world_to_screen (out: *mut f32, camera: *mut c_void, position: *const f32, width: i32, s5: i32) -> *mut f32
}

game_fn! {
    /// `helldivers2.exe+0x4D3BD0` (hook or call).
    /// Unprojects a screen point at a depth to a world point with a camera. Camera.screen_to_world
    /// calls it..
    CameraScreenToWorldFn / camera_screen_to_world / "CameraScreenToWorld" / hook_camera_screen_to_world (out: *mut f32, camera: *mut c_void, position: *const f32, depth: f32, s5: i32, s6: i32) -> *mut f32
}

game_fn! {
    /// `helldivers2.exe+0x4CFA10` (hook or call).
    /// Queues a render command (4) carrying a camera's near and far range and projection type.
    /// Camera.set_near_range, set_far_range and set_projection_type write the field and call it..
    CameraSyncRangeFn / camera_sync_range / "CameraSyncRange" / hook_camera_sync_range (camera: *mut c_void) -> ()
}

game_fn! {
    /// `helldivers2.exe+0x4CFAB0` (hook or call).
    /// Queues a render command carrying a camera's fov and frustum fields (+0x34, 32 bytes).
    /// Camera.set_vertical_fov, set_orthographic_view, set_frustum and set_frustum_half_angles
    /// write the fields and call it..
    CameraSyncFrustumFn / camera_sync_frustum / "CameraSyncFrustum" / hook_camera_sync_frustum (camera: *mut c_void) -> ()
}

game_fn! {
    /// `helldivers2.exe+0x4CF620` (hook or call).
    /// Queues a render command with a camera's local matrix. Camera.set_local calls it..
    CameraSetLocalFn / camera_set_local / "CameraSetLocal" / hook_camera_set_local (camera: *mut c_void, pose: *const f32, r8: u32) -> ()
}

game_fn! {
    /// `helldivers2.exe+0x30DCC0` (hook or call).
    /// How far a world point is inside a camera's frustum (above 0 inside). Camera.inside_frustum
    /// calls it..
    CameraInsideFrustumFn / camera_inside_frustum / "CameraInsideFrustum" / hook_camera_inside_frustum (camera: *mut c_void, point: *const f32) -> f32
}

game_fn! {
    /// `helldivers2.exe+0x267990` (hook or call).
    /// Queues a gui render command (0xA) for a primitive with the gui's current transform and
    /// returns where its vertices go (36 bytes each), for the caller to fill. Every Gui drawing
    /// call reaches it (Gui.rect, triangle, bitmap and their update/destroy)..
    GuiAddPrimitiveFn / gui_add_primitive / "GuiAddPrimitive" / hook_gui_add_primitive (gui: *mut c_void, id: u32, layer: u32, vertices: u32, s5: *mut c_void) -> *mut c_void
}

game_fn! {
    /// `helldivers2.exe+0x262060` (hook or call).
    /// Draws (or redraws) a coloured rectangle in a gui as two triangles through GuiAddPrimitive.
    /// Gui.rect and Gui.update_rect call it..
    GuiRectFn / gui_rect / "GuiRect" / hook_gui_rect (gui: *mut c_void, id: u32, rect: *const c_void) -> ()
}

game_fn! {
    /// `helldivers2.exe+0x5F2290` (hook or call).
    /// A loaded resource's data, following resource overrides, under the manager's lock (+0x2D8).
    /// Application.get_animation calls it (type 0x931E336D7646CC26), as do 25 other binding
    /// sites..
    ResourceManagerGetFn / resource_manager_get / "ResourceManagerGet" / hook_resource_manager_get (manager: *mut c_void, type_: *const u64, name: *const u64, resolved: *mut u64) -> *mut c_void
}

game_fn! {
    /// `helldivers2.exe+0x5F2160` (hook or call).
    /// Whether a resource is loaded and can be had now. Application.can_get calls it;
    /// CanGetResource wraps it with plain hashes..
    ResourceManagerCanGetFn / resource_manager_can_get / "ResourceManagerCanGet" / hook_resource_manager_can_get (manager: *mut c_void, type_: *const u64, name: *const u64) -> u8
}

game_fn! {
    /// `helldivers2.exe+0x5F26F0` (hook or call).
    /// Points later lookups of a resource name at another name (through 0x5F3210).
    /// Application.set_resource_override calls it; SetResourceOverride is the plain-hash version..
    ResourceManagerSetOverrideFn / resource_manager_set_override / "ResourceManagerSetOverride" / hook_resource_manager_set_override (manager: *mut c_void, type_: *const u64, name: *const u64, target: *const u64) -> ()
}

game_fn! {
    /// `helldivers2.exe+0x5F27D0` (hook or call).
    /// Takes a resource override out under the manager's lock (+0x2D8).
    /// Application.clear_resource_override calls it; ClearResourceOverride wraps it with plain
    /// hashes..
    ResourceManagerClearOverrideFn / resource_manager_clear_override / "ResourceManagerClearOverride" / hook_resource_manager_clear_override (manager: *mut c_void, type_: *const u64, name: *const u64) -> ()
}

game_fn! {
    /// `helldivers2.exe+0x5D5960` (hook or call).
    /// MurmurHash64A of a name with every byte lowercased first (0x12BC14C); the third argument
    /// register's home slot is reused as scratch, so no seed is read. Application.string_to_id64
    /// calls it..
    MurmurHash64ALowerFn / murmur_hash64_a_lower / "MurmurHash64ALower" / hook_murmur_hash64_a_lower (key: *const c_char, len: i32, r8: u64) -> u64
}

game_fn! {
    /// `helldivers2.exe+0x10AE20` (hook or call).
    /// The pixel size of a window's back buffer. Application.back_buffer_size,
    /// Application.resolution and Camera.world_to_screen call it..
    RendererWindowSizeFn / renderer_window_size / "RendererWindowSize" / hook_renderer_window_size (renderer: *mut c_void, window: u32, width: *mut u32, height: *mut u32) -> ()
}

game_fn! {
    /// `helldivers2.exe+0x317FB0` (hook or call).
    /// Destroys a world: shuts its systems down, frees it through EngineCore's vtable +0x40 and
    /// takes it out of the engine's world list (EngineCore +0x590 count, +0x598 array).
    /// Application.release_world calls it..
    ReleaseWorldFn / release_world / "ReleaseWorld" / hook_release_world (world: *mut c_void) -> ()
}

game_fn! {
    /// `helldivers2.exe+0x76B40` (hook or call).
    /// Makes a viewport for a world from a named viewport template and returns it.
    /// Application.create_viewport calls it..
    CreateViewportFn / create_viewport / "CreateViewport" / hook_create_viewport (engine: *mut c_void, world: *mut c_void, viewport_template: u32) -> *mut c_void
}

game_fn! {
    /// `helldivers2.exe+0x1B3D20` (hook or call).
    /// Destroys a viewport and takes it out of its world's viewport list.
    /// Application.destroy_viewport calls it..
    WorldDestroyViewportFn / world_destroy_viewport / "WorldDestroyViewport" / hook_world_destroy_viewport (world: *mut c_void, viewport: *mut c_void) -> ()
}

game_fn! {
    /// `helldivers2.exe+0x171A20` (hook or call).
    /// Makes the network manager's game session (0x6140 bytes, built by 0x28D640) at its +0xA8,
    /// then runs NetworkManagerUpdate. Network.create_game_session calls it..
    NetworkCreateGameSessionFn / network_create_game_session / "NetworkCreateGameSession" / hook_network_create_game_session (manager: *mut c_void) -> ()
}

game_fn! {
    /// `helldivers2.exe+0x171AE0` (hook or call).
    /// Destroys the network manager's game session and clears its +0xA8.
    /// Network.shutdown_game_session calls it..
    NetworkShutdownGameSessionFn / network_shutdown_game_session / "NetworkShutdownGameSession" / hook_network_shutdown_game_session (manager: *mut c_void) -> ()
}

game_fn! {
    /// `helldivers2.exe+0x32C4B0` (hook or call).
    /// Whether a point is inside one of a level's named volumes (1 inside).
    /// Level.is_point_inside_volume calls it..
    LevelIsPointInsideVolumeFn / level_is_point_inside_volume / "LevelIsPointInsideVolume" / hook_level_is_point_inside_volume (level: *mut c_void, volume: u32, point: *const f32) -> u32
}

game_fn! {
    /// `helldivers2.exe+0x32BA90` (hook or call).
    /// A random point inside one of a level's named volumes. Level.random_point_inside_volume
    /// calls it..
    LevelRandomPointInsideVolumeFn / level_random_point_inside_volume / "LevelRandomPointInsideVolume" / hook_level_random_point_inside_volume (out: *mut f32, level: *mut c_void, volume: u32) -> *mut f32
}

game_fn! {
    /// `helldivers2.exe+0x32CE50` (hook or call).
    /// Whether a level has a volume of that name (non-zero yes). Level.has_volume calls it..
    LevelHasVolumeFn / level_has_volume / "LevelHasVolume" / hook_level_has_volume (level: *mut c_void, volume: u32) -> u32
}

game_fn! {
    /// `helldivers2.exe+0xEAA20` (hook or call).
    /// A unit's index in its level, -1 when the level does not own it. Level.unit_index calls it..
    LevelUnitIndexFn / level_unit_index / "LevelUnitIndex" / hook_level_unit_index (level: *mut c_void, unit: *mut c_void) -> i32
}

game_fn! {
    /// `helldivers2.exe+0xDB230` (hook or call).
    /// Spawns every layer of a level. Level.spawn_all_layers calls it..
    LevelSpawnAllLayersFn / level_spawn_all_layers / "LevelSpawnAllLayers" / hook_level_spawn_all_layers (level: *mut c_void) -> ()
}

game_fn! {
    /// `helldivers2.exe+0x201C20` (hook or call).
    /// The index of a variable in a unit's animation state machine (the unit's vtable +0x1B0); -1
    /// and a logged error when the unit has none. Unit.animation_find_variable calls it..
    UnitAnimationFindVariableFn / unit_animation_find_variable / "UnitAnimationFindVariable" / hook_unit_animation_find_variable (unit: u32, name: u32) -> i32
}

game_fn! {
    /// `helldivers2.exe+0x201CD0` (hook or call).
    /// Whether a unit's animation state machine has a variable of that name (non-zero yes).
    /// Unit.animation_has_variable calls it..
    UnitAnimationHasVariableFn / unit_animation_has_variable / "UnitAnimationHasVariable" / hook_unit_animation_has_variable (unit: u32, name: u32) -> u32
}

game_fn! {
    /// `helldivers2.exe+0x201D50` (hook or call).
    /// An animation state machine variable's value. Unit.animation_get_variable calls it..
    UnitAnimationGetVariableFn / unit_animation_get_variable / "UnitAnimationGetVariable" / hook_unit_animation_get_variable (unit: u32, index: i32) -> f32
}

game_fn! {
    /// `helldivers2.exe+0x201DF0` (hook or call).
    /// Sets an animation state machine variable. Unit.animation_set_variable calls it..
    UnitAnimationSetVariableFn / unit_animation_set_variable / "UnitAnimationSetVariable" / hook_unit_animation_set_variable (unit: u32, index: i32, value: f32) -> ()
}

game_fn! {
    /// `helldivers2.exe+0x205680` (hook or call).
    /// Shows or hides a whole unit: queues a world command (0x15) in the unit's world (a 0x58-byte
    /// slot at +0x10038, taken with a lock add on +0x170038). Unit.set_unit_visibility calls it..
    UnitSetUnitVisibilityFn / unit_set_unit_visibility / "UnitSetUnitVisibility" / hook_unit_set_unit_visibility (unit: u32, visible: u8, r8: u8) -> ()
}

game_fn! {
    /// `helldivers2.exe+0x205870` (hook or call).
    /// Shows or hides one of a unit's visibility groups: tells the unit (its vtable +0xC0) and
    /// queues a world command (0x16). Unit.set_visibility calls it..
    UnitSetVisibilityGroupFn / unit_set_visibility_group / "UnitSetVisibilityGroup" / hook_unit_set_visibility_group (unit: u32, group: u32, visible: u32) -> ()
}

game_fn! {
    /// `helldivers2.exe+0x200D80` (hook or call).
    /// Crossfades a unit's animation player to an animation and returns the id the binding hands
    /// back to Lua. Unit.crossfade_animation calls it..
    UnitCrossfadeAnimationFn / unit_crossfade_animation / "UnitCrossfadeAnimation" / hook_unit_crossfade_animation (unit: u32, animation: u64, layer: i32, blend_time: f32, s5: u32, s6: u32) -> i32
}

game_fn! {
    /// `helldivers2.exe+0x201130` (hook or call).
    /// Sets the play time of a crossfade on a unit's animation player (its vtable +0x1A8), passing
    /// on to 0x561010 with the time as a double. Unit.crossfade_animation_set_time calls it..
    UnitCrossfadeAnimationSetTimeFn / unit_crossfade_animation_set_time / "UnitCrossfadeAnimationSetTime" / hook_unit_crossfade_animation_set_time (unit: u32, id: u32, time: f32, r9: u32) -> ()
}

game_fn! {
    /// `helldivers2.exe+0x204880` (hook or call).
    /// Copies another unit's scene graph local poses onto a unit. Unit.copy_scene_graph_local_from
    /// calls it..
    UnitCopySceneGraphLocalFromFn / unit_copy_scene_graph_local_from / "UnitCopySceneGraphLocalFrom" / hook_unit_copy_scene_graph_local_from (unit: u32, source: u32) -> ()
}

game_fn! {
    /// `helldivers2.exe+0x202D30` (hook or call).
    /// Sets a unit's animation bones LOD. Unit.set_bones_lod calls it..
    UnitSetBonesLodFn / unit_set_bones_lod / "UnitSetBonesLod" / hook_unit_set_bones_lod (unit: u32, lod: i32) -> ()
}

game_fn! {
    /// `helldivers2.exe+0x202EA0` (hook or call).
    /// The index of a named constraint target in a unit's animation state machine (the binding
    /// adds the Lua index base). Unit.animation_find_constraint_target calls it..
    UnitAnimationFindConstraintTargetFn / unit_animation_find_constraint_target / "UnitAnimationFindConstraintTarget" / hook_unit_animation_find_constraint_target (unit: u32, name: u32) -> i32
}

game_fn! {
    /// `helldivers2.exe+0x203120` (hook or call).
    /// Sets an animation constraint target from a position. Unit.animation_set_constraint_target
    /// calls it when given a Vector3..
    UnitAnimationSetConstraintTargetPositionFn / unit_animation_set_constraint_target_position / "UnitAnimationSetConstraintTargetPosition" / hook_unit_animation_set_constraint_target_position (unit: u32, index: i32, position: *const f32) -> ()
}

game_fn! {
    /// `helldivers2.exe+0x50BF90` (hook or call).
    /// Queues a render command (0x80 bytes) with a light's current settings; every Light setter
    /// writes its field and then calls it (Light.set_color, set_intensity, set_falloff_end,
    /// set_casts_shadows...)..
    LightSyncFn / light_sync / "LightSync" / hook_light_sync (light: *mut c_void) -> ()
}

game_fn! {
    /// `helldivers2.exe+0x50BE10` (hook or call).
    /// Turns a light on or off, syncs it (LightSync) and tells the renderer's light list.
    /// Light.set_enabled calls it..
    LightSetEnabledFn / light_set_enabled / "LightSetEnabled" / hook_light_set_enabled (light: *mut c_void, enabled: u8) -> ()
}

game_fn! {
    /// `helldivers2.exe+0x338960` (hook or call).
    /// Destroys a level: flushes the world's pending unit queues (0x12D600) when needed, takes the
    /// level out of the world's list and frees it. World.destroy_level calls it..
    WorldDestroyLevelFn / world_destroy_level / "WorldDestroyLevel" / hook_world_destroy_level (world: *mut c_void, level: *mut c_void, r8: u32) -> ()
}

game_fn! {
    /// `helldivers2.exe+0x338D60` (hook or call).
    /// Destroys a prefab the world loaded, the same way WorldDestroyLevel does a level.
    /// World.destroy_prefab calls it..
    WorldDestroyPrefabFn / world_destroy_prefab / "WorldDestroyPrefab" / hook_world_destroy_prefab (world: *mut c_void, prefab: *mut c_void, r8: u32) -> ()
}

game_fn! {
    /// `helldivers2.exe+0x337810` (hook or call).
    /// Plays a camera shake at a world position and returns its id. World.play_camera_shake calls
    /// it..
    WorldPlayCameraShakeFn / world_play_camera_shake / "WorldPlayCameraShake" / hook_world_play_camera_shake (world: *mut c_void, shake: u64, position: *const f32, near: f32, s5: f32) -> i32
}

game_fn! {
    /// `helldivers2.exe+0x22D8A0` (hook or call).
    /// Stops a playing camera shake. World.stop_camera_shake calls it..
    WorldStopCameraShakeFn / world_stop_camera_shake / "WorldStopCameraShake" / hook_world_stop_camera_shake (shakes: *mut c_void, id: i32, r8: u8) -> ()
}

game_fn! {
    /// `helldivers2.exe+0x1B0670` (hook or call).
    /// Runs a world's animation update for a time step. World.update_animations,
    /// update_animations_with_callback and update_with_callback call it..
    WorldUpdateAnimationsFn / world_update_animations / "WorldUpdateAnimations" / hook_world_update_animations (world: *mut c_void, dt: f32) -> ()
}

game_fn! {
    /// `helldivers2.exe+0x273110` (hook or call).
    /// Destroys a gui (World.create_screen_gui or create_world_gui's). World.destroy_gui calls
    /// it..
    GuiManagerDestroyGuiFn / gui_manager_destroy_gui / "GuiManagerDestroyGui" / hook_gui_manager_destroy_gui (guis: *mut c_void, gui: *mut c_void) -> ()
}

game_fn! {
    /// `game.dll+0x581320` (hook or call).
    /// SpawnEntity's first component pass: for each of the new entity's components, its add
    /// callback with the spawn data (kind 1), then its setup callback with a scratch buffer (kind
    /// 2). Every machine runs it for every spawn, remote ones included; the entity is not in the
    /// network id map yet.
    SpawnEntityAddComponentsFn / spawn_entity_add_components / "SpawnEntityAddComponents" / hook_spawn_entity_add_components (components: *mut c_void, record: *mut u8, info: *const u8, ids: *mut c_void) -> ()
}

game_fn! {
    /// `game.dll+0x581780` (hook or call).
    /// SpawnEntity's last component pass: each of the new entity's components' finish callback
    /// (kind 3), after every component exists and the entity is in the network id map. A detour
    /// that runs after the original sees the entity fully set up, before SpawnEntity returns.
    SpawnEntityFinishComponentsFn / spawn_entity_finish_components / "SpawnEntityFinishComponents" / hook_spawn_entity_finish_components (components: *mut c_void, record: *mut u8, info: *const u8) -> ()
}

game_fn! {
    /// `game.dll+0xFDBB40` (hook or call).
    /// Switches an entity on in this machine's world: finds it by network id, sets bit 0 of its
    /// record's flags (+0x14) and runs each of its components' activate callbacks (kinds 6, then
    /// 7), unless a tag or a session flag holds it back. The session calls it through LocalUser's
    /// entity callbacks (+0xB3D0, set at 0x134D470, via a thunk at 0xBA0670).
    ActivateEntityFn / activate_entity / "ActivateEntity" / hook_activate_entity (unused: *mut c_void, entity: u32, ctx: u64, r9: u64) -> ()
}

game_fn! {
    /// `game.dll+0xFDBEA0` (hook or call).
    /// Switches an entity off in this machine's world: finds it by network id and, if it is on
    /// (bit 0 of its record's flags, +0x14), runs each of its components' deactivate callbacks
    /// (kinds 9, then 8) and clears the bit. The entity stays in the world. The session calls it
    /// through LocalUser's entity callbacks (+0xB3D8, set at 0x134D470, via a thunk at 0xBA0680).
    DeactivateEntityFn / deactivate_entity / "DeactivateEntity" / hook_deactivate_entity (unused: *mut c_void, entity: u32, ctx: u64) -> ()
}

game_fn! {
    /// `helldivers2.exe+0x1FC450` (hook or call).
    /// A node's position in its parent's space: 3 floats, inside its local transform
    /// (UnitLocalPose +0x24): the Unit table's (EngineApi +0x18, 0x27C8F10) +0x0.
    UnitLocalPositionFn / unit_local_position / "UnitLocalPosition" / hook_unit_local_position (unit: u32, node: i32) -> *const f32
}

game_fn! {
    /// `helldivers2.exe+0x1FC490` (hook or call).
    /// A node's rotation in its parent's space, written to out as a quaternion; returns out: the
    /// Unit table's (EngineApi +0x18, 0x27C8F10) +0x8.
    UnitLocalRotationFn / unit_local_rotation / "UnitLocalRotation" / hook_unit_local_rotation (out: *mut f32, unit: u32, node: i32) -> *mut f32
}

game_fn! {
    /// `helldivers2.exe+0x1FC4E0` (hook or call).
    /// A node's scale in its parent's space: 3 floats, inside its local transform (UnitLocalPose
    /// +0x30): the Unit table's (EngineApi +0x18, 0x27C8F10) +0x10.
    UnitLocalScaleFn / unit_local_scale / "UnitLocalScale" / hook_unit_local_scale (unit: u32, node: i32) -> *const f32
}

game_fn! {
    /// `helldivers2.exe+0x1FC520` (hook or call).
    /// A node's local transform: a 3x3 rotation (9 floats), then its position (+0x24, 3 floats)
    /// and scale (+0x30, 3 floats): the Unit table's (EngineApi +0x18, 0x27C8F10) +0x18.
    UnitLocalPoseFn / unit_local_pose / "UnitLocalPose" / hook_unit_local_pose (unit: u32, node: i32) -> *const f32
}

game_fn! {
    /// `helldivers2.exe+0x1FD220` (hook or call).
    /// A node's world pose: a 4x4 matrix of floats, its position in the last row (+0x30). The game
    /// reads it from more than 1,500 places: the Unit table's (EngineApi +0x18, 0x27C8F10) +0x90.
    UnitWorldPoseFn / unit_world_pose / "UnitWorldPose" / hook_unit_world_pose (unit: u32, node: i32) -> *const f32
}

game_fn! {
    /// `helldivers2.exe+0x1FD1D0` (hook or call).
    /// A node's world rotation, written to out as a quaternion; returns out: the Unit table's
    /// (EngineApi +0x18, 0x27C8F10) +0xb0.
    UnitWorldRotationFn / unit_world_rotation / "UnitWorldRotation" / hook_unit_world_rotation (out: *mut f32, unit: u32, node: i32) -> *mut f32
}

game_fn! {
    /// `helldivers2.exe+0x200CD0` (hook or call).
    /// How many meshes a unit has: the Unit table's (EngineApi +0x18, 0x27C8F10) +0x1a8.
    UnitNumMeshesFn / unit_num_meshes / "UnitNumMeshes" / hook_unit_num_meshes (unit: u32) -> u32
}

game_fn! {
    /// `helldivers2.exe+0x200D20` (hook or call).
    /// A unit's mesh object by index: the Unit table's (EngineApi +0x18, 0x27C8F10) +0x1b8.
    UnitMeshFn / unit_mesh / "UnitMesh" / hook_unit_mesh (unit: u32, index: i32) -> *mut c_void
}

game_fn! {
    /// `helldivers2.exe+0x1FEEB0` (hook or call).
    /// 1 while the handle names a unit that exists, 0 once it is gone. Only the low byte is the
    /// answer: the rest of the register is left over: the Unit table's (EngineApi +0x18,
    /// 0x27C8F10) +0x720.
    UnitAliveFn / unit_alive / "UnitAlive" / hook_unit_alive (unit: u32) -> u8
}

game_fn! {
    /// `helldivers2.exe+0x2016E0` (hook or call).
    /// 1 when the unit has an animation blender (what the animation calls need), else 0: the Unit
    /// table's (EngineApi +0x18, 0x27C8F10) +0x348.
    UnitHasAnimationBlenderFn / unit_has_animation_blender / "UnitHasAnimationBlender" / hook_unit_has_animation_blender (unit: u32) -> u8
}

game_fn! {
    /// `helldivers2.exe+0x2016B0` (hook or call).
    /// 1 when the unit has an animation state machine, else 0: the Unit table's (EngineApi +0x18,
    /// 0x27C8F10) +0x340.
    UnitHasAnimationStateMachineFn / unit_has_animation_state_machine / "UnitHasAnimationStateMachine" / hook_unit_has_animation_state_machine (unit: u32) -> u8
}

game_fn! {
    /// `helldivers2.exe+0x202DA0` (hook or call).
    /// How the root bone's animation moves the unit, as bits: 1 position, 2 rotation, 4 scale (0
    /// none, 7 all three); in Lua the names delta_position, delta_rotation and so on. Call it only
    /// on a unit whose UnitHasAnimationBlender is 1: it reads the blender without checking for
    /// one: the Unit table's (EngineApi +0x18, 0x27C8F10) +0x208.
    UnitAnimationRootModeFn / unit_animation_root_mode / "UnitAnimationRootMode" / hook_unit_animation_root_mode (unit: u32) -> u32
}

game_fn! {
    /// `helldivers2.exe+0x202DE0` (hook or call).
    /// Which parts of a bone its animation drives, as bits: 1 position, 2 rotation, 4 scale (0
    /// none, 7 all three); in Lua position, rotation, position_and_rotation and so on. Call it
    /// only on a unit whose UnitHasAnimationBlender is 1: it reads the blender without checking
    /// for one: the Unit table's (EngineApi +0x18, 0x27C8F10) +0x210.
    UnitAnimationBoneModeFn / unit_animation_bone_mode / "UnitAnimationBoneMode" / hook_unit_animation_bone_mode (unit: u32) -> u32
}

game_fn! {
    /// `helldivers2.exe+0x2047B0` (hook or call).
    /// How many scene graph nodes a unit has: the Unit table's (EngineApi +0x18, 0x27C8F10)
    /// +0x4f8.
    UnitNumSceneGraphItemsFn / unit_num_scene_graph_items / "UnitNumSceneGraphItems" / hook_unit_num_scene_graph_items (unit: u32) -> u32
}

game_fn! {
    /// `helldivers2.exe+0x2047D0` (hook or call).
    /// A node's parent, packed: the low 16 bits are 1 when the node has a parent, and the parent's
    /// node index is in the high 16 bits (the root has none): the Unit table's (EngineApi +0x18,
    /// 0x27C8F10) +0x500.
    UnitSceneGraphParentFn / unit_scene_graph_parent / "UnitSceneGraphParent" / hook_unit_scene_graph_parent (unit: u32, node: i32) -> u32
}

game_fn! {
    /// `helldivers2.exe+0x203DD0` (hook or call).
    /// How many terrains a unit has: the Unit table's (EngineApi +0x18, 0x27C8F10) +0x450.
    UnitNumTerrainsFn / unit_num_terrains / "UnitNumTerrains" / hook_unit_num_terrains (unit: u32) -> u32
}

game_fn! {
    /// `helldivers2.exe+0x204A70` (hook or call).
    /// How many LOD objects a unit has: the Unit table's (EngineApi +0x18, 0x27C8F10) +0x538.
    UnitNumLodObjectsFn / unit_num_lod_objects / "UnitNumLodObjects" / hook_unit_num_lod_objects (unit: u32) -> u32
}

game_fn! {
    /// `helldivers2.exe+0x204CF0` (hook or call).
    /// How many lights a unit has: the Unit table's (EngineApi +0x18, 0x27C8F10) +0x580.
    UnitNumLightsFn / unit_num_lights / "UnitNumLights" / hook_unit_num_lights (unit: u32) -> u32
}

game_fn! {
    /// `helldivers2.exe+0x1FC3C0` (hook or call).
    /// How many cameras a unit has: the Unit table's (EngineApi +0x18, 0x27C8F10) +0x6a0.
    UnitNumCamerasFn / unit_num_cameras / "UnitNumCameras" / hook_unit_num_cameras (unit: u32) -> u32
}

game_fn! {
    /// `helldivers2.exe+0x1FE590` (hook or call).
    /// 1 when the unit has a node of this name, else 0. Only the low byte is the answer: the rest
    /// of the register is left over: the Unit table's (EngineApi +0x18, 0x27C8F10) +0x6d0.
    UnitHasNodeFn / unit_has_node / "UnitHasNode" / hook_unit_has_node (unit: u32, name: u32) -> u8
}

game_fn! {
    /// `helldivers2.exe+0x1FE5E0` (hook or call).
    /// A node's index by name, below UnitNumSceneGraphItems. Call it only after UnitHasNode says
    /// 1: Lua's Unit.node stops on a missing name, and that case is not reviewed: the Unit table's
    /// (EngineApi +0x18, 0x27C8F10) +0x6d8.
    UnitNodeFn / unit_node / "UnitNode" / hook_unit_node (unit: u32, name: u32) -> u32
}

game_fn! {
    /// `helldivers2.exe+0x1FEE50` (hook or call).
    /// The level a unit belongs to, or NULL; most units have none: the Unit table's (EngineApi
    /// +0x18, 0x27C8F10) +0x710.
    UnitLevelFn / unit_level / "UnitLevel" / hook_unit_level (unit: u32) -> *mut c_void
}

game_fn! {
    /// `helldivers2.exe+0x1FEED0` (hook or call).
    /// A unit's 64-bit id, or 0 when it has none. It is a value, not a pointer: the Unit table's
    /// (EngineApi +0x18, 0x27C8F10) +0x728.
    UnitIdFn / unit_id / "UnitId" / hook_unit_id (unit: u32) -> u64
}

game_fn! {
    /// `helldivers2.exe+0x1FC120` (hook or call).
    /// The unit resource's name as an IdString64 (MurmurHash64A of the path, seed 0): the Unit
    /// table's (EngineApi +0x18, 0x27C8F10) +0x750.
    UnitResourceNameFn / unit_resource_name / "UnitResourceName" / hook_unit_resource_name (unit: u32) -> u64
}

game_fn! {
    /// `helldivers2.exe+0x1FEF30` (hook or call).
    /// A unit's debug name. In this build that is "#ID[" + UnitResourceName in 16 hex digits +
    /// "]": the Unit table's (EngineApi +0x18, 0x27C8F10) +0x738.
    UnitDebugNameFn / unit_debug_name / "UnitDebugName" / hook_unit_debug_name (unit: u32) -> *const c_char
}
