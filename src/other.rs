use crate::Accessible;
use atspi_sys::*;
use glib::translate::*;

pub fn get_desktop(i: i32) -> Option<Accessible> {
    unsafe {
        let ret: *mut AtspiAccessible = atspi_get_desktop(i);
        if ret.is_null() {
            None
        } else {
            Some(from_glib_full(ret))
        }
    }
}

// This function is known not to work in AT-SPI
pub fn get_desktop_count() -> i32 {
    unsafe { atspi_get_desktop_count() }
}
