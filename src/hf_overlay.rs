// Screens drawn over the game, from `native/mod-sdk/hellforge_overlay.h`.
//
// The host draws a screen into the game's frames, on Direct3D 11 and 12, in SDR
// and HDR. A mod adds one with [`HellForgeApi::screen_add`]; its key opens and
// closes it, or [`HellForgeApi::screen_show`]. While a modal screen is open,
// the mouse and keyboard are the screen's and the game gets none of them.

#![allow(dead_code)]

use crate::hf_api::{HF_API, HellForgeApi, util::cstring};
use crate::rust_ffi::{
    HF_ALIGN_CENTRE, HF_ALIGN_LEFT, HF_ALIGN_RIGHT, HF_FACE_BODY, HF_FACE_TITLE, HF_MOUSE_LEFT,
    HF_MOUSE_MIDDLE, HF_MOUSE_RIGHT, HF_SCREEN_MODAL, HfColour, HfDraw, HfImage, HfInput, HfPoint,
    HfRect, HfScreen,
};
use std::ffi::c_void;
use std::os::raw::{c_char, c_int};
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::sync::Mutex;

/// A point in screen units.
pub type Point = HfPoint;

/// A rectangle in screen units.
pub type Rect = HfRect;

/// A colour as the C SDK's `HfColour`: `HF_RGBA(r, g, b, a)`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    /// No colour (alpha 0).
    pub const TRANSPARENT: Color = Color::rgba(0, 0, 0, 0);

    /// An opaque colour.
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self::rgba(r, g, b, 255)
    }

    /// A colour with an alpha channel.
    pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Color { r, g, b, a }
    }
}

impl From<HfColour> for Color {
    fn from(colour: HfColour) -> Self {
        Color {
            r: (colour & 0xFF) as u8,
            g: ((colour >> 8) & 0xFF) as u8,
            b: ((colour >> 16) & 0xFF) as u8,
            a: ((colour >> 24) & 0xFF) as u8,
        }
    }
}

impl From<Color> for HfColour {
    fn from(colour: Color) -> Self {
        ((colour.a as u32) << 24)
            | ((colour.b as u32) << 16)
            | ((colour.g as u32) << 8)
            | (colour.r as u32)
    }
}

/// One of the game's fonts (`HF_FACE_*`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Face {
    /// The game's regular face (`HF_FACE_BODY`).
    Body,
    /// The game's medium face (`HF_FACE_TITLE`).
    Title,
}

impl Face {
    /// The raw `HF_FACE_*` value.
    pub const fn as_raw(self) -> u32 {
        match self {
            Face::Body => HF_FACE_BODY,
            Face::Title => HF_FACE_TITLE,
        }
    }
}

/// Text alignment on x (`HF_ALIGN_*`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Align {
    Left,
    Centre,
    Right,
}

impl Align {
    /// The raw `HF_ALIGN_*` value.
    pub const fn as_raw(self) -> u32 {
        match self {
            Align::Left => HF_ALIGN_LEFT,
            Align::Centre => HF_ALIGN_CENTRE,
            Align::Right => HF_ALIGN_RIGHT,
        }
    }
}

/// The mouse and keys for one frame, as the draw callback sees them.
#[derive(Clone, Copy, Debug)]
pub struct InputState {
    /// The mouse, in screen units.
    pub mouse_pos: Point,
    /// Wheel notches since the last frame, positive away from the player.
    pub wheel_delta: f32,
    pub is_left_down: bool,
    pub is_right_down: bool,
    pub is_middle_down: bool,
    /// Mouse buttons pressed since the last frame (`HF_MOUSE_*` bits).
    pub pressed: u32,
    /// Mouse buttons released since the last frame (`HF_MOUSE_*` bits).
    pub released: u32,
    /// Keys pressed since the last frame, as Windows virtual-key codes.
    pub pressed_keys: [u32; 16],
    /// How many entries of [`InputState::pressed_keys`] are set.
    pub key_count: u32,
    /// The screen was asked to close; return `false` from `draw` once its
    /// closing animation is finished.
    pub is_closing: bool,
}

impl InputState {
    /// Converts a host `HfInput` into a safe [`InputState`].
    ///
    /// Only fields the host's `size` covers are read; the rest are defaults,
    /// so a newer Rust binding can run on an older host.
    pub fn from_ffi(input: &HfInput) -> Self {
        // SAFETY: `input` is a live reference, so its first `size` bytes (the
        // field itself) are readable.
        unsafe { Self::from_ptr(input) }
    }

