/* fn clear_selection(&self) -> Result<(), Error> {
    unsafe {
        let mut error = ptr::null_mut();
        let _ = atspi_sys::atspi_selection_clear_selection(self.as_ref().to_glib_none().0, &mut error);
        if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
    }
} */

use crate::Accessible;
use glib::translate::*;

pub fn get_desktop_count() -> i32 {
    unsafe { atspi_sys::atspi_get_desktop_count() as i32 }
}

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
