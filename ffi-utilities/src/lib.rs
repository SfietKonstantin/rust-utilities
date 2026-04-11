#![deny(missing_docs)]

//! Types that have Rust memory management but can be exposed to C
//!
//! This crate offers some low level types that are convenient to use in Rust,
//! yet can be exposed to C. These types can be used to design functions that
//! are `extern "C"` and discovered by `cbindgen`.
//!
//! - [FBox] and [FNullableBox] are allocated values represented as pointers
//! - [FSlice] is an allocated slice represented as a pointer and size.
//! - [FStr] is a borrowed `const char *`
//! - [FString] and [FNullableString] are allocated strings represented as pointers
//!
//! As [FStr], [FString] and [FNullableString] only convert to `CStr` or `CString`,
//! this crate also provides utility extension traits to convert easily between Rust
//! strings and C strings: [StringExt] and [CStringExt].

mod cstrext;
mod fbox;
mod fslice;
mod fstr;

pub use self::cstrext::*;
pub use self::fbox::*;
pub use self::fslice::*;
pub use self::fstr::*;