    /// # Safety
    /// `input` must point to a readable host `HfInput`, whose first `size`
    /// bytes are valid.
    unsafe fn from_ptr(input: *const HfInput) -> Self {
        let base = input.cast::<u8>();
        // The leading `size` field is always present.
        let size = unsafe { base.cast::<u32>().read_unaligned() as usize };

        // SAFETY: `field` reads only bytes the host's `size` says are there.
        unsafe {
            InputState {
                mouse_pos: Point {
                    x: field(base, size, std::mem::offset_of!(HfInput, x), 0.0),
                    y: field(base, size, std::mem::offset_of!(HfInput, y), 0.0),
                },
                wheel_delta: field(base, size, std::mem::offset_of!(HfInput, wheel), 0.0),
                is_left_down: field(base, size, std::mem::offset_of!(HfInput, down), 0u32)
                    & HF_MOUSE_LEFT
                    != 0,
                is_right_down: field(base, size, std::mem::offset_of!(HfInput, down), 0u32)
                    & HF_MOUSE_RIGHT
                    != 0,
                is_middle_down: field(base, size, std::mem::offset_of!(HfInput, down), 0u32)
                    & HF_MOUSE_MIDDLE
                    != 0,
                pressed: field(base, size, std::mem::offset_of!(HfInput, pressed), 0),
                released: field(base, size, std::mem::offset_of!(HfInput, released), 0),
                pressed_keys: field(base, size, std::mem::offset_of!(HfInput, keys), [0; 16]),
                key_count: field(base, size, std::mem::offset_of!(HfInput, key_count), 0u32)
                    .min(16),
                is_closing: field(base, size, std::mem::offset_of!(HfInput, closing), 0u32) != 0,
            }
        }
    }
}

/// Reads a `T` at `offset` when the host's `size` covers it.
///
/// # Safety
/// `base` must point to at least `size` readable bytes.
unsafe fn field<T: Copy>(base: *const u8, size: usize, offset: usize, default: T) -> T {
    if size >= offset + std::mem::size_of::<T>() {
        unsafe { base.add(offset).cast::<T>().read_unaligned() }
    } else {
        default
    }
}

type RectFn = unsafe extern "C" fn(f32, f32, f32, f32, HfColour, HfColour, f32);
type LinesFn = unsafe extern "C" fn(*const Point, u32, HfColour, f32, c_int);
type PolygonFn = unsafe extern "C" fn(*const Point, u32, HfColour);
type CircleFn = unsafe extern "C" fn(f32, f32, f32, HfColour, HfColour, f32);
type TextFn = unsafe extern "C" fn(f32, f32, *const c_char, f32, u32, u32, HfColour) -> f32;
type TextWidthFn = unsafe extern "C" fn(*const c_char, f32, u32) -> f32;
type ImageCreateFn = unsafe extern "C" fn(u32, u32, *const c_void) -> *mut HfImage;
type ImageUpdateFn = unsafe extern "C" fn(*mut HfImage, *const c_void);
type ImageFreeFn = unsafe extern "C" fn(*mut HfImage);
type ImageDrawFn = unsafe extern "C" fn(*mut HfImage, *const Rect, *const Rect, HfColour);
type ClipPushFn = unsafe extern "C" fn(f32, f32, f32, f32);
type ClipPopFn = unsafe extern "C" fn();
type BackdropFn = unsafe extern "C" fn(f32, f32, f32);
type HidePointerFn = unsafe extern "C" fn();

/// Drawing for one frame, from the host's `HfDraw`.
///
/// Only valid inside a screen's draw callback: the trampoline is the only thing
/// that constructs it, and it does not outlive the callback's borrow.
pub struct DrawingContext {
    draw: *const HfDraw,
}

impl DrawingContext {
    /// # Safety
    /// `draw` must be the pointer the host passed to a draw callback, valid for
    /// the duration of that callback.
    unsafe fn new(draw: *const HfDraw) -> Self {
        DrawingContext { draw }
    }

    /// The host's `sizeof(HfDraw)`, for the functions added at the end.
    fn host_size(&self) -> usize {
        // SAFETY: `size` is the first field and always present.
        unsafe { self.draw.cast::<u32>().read_unaligned() as usize }
    }

