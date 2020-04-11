// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use atspi_sys;
use glib;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;
use std::ptr;
use Accessible;

glib_wrapper! {
    pub struct Collection(Interface<atspi_sys::AtspiCollection>);

    match fn {
        get_type => || atspi_sys::atspi_collection_get_type(),
    }
}

pub const NONE_COLLECTION: Option<&Collection> = None;

pub trait CollectionExt: 'static {
    fn get_active_descendant(&self) -> Result<Accessible, glib::Error>;

    //fn get_matches<P: IsA<MatchRule>>(&self, rule: &P, sortby: CollectionSortOrder, count: i32, traverse: bool) -> Result</*Unknown conversion*//*Unimplemented*/Array TypeId { ns_id: 1, id: 17 }, glib::Error>;

    //fn get_matches_from<P: IsA<Accessible>, Q: IsA<MatchRule>>(&self, current_object: &P, rule: &Q, sortby: CollectionSortOrder, tree: CollectionTreeTraversalType, count: i32, traverse: bool) -> Result</*Unknown conversion*//*Unimplemented*/Array TypeId { ns_id: 1, id: 17 }, glib::Error>;

    //fn get_matches_to<P: IsA<Accessible>, Q: IsA<MatchRule>>(&self, current_object: &P, rule: &Q, sortby: CollectionSortOrder, tree: CollectionTreeTraversalType, limit_scope: bool, count: i32, traverse: bool) -> Result</*Unknown conversion*//*Unimplemented*/Array TypeId { ns_id: 1, id: 17 }, glib::Error>;

    fn is_ancestor_of<P: IsA<Accessible>>(&self, test: &P) -> Result<(), glib::Error>;
}

impl<O: IsA<Collection>> CollectionExt for O {
    fn get_active_descendant(&self) -> Result<Accessible, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = atspi_sys::atspi_collection_get_active_descendant(
                self.as_ref().to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    //fn get_matches<P: IsA<MatchRule>>(&self, rule: &P, sortby: CollectionSortOrder, count: i32, traverse: bool) -> Result</*Unknown conversion*//*Unimplemented*/Array TypeId { ns_id: 1, id: 17 }, glib::Error> {
    //    unsafe { TODO: call atspi_sys:atspi_collection_get_matches() }
    //}

    //fn get_matches_from<P: IsA<Accessible>, Q: IsA<MatchRule>>(&self, current_object: &P, rule: &Q, sortby: CollectionSortOrder, tree: CollectionTreeTraversalType, count: i32, traverse: bool) -> Result</*Unknown conversion*//*Unimplemented*/Array TypeId { ns_id: 1, id: 17 }, glib::Error> {
    //    unsafe { TODO: call atspi_sys:atspi_collection_get_matches_from() }
    //}

    //fn get_matches_to<P: IsA<Accessible>, Q: IsA<MatchRule>>(&self, current_object: &P, rule: &Q, sortby: CollectionSortOrder, tree: CollectionTreeTraversalType, limit_scope: bool, count: i32, traverse: bool) -> Result</*Unknown conversion*//*Unimplemented*/Array TypeId { ns_id: 1, id: 17 }, glib::Error> {
    //    unsafe { TODO: call atspi_sys:atspi_collection_get_matches_to() }
    //}

    fn is_ancestor_of<P: IsA<Accessible>>(&self, test: &P) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = atspi_sys::atspi_collection_is_ancestor_of(
                self.as_ref().to_glib_none().0,
                test.as_ref().to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }
}

impl fmt::Display for Collection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Collection")
    }
}
