// Screens in the game's own UI, from `native/mod-sdk/hellforge_ui.h`.
//
// A screen is XAML parsed by the game's NoesisGUI, so its styles, fonts and
// themed controls are available. Name the elements the code talks to
// (`x:Name`). Every call may come from any thread: HellForge runs it on the
// game thread at its next update.

#![allow(dead_code)]

use crate::hf_api::{
    HF_API, HellForgeApi,
    util::{cstring, string_from_ptr},
};
use crate::rust_ffi::{HF_UI_MENU, HF_UI_OVERLAY, HfMod};
use std::ffi::c_void;
use std::os::raw::{c_char, c_int};
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::sync::Mutex;

/// The kind of screen [`HellForgeApi::ui_open`] opens (`HF_UI_*`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ScreenKind {
    /// Over the game, on the ship and in missions; no cursor of its own, so not
    /// for clicking.
    Overlay,
    /// A menu, as the game's own: the cursor shows, the player's movement and
    /// camera are held, and Back (Esc) closes it. One mod menu at a time; it
    /// opens once no game menu is open.
    Menu,
}

impl ScreenKind {
    /// The raw `HF_UI_*` value.
    pub const fn as_raw(self) -> u32 {
        match self {
            ScreenKind::Overlay => HF_UI_OVERLAY,
            ScreenKind::Menu => HF_UI_MENU,
        }
    }
}

/// What a screen callback saw.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ScreenEvent<'a> {
    /// A named Button (or anything with a Command) was clicked.
    Click { screen: i32, element: &'a str },
    /// The screen closed: [`HellForgeApi::ui_close`], or Back on a menu.
    Closed { screen: i32 },
}

impl ScreenEvent<'_> {
    /// The screen the event came from.
    pub const fn screen(self) -> i32 {
        match self {
            ScreenEvent::Click { screen, .. } | ScreenEvent::Closed { screen } => screen,
        }
    }
}

/// Maps the callback's element (`""` or null when the screen closes) to an
/// event.
fn event_from(screen: i32, element: Option<&str>) -> ScreenEvent<'_> {
    match element {
        Some(element) if !element.is_empty() => ScreenEvent::Click { screen, element },
        _ => ScreenEvent::Closed { screen },
    }
}

