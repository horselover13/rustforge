use super::{HF_API, HellForgeApi};
use crate::rust_ffi::{
    HfEvent, HfEvent_HF_EVENT_ENTITIES_READY, HfEvent_HF_EVENT_GAME_LOADED,
    HfEvent_HF_EVENT_MODS_CHANGED, HfEvent_HF_EVENT_NETWORK_READY,
    HfEvent_HF_EVENT_PROJECTILES_READY, HfEvent_HF_EVENT_SESSION_END, HfMod,
};
use std::ffi::c_void;
use std::fmt;
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::sync::Mutex;

// The bindgen'd names are not upper case, so give the values local names that are.
const HF_EVENT_GAME_LOADED: HfEvent = HfEvent_HF_EVENT_GAME_LOADED;
const HF_EVENT_NETWORK_READY: HfEvent = HfEvent_HF_EVENT_NETWORK_READY;
const HF_EVENT_ENTITIES_READY: HfEvent = HfEvent_HF_EVENT_ENTITIES_READY;
const HF_EVENT_PROJECTILES_READY: HfEvent = HfEvent_HF_EVENT_PROJECTILES_READY;
const HF_EVENT_MODS_CHANGED: HfEvent = HfEvent_HF_EVENT_MODS_CHANGED;
const HF_EVENT_SESSION_END: HfEvent = HfEvent_HF_EVENT_SESSION_END;

/// A load event that can be registered with [`HfApi::on`](crate::rust_ffi::HfApi).
///
/// Events that are not known to this SDK version are preserved as [`Event::Other`]
/// so that newer loaders can still be used without losing the raw value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Event {
    /// The game's code is loaded and can be inspected or hooked.
    GameLoaded,
    /// The engine's network configuration is available.
    NetworkReady,
    /// The game's entity component data is loaded.
    EntitiesReady,
    /// The projectile and damage lists are loaded.
    ProjectilesReady,
    /// The session's mods changed while the game is running.
    ModsChanged,
    /// The session is ending.
    SessionEnd,
    /// An event value that this SDK version does not know about.
    Other(HfEvent),
}

impl Event {
    /// The raw value the C API expects.
    pub const fn as_raw(self) -> HfEvent {
        match self {
            Event::GameLoaded => HF_EVENT_GAME_LOADED,
            Event::NetworkReady => HF_EVENT_NETWORK_READY,
            Event::EntitiesReady => HF_EVENT_ENTITIES_READY,
            Event::ProjectilesReady => HF_EVENT_PROJECTILES_READY,
            Event::ModsChanged => HF_EVENT_MODS_CHANGED,
            Event::SessionEnd => HF_EVENT_SESSION_END,
            Event::Other(raw) => raw,
        }
    }

    /// Converts a raw event value into an [`Event`].
    pub const fn from_raw(raw: HfEvent) -> Self {
        match raw {
            HF_EVENT_GAME_LOADED => Event::GameLoaded,
            HF_EVENT_NETWORK_READY => Event::NetworkReady,
            HF_EVENT_ENTITIES_READY => Event::EntitiesReady,
            HF_EVENT_PROJECTILES_READY => Event::ProjectilesReady,
            HF_EVENT_MODS_CHANGED => Event::ModsChanged,
            HF_EVENT_SESSION_END => Event::SessionEnd,
            other => Event::Other(other),
        }
    }
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Event::GameLoaded => f.write_str("GameLoaded"),
            Event::NetworkReady => f.write_str("NetworkReady"),
            Event::EntitiesReady => f.write_str("EntitiesReady"),
            Event::ProjectilesReady => f.write_str("ProjectilesReady"),
            Event::ModsChanged => f.write_str("ModsChanged"),
            Event::SessionEnd => f.write_str("SessionEnd"),
            Event::Other(raw) => write!(f, "Other({raw})"),
        }
    }
}

/// Why registering a callback failed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CallbackError {
    /// No API has been installed yet (the loader has not called `hellforge_mod_init`).
    NotInitialized,
    /// This loader does not expose `HfApi::on`.
    Unsupported,
    /// The loader refused the registration and returned this status code.
    Refused(i32),
}

impl fmt::Display for CallbackError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CallbackError::NotInitialized => f.write_str("the HellForge API is not initialized"),
            CallbackError::Unsupported => {
                f.write_str("this HellForge loader does not support on()")
            }
            CallbackError::Refused(code) => {
                // this one specifically returns std::result::Result
                write!(f, "HellForge refused the callback (status {code})")
                    .expect("Failed to write");
                Ok(())
            }
        }
    }
}

impl std::error::Error for CallbackError {}

