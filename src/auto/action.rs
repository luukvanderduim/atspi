// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use atspi_sys;
use glib;
use glib::GString;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;
use std::ptr;

glib_wrapper! {
    pub struct Action(Interface<atspi_sys::AtspiAction>);

    match fn {
        get_type => || atspi_sys::atspi_action_get_type(),
    }
}

pub const NONE_ACTION: Option<&Action> = None;

pub trait ActionExt: 'static {
    fn do_action(&self, i: i32) -> Result<(), glib::Error>;

    fn get_action_description(&self, i: i32) -> Result<GString, glib::Error>;

    fn get_action_name(&self, i: i32) -> Result<GString, glib::Error>;

    fn get_key_binding(&self, i: i32) -> Result<GString, glib::Error>;

    fn get_localized_name(&self, i: i32) -> Result<GString, glib::Error>;

    fn get_n_actions(&self) -> Result<i32, glib::Error>;
}

impl<O: IsA<Action>> ActionExt for O {
    fn do_action(&self, i: i32) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = atspi_sys::atspi_action_do_action(self.as_ref().to_glib_none().0, i, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn get_action_description(&self, i: i32) -> Result<GString, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = atspi_sys::atspi_action_get_action_description(self.as_ref().to_glib_none().0, i, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    fn get_action_name(&self, i: i32) -> Result<GString, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = atspi_sys::atspi_action_get_action_name(self.as_ref().to_glib_none().0, i, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    fn get_key_binding(&self, i: i32) -> Result<GString, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = atspi_sys::atspi_action_get_key_binding(self.as_ref().to_glib_none().0, i, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    fn get_localized_name(&self, i: i32) -> Result<GString, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = atspi_sys::atspi_action_get_localized_name(self.as_ref().to_glib_none().0, i, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    fn get_n_actions(&self) -> Result<i32, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = atspi_sys::atspi_action_get_n_actions(self.as_ref().to_glib_none().0, &mut error);
            if error.is_null() { Ok(ret) } else { Err(from_glib_full(error)) }
        }
    }
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Action")
    }
}
