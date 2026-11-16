// Lets `#[hf_entry]`-generated code use `::rustforge::...` inside this crate too.
extern crate self as rustforge;

pub use rustforge_macros::hf_entry;

pub mod hf_api;
pub mod hf_component;
pub mod hf_customization;
pub mod hf_functions;
pub mod hf_globals;
pub mod hf_net;
pub mod hf_overlay;
pub mod hf_ui;
#[allow(warnings)]
pub mod rust_ffi;
