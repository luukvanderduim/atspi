// This file was generated by gir (https://github.com/gtk-rs/gir @ 7cca816)
// from gir-files (https://github.com/gtk-rs/gir-files @ 02dfee2)
// DO NOT EDIT

use Accessible;
use RelationType;
use atspi_sys;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;

glib_wrapper! {
    pub struct Relation(Object<atspi_sys::AtspiRelation, atspi_sys::AtspiRelationClass, RelationClass>);

    match fn {
        get_type => || atspi_sys::atspi_relation_get_type(),
    }
}

pub const NONE_RELATION: Option<&Relation> = None;

pub trait RelationExt: 'static {
    fn get_n_targets(&self) -> i32;

    fn get_relation_type(&self) -> RelationType;

    fn get_target(&self, i: i32) -> Option<Accessible>;
}

impl<O: IsA<Relation>> RelationExt for O {
    fn get_n_targets(&self) -> i32 {
        unsafe {
            atspi_sys::atspi_relation_get_n_targets(self.as_ref().to_glib_none().0)
        }
    }

    fn get_relation_type(&self) -> RelationType {
        unsafe {
            from_glib(atspi_sys::atspi_relation_get_relation_type(self.as_ref().to_glib_none().0))
        }
    }

    fn get_target(&self, i: i32) -> Option<Accessible> {
        unsafe {
            from_glib_full(atspi_sys::atspi_relation_get_target(self.as_ref().to_glib_none().0, i))
        }
    }
}

impl fmt::Display for Relation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Relation")
    }
}
