// Game functions by name, from `native/mod-sdk/hellforge_functions.h`.

#![allow(dead_code)]

use crate::hf_api::HellForgeApi;
use std::ffi::c_void;
use std::mem::{size_of, transmute_copy};

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
