use std::path::Path;

fn main() {
    let bindings = bindgen::Builder::default()
        .header("./wrapper.h")
        .clang_arg("-Wno-macro-redefined")
        .clang_arg("-DHF_API=")
        .clang_arg("-DHF_ASSERT_SIZE(x,y)=")
        .clang_arg("-DHF_ASSERT_OFFSET(x,y,z)=")
        .clang_arg("-fdeclspec")
        .generate()
        .expect("HF bindings generation failed");

    let out_path = Path::new("./src/");
    bindings
        .write_to_file(out_path.join("rust_ffi.rs"))
        .expect("HF bindings generation failed");
}
