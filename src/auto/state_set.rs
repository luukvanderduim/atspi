// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use atspi_sys;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;
use StateType;

glib_wrapper! {
    pub struct StateSet(Object<atspi_sys::AtspiStateSet, atspi_sys::AtspiStateSetClass, StateSetClass>);

    match fn {
        get_type => || atspi_sys::atspi_state_set_get_type(),
    }
}

impl StateSet {
    //pub fn new(states: /*Unknown conversion*//*Unimplemented*/Array TypeId { ns_id: 1, id: 62 }) -> StateSet {
    //    unsafe { TODO: call atspi_sys:atspi_state_set_new() }
    //}
}

pub const NONE_STATE_SET: Option<&StateSet> = None;

pub trait StateSetExt: 'static {
    fn add(&self, state: StateType);

    fn compare<P: IsA<StateSet>>(&self, set2: &P) -> Option<StateSet>;

    fn contains(&self, state: StateType) -> bool;

    fn equals<P: IsA<StateSet>>(&self, set2: &P) -> bool;

    //fn get_states(&self) -> /*Unknown conversion*//*Unimplemented*/Array TypeId { ns_id: 1, id: 62 };

    fn is_empty(&self) -> bool;

    fn remove(&self, state: StateType);

    fn set_by_name(&self, name: &str, enabled: bool);
}

impl<O: IsA<StateSet>> StateSetExt for O {
    fn add(&self, state: StateType) {
        unsafe {
            atspi_sys::atspi_state_set_add(self.as_ref().to_glib_none().0, state.to_glib());
        }
    }

    fn compare<P: IsA<StateSet>>(&self, set2: &P) -> Option<StateSet> {
        unsafe {
            from_glib_full(atspi_sys::atspi_state_set_compare(self.as_ref().to_glib_none().0, set2.as_ref().to_glib_none().0))
        }
    }

    fn contains(&self, state: StateType) -> bool {
        unsafe {
            from_glib(atspi_sys::atspi_state_set_contains(self.as_ref().to_glib_none().0, state.to_glib()))
        }
    }

    fn equals<P: IsA<StateSet>>(&self, set2: &P) -> bool {
        unsafe {
            from_glib(atspi_sys::atspi_state_set_equals(self.as_ref().to_glib_none().0, set2.as_ref().to_glib_none().0))
        }
    }

    //fn get_states(&self) -> /*Unknown conversion*//*Unimplemented*/Array TypeId { ns_id: 1, id: 62 } {
    //    unsafe { TODO: call atspi_sys:atspi_state_set_get_states() }
    //}

    fn is_empty(&self) -> bool {
        unsafe {
            from_glib(atspi_sys::atspi_state_set_is_empty(self.as_ref().to_glib_none().0))
        }
    }

    fn remove(&self, state: StateType) {
        unsafe {
            atspi_sys::atspi_state_set_remove(self.as_ref().to_glib_none().0, state.to_glib());
        }
    }

    fn set_by_name(&self, name: &str, enabled: bool) {
        unsafe {
            atspi_sys::atspi_state_set_set_by_name(self.as_ref().to_glib_none().0, name.to_glib_none().0, enabled.to_glib());
        }
    }
}

impl fmt::Display for StateSet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "StateSet")
    }
}
