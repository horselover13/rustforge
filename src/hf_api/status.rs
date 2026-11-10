// The mod's status line for the player.

#![allow(dead_code)]

use super::{HellForgeApi, util::cstring};

impl HellForgeApi {
    /// Sets the mod's status for the player.
    ///
    /// One line, at most 191 bytes; `None` or an empty string clears it.
    /// Returns `false` when the loader is too old or the text contains an
    /// interior NUL byte.
    pub fn set_status(&self, text: Option<&str>) -> bool {
        if !self.supports(16) {
            return false;
        }

        let Some(set_status) = self.raw_api().set_status else {
            return false;
        };

        let text = match text {
            Some(text) => match cstring(text) {
                Some(text) => Some(text),
                None => return false,
            },
            None => None,
        };
        let text = text.as_ref().map_or(std::ptr::null(), |text| text.as_ptr());

        unsafe { set_status(self.mod_handle(), text) };
        true
    }
}
