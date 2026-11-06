# Rust Forge - HellForge Mod SDK 16 Implementation

## What This Is

A Rust library for the HellForge modding framework (game build 25480438, SDK 16). It provides safe access to game functions and event callbacks via FFI.

## Key Features

- **Safe Rust Callbacks**: Register closures as event handlers without raw FFI
- **Compile-time Closure Compilation**: Proc macros convert Rust closures to extern "C" fn pointers
- **Event Handlers**: Game loaded, mission start, kill credit events
- **Mod System**: JSON manifest for HellForge modding (SDK 16+ live mods supported)

## Project Structure

```
.
├── examples/simple_mod_callbacks.rs       # Example: Using safe closures
├── src/
│   ├── lib.rs                             # Main library entry point
│   ├── rust_ffi.rs                        # C headers from bindings (bindgen'd)
│   ├── compile_closure.rs                 # Proc macro for closure compilation
│   └── hf_mod/                            # Event system wrapper
│       └── callback.rs                    # Safe callbacks + macros
├── DOCS.md                                # Full SDK documentation
├── build.rs                              # Build script (bindgen + proc macros)
└── Cargo.toml                            # Rust dependencies
```

## Quick Start: Registering a Callback

### 1. Write your Rust closure
```rust
use rustforge::on_game_loaded; // From hf_mod/callback.rs module

// Your safe Rust logic (no unsafe needed for callbacks!)
on_game_loaded!(0x12345678u32, |mod_ptr: &mut HfMod| {
    let fire_rate = mod_ptr.fire_rate;  // Access HellForge API via pointer
    
    println!("Game loaded! Fire rate increased to 120 RPM");
});
```

### 2. HellForge's init thread calls it automatically
- The macro `on_game_loaded!()` registers your closure with the event system
- When the DLL loads, the init thread fires game events and calls our FFI functions
- Your closures are invoked safely via Box<dyn Fn> - no manual unsafe handling needed

## Using Safe Rust Closures Instead of Raw Extern "C" Functions

### Without Our System (Raw FFI)
```rust
// Direct raw function pointer - unsafe, must match exact signature
my_mod.callbacks.push(unsafe extern "C" fn(_mod_ptr: &mut HfModPtr, _udata_raw: *const c_int));
```

### With Our System (Safe Closures)
```rust
// Your closure is compiled to FFI automatically - safe Rust logic!
on_game_loaded!(0x12345678u32, |mod_ptr: &mut HfModPtr| {
    println!("Game loaded!");  // Safe Rust inside callback
    
    // Access HellForge API safely via pointer conversions
});
```

## Event Types Supported

| Callback | Event ID | Param1 | Action |
|----------|----------|--------|--------|
| `on_game_loaded!()` | 6 | 0 | Fire rate increased to 120 RPM |
| `on_mission_start!()` | 1 | 0 | Damage increased by 50% |
| `on_kill_credit!()` | - | 0 | Awards kill credits to mod's peer id |

## Documentation

- **Full SDK Guide**: See `DOCS.md` for 2,800+ lines of complete HellForge SDK 16 documentation
- **API Reference**: All enums, structs, and events documented by file type/offset with timing tags
- **Component Records**: All entity components + settings tables (SDK 6+)

## Build Requirements

```toml
# Cargo.toml entries:
[build-dependencies]
bindgen = "0.73.2"

proc-macro = "1.0"  # For compiling Rust closures to FFI

[lib]
crate-type = ["cdylib"]  # Must be cdylib for load-on-demand mode
```