/// The trampoline the loader calls on a click or when a screen closes. `user`
/// is a `Box<Mutex<F>>` whose ownership was handed to the loader by
/// [`HellForgeApi::ui_on`].
unsafe extern "C" fn trampoline<F>(
    _mod: *mut HfMod,
    screen: c_int,
    element: *const c_char,
    user: *mut c_void,
) where
    F: FnMut(&HellForgeApi, ScreenEvent<'_>) + Send + 'static,
{
    if user.is_null() {
        return;
    }

    let Some(api) = HF_API.get() else {
        return;
    };

    // The loader keeps this allocation for the session: a click callback may
    // run every time its element is clicked, and there is no unregister call,
    // so it is not reclaimed here. A poisoned lock means a previous invocation
    // panicked; the closure is still valid, so run it anyway.
    let callback = unsafe { &*(user as *const Mutex<F>) };
    let mut callback = match callback.lock() {
        Ok(callback) => callback,
        Err(poisoned) => poisoned.into_inner(),
    };

    let element = unsafe { string_from_ptr(element) };
    let event = event_from(screen, element.as_deref());

    if catch_unwind(AssertUnwindSafe(|| callback(api, event))).is_err() {
        // Do not let a panic unwind into the loader's C++ frames. The panic
        // hook has already logged the reason.
        api.log("rustforge: UI callback panicked");
    }
}

impl HellForgeApi {
    /// Opens a screen.
    ///
    /// `xaml` is a file of the mod's folder (`"ui/shop.xaml"`) or XAML text.
    /// The screen shows at the next game update; at most 64 mods' screens exist
    /// across all mods.
    ///
    /// Returns the screen's id, or `None` when refused (the mod's log says
    /// why).
    pub fn ui_open(&self, xaml: &str, kind: ScreenKind) -> Option<i32> {
        if !self.supports(19) {
            return None;
        }

        let xaml = cstring(xaml)?;
        let ui_open = self.raw_api().ui_open?;

        let screen = unsafe { ui_open(self.mod_handle(), xaml.as_ptr(), kind.as_raw()) };

        (screen != 0).then_some(screen)
    }

    /// Sets a property of a named element of `screen` from text, as XAML
    /// would.
    ///
    /// `element` `None` (or empty) sets the screen's root. For example:
    /// `"Text", "SUPPLY"`; `"Visibility", "Collapsed"`; `"Foreground",
    /// "#FFE900"`.
    ///
    /// Returns `true` when queued, `false` when refused or when a string
    /// contains an interior NUL byte.
    pub fn ui_set(&self, screen: i32, element: Option<&str>, property: &str, value: &str) -> bool {
        if !self.supports(19) {
            return false;
        }

        let Some(ui_set) = self.raw_api().ui_set else {
            return false;
        };

        let Some(property) = cstring(property) else {
            return false;
        };
        let Some(value) = cstring(value) else {
            return false;
        };
        let element = match element.filter(|element| !element.is_empty()) {
            Some(element) => match cstring(element) {
                Some(element) => Some(element),
                None => return false,
            },
            None => None,
        };
        let element = element
            .as_ref()
            .map_or(std::ptr::null(), |element| element.as_ptr());

        unsafe {
            ui_set(
                self.mod_handle(),
                screen,
                element,
                property.as_ptr(),
                value.as_ptr(),
            ) == 0
        }
    }

    /// Calls `callback` when `element` of `screen` is clicked.
    ///
    /// With `element` `None` (or empty), calls it when the screen closes
    /// ([`HellForgeApi::ui_close`], or Back on a menu), as
    /// [`ScreenEvent::Closed`].
    ///
    /// The callback runs on the game thread, so it must be `Send`: `ui_on`
    /// may be called from any thread. The loader keeps it for the session
    /// (the API has no unregister call), so it is not reclaimed until the DLL
    /// is unloaded. Returns `true` when queued, `false` when refused.
    pub fn ui_on<F>(&self, screen: i32, element: Option<&str>, callback: F) -> bool
    where
        F: FnMut(&HellForgeApi, ScreenEvent<'_>) + Send + 'static,
    {
        if !self.supports(19) {
            return false;
        }

        let Some(ui_on) = self.raw_api().ui_on else {
            return false;
        };

        let element = match element.filter(|element| !element.is_empty()) {
            Some(element) => match cstring(element) {
                Some(element) => Some(element),
                None => return false,
            },
            None => None,
        };
        let element = element
            .as_ref()
            .map_or(std::ptr::null(), |element| element.as_ptr());

        let user = Box::into_raw(Box::new(Mutex::new(callback))) as *mut c_void;
        let status = unsafe {
            ui_on(
                self.mod_handle(),
                screen,
                element,
                Some(trampoline::<F>),
                user,
            )
        };

        if status != 0 {
            // Registration failed, so the loader will never call the
            // trampoline: take the allocation back instead of leaking it.
            drop(unsafe { Box::from_raw(user as *mut Mutex<F>) });
            return false;
        }

        true
    }

    /// Closes one of the mod's screens.
    ///
    /// A menu closes as the game closes its own. Returns `true` when queued,
    /// `false` when refused.
    pub fn ui_close(&self, screen: i32) -> bool {
        if !self.supports(19) {
            return false;
        }

        let Some(ui_close) = self.raw_api().ui_close else {
            return false;
        };

        unsafe { ui_close(self.mod_handle(), screen) == 0 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn kinds_match_the_header() {
        assert_eq!(ScreenKind::Overlay.as_raw(), 0);
        assert_eq!(ScreenKind::Menu.as_raw(), 1);
    }

    #[test]
    fn empty_or_missing_element_is_a_close() {
        assert_eq!(event_from(7, None), ScreenEvent::Closed { screen: 7 });
        assert_eq!(event_from(7, Some("")), ScreenEvent::Closed { screen: 7 });
        assert_eq!(
            event_from(7, Some("Ok")),
            ScreenEvent::Click {
                screen: 7,
                element: "Ok"
            }
        );
    }
}
