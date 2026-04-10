use std::ffi::{CStr, CString, c_char};
use std::ptr::{NonNull, null_mut};

/// A C compatible string reference
///
/// This struct exposes a `const char *` to FFI. It is usually for
/// input parameter.
///
/// It can be converted to a `CStr`, meaning that convenient methods
/// from [CStringExt][crate::CStringExt] can be used.
#[repr(transparent)]
pub struct FStr(*const c_char);

impl FStr {
    /// Converts this string reference to a `CStr`
    pub fn as_c_str(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.0) }
    }
}

impl AsRef<CStr> for FStr {
    fn as_ref(&self) -> &CStr {
        self.as_c_str()
    }
}

impl<'a> From<&'a FStr> for &'a CStr {
    fn from(value: &'a FStr) -> Self {
        value.as_c_str()
    }
}

/// An allocated string represented by a pointer
///
/// This struct wraps an allocated string and exposes it as a pointer
/// to FFI. The pointer can never be null.
///
/// This type is usually used for returned types, and to avoid memory leaks
/// a function to drop this struct must be provided.
///
/// For optional values (and nullable pointers), see [FNullableString]
#[repr(transparent)]
pub struct FString(NonNull<c_char>);

impl FString {
    /// Constructor
    ///
    /// To convert strings to `CString`, convenient methods
    /// from [StringExt][crate::StringExt] can be used.
    pub fn new(value: CString) -> Self {
        let ptr = value.into_raw();
        let non_null = unsafe { NonNull::new_unchecked(ptr) };
        FString(non_null)
    }
}

impl Drop for FString {
    fn drop(&mut self) {
        let value = unsafe { CString::from_raw(self.0.as_ptr()) };
        drop(value);
    }
}

impl From<CString> for FString {
    fn from(value: CString) -> Self {
        Self::new(value)
    }
}

/// An optional allocated string represented by a pointer
///
/// This struct wraps an allocated string and exposes it as a pointer
/// to FFI. The pointer can be null.
///
/// This type is usually used for returned types, and to avoid memory leaks
/// a function to drop this struct must be provided.
///
/// For non-optional values it's best to use [FString]
#[repr(transparent)]
pub struct FNullableString(*mut c_char);

impl FNullableString {
    /// Constructor
    ///
    /// To convert strings to `CString`, convenient methods
    /// from [StringExt][crate::StringExt] can be used.
    pub fn new(value: Option<CString>) -> Self {
        if let Some(value) = value {
            let ptr = value.into_raw();
            Self(ptr)
        } else {
            Self(null_mut())
        }
    }
}

impl Drop for FNullableString {
    fn drop(&mut self) {
        if !self.0.is_null() {
            let value = unsafe { CString::from_raw(self.0) };
            drop(value);
        }
    }
}

impl From<Option<CString>> for FNullableString {
    fn from(value: Option<CString>) -> Self {
        Self::new(value)
    }
}
