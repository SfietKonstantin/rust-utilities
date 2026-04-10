#![deny(missing_docs)]

//! Types that have Rust memory management but can be exposed to C
//!
//! This crate offers some low level types that are convenient to use in Rust,
//! yet can be exposed to C. These types can be used to design functions that
//! are `extern "C"` and discovered by `cbindgen`.
//!
//! - [FBox] and [FNullableBox] are boxed values representing them as pointers

mod fbox;

pub use self::fbox::*;
