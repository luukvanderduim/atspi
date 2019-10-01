// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Accessible;
use Error;
use Object;
use Range;
use atspi_sys;
use glib::GString;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;
use std::ptr;

glib_wrapper! {
    pub struct Hyperlink(Object<atspi_sys::AtspiHyperlink, atspi_sys::AtspiHyperlinkClass, HyperlinkClass>) @extends Object;

    match fn {
        get_type => || atspi_sys::atspi_hyperlink_get_type(),
    }
}

pub const NONE_HYPERLINK: Option<&Hyperlink> = None;

pub trait HyperlinkExt: 'static {
    fn get_end_index(&self) -> Result<i32, Error>;

    fn get_index_range(&self) -> Result<Range, Error>;

    fn get_n_anchors(&self) -> Result<i32, Error>;

    fn get_object(&self, i: i32) -> Result<Accessible, Error>;

    fn get_start_index(&self) -> Result<i32, Error>;

    fn get_uri(&self, i: i32) -> Result<GString, Error>;

    fn is_valid(&self) -> Result<(), Error>;
}

impl<O: IsA<Hyperlink>> HyperlinkExt for O {
    fn get_end_index(&self) -> Result<i32, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = atspi_sys::atspi_hyperlink_get_end_index(self.as_ref().to_glib_none().0, &mut error);
            if error.is_null() { Ok(ret) } else { Err(from_glib_full(error)) }
        }
    }

    fn get_index_range(&self) -> Result<Range, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = atspi_sys::atspi_hyperlink_get_index_range(self.as_ref().to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    fn get_n_anchors(&self) -> Result<i32, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = atspi_sys::atspi_hyperlink_get_n_anchors(self.as_ref().to_glib_none().0, &mut error);
            if error.is_null() { Ok(ret) } else { Err(from_glib_full(error)) }
        }
    }

    fn get_object(&self, i: i32) -> Result<Accessible, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = atspi_sys::atspi_hyperlink_get_object(self.as_ref().to_glib_none().0, i, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    fn get_start_index(&self) -> Result<i32, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = atspi_sys::atspi_hyperlink_get_start_index(self.as_ref().to_glib_none().0, &mut error);
            if error.is_null() { Ok(ret) } else { Err(from_glib_full(error)) }
        }
    }

    fn get_uri(&self, i: i32) -> Result<GString, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = atspi_sys::atspi_hyperlink_get_uri(self.as_ref().to_glib_none().0, i, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    fn is_valid(&self) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = atspi_sys::atspi_hyperlink_is_valid(self.as_ref().to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }
}

impl fmt::Display for Hyperlink {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Hyperlink")
    }
}
