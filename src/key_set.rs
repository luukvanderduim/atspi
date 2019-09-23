use glib::translate::*;
use glib_sys::*;
use std::ffi::CString;
use std::mem;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct KeySet {
    pub keysyms: Box<u32>,        //*mut c_uint
    pub keycodes: Box<u16>,       // *mut
    pub keystrings: Vec<CString>, // *mut *mut char**
    pub len: i16,                 // c_short
}

#[doc(hidden)]
impl Uninitialized for KeySet {
    #[inline]
    unsafe fn uninitialized() -> Self {
        mem::zeroed()
    }
}

#[doc(hidden)]
impl ToGlibPtr<*const atspi_sys::AtspiKeySet> for KeySet {
    type Storage = &Self;

    #[inline]
    fn to_glib_none(&self) -> Stash<*const gtk_sys::GtkRequisition, Self> {
        let ptr: *const KeySet = &*self;
        Stash(ptr as *const atspi_sys::AtspiKeySet, self)
    }
}

#[doc(hidden)]
impl ToGlibPtrMut<*mut atspi_sys::AtspiKeySet> for KeySet {
    type Storage = &'a mut Self;

    #[inline]
    fn to_glib_none_mut(&'a mut self) -> StashMut<'a, *mut atspi_sys::AtspiKeySet, Self> {
        let ptr: *mut KeySet = &mut *self;
        StashMut(ptr as *mut atspi_sys::AtspiKeySet, self)
    }
}
