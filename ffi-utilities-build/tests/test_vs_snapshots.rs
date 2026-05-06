mod testlib;

use cbindgen::{Bindings, Language};
use ffi_utilities_build::post_process_bindings;
use std::io::{Error, Result};
use testlib::generate_bindings_for_crate;

#[test]
fn test_transparent_substitution() {
    let output = run_test(Language::C, "transparent-substitution").unwrap();
    insta::assert_snapshot!(output);
}

pub fn run_test(language: Language, crate_dir: &str) -> Result<String> {
    let bindings = generate_bindings_for_crate(language, crate_dir).map_err(Error::other)?;
    let bindings = post_process_bindings(bindings);
    write_bindings_to_string(bindings)
}

fn write_bindings_to_string(bindings: Bindings) -> Result<String> {
    let mut output = Vec::new();
    bindings.write(&mut output);
    String::from_utf8(output).map_err(Error::other)
}