    fn get<T: Copy>(&self, offset: usize) -> Option<T> {
        if self.host_size() >= offset + std::mem::size_of::<T>() {
            // SAFETY: the host's `size` covers this field.
            Some(unsafe {
                self.draw
                    .cast::<u8>()
                    .add(offset)
                    .cast::<T>()
                    .read_unaligned()
            })
        } else {
            None
        }
    }

    /// A function pointer, or `None` when the host's struct ends before it.
    fn function<T: Copy>(&self, offset: usize) -> Option<T> {
        // Read as `Option<T>`: a C null is a valid `None`, not an invalid `T`.
        self.get::<Option<T>>(offset).flatten()
    }

    /// The screen's width in units (1080 units high, width units wide).
    pub fn width(&self) -> f32 {
        self.get(std::mem::offset_of!(HfDraw, width)).unwrap_or(0.0)
    }

    /// The screen's height in units (always 1080).
    pub fn height(&self) -> f32 {
        self.get(std::mem::offset_of!(HfDraw, height))
            .unwrap_or(0.0)
    }

    /// Pixels per unit (the window's height / 1080).
    pub fn pixels(&self) -> f32 {
        self.get(std::mem::offset_of!(HfDraw, pixels))
            .unwrap_or(0.0)
    }

    /// A rectangle, filled and optionally outlined.
    pub fn rect(&self, rect: Rect, fill: Color, outline: Option<Color>, thickness: f32) {
        let Some(rect_fn) = self.function::<RectFn>(std::mem::offset_of!(HfDraw, rect)) else {
            return;
        };
        let (outline, thickness) = outline_parts(outline, thickness);

        // SAFETY: a function pointer the host supplied for this frame.
        unsafe {
            rect_fn(
                rect.x0,
                rect.y0,
                rect.x1,
                rect.y1,
                fill.into(),
                outline,
                thickness,
            )
        }
    }

    /// An open (`closed = false`) or closed line through `points`.
    pub fn lines(&self, points: &[Point], colour: Color, thickness: f32, closed: bool) {
        let Some(lines) = self.function::<LinesFn>(std::mem::offset_of!(HfDraw, lines)) else {
            return;
        };

        // SAFETY: `points` is a live slice of the layout the host expects.
        unsafe {
            lines(
                points.as_ptr(),
                points.len() as u32,
                colour.into(),
                thickness,
                c_int::from(closed),
            )
        }
    }

    /// A filled polygon; the host triangulates concave ones.
    pub fn polygon(&self, points: &[Point], fill: Color) {
        let Some(polygon) = self.function::<PolygonFn>(std::mem::offset_of!(HfDraw, polygon))
        else {
            return;
        };

        // SAFETY: `points` is a live slice of the layout the host expects.
        unsafe { polygon(points.as_ptr(), points.len() as u32, fill.into()) }
    }

    /// A circle, filled and optionally outlined.
    pub fn circle(
        &self,
        centre: Point,
        radius: f32,
        fill: Color,
        outline: Option<Color>,
        thickness: f32,
    ) {
        let Some(circle) = self.function::<CircleFn>(std::mem::offset_of!(HfDraw, circle)) else {
            return;
        };
        let (outline, thickness) = outline_parts(outline, thickness);

        // SAFETY: a function pointer the host supplied for this frame.
        unsafe { circle(centre.x, centre.y, radius, fill.into(), outline, thickness) }
    }

    /// One line of UTF-8 text with its cap height centred on `at.y`.
    ///
    /// Returns its width in units, or 0 when the text contains an interior NUL
    /// byte or the host is too old for the call.
    pub fn text(
        &self,
        at: Point,
        text: &str,
        size: f32,
        face: Face,
        align: Align,
        colour: Color,
    ) -> f32 {
        let Some(text_fn) = self.function::<TextFn>(std::mem::offset_of!(HfDraw, text)) else {
            return 0.0;
        };
        let Some(text) = cstring(text) else {
            return 0.0;
        };

        // SAFETY: `text` is a live NUL-terminated string.
        unsafe {
            text_fn(
                at.x,
                at.y,
                text.as_ptr(),
                size,
                face.as_raw(),
                align.as_raw(),
                colour.into(),
            )
        }
    }

