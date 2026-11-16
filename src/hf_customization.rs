// Customization items of a mod's own, from
// `native/mod-sdk/hellforge_customization.h`.
//
// An attachment (SDK 17, `add_attachment`) is a copy of a vanilla weapon
// customization item (a magazine, muzzle, optic...) under an id of its own,
// offered on weapons the mod added. A pattern (SDK 18, `add_pattern`) is a copy
// of a vanilla weapon paint or unit skin with textures swapped in.
//
// Both are added from `hellforge_mod_init`. The typed builders set `size` and
// keep the C strings alive for the call.

#![allow(dead_code)]

use crate::hf_api::{HellForgeApi, util::cstring};
use crate::rust_ffi::{
    HF_PATTERN_COLOURS, HF_PATTERN_SWAPS, HfAttachment, HfPattern, HfTextureSwap,
};
use std::ffi::CString;
use std::os::raw::c_char;
use std::ptr;

/// The most weapons one attachment or weapon pattern may name.
const WEAPON_MAX: usize = 16;

impl HellForgeApi {
    /// Adds a new weapon attachment (call from `hellforge_mod_init`, after the
    /// weapons it names).
    ///
    /// `attachment` is a zeroed [`HfAttachment`] whose `size` is set. Returns
    /// the new item's id (1 and up), or 0 on failure.
    ///
    /// Prefer [`Attachment`], which fills the struct and its strings for you.
    pub fn add_attachment(&self, attachment: &HfAttachment) -> u32 {
        if !self.supports(17) {
            return 0;
        }

        let Some(add_attachment) = self.raw_api().add_attachment else {
            return 0;
        };

        unsafe { add_attachment(self.mod_handle(), attachment) }
    }

    /// Adds a new pattern (call from `hellforge_mod_init`).
    ///
    /// `pattern` is a zeroed [`HfPattern`] whose `size` is set. Returns the
    /// pattern's id (1 and up), or 0 on failure.
    ///
    /// Prefer [`Pattern`], which fills the struct and its arrays for you.
    pub fn add_pattern(&self, pattern: &HfPattern) -> u32 {
        if !self.supports(18) {
            return 0;
        }

        let Some(add_pattern) = self.raw_api().add_pattern else {
            return 0;
        };

        unsafe { add_pattern(self.mod_handle(), pattern) }
    }
}

/// A new weapon attachment (SDK 17, `add_attachment`).
///
/// ```ignore
/// Attachment::new(0x73CA1B5E, "yourname/your_mod/drum", [rifle_id])
///     .unit(api.resource_hash("yourname/your_mod/drum_magazine"))
///     .name_text("Drum")
///     .register(api);
/// ```
#[derive(Clone, Debug)]
pub struct Attachment {
    base: u32,
    name: String,
    unit: u64,
    name_text: Option<String>,
    description: Option<String>,
    weapons: Vec<u32>,
}

impl Attachment {
    /// A copy of vanilla item `base`, offered on `weapons` (1 to 16 ids
    /// [`HellForgeApi::add_weapon`] returned).
    pub fn new(base: u32, name: impl Into<String>, weapons: impl IntoIterator<Item = u32>) -> Self {
        Attachment {
            base,
            name: name.into(),
            unit: 0,
            name_text: None,
            description: None,
            weapons: weapons.into_iter().collect(),
        }
    }

    /// Its model: a resource id ([`HellForgeApi::resource_hash`]) drawn at the
    /// weapon's `attach_` joint; 0 keeps the base's.
    pub fn unit(mut self, resource: u64) -> Self {
        self.unit = resource;
        self
    }

    /// Its name in the armory; absent keeps the base's.
    pub fn name_text(mut self, text: impl Into<String>) -> Self {
        self.name_text = Some(text.into());
        self
    }

    /// Its description in the armory; absent keeps the base's.
    pub fn description(mut self, text: impl Into<String>) -> Self {
        self.description = Some(text.into());
        self
    }

    /// Adds it; returns its id (1 and up), or 0 on failure.
    ///
    /// Fails when the loader is older than SDK 17, the weapon list is empty or
    /// longer than 16, or a string has an interior NUL byte.
    pub fn register(&self, api: &HellForgeApi) -> u32 {
        if !self.valid() {
            return 0;
        }

        self.raw().map_or(0, |raw| api.add_attachment(&raw.raw))
    }

    fn valid(&self) -> bool {
        !self.weapons.is_empty() && self.weapons.len() <= WEAPON_MAX
    }

    fn raw(&self) -> Option<RawAttachment> {
        let name = cstring(&self.name)?;
        let name_text = optional_cstring(self.name_text.as_deref())?;
        let description = optional_cstring(self.description.as_deref())?;
        let weapons = self.weapons.clone();

        let raw = HfAttachment {
            size: std::mem::size_of::<HfAttachment>() as u32,
            base: self.base,
            name: name.as_ptr(),
            unit: self.unit,
            name_text: cstr_ptr(&name_text),
            description: cstr_ptr(&description),
            weapons: weapons.as_ptr(),
            weapon_count: weapons.len() as u32,
        };

        Some(RawAttachment {
            raw,
            _name: name,
            _name_text: name_text,
            _description: description,
            _weapons: weapons,
        })
    }
}

