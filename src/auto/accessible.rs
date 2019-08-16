// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Action;
use Cache;
use Collection;
use Component;
use Document;
use EditableText;
use Error;
use Hyperlink;
use Hypertext;
use Image;
use Object;
use Role;
use Selection;
use StateSet;
use Table;
use TableCell;
use Text;
use Value;
use atspi_sys;
use glib::GString;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;
use std::ptr;

glib_wrapper! {
    pub struct Accessible(Object<atspi_sys::AtspiAccessible, atspi_sys::AtspiAccessibleClass, AccessibleClass>) @extends Object, @implements Action, Collection, Component, Document, EditableText, Hypertext, Image, Selection, Table, TableCell, Text, Value;

    match fn {
        get_type => || atspi_sys::atspi_accessible_get_type(),
    }
}

pub const NONE_ACCESSIBLE: Option<&Accessible> = None;

pub trait AccessibleExt: 'static {
    fn clear_cache(&self);

    fn get_action_iface(&self) -> Option<Action>;

    fn get_application(&self) -> Result<Accessible, Error>;

    fn get_atspi_version(&self) -> Result<GString, Error>;

    //fn get_attributes(&self) -> Result</*Unknown conversion*//*Unimplemented*/HashTable TypeId { ns_id: 0, id: 28 }/TypeId { ns_id: 0, id: 28 }, Error>;

    //fn get_attributes_as_array(&self) -> Result</*Unknown conversion*//*Unimplemented*/Array TypeId { ns_id: 0, id: 28 }, Error>;

    fn get_child_at_index(&self, child_index: i32) -> Result<Accessible, Error>;

    fn get_child_count(&self) -> Result<i32, Error>;

    fn get_collection_iface(&self) -> Option<Collection>;

    fn get_component_iface(&self) -> Option<Component>;

    fn get_description(&self) -> Result<GString, Error>;

    fn get_document_iface(&self) -> Option<Document>;

    fn get_editable_text_iface(&self) -> Option<EditableText>;

    fn get_hyperlink(&self) -> Option<Hyperlink>;

    fn get_hypertext_iface(&self) -> Option<Hypertext>;

    fn get_id(&self) -> Result<i32, Error>;

    fn get_image_iface(&self) -> Option<Image>;

    fn get_index_in_parent(&self) -> Result<i32, Error>;

    //fn get_interfaces(&self) -> /*Unknown conversion*//*Unimplemented*/Array TypeId { ns_id: 0, id: 28 };

    fn get_localized_role_name(&self) -> Result<GString, Error>;

    fn get_name(&self) -> Result<GString, Error>;

    fn get_object_locale(&self) -> Result<GString, Error>;

    fn get_parent(&self) -> Result<Option<Accessible>, Error>;

    fn get_process_id(&self) -> Result<(), Error>;

    //fn get_relation_set(&self) -> Result</*Unknown conversion*//*Unimplemented*/Array TypeId { ns_id: 1, id: 19 }, Error>;

    fn get_role(&self) -> Result<Role, Error>;

    fn get_role_name(&self) -> Result<GString, Error>;

    fn get_selection_iface(&self) -> Option<Selection>;

    fn get_state_set(&self) -> Option<StateSet>;

    fn get_table_cell(&self) -> Option<TableCell>;

    fn get_table_iface(&self) -> Option<Table>;

    fn get_text_iface(&self) -> Option<Text>;

    fn get_toolkit_name(&self) -> Result<GString, Error>;

    fn get_toolkit_version(&self) -> Result<GString, Error>;

    fn get_value_iface(&self) -> Option<Value>;

    fn set_cache_mask(&self, mask: Cache);
}

impl<O: IsA<Accessible>> AccessibleExt for O {
    fn clear_cache(&self) {
        unsafe {
            atspi_sys::atspi_accessible_clear_cache(self.as_ref().to_glib_none().0);
        }
    }

    fn get_action_iface(&self) -> Option<Action> {
        unsafe {
            from_glib_full(atspi_sys::atspi_accessible_get_action_iface(self.as_ref().to_glib_none().0))
        }
    }