    /// The width [`DrawingContext::text`] would use.
    pub fn text_width(&self, text: &str, size: f32, face: Face) -> f32 {
        let Some(text_width) =
            self.function::<TextWidthFn>(std::mem::offset_of!(HfDraw, text_width))
        else {
            return 0.0;
        };
        let Some(text) = cstring(text) else {
            return 0.0;
        };

        // SAFETY: `text` is a live NUL-terminated string.
        unsafe { text_width(text.as_ptr(), size, face.as_raw()) }
    }

    /// Makes a picture from `width * height` RGBA8 pixels, row-major from the
    /// top, or `None` when the host cannot make it or `rgba` is too short.
    ///
    /// The picture lives until the returned [`Image`] is dropped.
    pub fn image_create(&self, width: u32, height: u32, rgba: &[u8]) -> Option<Image> {
        if rgba.len() < rgba_len(width, height)? {
            return None;
        }

        let create = self.function::<ImageCreateFn>(std::mem::offset_of!(HfDraw, image_create))?;

        // SAFETY: `rgba` is at least the expected number of bytes.
        let image = unsafe { create(width, height, rgba.as_ptr().cast()) };
        if image.is_null() {
            return None;
        }

        Some(Image {
            image,
            width,
            height,
            update: self.function::<ImageUpdateFn>(std::mem::offset_of!(HfDraw, image_update)),
            free: self.function::<ImageFreeFn>(std::mem::offset_of!(HfDraw, image_free)),
        })
    }

    /// Draws a picture into `box_`, bilinear, multiplied by `tint`.
    ///
    /// `part` is the piece to draw, in 0..1 of its width and height; `None` is
    /// all of it.
    pub fn image(&self, image: &Image, box_: Rect, part: Option<Rect>, tint: Color) {
        let Some(image_fn) = self.function::<ImageDrawFn>(std::mem::offset_of!(HfDraw, image))
        else {
            return;
        };
        let part = part.as_ref().map_or(std::ptr::null(), std::ptr::from_ref);

        // SAFETY: `image`, and the boxes, are live for the call.
        unsafe { image_fn(image.image, &box_, part, tint.into()) }
    }

    /// Cuts drawing off outside `rect` until [`DrawingContext::clip_pop`].
    pub fn clip_push(&self, rect: Rect) {
        let Some(clip_push) = self.function::<ClipPushFn>(std::mem::offset_of!(HfDraw, clip_push))
        else {
            return;
        };

        // SAFETY: a function pointer the host supplied for this frame.
        unsafe { clip_push(rect.x0, rect.y0, rect.x1, rect.y1) }
    }

    /// Ends the innermost [`DrawingContext::clip_push`].
    pub fn clip_pop(&self) {
        let Some(clip_pop) = self.function::<ClipPopFn>(std::mem::offset_of!(HfDraw, clip_pop))
        else {
            return;
        };

        // SAFETY: a function pointer the host supplied for this frame.
        unsafe { clip_pop() }
    }

    /// The game's frame as it was when the screen opened: dimmed (`0` none ..
    /// `1` black), desaturated (`0` .. `1`) and blurred (`radius` in units).
    pub fn backdrop(&self, dim: f32, desaturate: f32, blur: f32) {
        let Some(backdrop) = self.function::<BackdropFn>(std::mem::offset_of!(HfDraw, backdrop))
        else {
            return;
        };

        // SAFETY: a function pointer the host supplied, and its struct covers
        // the field (checked by `function`).
        unsafe { backdrop(dim, desaturate, blur) }
    }

    /// Hides the mouse pointer while a modal screen draws its own cursor.
    ///
    /// Call it every frame the cursor is over the screen's own cursor. Older
    /// hosts end at `backdrop`, so this does nothing there.
    pub fn hide_pointer(&self) {
        let Some(hide_pointer) =
            self.function::<HidePointerFn>(std::mem::offset_of!(HfDraw, hide_pointer))
        else {
            return;
        };

        // SAFETY: a function pointer the host supplied, and its struct covers
        // the field (checked by `function`).
        unsafe { hide_pointer() }
    }
}

fn outline_parts(outline: Option<Color>, thickness: f32) -> (HfColour, f32) {
    match outline {
        Some(outline) => (outline.into(), thickness),
        // No outline: thickness 0, fully transparent.
        None => (Color::TRANSPARENT.into(), 0.0),
    }
}

fn rgba_len(width: u32, height: u32) -> Option<usize> {
    (width as usize)
        .checked_mul(height as usize)?
        .checked_mul(4)
}

