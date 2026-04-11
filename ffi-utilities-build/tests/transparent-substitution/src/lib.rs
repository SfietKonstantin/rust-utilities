use ffi_utilities::{FBox, FSlice, FStr, FString};

#[unsafe(no_mangle)]
pub extern "C" fn consume_value(#[allow(unused)] value: FStr) {
    unimplemented!()
}

#[unsafe(no_mangle)]
pub extern "C" fn produce_value() -> FString {
    unimplemented!()
}

pub struct Opaque;

#[unsafe(no_mangle)]
pub extern "C" fn new_instance() -> FBox<Opaque> {
    FBox::new(Opaque)
}

#[unsafe(no_mangle)]
pub extern "C" fn delete_instance(instance: FBox<Opaque>) {
    drop(instance);
}

#[unsafe(no_mangle)]
pub extern "C" fn new_slice() -> FSlice<FString> {
    FSlice::from([])
}

pub extern "C" fn delete_slice(slice: FSlice<FString>) {
    drop(slice);
}
