use ffi_utilities::FBox;

pub struct Service(i32);

#[unsafe(no_mangle)]
pub extern "C" fn service_new() -> FBox<Service> {
    FBox::from(Service(42))
}

#[unsafe(no_mangle)]
pub extern "C" fn service_delete(opaque: FBox<Service>) {
    drop(opaque);
}

#[unsafe(no_mangle)]
pub extern "C" fn service_get_value(opaque: &Service) -> i32 {
    opaque.0
}
