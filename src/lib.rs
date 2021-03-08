// Copyright 2013-2018, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <https://opensource.org/licenses/MIT>

#![cfg_attr(feature = "dox", feature(doc_cfg))]
#![allow(unused_imports)]

pub extern crate ffi;
pub use crate::Accessible;

extern crate libc;
#[macro_use]
extern crate bitflags;

#[doc(hidden)]
extern crate glib_sys;
extern crate gobject_sys;
#[macro_use]
extern crate glib;

#[macro_use]
mod rt;

#[allow(unused_imports)]
mod auto;
pub use auto::*;

pub mod prelude;

mod rect;
pub use crate::rect::*;

mod event;
pub use crate::event::*;

pub mod other;
pub use other::{get_desktop, get_desktop_count};


