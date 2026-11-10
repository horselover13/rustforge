// Game-thread callbacks and game-event listeners.

#![allow(dead_code)]

use super::{HF_API, HellForgeApi, callback::CallbackError};
use crate::rust_ffi::{HfListener, HfMod};
use std::ffi::c_void;
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::sync::Mutex;

/// The trampoline the loader calls every game update.
unsafe extern "C" fn tick_trampoline<F>(_mod: *mut HfMod, dt: f32, user: *mut c_void)
where
    F: FnMut(&HellForgeApi, f32) + Send + 'static,
{
    if user.is_null() {
        return;
    }

    let Some(api) = HF_API.get() else {
        return;
    };

    // The loader owns this allocation for the session: on_game_thread has no
    // unregister call, so it is not reclaimed here.
    let callback = unsafe { &*(user as *const Mutex<F>) };
    let mut callback = match callback.lock() {
        Ok(callback) => callback,
        Err(poisoned) => poisoned.into_inner(),
    };

    if catch_unwind(AssertUnwindSafe(|| callback(api, dt))).is_err() {
        eprintln!("rustforge: on_game_thread callback panicked");
    }
}

impl HellForgeApi {
    /// Calls `callback` every game update on the game thread.
    ///
    /// The callback runs before the game's own update, every frame: keep it
    /// short. It is `Send` because the game thread is not the registering
    /// thread, and it stays registered for the session (the API has no
    /// unregister call), so a closure that never runs is not reclaimed until
    /// the DLL is unloaded.
    pub fn on_game_thread<F>(&self, callback: F) -> Result<(), CallbackError>
    where
        F: FnMut(&HellForgeApi, f32) + Send + 'static,
    {
        if !self.supports(6) {
            return Err(CallbackError::Unsupported);
        }

        let on_game_thread = self
            .raw_api()
            .on_game_thread
            .ok_or(CallbackError::Unsupported)?;

        let user = Box::into_raw(Box::new(Mutex::new(callback))) as *mut c_void;
        let status = unsafe { on_game_thread(self.mod_handle(), Some(tick_trampoline::<F>), user) };

        if status == 0 {
            // The loader never took the callback.
            drop(unsafe { Box::from_raw(user as *mut Mutex<F>) });
            return Err(CallbackError::Refused(0));
        }

        Ok(())
    }

    /// Listens to one kind of game event (`HF_ON_*`).
    ///
    /// `listener` receives an `HfEventInfo` whose real layout is the event
    /// kind's struct (for example `HfStateEvent`); `user` is passed back
    /// unchanged. Returns `true` when the listener was registered.
    ///
    /// # Safety
    /// `listener`, when called, must treat its event pointer as the layout of
    /// `kind` and must not keep it after returning. `user` must remain valid
    /// for the session, or be null.
    pub unsafe fn subscribe(
        &self,
        kind: u32,
        listener: HfListener,
        user: *mut c_void,
        priority: i32,
    ) -> bool {
        if !self.supports(7) {
            return false;
        }

        let Some(subscribe) = self.raw_api().subscribe else {
            return false;
        };

        unsafe { subscribe(self.mod_handle(), kind, listener, user, priority) != 0 }
    }
}
