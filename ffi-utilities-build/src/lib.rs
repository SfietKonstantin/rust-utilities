#![deny(missing_docs)]

//! Build script support for `ffi-utilities`
//!
//! This crate uses [`cbindgen`](https://crates.io/crates/cbindgen) to
//! generate headers for an FFI crate, while cleaning the generated headers
//! to look more C or C++ native.
//!
//! Use [generate_header] in a build script to generate the C header for a
//! crate.
//!
//! If you need more control over how `cbindgen` is run, you can use
//! [post_process_bindings] to just run the header cleanup.

use cbindgen::ir::{Field, Function, FunctionArgument, ItemContainer, Struct, Type};
use cbindgen::{Bindings, Builder, Config, Error};
use std::collections::HashMap;
use std::path::Path;

/// Generate a C header for a crate
///
/// Given a crate dir and an output path, this function will write a C header
/// containing all exported C functions of that crate (using `cbindgen` under
/// the hood)
///
/// ## Example
///
/// ```no_run
/// # use ffi_utilities_build::generate_header;
/// use std::env;
///
/// let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
/// ffi_utilities_build::generate_header(crate_dir, "my-header.h").unwrap();
/// ```
pub fn generate_header<P, O>(crate_dir: P, output_path: O) -> Result<(), Error>
where
    P: AsRef<Path>,
    O: AsRef<Path>,
{
    let mut config = Config::from_root_or_default(crate_dir.as_ref());
    config.parse.parse_deps = true;
    config.parse.include = if let Some(mut include) = config.parse.include {
        include.push("ffi-utilities".to_string());
        Some(include)
    } else {
        Some(vec!["ffi-utilities".to_string()])
    };

    let bindings = Builder::new()
        .with_config(config)
        .with_crate(crate_dir)
        .generate()?;

    post_process_bindings(bindings).write_to_file(output_path);

    Ok(())
}

/// Bindings post processor
///
/// This function cleans up bindings parsed by `cbindgen` to remove all
/// transparent types and replace them with the types they wrap.
///
/// This is useful when working with `ffi-utilities` as it uses many wrapper
/// types for resource management.
pub fn post_process_bindings(mut bindings: Bindings) -> Bindings {
    let (items, mapping) = partition_transparent_rep(bindings.items);
    bindings.items = items
        .into_iter()
        .map(|i| post_process_item(i, &mapping))
        .collect();
    bindings.functions = bindings
        .functions
        .into_iter()
        .map(|f| post_process_function(f, &mapping))
        .collect();
    bindings
}

struct TransparentMapping(HashMap<String, Struct>);

impl TransparentMapping {
    fn find_type(&self, name: &str) -> Option<Type> {
        let s = self.0.get(name)?;
        let field = s.fields.first()?;
        Some(field.ty.clone())
    }

    fn resolve_type(&self, ty: Type) -> Type {
        match ty {
            Type::Ptr {
                ty,
                is_const,
                is_nullable,
                is_ref,
            } => {
                let ty = Box::new(self.resolve_type(*ty));
                Type::Ptr {
                    ty,
                    is_const,
                    is_nullable,
                    is_ref,
                }
            }
            Type::Path(path) => self
                .find_type(path.export_name())
                .unwrap_or(Type::Path(path)),
            Type::Primitive(_) => ty,
            Type::Array(ty, expr) => {
                let ty = Box::new(self.resolve_type(*ty));
                Type::Array(ty, expr)
            }
            Type::FuncPtr {
                ret,
                args,
                is_nullable,
                never_return,
            } => {
                let ret = Box::new(self.resolve_type(*ret));
                let args = args
                    .into_iter()
                    .map(|(n, a)| (n, self.resolve_type(a)))
                    .collect();
                Type::FuncPtr {
                    ret,
                    args,
                    is_nullable,
                    never_return,
                }
            }
        }
    }
}

fn partition_transparent_rep(
    input_items: Vec<ItemContainer>,
) -> (Vec<ItemContainer>, TransparentMapping) {
    let mut items = Vec::new();
    let mut mapping = HashMap::new();

    for item in input_items {
        match item {
            ItemContainer::Struct(s) if s.is_transparent => {
                mapping.insert(s.export_name.clone(), s);
            }
            _ => {
                items.push(item);
            }
        }
    }

    (items, TransparentMapping(mapping))
}

fn post_process_item(item: ItemContainer, mapping: &TransparentMapping) -> ItemContainer {
    match item {
        ItemContainer::Constant(mut c) => {
            c.ty = mapping.resolve_type(c.ty);
            ItemContainer::Constant(c)
        }
        ItemContainer::Static(mut s) => {
            s.ty = mapping.resolve_type(s.ty);
            ItemContainer::Static(s)
        }
        ItemContainer::Struct(mut s) => {
            s.fields = s
                .fields
                .into_iter()
                .map(|f| post_process_field(f, mapping))
                .collect();
            ItemContainer::Struct(s)
        }
        ItemContainer::Union(mut u) => {
            u.fields = u
                .fields
                .into_iter()
                .map(|f| post_process_field(f, mapping))
                .collect();
            ItemContainer::Union(u)
        }
        ItemContainer::Typedef(mut t) => {
            t.aliased = mapping.resolve_type(t.aliased);
            ItemContainer::Typedef(t)
        }
        _ => item,
    }
}

fn post_process_field(mut field: Field, mapping: &TransparentMapping) -> Field {
    field.ty = mapping.resolve_type(field.ty);
    field
}

fn post_process_function(mut function: Function, mapping: &TransparentMapping) -> Function {
    function.ret = mapping.resolve_type(function.ret);
    function.args = function
        .args
        .into_iter()
        .map(|a| post_process_function_argument(a, mapping))
        .collect();
    function
}

fn post_process_function_argument(
    mut arg: FunctionArgument,
    mapping: &TransparentMapping,
) -> FunctionArgument {
    arg.ty = mapping.resolve_type(arg.ty);
    arg
}
