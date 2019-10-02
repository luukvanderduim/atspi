use crate::Accessible;
use glib::translate::*;

pub fn get_desktop(i: i32) -> Option<Accessible> {
    unsafe {
        let ret = atspi_sys::atspi_get_desktop(i);
        if ret.is_null() {
            None
        } else {
            Some(from_glib_full(ret))
        }
    }
}

pub fn get_desktop_count() -> i32 {
    unsafe { atspi_sys::atspi_get_desktop_count() as i32 }
}
