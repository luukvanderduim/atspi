use crate::Accessible;
use ffi;
use glib::translate::*;

pub fn get_desktop(i: i32) -> Option<Accessible> {
    unsafe {
        let ret: *mut ffi::AtspiAccessible = ffi::atspi_get_desktop(i);
        if ret.is_null() {
            None
        } else {
            Some(from_glib_full(ret))
        }
    }
}

pub fn get_desktop_count() -> i32 {
    unsafe { ffi::atspi_get_desktop_count() }
}

pub fn init() -> bool {
    unsafe { ffi::atspi_init() == 0 }
}

pub fn exit() -> bool {
    unsafe { ffi::atspi_exit() == 0 }
}