/// Keeps the data an [`HfAttachment`] points at alive for the call.
struct RawAttachment {
    raw: HfAttachment,
    _name: CString,
    _name_text: Option<CString>,
    _description: Option<CString>,
    _weapons: Vec<u32>,
}

/// A new pattern (SDK 18, `add_pattern`).
///
/// ```ignore
/// Pattern::new(0x8F739A74, "yourname/your_mod/tiger") // Forest Camo, a paint
///     .swap(0x6C93BF50B44EFBEE, api.resource_hash("yourname/your_mod/patterns/tiger"))
///     .colour([0.6, 0.35, 0.1])
///     .register(api);
/// ```
#[derive(Clone, Debug)]
pub struct Pattern {
    base: u32,
    name: String,
    name_text: Option<String>,
    description: Option<String>,
    pattern_lut: u64,
    swaps: Vec<(u64, u64)>,
    colours: Vec<[f32; 3]>,
    weapons: Vec<u32>,
}

impl Pattern {
    /// A copy of vanilla pattern `base`: a weapon paint or a unit skin (not a
    /// default).
    pub fn new(base: u32, name: impl Into<String>) -> Self {
        Pattern {
            base,
            name: name.into(),
            name_text: None,
            description: None,
            pattern_lut: 0,
            swaps: Vec::new(),
            colours: Vec::new(),
            weapons: Vec::new(),
        }
    }

    /// Its name in the armory or hangar; absent keeps the base's.
    pub fn name_text(mut self, text: impl Into<String>) -> Self {
        self.name_text = Some(text.into());
        self
    }

    /// A weapon pattern's description; absent keeps the base's.
    pub fn description(mut self, text: impl Into<String>) -> Self {
        self.description = Some(text.into());
        self
    }

    /// A unit skin's pattern colour: a `pattern_lut` resource id
    /// ([`HellForgeApi::resource_hash`]); 0 keeps the base's.
    pub fn pattern_lut(mut self, resource: u64) -> Self {
        self.pattern_lut = resource;
        self
    }

    /// Replaces one of the base's textures (`from`) with one of the mod's
    /// (`to`), by resource id.
    pub fn swap(mut self, from: u64, to: u64) -> Self {
        self.swaps.push((from, to));
        self
    }

    /// Replaces several of the base's textures at once.
    pub fn swaps(mut self, swaps: impl IntoIterator<Item = (u64, u64)>) -> Self {
        self.swaps.extend(swaps);
        self
    }

    /// A swatch colour on its card, linear RGB.
    pub fn colour(mut self, colour: [f32; 3]) -> Self {
        self.colours.push(colour);
        self
    }

    /// The swatch colours on its card (up to 8); absent keeps the base's.
    pub fn colours(mut self, colours: impl IntoIterator<Item = [f32; 3]>) -> Self {
        self.colours.extend(colours);
        self
    }

    /// A weapon pattern's weapons (up to 16 ids); empty means every weapon
    /// offering the base. Not read for unit skins.
    pub fn weapons(mut self, weapons: impl IntoIterator<Item = u32>) -> Self {
        self.weapons.extend(weapons);
        self
    }

    /// Adds it; returns its id (1 and up), or 0 on failure.
    ///
    /// Fails when the loader is older than SDK 18, there are more than 16
    /// texture swaps or 8 colours, more than 16 weapons, or a string has an
    /// interior NUL byte.
    pub fn register(&self, api: &HellForgeApi) -> u32 {
        if !self.valid() {
            return 0;
        }

        self.raw().map_or(0, |raw| api.add_pattern(&raw.raw))
    }

    fn valid(&self) -> bool {
        self.swaps.len() <= HF_PATTERN_SWAPS as usize
            && self.colours.len() <= HF_PATTERN_COLOURS as usize
            && self.weapons.len() <= WEAPON_MAX
    }

    fn raw(&self) -> Option<RawPattern> {
        let name = cstring(&self.name)?;
        let name_text = optional_cstring(self.name_text.as_deref())?;
        let description = optional_cstring(self.description.as_deref())?;
        let swaps: Vec<HfTextureSwap> = self
            .swaps
            .iter()
            .map(|&(from, to)| HfTextureSwap { from, to })
            .collect();
        let colours = self.colours.clone();
        let weapons = self.weapons.clone();

        let raw = HfPattern {
            size: std::mem::size_of::<HfPattern>() as u32,
            base: self.base,
            name: name.as_ptr(),
            name_text: cstr_ptr(&name_text),
            description: cstr_ptr(&description),
            pattern_lut: self.pattern_lut,
            swaps: slice_ptr(&swaps),
            swap_count: swaps.len() as u32,
            colours: slice_ptr(&colours),
            colour_count: colours.len() as u32,
            weapons: slice_ptr(&weapons),
            weapon_count: weapons.len() as u32,
        };

        Some(RawPattern {
            raw,
            _name: name,
            _name_text: name_text,
            _description: description,
            _swaps: swaps,
            _colours: colours,
            _weapons: weapons,
        })
    }
}