/// A picture made by [`DrawingContext::image_create`].
///
/// Drop it inside the draw callback (on the host's render thread): it frees the
/// host's picture. It is not `Send`, so it cannot move to another thread.
pub struct Image {
    image: *mut HfImage,
    width: u32,
    height: u32,
    update: Option<ImageUpdateFn>,
    free: Option<ImageFreeFn>,
}

impl Image {
    /// Replaces all its pixels, or does nothing when `rgba` is too short.
    pub fn update(&mut self, rgba: &[u8]) {
        let Some(expected) = rgba_len(self.width, self.height) else {
            return;
        };
        if rgba.len() < expected {
            return;
        }

        if let Some(update) = self.update {
            // SAFETY: `rgba` is at least `width * height * 4` bytes.
            unsafe { update(self.image, rgba.as_ptr().cast()) }
        }
    }
}

impl Drop for Image {
    fn drop(&mut self) {
        if let Some(free) = self.free {
            // SAFETY: `image` came from the host's `image_create` and has not
            // been freed yet.
            unsafe { free(self.image) }
        }
    }
}

/// A screen's draw callback.
pub trait ScreenDraw: Send {
    /// Draws one frame. Return `true` to be drawn again, `false` once closing
    /// has finished.
    fn draw(&mut self, draw: &DrawingContext, input: &InputState, dt: f32) -> bool;
}

impl<F> ScreenDraw for F
where
    F: FnMut(&DrawingContext, &InputState, f32) -> bool + Send,
{
    fn draw(&mut self, draw: &DrawingContext, input: &InputState, dt: f32) -> bool {
        self(draw, input, dt)
    }
}

/// A screen's opened/closed callback.
pub trait ScreenOpened: Send {
    /// The screen opened (`true`, before the first draw) or closed (`false`,
    /// after the last).
    fn opened(&mut self, open: bool);
}

impl<F> ScreenOpened for F
where
    F: FnMut(bool) + Send,
{
    fn opened(&mut self, open: bool) {
        self(open)
    }
}

/// A screen's can-open callback.
pub trait ScreenCanOpen: Send {
    /// Whether the screen may open when its key is pressed.
    fn can_open(&mut self) -> bool;
}

impl<F> ScreenCanOpen for F
where
    F: FnMut() -> bool + Send,
{
    fn can_open(&mut self) -> bool {
        self()
    }
}

/// The default [`ScreenOpened`]: does nothing.
#[derive(Clone, Copy, Debug, Default)]
pub struct NoOpened;

impl ScreenOpened for NoOpened {
    fn opened(&mut self, _open: bool) {}
}

/// The default [`ScreenCanOpen`]: always allows.
#[derive(Clone, Copy, Debug, Default)]
pub struct NoCanOpen;

impl ScreenCanOpen for NoCanOpen {
    fn can_open(&mut self) -> bool {
        true
    }
}

/// A screen to add: its name, key, flags and callbacks.
pub struct ScreenSpec<D, O, C> {
    name: String,
    key: u32,
    modal: bool,
    draw: D,
    opened: O,
    can_open: C,
}

impl<D> ScreenSpec<D, NoOpened, NoCanOpen>
where
    D: ScreenDraw,
{
    /// A screen with no `opened` or `can_open` callback.
    ///
    /// `key` is the virtual key that opens and closes it (`'M' as u32`), or 0
    /// for none (open it with [`HellForgeApi::screen_show`]).
    pub fn new(name: impl Into<String>, key: u32, modal: bool, draw: D) -> Self {
        ScreenSpec {
            name: name.into(),
            key,
            modal,
            draw,
            opened: NoOpened,
            can_open: NoCanOpen,
        }
    }
}

impl<D, O, C> ScreenSpec<D, O, C> {
    /// Called when the screen opens and after it closes.
    pub fn on_opened<F>(self, opened: F) -> ScreenSpec<D, F, C>
    where
        F: ScreenOpened,
    {
        ScreenSpec {
            name: self.name,
            key: self.key,
            modal: self.modal,
            draw: self.draw,
            opened,
            can_open: self.can_open,
        }
    }

    /// Called when its key is pressed, to decide whether it may open.
    pub fn on_can_open<F>(self, can_open: F) -> ScreenSpec<D, O, F>
    where
        F: ScreenCanOpen,
    {
        ScreenSpec {
            name: self.name,
            key: self.key,
            modal: self.modal,
            draw: self.draw,
            opened: self.opened,
            can_open,
        }
    }
}

