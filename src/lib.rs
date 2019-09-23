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
extern crate libdbus_sys as dbus;

#[macro_use]
extern crate glib;

#[macro_use]
mod rt;

mod auto;
pub use auto::*;

//mod key_set;
//pub use key_set::KeySet;

use dbus::DBusConnection;
use glib::Error;
