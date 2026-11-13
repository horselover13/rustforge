# RustForge

RustForge is an experimental simple Rust binding library for HellForge, that translates its C structs and functions into semi-functional safe Rust alternatives.

It is meant for those who do not trust themselves with writing C code (like myself) and want to get their hands on a cleanier (even if weirder) language.

## Installation

### Versioning

You might want to say, use a different SDK version for some mod that is meant for an older, or even newer (nightly) version. 

The version system for RustForge is pretty simple:

```
0.[SDK NUMBER].[PATCH NUMBER]
```

It is kind of similar to SemVer, just its more of a lobotomized version of it. RustForge meant for a newer SDK will not work with older SDK bindings. Newer SDK bindings, if unchanged, may work with older RustForge version, however, you won't have access to new Rust-safe functions.

### Actual installation

At the moment, i will purposefully not get this up on Cargo, so you will have to drop it into your project folder and use as is.

1. `git clone` the repository.

```bash 
git clone https://github.com/horselover13/rustforge.git
```

2. Drop RustForge somewhere in your project directory, add it to `.gitignore` (redistributing code would not be pretty) and manually set its path in `Cargo.toml`:

```toml
[dependencies]
rustforge = { path = "./rustforge/" }
```

3. Grab the latest [HellForge mod SDK](https://github.com/SkyeShade/HellForge/tree/main/native/mod-sdk) and drop it directly into RustForge's root folder.

4. Use it as you wish.

## Getting Rust to build a HellForge mod

All RustForge mods should be precompiled. HellForge's onboard Zig compiler, obviously, does not compile Rust. Second, the library must be compiled as specifically a DLL. 

First of all, your project should be a library. Its a DLL after all.

```bash 
cargo init --lib
# OR
cargo new my_awesome_mod --lib
```

Now, tell Cargo in `Cargo.toml` that this is a DLL and only a DLL, and that it shouldn't even DARE to compile it as a stupid `.o` file:

```toml
[dependencies]
rustforge = { path = "./rustforge/" }

[lib]
crate-type = ["cdylib", "rlib"]
```

### Note for Linux users

You will also have to specify the target as Windows. And also, you will have to use MinGW.

First of all, get MinGW if you haven't already, using your favorite package manager.

```bash
sudo apt install mingw-w64 # debian or debian based
sudo dnf install mingw64-gcc mingw32-gcc # fedora
sudo pacman -S --needed mingw-w64-gcc # arch people (or arch based people)
```

Then, you need to get Rust to use mingw as linker, and also, by default, compile for Windows. Make a new `.cargo/config.toml` file.

```toml
[build]
rustflags= ["-C", "link-args=-Wl,-rpath='$ORIGIN'/native/mod-sdk/"]
target = "x86_64-pc-windows-gnu"

[target.x86_64-pc-windows-gnu]
linker = "x86_64-w64-mingw32-gcc"
```

Theoretically, it should work afterwards. I think.

## Usage

RustForge has, roughly, 2 primary parts:

- `rustforge::rust_ffi` is a direct conversion of HF mod SDK into Rust, via `bindgen`. It is done automatically, when RustForge builds, and is done via `wrapper.h`, header file that merges all other header files. `hellforge_content.h` specifically is not implemented as a part of the second part, so you will have to use `rust_ffi`. However, it will be always the SDK you download, even if unsafe.

- `rustforge::hf_api, rustforge::hf_functions, rustforge::hf_globals, rustforge::hf_component` are Rust safe implementations of their respective header files. It all ties back to `rustforge::rust_ffi`.

### Differences from the normal C SDK

1. HF_API is a singleton that you retrieve with `rustforge::hf_api::HF_API.get().unwrap()`, or with `hf_api!()` macro.

2. External `hellforge_mod_init` function is already implemented. Mark the function that sets your mod up with `#[hf_entry]`; HellForge calls it after the API has been installed. It takes either no arguments or one `&HellForgeApi`:

```rust
use rustforge::{hf_api::HellForgeApi, hf_entry};

#[hf_entry]
fn init(api: &HellForgeApi) {
    api.log("hello from RustForge");
}
```

3. You don't have to pass that stupid `HfMod` everywhere. RustForge does that for you.
