use std::env;
use std::path::Path;

fn main() {
    if env::var("CARGO_CFG_RUST_ANALYZER").is_ok() {
        return;
    }

    // Compile Rust closures into FFI-compatible functions for HellForge events
    let out_path = Path::new("./src/");
    
    // Generate bindgen'd C headers from wrapper.h
    let rust_ffi_bindings = bindgen::Builder::default()
        .header("./wrapper.h")
        .clang_arg("-Wno-macro-redefined")
        .clang_arg("-DHF_API=")
        .clang_arg("-DHF_ASSERT_SIZE(x,y)=")
        .clang_arg("-DHF_ASSERT_OFFSET(x,y,z)=")
        .clang_arg("-fdeclspec")
        .generate()
        .expect("HF bindings generation failed");
    
    // Write bindgen'd FFI definitions to separate file
    rust_ffi_bindings.write_to_file(out_path.join("rust_ffi.rs"))
        .expect("HF bindings generation failed");
    
    // Compile Rust closures in callback.rs into FFI-compatible functions
    println!("cargo:rerun-if-changed=src/hf_mod/callback.rs");
}
