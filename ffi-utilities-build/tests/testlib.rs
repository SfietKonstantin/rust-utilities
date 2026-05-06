use cbindgen::{Bindings, Builder, Error as CBError, Language};
use std::path::Path;

pub fn generate_bindings_for_crate(
    language: Language,
    crate_dir: &str,
) -> Result<Bindings, CBError> {
    Builder::new()
        .with_language(language)
        .with_parse_deps(true)
        .with_parse_include(&["ffi-utilities"])
        .with_crate(Path::new("tests").join(crate_dir))
        .with_documentation(false)
        .generate()
}
