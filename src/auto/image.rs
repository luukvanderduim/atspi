// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::CoordType;
use crate::Point;
use crate::Rect;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;
use std::ptr;

glib::wrapper! {
    pub struct Image(Interface<ffi::AtspiImage>);

    match fn {
        get_type => || ffi::atspi_image_get_type(),
    }
}

pub const NONE_IMAGE: Option<&Image> = None;

pub trait ImageExt: 'static {
    #[doc(alias = "atspi_image_get_image_description")]
    fn get_image_description(&self) -> Result<glib::GString, glib::Error>;

    #[doc(alias = "atspi_image_get_image_extents")]
    fn get_image_extents(&self, ctype: CoordType) -> Result<Rect, glib::Error>;

    #[doc(alias = "atspi_image_get_image_locale")]
    fn get_image_locale(&self) -> Result<glib::GString, glib::Error>;

    #[doc(alias = "atspi_image_get_image_position")]
    fn get_image_position(&self, ctype: CoordType) -> Result<Point, glib::Error>;

    #[doc(alias = "atspi_image_get_image_size")]
    fn get_image_size(&self) -> Result<Point, glib::Error>;
}

impl<O: IsA<Image>> ImageExt for O {
    fn get_image_description(&self) -> Result<glib::GString, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::atspi_image_get_image_description(self.as_ref().to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    fn get_image_extents(&self, ctype: CoordType) -> Result<Rect, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::atspi_image_get_image_extents(self.as_ref().to_glib_none().0, ctype.to_glib(), &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    fn get_image_locale(&self) -> Result<glib::GString, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::atspi_image_get_image_locale(self.as_ref().to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    fn get_image_position(&self, ctype: CoordType) -> Result<Point, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::atspi_image_get_image_position(self.as_ref().to_glib_none().0, ctype.to_glib(), &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    fn get_image_size(&self) -> Result<Point, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::atspi_image_get_image_size(self.as_ref().to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }
}

impl fmt::Display for Image {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Image")
    }
}