/// The state the loader keeps for one screen (`HfScreen::user`).
struct ScreenUser<D, O, C> {
    draw: Mutex<D>,
    opened: Mutex<O>,
    can_open: Mutex<C>,
}

unsafe extern "C" fn draw_trampoline<D, O, C>(
    user: *mut c_void,
    draw: *const HfDraw,
    input: *const HfInput,
    dt: f32,
) -> c_int
where
    D: ScreenDraw + 'static,
    O: ScreenOpened + 'static,
    C: ScreenCanOpen + 'static,
{
    if user.is_null() || draw.is_null() || input.is_null() {
        return 0;
    }

    let Some(api) = HF_API.get() else {
        return 0;
    };

    // SAFETY: the loader gives back the pointer `screen_add` handed it.
    let user = unsafe { &*user.cast::<ScreenUser<D, O, C>>() };
    // SAFETY: both pointers are the host's, for this callback.
    let (draw, input) = unsafe { (DrawingContext::new(draw), InputState::from_ptr(input)) };

    let mut callback = match user.draw.lock() {
        Ok(callback) => callback,
        // A previous draw panicked; the callback is still valid.
        Err(poisoned) => poisoned.into_inner(),
    };

    match catch_unwind(AssertUnwindSafe(|| callback.draw(&draw, &input, dt))) {
        Ok(open) => c_int::from(open),
        Err(_) => {
            // Do not let a panic unwind into the host's frames.
            api.log("rustforge: overlay draw callback panicked");
            0
        }
    }
}

unsafe extern "C" fn opened_trampoline<D, O, C>(user: *mut c_void, open: c_int)
where
    D: ScreenDraw + 'static,
    O: ScreenOpened + 'static,
    C: ScreenCanOpen + 'static,
{
    if user.is_null() {
        return;
    }

    let Some(api) = HF_API.get() else {
        return;
    };

    // SAFETY: the loader gives back the pointer `screen_add` handed it.
    let user = unsafe { &*user.cast::<ScreenUser<D, O, C>>() };
    let mut callback = match user.opened.lock() {
        Ok(callback) => callback,
        Err(poisoned) => poisoned.into_inner(),
    };

    if catch_unwind(AssertUnwindSafe(|| callback.opened(open != 0))).is_err() {
        api.log("rustforge: overlay opened callback panicked");
    }
}

unsafe extern "C" fn can_open_trampoline<D, O, C>(user: *mut c_void) -> c_int
where
    D: ScreenDraw + 'static,
    O: ScreenOpened + 'static,
    C: ScreenCanOpen + 'static,
{
    if user.is_null() {
        return 0;
    }

    let Some(api) = HF_API.get() else {
        return 0;
    };

    // SAFETY: the loader gives back the pointer `screen_add` handed it.
    let user = unsafe { &*user.cast::<ScreenUser<D, O, C>>() };
    let mut callback = match user.can_open.lock() {
        Ok(callback) => callback,
        Err(poisoned) => poisoned.into_inner(),
    };

    match catch_unwind(AssertUnwindSafe(|| callback.can_open())) {
        Ok(allowed) => c_int::from(allowed),
        Err(_) => {
            api.log("rustforge: overlay can_open callback panicked");
            0
        }
    }
}

impl HellForgeApi {
    /// Adds a screen drawn over the game.
    ///
    /// `key` is the virtual key that opens and closes it (`'M' as u32`), or 0
    /// for none (open it with [`HellForgeApi::screen_show`]).
    ///
    /// Returns the screen's id (1 and up), or `None` when the loader is older
    /// than SDK 17, the name has an interior NUL byte, or the loader refuses it
    /// (a key another screen already uses, say; the mod's log says why).
    pub fn screen_add<D, O, C>(&self, screen: ScreenSpec<D, O, C>) -> Option<i32>
    where
        D: ScreenDraw + 'static,
        O: ScreenOpened + 'static,
        C: ScreenCanOpen + 'static,
    {
        if !self.supports(17) {
            return None;
        }

        let add_screen = self.raw_api().add_screen?;
        let name = cstring(&screen.name)?;

        let user = Box::into_raw(Box::new(ScreenUser {
            draw: Mutex::new(screen.draw),
            opened: Mutex::new(screen.opened),
            can_open: Mutex::new(screen.can_open),
        }));

        let raw = HfScreen {
            size: std::mem::size_of::<HfScreen>() as u32,
            // The loader copies the name before returning.
            name: name.as_ptr(),
            key: screen.key,
            flags: if screen.modal { HF_SCREEN_MODAL } else { 0 },
            user: user.cast(),
            draw: Some(draw_trampoline::<D, O, C>),
            opened: Some(opened_trampoline::<D, O, C>),
            can_open: Some(can_open_trampoline::<D, O, C>),
        };

        // SAFETY: `raw` describes a live `HfScreen`, and `user` stays allocated
        // until the session ends (the API has no remove call).
        let id = unsafe { add_screen(self.mod_handle(), &raw) };

        if id == 0 {
            // The loader refused it and will never call the callbacks: take the
            // allocation back instead of leaking it.
            drop(unsafe { Box::from_raw(user) });
            return None;
        }

        Some(id)
    }

