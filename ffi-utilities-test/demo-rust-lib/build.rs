use std::env;

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    ffi_utilities_build::generate_header(crate_dir, "demo-rust-lib.h").unwrap();
}