/// Keeps the data an [`HfPattern`] points at alive for the call.
struct RawPattern {
    raw: HfPattern,
    _name: CString,
    _name_text: Option<CString>,
    _description: Option<CString>,
    _swaps: Vec<HfTextureSwap>,
    _colours: Vec<[f32; 3]>,
    _weapons: Vec<u32>,
}

/// A C string for `value`, or `None` when it has an interior NUL byte.
///
/// The outer option is `None` only for a NUL error; the inner one is an absent
/// value the C API spells as a null pointer.
fn optional_cstring(value: Option<&str>) -> Option<Option<CString>> {
    match value {
        Some(value) => cstring(value).map(Some),
        None => Some(None),
    }
}

fn cstr_ptr(value: &Option<CString>) -> *const c_char {
    value.as_ref().map_or(ptr::null(), |value| value.as_ptr())
}

/// A slice's pointer, or null when it is empty (the C API's "none").
fn slice_ptr<T>(values: &[T]) -> *const T {
    if values.is_empty() {
        ptr::null()
    } else {
        values.as_ptr()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem::size_of;

    #[test]
    fn attachment_limits_match_the_header() {
        assert!(!Attachment::new(1, "mod/item", []).valid());
        assert!(!Attachment::new(1, "mod/item", 0..17).valid());
        assert!(Attachment::new(1, "mod/item", [1]).valid());
        assert!(Attachment::new(1, "mod/item", 0..16).valid());
    }

    #[test]
    fn pattern_limits_match_the_header() {
        let mut pattern = Pattern::new(1, "mod/item");
        for i in 0..HF_PATTERN_SWAPS {
            pattern = pattern.swap(i.into(), i.into());
        }
        assert!(pattern.valid());
        assert!(!pattern.swap(0, 0).valid());

        let mut pattern = Pattern::new(1, "mod/item");
        for i in 0..HF_PATTERN_COLOURS {
            pattern = pattern.colour([i as f32; 3]);
        }
        assert!(pattern.valid());
        assert!(!pattern.colour([0.0; 3]).valid());
        assert!(Pattern::new(1, "mod/item").valid());
    }

    #[test]
    fn attachment_builds_the_raw_struct() {
        let spec = Attachment::new(7, "mod/drum", [3, 4])
            .unit(0xABCD)
            .name_text("Drum")
            .description("Big.");

        let holder = spec.raw().expect("a valid attachment");
        let raw = holder.raw;

        assert_eq!(raw.size, size_of::<HfAttachment>() as u32);
        assert_eq!(raw.base, 7);
        assert_eq!(raw.unit, 0xABCD);
        assert_eq!(raw.weapon_count, 2);
        assert!(!raw.name.is_null());
        assert!(!raw.name_text.is_null());
        assert!(!raw.description.is_null());
        assert!(!raw.weapons.is_null());
    }

    #[test]
    fn pattern_builds_the_raw_struct() {
        let spec = Pattern::new(0x8F73_9A74, "mod/tiger")
            .name_text("Tiger Stripe")
            .description("Orange and black.")
            .pattern_lut(0x1234)
            .swap(1, 2)
            .colour([0.6, 0.35, 0.1])
            .weapons([9]);

        let holder = spec.raw().expect("a valid pattern");
        let raw = holder.raw;

        assert_eq!(raw.size, size_of::<HfPattern>() as u32);
        assert_eq!(raw.base, 0x8F73_9A74);
        assert_eq!(raw.pattern_lut, 0x1234);
        assert_eq!(raw.swap_count, 1);
        assert_eq!(raw.colour_count, 1);
        assert_eq!(raw.weapon_count, 1);
        assert!(!raw.swaps.is_null());
        assert!(!raw.colours.is_null());
        assert!(!raw.weapons.is_null());
    }

    #[test]
    fn an_empty_optional_array_is_a_null_pointer() {
        let holder = Pattern::new(1, "mod/item").raw().expect("a valid pattern");
        let raw = holder.raw;

        assert!(raw.swaps.is_null());
        assert!(raw.colours.is_null());
        assert!(raw.weapons.is_null());
        assert_eq!(raw.swap_count, 0);
        assert_eq!(raw.colour_count, 0);
        assert_eq!(raw.weapon_count, 0);
    }

    #[test]
    fn an_interior_nul_is_refused() {
        assert!(Attachment::new(1, "bad\0name", [1]).raw().is_none());
        assert!(
            Pattern::new(1, "mod/item")
                .name_text("bad\0text")
                .raw()
                .is_none()
        );
    }
}
