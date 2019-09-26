// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use atspi_sys;
use gobject_sys;

glib_wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Rect(Boxed<atspi_sys::AtspiRect>);

    match fn {
        copy => |ptr| gobject_sys::g_boxed_copy(atspi_sys::atspi_rect_get_type(), ptr as *mut _) as *mut atspi_sys::AtspiRect,
        free => |ptr| gobject_sys::g_boxed_free(atspi_sys::atspi_rect_get_type(), ptr as *mut _),
        get_type => || atspi_sys::atspi_rect_get_type(),
    }
}
