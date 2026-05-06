mod testlib;

use cbindgen::{Bindings, Language};
use ffi_utilities_build::post_process_bindings;
use std::fs::{File, create_dir_all, remove_dir_all, write};
use std::io::{BufWriter, Error, Result};
use std::path::{Path, PathBuf};
use testlib::generate_bindings_for_crate;

#[test]
fn test_transparent_substitution() {
    run_test(Language::C, "transparent-substitution").unwrap();
}

const MAIN_C: &[u8] = include_bytes!("main.c");

pub fn run_test(language: Language, crate_dir: &str) -> Result<()> {
    let bindings = generate_bindings_for_crate(language, crate_dir).map_err(Error::other)?;
    let bindings = post_process_bindings(bindings);
    let build_dir = write_build_dir(crate_dir, bindings)?;
    try_compile(&build_dir, language)
}

fn write_build_dir(name: &str, bindings: Bindings) -> Result<PathBuf> {
    // Create the build dir
    let build_dir = Path::new("tests").join("build").join(name);
    if build_dir.exists() {
        remove_dir_all(&build_dir)?;
    }
    create_dir_all(&build_dir)?;

    // Write header
    let file = File::create(build_dir.join("header.h"))?;
    let mut writer = BufWriter::new(file);
    bindings.write(&mut writer);

    // Write simple source file
    write(build_dir.join("main.c"), MAIN_C)?;

    Ok(build_dir)
}

fn try_compile(build_dir: &Path, language: Language) -> Result<()> {
    let mut builder = cc::Build::new();
    builder
        .file(build_dir.join("main.c"))
        .include(build_dir)
        .host(current_platform::CURRENT_PLATFORM)
        .target(current_platform::CURRENT_PLATFORM)
        .opt_level(0)
        .out_dir(build_dir);

    if let Language::Cxx = language {
        builder.cpp(true);
    }

    builder.try_compile("test").map_err(Error::other)
}
