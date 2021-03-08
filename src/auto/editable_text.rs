// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::object::IsA;
use glib::translate::*;
use std::fmt;
use std::ptr;

glib::wrapper! {
    pub struct EditableText(Interface<ffi::AtspiEditableText>);

    match fn {
        get_type => || ffi::atspi_editable_text_get_type(),
    }
}

pub const NONE_EDITABLE_TEXT: Option<&EditableText> = None;

pub trait EditableTextExt: 'static {
    #[doc(alias = "atspi_editable_text_copy_text")]
    fn copy_text(&self, start_pos: i32, end_pos: i32) -> Result<(), glib::Error>;

    #[doc(alias = "atspi_editable_text_cut_text")]
    fn cut_text(&self, start_pos: i32, end_pos: i32) -> Result<(), glib::Error>;

    #[doc(alias = "atspi_editable_text_delete_text")]
    fn delete_text(&self, start_pos: i32, end_pos: i32) -> Result<(), glib::Error>;

    #[doc(alias = "atspi_editable_text_insert_text")]
    fn insert_text(&self, position: i32, text: &str) -> Result<(), glib::Error>;

    #[doc(alias = "atspi_editable_text_paste_text")]
    fn paste_text(&self, position: i32) -> Result<(), glib::Error>;

    #[doc(alias = "atspi_editable_text_set_text_contents")]
    fn set_text_contents(&self, new_contents: &str) -> Result<(), glib::Error>;
}

impl<O: IsA<EditableText>> EditableTextExt for O {
    fn copy_text(&self, start_pos: i32, end_pos: i32) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::atspi_editable_text_copy_text(self.as_ref().to_glib_none().0, start_pos, end_pos, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn cut_text(&self, start_pos: i32, end_pos: i32) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::atspi_editable_text_cut_text(self.as_ref().to_glib_none().0, start_pos, end_pos, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn delete_text(&self, start_pos: i32, end_pos: i32) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::atspi_editable_text_delete_text(self.as_ref().to_glib_none().0, start_pos, end_pos, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn insert_text(&self, position: i32, text: &str) -> Result<(), glib::Error> {
        let length = text.len() as i32;
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::atspi_editable_text_insert_text(self.as_ref().to_glib_none().0, position, text.to_glib_none().0, length, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn paste_text(&self, position: i32) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::atspi_editable_text_paste_text(self.as_ref().to_glib_none().0, position, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn set_text_contents(&self, new_contents: &str) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::atspi_editable_text_set_text_contents(self.as_ref().to_glib_none().0, new_contents.to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }
}

impl fmt::Display for EditableText {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("EditableText")
    }
}
