use std::ptr::{NonNull, null_mut};

/// A boxed value represented by a pointer
///
/// This struct wraps a value in a `Box`, but exposes
/// it as a pointer. The pointer can never be null.
///
/// For optional values (and nullable pointers), see [FNullableBox]
#[repr(transparent)]
pub struct FBox<T>(NonNull<T>);

impl<T> FBox<T> {
    /// Constructor
    ///
    /// This constructor will wrap the value inside a `Box`.
    pub fn new(value: T) -> Self {
        let value = Box::new(value);
        let ptr = Box::into_raw(value);
        let non_null = unsafe { NonNull::new_unchecked(ptr) };
        FBox(non_null)
    }
}

impl<T> Drop for FBox<T> {
    fn drop(&mut self) {
        let value = unsafe { Box::from_raw(self.0.as_ptr()) };
        drop(value);
    }
}

unsafe impl<T> Send for FBox<T> where Box<T>: Send {}
unsafe impl<T> Sync for FBox<T> where Box<T>: Sync {}

impl<T> From<T> for FBox<T> {
    fn from(value: T) -> Self {
        FBox::new(value)
    }
}

/// An optional boxed value represented by a pointer
///
/// This struct wraps its optional content in a `Box`, but exposes
/// it as a pointer. The pointer can then be `nullptr`.
///
/// For non-optional values it's best to use [FBox]
#[repr(transparent)]
pub struct FNullableBox<T>(*mut T);

impl<T> FNullableBox<T> {
    /// Constructor
    ///
    /// This constructor will wrap the value inside a `Box`.
    pub fn new(value: Option<T>) -> Self {
        if let Some(value) = value {
            let value = Box::new(value);
            let ptr = Box::into_raw(value);
            Self(ptr)
        } else {
            Self(null_mut())
        }
    }
}

impl<T> Drop for FNullableBox<T> {
    fn drop(&mut self) {
        if !self.0.is_null() {
            let value = unsafe { Box::from_raw(self.0) };
            drop(value);
        }
    }
}

unsafe impl<T> Send for FNullableBox<T> where Box<T>: Send {}
unsafe impl<T> Sync for FNullableBox<T> where Box<T>: Sync {}

impl<T> From<Option<T>> for FNullableBox<T> {
    fn from(value: Option<T>) -> Self {
        Self::new(value)
    }
}
