use crate::rust_ffi::*;
use std::ffi::CStr;
use std::ptr;

/// A safe structure wrapping the raw C screen point.
/// Defines layout explicitly matching the C struct for FFI compatibility.
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

/// A safe structure wrapping the raw C rectangle.
#[derive(Debug, Clone, Copy)]
pub struct Rect {
    pub x0: f32,
    pub y0: f32,
    pub x1: f32,
    pub y1: f32,
}

/// A safe representation of the raw C color value `HfColour`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl From<HfColour> for Color {
    fn from(colour: HfColour) -> Self {
        Color {
            r: ((colour & 0xFF) as u8), // Simplified from identity_op
            g: ((colour >> 8) & 0xFF) as u8,
            b: ((colour >> 16) & 0xFF) as u8,
            a: ((colour >> 24) & 0xFF) as u8,
        }
    }
}

// Fixes clippy::from_over_into: Use From instead of Into
impl From<Color> for HfColour {
    fn from(val: Color) -> Self {
        ((val.a as u32) << 24) | ((val.b as u32) << 16) | ((val.g as u32) << 8) | (val.r as u32)
    }
}

/// A safe input state for a single frame.
#[derive(Debug, Default)]
pub struct InputState {
    pub mouse_pos: Point,
    pub wheel_delta: f32,
    pub is_left_down: bool,
    pub is_right_down: bool,
    pub is_middle_down: bool,
    pub pressed_keys: [u32; 16],
    pub key_count: u32,
    pub is_closing: bool,
}

impl InputState {
    /// Converts raw `HfInput` from FFI into a safe `InputState`.
    pub fn from_ffi(input: &HfInput) -> Self {
        let mut pressed_array = [0u32; 16];
        unsafe {
            ptr::copy_nonoverlapping(input.keys.as_ptr(), pressed_array.as_mut_ptr(), 16);
        }

        InputState {
            mouse_pos: Point {
                x: input.x,
                y: input.y,
            },
            wheel_delta: input.wheel,
            is_left_down: (input.down & HF_MOUSE_LEFT) != 0,
            is_right_down: (input.down & HF_MOUSE_RIGHT) != 0,
            is_middle_down: (input.down & HF_MOUSE_MIDDLE) != 0,
            pressed_keys: pressed_array,
            key_count: input.key_count,
            is_closing: input.closing != 0,
        }
    }
}

/// Provides a safe drawing context wrapper over the raw `HfDraw` FFI structure.
pub struct DrawingContext<'a> {
    inner: &'a HfDraw,
}

impl<'a> DrawingContext<'a> {
    /// Creates a safe drawing context from the raw FFI context.
    pub fn new(draw: &'a HfDraw) -> Self {
        DrawingContext { inner: draw }
    }

    /// Renders a filled rectangle using the FFI context.
    pub fn rect(&self, x0: f32, y0: f32, x1: f32, y1: f32, color: Color, thickness: f32) {
        unsafe {
            if let Some(rect_func) = self.inner.rect {
                rect_func(x0, y0, x1, y1, color.into(), color.into(), thickness);
            }
        }
    }

    /// Renders a line segment or closed polygon.
    pub fn lines(&self, points: &[Point], color: Color, thickness: f32, closed: bool) {
        unsafe {
            if let Some(lines_func) = self.inner.lines {
                // Force cast to the type expected by C, as our Point is #[repr(C)]
                lines_func(
                    points.as_ptr() as *const HfPoint,
                    points.len() as u32,
                    color.into(),
                    thickness,
                    closed as i32,
                );
            }
        }
    }

    /// Renders a filled polygon.
    pub fn polygon(&self, points: &[Point], color: Color) {
        unsafe {
            if let Some(polygon_func) = self.inner.polygon {
                // Force cast to the type expected by C, as our Point is #[repr(C)]
                polygon_func(
                    points.as_ptr() as *const HfPoint,
                    points.len() as u32,
                    color.into(),
                );
            }
        }
    }

    /// Renders a circle.
    pub fn circle(
        &self,
        x: f32,
        y: f32,
        radius: f32,
        fill_color: Color,
        outline_color: Color,
        thickness: f32,
    ) {
        unsafe {
            if let Some(circle_func) = self.inner.circle {
                circle_func(
                    x,
                    y,
                    radius,
                    fill_color.into(),
                    outline_color.into(),
                    thickness,
                );
            }
        }
    }
}

/// Represents a modded overlay screen in a safe, idiomatic manner.
pub struct Screen {
    _name: String,
    _key: u32,   // Used for FFI compatibility
    _flags: u32, // Used for FFI compatibility
    user_data: *mut (),
    draw_callback: unsafe fn(*mut (), &HfDraw, &HfInput, f32) -> i32,
    opened_callback: unsafe fn(*mut (), i32),
    _can_open_callback: unsafe fn(*mut ()) -> i32, // Used for FFI compatibility
}

impl Screen {
    /// Creates a safe `Screen` wrapper from an FFI definition.
    ///
    /// # Safety
    /// The caller must ensure that the FFI pointers and user data are valid for the lifetime
    /// of the created `Screen` instance and the system that uses it.
    pub unsafe fn from_ffi(
        name: &CStr,
        _key: u32,   // Parameter required by FFI, but unused in safe logic
        _flags: u32, // Parameter required by FFI, but unused in safe logic
        user_data: *mut (),
        draw_fn: fn(*mut (), &HfDraw, &HfInput, f32) -> i32,
        opened_fn: fn(*mut (), i32),
        can_open_fn: fn(*mut ()) -> i32,
    ) -> Self {
        Screen {
            _name: name.to_string_lossy().into_owned(),
            _key,   // Fixed: Removed redundant field name
            _flags, // Fixed: Removed redundant field name
            user_data,
            draw_callback: draw_fn,
            opened_callback: opened_fn,
            _can_open_callback: can_open_fn,
        }
    }

    /// Executes the draw loop for one frame.
    /// Returns true if the screen should be drawn again, false if it is closing.
    ///
    /// # Safety
    /// This is unsafe as it invokes the custom user-supplied callback function.
    #[allow(unsafe_op_in_unsafe_fn)]
    pub unsafe fn render_frame(&self, draw: &HfDraw, input: &HfInput, dt: f32) -> bool {
        let draw_result = (self.draw_callback)(self.user_data, draw, input, dt);

        if draw_result == 0 {
            // Call the opened/closed callback for signaling screen state change
            (self.opened_callback)(self.user_data, 0);
            return false;
        }
        true
    }
}

/// Provides a safe interface to the HellForge Overlay System.
pub struct OverlaySystem;

impl OverlaySystem {
    /// Initializes the overlay system.
    pub fn init() -> Result<Self, &'static str> {
        Ok(OverlaySystem)
    }

    /// Registers a new overlay screen.
    ///
    /// # Safety
    /// The user-provided functions and data must be valid for the entire lifetime of the system.
    /// Failure to maintain this contract will result in Undefined Behavior.
    #[allow(clippy::too_many_arguments, unsafe_op_in_unsafe_fn)]
    pub unsafe fn register_screen(
        &self,
        name: &CStr,
        _key: u32,   // Parameter required by FFI
        _flags: u32, // Parameter required by FFI
        user_data: *mut (),
        draw_fn: fn(*mut (), &HfDraw, &HfInput, f32) -> i32,
        opened_fn: fn(*mut (), i32),
        can_open_fn: fn(*mut ()) -> i32,
    ) -> Screen {
        Screen::from_ffi(
            name,
            _key,
            _flags,
            user_data,
            draw_fn,
            opened_fn,
            can_open_fn,
        )
    }
}

