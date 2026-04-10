use std::ptr::{NonNull, slice_from_raw_parts_mut};

/// A boxed slice represented by a pointer and size
///
/// This struct wraps a boxed (allocated) slice and exposes it to FFI
/// as a pointer and size.
///
/// This type is usually used for returned types, and to avoid memory leaks
/// a function to drop this struct must be provided.
#[repr(C)]
pub struct FSlice<T> {
    items: NonNull<T>,
    size: usize,
}

impl<T> FSlice<T> {
    /// Constructor
    pub fn new(value: Box<[T]>) -> Self {
        let size = value.len();
        let ptr = Box::into_raw(value) as *mut T;
        let items = unsafe { NonNull::new_unchecked(ptr) };
        FSlice { items, size }
    }
}

impl<T> Drop for FSlice<T> {
    fn drop(&mut self) {
        let raw = slice_from_raw_parts_mut(self.items.as_ptr(), self.size);
        let value = unsafe { Box::from_raw(raw) };
        drop(value);
    }
}

impl<T> From<Box<[T]>> for FSlice<T> {
    fn from(value: Box<[T]>) -> Self {
        Self::new(value)
    }
}

impl<T> From<Vec<T>> for FSlice<T> {
    fn from(value: Vec<T>) -> Self {
        Self::new(value.into_boxed_slice())
    }
}

impl<T, const N: usize> From<[T; N]> for FSlice<T> {
    fn from(value: [T; N]) -> Self {
        Self::new(Box::new(value))
    }
}
