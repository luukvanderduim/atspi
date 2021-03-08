// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::translate::*;

glib::wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Rect(Boxed<ffi::AtspiRect>);

    match fn {
        copy => |ptr| glib::gobject_ffi::g_boxed_copy(ffi::atspi_rect_get_type(), ptr as *mut _) as *mut ffi::AtspiRect,
        free => |ptr| glib::gobject_ffi::g_boxed_free(ffi::atspi_rect_get_type(), ptr as *mut _),
        get_type => || ffi::atspi_rect_get_type(),
    }
}