/// The trampoline the loader calls. `user` is a `Box<Mutex<F>>` whose ownership was
/// handed to the loader by [`HellForgeApi::on`].
unsafe extern "C" fn trampoline<F>(_mod: *mut HfMod, raw_event: HfEvent, user: *mut c_void)
where
    F: FnMut(&HellForgeApi, Event) + Send + 'static,
{
    if user.is_null() {
        return;
    }

    let Some(api) = HF_API.get() else {
        return;
    };

    // The loader owns this allocation for the rest of the session: `on` callbacks
    // may fire more than once (EntitiesReady and ModsChanged can), so it must not
    // be reclaimed here. A poisoned lock means a previous invocation panicked; the
    // closure is still valid, so run it anyway.
    let callback = unsafe { &*(user as *const Mutex<F>) };
    let mut callback = match callback.lock() {
        Ok(callback) => callback,
        Err(poisoned) => poisoned.into_inner(),
    };

    let event = Event::from_raw(raw_event);
    if catch_unwind(AssertUnwindSafe(|| callback(api, event))).is_err() {
        // Do not let a panic unwind into the loader's C++ frames. The panic hook has
        // already printed the panic itself; there is no safe API to log through here
        // without allocating a C string and risking re-entrancy.
        eprintln!("rustforge: callback for {event} panicked");
    }
}

impl HellForgeApi {
    /// Registers `callback` to run when `event` occurs.
    ///
    /// The callback runs on the loader's thread, so it must be `Send`. It may run
    /// more than once for events that can repeat (`EntitiesReady`, `ModsChanged`).
    /// The loader revokes callbacks when the mod is unloaded; the closure itself is
    /// owned by the loader for the rest of the session, so one registered for an
    /// event that never fires is not reclaimed until the DLL is unloaded.
    pub fn on<F>(&self, event: Event, callback: F) -> Result<(), CallbackError>
    where
        F: FnMut(&HellForgeApi, Event) + Send + 'static,
    {
        unsafe {
            let c_api = &*self.c_api.ptr();
            let on_fn = c_api.on.ok_or(CallbackError::Unsupported)?;

            let user = Box::into_raw(Box::new(Mutex::new(callback))) as *mut c_void;
            let status = on_fn(
                self.hf_mod.ptr() as *mut HfMod,
                event.as_raw(),
                Some(trampoline::<F>),
                user,
            );

            if status != 0 {
                // Registration failed, so the loader will never call the trampoline:
                // take the allocation back instead of leaking it.
                drop(Box::from_raw(user as *mut Mutex<F>));
                return Err(CallbackError::Refused(status));
            }

            Ok(())
        }
    }

    /// Registers a callback for [`Event::GameLoaded`].
    pub fn on_game_loaded<F>(&self, mut callback: F) -> Result<(), CallbackError>
    where
        F: FnMut(&HellForgeApi) + Send + 'static,
    {
        self.on(Event::GameLoaded, move |api, _| callback(api))
    }

    /// Registers a callback for [`Event::NetworkReady`].
    pub fn on_network_ready<F>(&self, mut callback: F) -> Result<(), CallbackError>
    where
        F: FnMut(&HellForgeApi) + Send + 'static,
    {
        self.on(Event::NetworkReady, move |api, _| callback(api))
    }

    /// Registers a callback for [`Event::EntitiesReady`].
    pub fn on_entities_ready<F>(&self, mut callback: F) -> Result<(), CallbackError>
    where
        F: FnMut(&HellForgeApi) + Send + 'static,
    {
        self.on(Event::EntitiesReady, move |api, _| callback(api))
    }

    /// Registers a callback for [`Event::ProjectilesReady`].
    pub fn on_projectiles_ready<F>(&self, mut callback: F) -> Result<(), CallbackError>
    where
        F: FnMut(&HellForgeApi) + Send + 'static,
    {
        self.on(Event::ProjectilesReady, move |api, _| callback(api))
    }

    /// Registers a callback for [`Event::ModsChanged`].
    pub fn on_mods_changed<F>(&self, mut callback: F) -> Result<(), CallbackError>
    where
        F: FnMut(&HellForgeApi) + Send + 'static,
    {
        self.on(Event::ModsChanged, move |api, _| callback(api))
    }

    /// Registers a callback for [`Event::SessionEnd`].
    pub fn on_session_end<F>(&self, mut callback: F) -> Result<(), CallbackError>
    where
        F: FnMut(&HellForgeApi) + Send + 'static,
    {
        self.on(Event::SessionEnd, move |api, _| callback(api))
    }
}

/// Registers a callback through the globally installed API.
///
/// This is a convenience wrapper around [`HellForgeApi::on`] for callers that do
/// not already hold the API instance.
pub fn on<F>(event: Event, callback: F) -> Result<(), CallbackError>
where
    F: FnMut(&HellForgeApi, Event) + Send + 'static,
{
    HF_API
        .get()
        .ok_or(CallbackError::NotInitialized)?
        .on(event, callback)
}