    fn get_application(&self) -> Result<Accessible, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = atspi_sys::atspi_accessible_get_application(self.as_ref().to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    fn get_atspi_version(&self) -> Result<GString, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = atspi_sys::atspi_accessible_get_atspi_version(self.as_ref().to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    //fn get_attributes(&self) -> Result</*Unknown conversion*//*Unimplemented*/HashTable TypeId { ns_id: 0, id: 28 }/TypeId { ns_id: 0, id: 28 }, Error> {
    //    unsafe { TODO: call atspi_sys:atspi_accessible_get_attributes() }
    //}

    //fn get_attributes_as_array(&self) -> Result</*Unknown conversion*//*Unimplemented*/Array TypeId { ns_id: 0, id: 28 }, Error> {
    //    unsafe { TODO: call atspi_sys:atspi_accessible_get_attributes_as_array() }
    //}

    fn get_child_at_index(&self, child_index: i32) -> Result<Accessible, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = atspi_sys::atspi_accessible_get_child_at_index(self.as_ref().to_glib_none().0, child_index, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    fn get_child_count(&self) -> Result<i32, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = atspi_sys::atspi_accessible_get_child_count(self.as_ref().to_glib_none().0, &mut error);
            if error.is_null() { Ok(ret) } else { Err(from_glib_full(error)) }
        }
    }

    fn get_collection_iface(&self) -> Option<Collection> {
        unsafe {
            from_glib_full(atspi_sys::atspi_accessible_get_collection_iface(self.as_ref().to_glib_none().0))
        }
    }

    fn get_component_iface(&self) -> Option<Component> {
        unsafe {
            from_glib_full(atspi_sys::atspi_accessible_get_component_iface(self.as_ref().to_glib_none().0))
        }
    }

    fn get_description(&self) -> Result<GString, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = atspi_sys::atspi_accessible_get_description(self.as_ref().to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    fn get_document_iface(&self) -> Option<Document> {
        unsafe {
            from_glib_full(atspi_sys::atspi_accessible_get_document_iface(self.as_ref().to_glib_none().0))
        }
    }

    fn get_editable_text_iface(&self) -> Option<EditableText> {
        unsafe {
            from_glib_full(atspi_sys::atspi_accessible_get_editable_text_iface(self.as_ref().to_glib_none().0))
        }
    }

    fn get_hyperlink(&self) -> Option<Hyperlink> {
        unsafe {
            from_glib_full(atspi_sys::atspi_accessible_get_hyperlink(self.as_ref().to_glib_none().0))
        }
    }

    fn get_hypertext_iface(&self) -> Option<Hypertext> {
        unsafe {
            from_glib_full(atspi_sys::atspi_accessible_get_hypertext_iface(self.as_ref().to_glib_none().0))
        }
    }

    fn get_id(&self) -> Result<i32, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = atspi_sys::atspi_accessible_get_id(self.as_ref().to_glib_none().0, &mut error);
            if error.is_null() { Ok(ret) } else { Err(from_glib_full(error)) }
        }
    }

    fn get_image_iface(&self) -> Option<Image> {
        unsafe {
            from_glib_full(atspi_sys::atspi_accessible_get_image_iface(self.as_ref().to_glib_none().0))
        }
    }

    fn get_index_in_parent(&self) -> Result<i32, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = atspi_sys::atspi_accessible_get_index_in_parent(self.as_ref().to_glib_none().0, &mut error);
            if error.is_null() { Ok(ret) } else { Err(from_glib_full(error)) }
        }
    }

    //fn get_interfaces(&self) -> /*Unknown conversion*//*Unimplemented*/Array TypeId { ns_id: 0, id: 28 } {
    //    unsafe { TODO: call atspi_sys:atspi_accessible_get_interfaces() }
    //}

    fn get_localized_role_name(&self) -> Result<GString, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = atspi_sys::atspi_accessible_get_localized_role_name(self.as_ref().to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    fn get_name(&self) -> Result<GString, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = atspi_sys::atspi_accessible_get_name(self.as_ref().to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    fn get_object_locale(&self) -> Result<GString, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = atspi_sys::atspi_accessible_get_object_locale(self.as_ref().to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_none(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    fn get_parent(&self) -> Result<Option<Accessible>, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = atspi_sys::atspi_accessible_get_parent(self.as_ref().to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    fn get_process_id(&self) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = atspi_sys::atspi_accessible_get_process_id(self.as_ref().to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    //fn get_relation_set(&self) -> Result</*Unknown conversion*//*Unimplemented*/Array TypeId { ns_id: 1, id: 19 }, Error> {
    //    unsafe { TODO: call atspi_sys:atspi_accessible_get_relation_set() }
    //}

    fn get_role(&self) -> Result<Role, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = atspi_sys::atspi_accessible_get_role(self.as_ref().to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    fn get_role_name(&self) -> Result<GString, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = atspi_sys::atspi_accessible_get_role_name(self.as_ref().to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    fn get_selection_iface(&self) -> Option<Selection> {
        unsafe {
            from_glib_full(atspi_sys::atspi_accessible_get_selection_iface(self.as_ref().to_glib_none().0))
        }
    }

    fn get_state_set(&self) -> Option<StateSet> {
        unsafe {
            from_glib_full(atspi_sys::atspi_accessible_get_state_set(self.as_ref().to_glib_none().0))
        }
    }

    fn get_table_cell(&self) -> Option<TableCell> {
        unsafe {
            from_glib_full(atspi_sys::atspi_accessible_get_table_cell(self.as_ref().to_glib_none().0))
        }
    }

    fn get_table_iface(&self) -> Option<Table> {
        unsafe {
            from_glib_full(atspi_sys::atspi_accessible_get_table_iface(self.as_ref().to_glib_none().0))
        }
    }

    fn get_text_iface(&self) -> Option<Text> {
        unsafe {
            from_glib_full(atspi_sys::atspi_accessible_get_text_iface(self.as_ref().to_glib_none().0))
        }
    }

    fn get_toolkit_name(&self) -> Result<GString, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = atspi_sys::atspi_accessible_get_toolkit_name(self.as_ref().to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    fn get_toolkit_version(&self) -> Result<GString, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = atspi_sys::atspi_accessible_get_toolkit_version(self.as_ref().to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    fn get_value_iface(&self) -> Option<Value> {
        unsafe {
            from_glib_full(atspi_sys::atspi_accessible_get_value_iface(self.as_ref().to_glib_none().0))
        }
    }

    fn set_cache_mask(&self, mask: Cache) {
        unsafe {
            atspi_sys::atspi_accessible_set_cache_mask(self.as_ref().to_glib_none().0, mask.to_glib());
        }
    }
}

impl fmt::Display for Accessible {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Accessible")
    }
}
