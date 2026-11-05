# RustForge

A Rust FFI layer inbetween HellForge's C Mod SDK and decently memory-safe Rust.

Created for those (like myself) who are the weird ones and must prefer using a Blazingly Fast 🚀🚀🚀 low-level language.

## Modules

`rustforge::rust_ffi` is a direct port of mod SDK's header files into Rust. They're unsafe (for obvious reasons),
but usually they're the most up-to-date ones, since they are automatically generated with `bindgen`.

`rustforge` module itself is unsafe `rustforge:rust_ffi` wrapped into hopefully safe function calls. `rustforge` includes:

- `rustforge::hf_api` - Rust-safe implementations of classes within `hellforge_mod.h`
- `rustforge::hf_component` - Rust-safe implementations of classes within `hellforge_components.h`
- `rustforge::hf_content` - Rust-safe implementations of classes within `hellforge_content.h`

