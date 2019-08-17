use glib::translate::*;
use glib_sys::*;
use gtk_sys;
use std::mem;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct KeySet {
    pub keysyms: gpointer<u32>,
    pub keycodes: gpointer<u16>,
    pub keystrings: *mut *mut char,
    pub len: c_short,
}

#[doc(hidden)]
impl Uninitialized for KeySet {
    #[inline]
    unsafe fn uninitialized() -> Self {
        mem::zeroed()
    }
}

#[doc(hidden)]
impl<'a> ToGlibPtr<'a, *const atspi_sys::AtspiKeySet> for KeySet {
    type Storage = &'a Self;

    #[inline]
    fn to_glib_none(&'a self) -> Stash<'a, *const gtk_sys::GtkRequisition, Self> {
        let ptr: *const KeySet = &*self;
        Stash(ptr as *const atspi_sys::AtspiKeySet, self)
    }
}

#[doc(hidden)]
impl<'a> ToGlibPtrMut<'a, *mut atspi_sys::AtspiKeySet> for KeySet {
    type Storage = &'a mut Self;

    #[inline]
    fn to_glib_none_mut(&'a mut self) -> StashMut<'a, *mut atspi_sys::AtspiKeySet, Self> {
        let ptr: *mut KeySet = &mut *self;
        StashMut(ptr as *mut atspi_sys::AtspiKeySet, self)
    }
}
