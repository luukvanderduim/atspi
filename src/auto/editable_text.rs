// This file was generated by gir (https://github.com/gtk-rs/gir @ 7cca816)
// from gir-files (https://github.com/gtk-rs/gir-files @ 02dfee2)
// DO NOT EDIT

use Error;
use atspi_sys;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;
use std::ptr;

glib_wrapper! {
    pub struct EditableText(Interface<atspi_sys::AtspiEditableText>);

    match fn {
        get_type => || atspi_sys::atspi_editable_text_get_type(),
    }
}

pub const NONE_EDITABLE_TEXT: Option<&EditableText> = None;

pub trait EditableTextExt: 'static {
    fn copy_text(&self, start_pos: i32, end_pos: i32) -> Result<(), Error>;

    fn cut_text(&self, start_pos: i32, end_pos: i32) -> Result<(), Error>;

    fn delete_text(&self, start_pos: i32, end_pos: i32) -> Result<(), Error>;

    fn insert_text(&self, position: i32, text: &str) -> Result<(), Error>;

    fn paste_text(&self, position: i32) -> Result<(), Error>;

    fn set_text_contents(&self, new_contents: &str) -> Result<(), Error>;
}

impl<O: IsA<EditableText>> EditableTextExt for O {
    fn copy_text(&self, start_pos: i32, end_pos: i32) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = atspi_sys::atspi_editable_text_copy_text(self.as_ref().to_glib_none().0, start_pos, end_pos, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn cut_text(&self, start_pos: i32, end_pos: i32) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = atspi_sys::atspi_editable_text_cut_text(self.as_ref().to_glib_none().0, start_pos, end_pos, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn delete_text(&self, start_pos: i32, end_pos: i32) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = atspi_sys::atspi_editable_text_delete_text(self.as_ref().to_glib_none().0, start_pos, end_pos, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn insert_text(&self, position: i32, text: &str) -> Result<(), Error> {
        let length = text.len() as i32;
        unsafe {
            let mut error = ptr::null_mut();
            let _ = atspi_sys::atspi_editable_text_insert_text(self.as_ref().to_glib_none().0, position, text.to_glib_none().0, length, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn paste_text(&self, position: i32) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = atspi_sys::atspi_editable_text_paste_text(self.as_ref().to_glib_none().0, position, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn set_text_contents(&self, new_contents: &str) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = atspi_sys::atspi_editable_text_set_text_contents(self.as_ref().to_glib_none().0, new_contents.to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }
}

impl fmt::Display for EditableText {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "EditableText")
    }
}
