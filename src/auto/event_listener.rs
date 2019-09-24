// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Error;
use Event;
use atspi_sys;
use glib::object::IsA;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::fmt;
use std::ptr;

glib_wrapper! {
    pub struct EventListener(Object<atspi_sys::AtspiEventListener, atspi_sys::AtspiEventListenerClass, EventListenerClass>);

    match fn {
        get_type => || atspi_sys::atspi_event_listener_get_type(),
    }
}

impl EventListener {
    pub fn new<P: Fn(&Event) + 'static>(callback: P) -> EventListener {
