use ffi_utilities::{CStringExt, FBox, FStr};
use std::ffi::CString;
use std::os::raw::c_char;

#[derive(Default)]
pub struct Service(i32, CString);

#[unsafe(no_mangle)]
pub extern "C" fn service_new() -> FBox<Service> {
    FBox::from(Service::default())
}

#[unsafe(no_mangle)]
pub extern "C" fn service_delete(service: FBox<Service>) {
    drop(service);
}

#[unsafe(no_mangle)]
pub extern "C" fn service_get_number(service: &Service) -> i32 {
    service.0
}

#[unsafe(no_mangle)]
pub extern "C" fn service_set_number(service: &mut Service, value: i32) {
    service.0 = value;
}

#[unsafe(no_mangle)]
pub extern "C" fn service_get_string(service: &Service) -> *const c_char {
    service.1.as_ptr()
}

#[unsafe(no_mangle)]
pub extern "C" fn service_print_string(service: &Service) {
    let string = service.1.to_string_lossy();
    println!("Printing from service: {string}");
}

#[unsafe(no_mangle)]
pub extern "C" fn service_set_string(service: &mut Service, value: FStr) {
    service.1 = CString::from(value.as_c_str());
}
