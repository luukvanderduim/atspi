// Copyright 2013-2018, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! # ATSPI bindings
//!
//! This library contains safe Rust bindings for ATSPI

extern crate libc;
#[macro_use]
extern crate bitflags;

extern crate atspi_sys;
extern crate glib_sys;
extern crate gobject_sys;

#[macro_use]
extern crate glib;
pub use glib::{Error, MainContext};
pub use glib_sys::GDestroyNotify as DestryNotify;

#[macro_use]
mod rt;

mod auto;
pub use crate::auto::*;

mod rect;
pub use crate::rect::*;

mod device_listener;
pub use crate::device_listener::DeviceListenerExt;
pub use crate::device_listener::{DeviceListener, DeviceListenerClass, NONE_DEVICE_LISTENER};

pub mod other;
pub use other::{get_desktop, get_desktop_count};