    /// Opens (`true`) or closes (`false`) one of the mod's screens.
    ///
    /// A close sets `input.closing`; draw is called until it returns `false`.
    /// Returns `true` when queued, `false` when the loader is older than SDK 17.
    pub fn screen_show(&self, screen: i32, open: bool) -> bool {
        if !self.supports(17) {
            return false;
        }

        let Some(screen_show) = self.raw_api().screen_show else {
            return false;
        };

        // SAFETY: `screen` is one of this mod's ids.
        unsafe { screen_show(self.mod_handle(), screen, c_int::from(open)) };
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem::{offset_of, size_of};

    #[test]
    fn colour_round_trips() {
        let colour = Color::rgba(1, 2, 3, 4);
        let raw: HfColour = colour.into();

        assert_eq!(raw, 0x0403_0201);
        assert_eq!(Color::from(raw), colour);
        assert_eq!(Color::rgb(255, 128, 0).a, 255);
    }

    #[test]
    fn faces_and_alignments_match_the_header() {
        assert_eq!(Face::Body.as_raw(), 0);
        assert_eq!(Face::Title.as_raw(), 1);
        assert_eq!(Align::Left.as_raw(), 0);
        assert_eq!(Align::Centre.as_raw(), 1);
        assert_eq!(Align::Right.as_raw(), 2);
    }

    #[test]
    fn input_state_reads_the_whole_struct() {
        let mut keys = [0u32; 16];
        keys[0] = 0x1B;
        keys[1] = b'M' as u32;

        let input = HfInput {
            size: size_of::<HfInput>() as u32,
            x: 10.0,
            y: 20.0,
            wheel: -1.5,
            down: HF_MOUSE_LEFT | HF_MOUSE_MIDDLE,
            pressed: HF_MOUSE_RIGHT,
            released: 0,
            keys,
            key_count: 2,
            closing: 1,
        };

        let state = InputState::from_ffi(&input);

        assert_eq!(state.mouse_pos.x, 10.0);
        assert_eq!(state.mouse_pos.y, 20.0);
        assert_eq!(state.wheel_delta, -1.5);
        assert!(state.is_left_down);
        assert!(state.is_middle_down);
        assert!(!state.is_right_down);
        assert_eq!(state.pressed, HF_MOUSE_RIGHT);
        assert_eq!(state.released, 0);
        assert_eq!(&state.pressed_keys[..2], &[0x1B, b'M' as u32]);
        assert_eq!(state.key_count, 2);
        assert!(state.is_closing);
    }

    #[test]
    fn input_state_ignores_fields_past_size() {
        let input = HfInput {
            // The mouse is there; `keys`, `key_count` and `closing` are not.
            size: offset_of!(HfInput, keys) as u32,
            x: 1.0,
            y: 2.0,
            wheel: 0.0,
            down: HF_MOUSE_LEFT,
            pressed: 0,
            released: 0,
            keys: [7; 16],
            key_count: 9,
            closing: 1,
        };

        let state = InputState::from_ffi(&input);

        assert!(state.is_left_down);
        assert_eq!(state.pressed_keys, [0; 16]);
        assert_eq!(state.key_count, 0);
        assert!(!state.is_closing);
    }

    #[test]
    fn key_count_is_clamped_to_the_array() {
        let input = HfInput {
            size: size_of::<HfInput>() as u32,
            x: 0.0,
            y: 0.0,
            wheel: 0.0,
            down: 0,
            pressed: 0,
            released: 0,
            keys: [0; 16],
            key_count: 99,
            closing: 0,
        };

        assert_eq!(InputState::from_ffi(&input).key_count, 16);
    }
}
