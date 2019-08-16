// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Error;
use Range;
#[cfg(any(feature = "v2_9_90", feature = "dox"))]
use TextGranularity;
#[cfg(any(feature = "v2_9_90", feature = "dox"))]
use TextRange;
use atspi_sys;
use glib::GString;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;
use std::ptr;

glib_wrapper! {
    pub struct Text(Interface<atspi_sys::AtspiText>);

    match fn {
        get_type => || atspi_sys::atspi_text_get_type(),
    }
}

pub const NONE_TEXT: Option<&Text> = None;

pub trait TextExt: 'static {
    fn add_selection(&self, start_offset: i32, end_offset: i32) -> Result<(), Error>;

    //fn get_attribute_run(&self, offset: i32, include_defaults: bool) -> Result<(/*Unknown conversion*//*Unimplemented*/HashTable TypeId { ns_id: 0, id: 28 }/TypeId { ns_id: 0, id: 28 }, i32, i32), Error>;

    #[cfg_attr(feature = "v2_10", deprecated)]
    fn get_attribute_value(&self, offset: i32, attribute_name: &str) -> Result<Option<GString>, Error>;

    //#[cfg_attr(feature = "v2_10", deprecated)]
    //fn get_attributes(&self, offset: i32) -> Result<(/*Unknown conversion*//*Unimplemented*/HashTable TypeId { ns_id: 0, id: 28 }/TypeId { ns_id: 0, id: 28 }, i32, i32), Error>;

    //fn get_bounded_ranges(&self, x: i32, y: i32, width: i32, height: i32, type_: /*Ignored*/CoordType, clipTypeX: /*Ignored*/TextClipType, clipTypeY: /*Ignored*/TextClipType) -> Result</*Unknown conversion*//*Unimplemented*/Array TypeId { ns_id: 1, id: 63 }, Error>;

    fn get_caret_offset(&self) -> Result<i32, Error>;

    fn get_character_at_offset(&self, offset: i32) -> Result<(), Error>;

    fn get_character_count(&self) -> Result<i32, Error>;

    //fn get_character_extents(&self, offset: i32, type_: /*Ignored*/CoordType) -> Result<Rect, Error>;

    //fn get_default_attributes(&self) -> Result</*Unknown conversion*//*Unimplemented*/HashTable TypeId { ns_id: 0, id: 28 }/TypeId { ns_id: 0, id: 28 }, Error>;

    fn get_n_selections(&self) -> Result<i32, Error>;

    //fn get_offset_at_point(&self, x: i32, y: i32, type_: /*Ignored*/CoordType) -> Result<i32, Error>;

    //fn get_range_extents(&self, start_offset: i32, end_offset: i32, type_: /*Ignored*/CoordType) -> Result<Rect, Error>;

    fn get_selection(&self, selection_num: i32) -> Result<Range, Error>;

    #[cfg(any(feature = "v2_9_90", feature = "dox"))]
    fn get_string_at_offset(&self, offset: i32, granularity: TextGranularity) -> Result<TextRange, Error>;

    fn get_text(&self, start_offset: i32, end_offset: i32) -> Result<GString, Error>;

    //fn get_text_after_offset(&self, offset: i32, type_: /*Ignored*/TextBoundaryType) -> Result<TextRange, Error>;

    //#[cfg_attr(feature = "v2_10", deprecated)]
    //fn get_text_at_offset(&self, offset: i32, type_: /*Ignored*/TextBoundaryType) -> Result<TextRange, Error>;

    fn get_text_attribute_value(&self, offset: i32, attribute_name: &str) -> Result<Option<GString>, Error>;

    //fn get_text_attributes(&self, offset: i32) -> Result<(/*Unknown conversion*//*Unimplemented*/HashTable TypeId { ns_id: 0, id: 28 }/TypeId { ns_id: 0, id: 28 }, i32, i32), Error>;

    //fn get_text_before_offset(&self, offset: i32, type_: /*Ignored*/TextBoundaryType) -> Result<TextRange, Error>;

    fn remove_selection(&self, selection_num: i32) -> Result<(), Error>;

    fn set_caret_offset(&self, new_offset: i32) -> Result<(), Error>;

    fn set_selection(&self, selection_num: i32, start_offset: i32, end_offset: i32) -> Result<(), Error>;
}

impl<O: IsA<Text>> TextExt for O {
    fn add_selection(&self, start_offset: i32, end_offset: i32) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = atspi_sys::atspi_text_add_selection(self.as_ref().to_glib_none().0, start_offset, end_offset, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    //fn get_attribute_run(&self, offset: i32, include_defaults: bool) -> Result<(/*Unknown conversion*//*Unimplemented*/HashTable TypeId { ns_id: 0, id: 28 }/TypeId { ns_id: 0, id: 28 }, i32, i32), Error> {
    //    unsafe { TODO: call atspi_sys:atspi_text_get_attribute_run() }
    //}

    fn get_attribute_value(&self, offset: i32, attribute_name: &str) -> Result<Option<GString>, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = atspi_sys::atspi_text_get_attribute_value(self.as_ref().to_glib_none().0, offset, attribute_name.to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    //fn get_attributes(&self, offset: i32) -> Result<(/*Unknown conversion*//*Unimplemented*/HashTable TypeId { ns_id: 0, id: 28 }/TypeId { ns_id: 0, id: 28 }, i32, i32), Error> {
    //    unsafe { TODO: call atspi_sys:atspi_text_get_attributes() }
    //}

    //fn get_bounded_ranges(&self, x: i32, y: i32, width: i32, height: i32, type_: /*Ignored*/CoordType, clipTypeX: /*Ignored*/TextClipType, clipTypeY: /*Ignored*/TextClipType) -> Result</*Unknown conversion*//*Unimplemented*/Array TypeId { ns_id: 1, id: 63 }, Error> {
    //    unsafe { TODO: call atspi_sys:atspi_text_get_bounded_ranges() }
    //}

    fn get_caret_offset(&self) -> Result<i32, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = atspi_sys::atspi_text_get_caret_offset(self.as_ref().to_glib_none().0, &mut error);
            if error.is_null() { Ok(ret) } else { Err(from_glib_full(error)) }
        }
    }

    fn get_character_at_offset(&self, offset: i32) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = atspi_sys::atspi_text_get_character_at_offset(self.as_ref().to_glib_none().0, offset, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn get_character_count(&self) -> Result<i32, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = atspi_sys::atspi_text_get_character_count(self.as_ref().to_glib_none().0, &mut error);
            if error.is_null() { Ok(ret) } else { Err(from_glib_full(error)) }
        }
    }

    //fn get_character_extents(&self, offset: i32, type_: /*Ignored*/CoordType) -> Result<Rect, Error> {
    //    unsafe { TODO: call atspi_sys:atspi_text_get_character_extents() }
    //}

    //fn get_default_attributes(&self) -> Result</*Unknown conversion*//*Unimplemented*/HashTable TypeId { ns_id: 0, id: 28 }/TypeId { ns_id: 0, id: 28 }, Error> {
    //    unsafe { TODO: call atspi_sys:atspi_text_get_default_attributes() }
    //}

    fn get_n_selections(&self) -> Result<i32, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = atspi_sys::atspi_text_get_n_selections(self.as_ref().to_glib_none().0, &mut error);
            if error.is_null() { Ok(ret) } else { Err(from_glib_full(error)) }
        }
    }

    //fn get_offset_at_point(&self, x: i32, y: i32, type_: /*Ignored*/CoordType) -> Result<i32, Error> {
    //    unsafe { TODO: call atspi_sys:atspi_text_get_offset_at_point() }
    //}

    //fn get_range_extents(&self, start_offset: i32, end_offset: i32, type_: /*Ignored*/CoordType) -> Result<Rect, Error> {
    //    unsafe { TODO: call atspi_sys:atspi_text_get_range_extents() }
    //}

    fn get_selection(&self, selection_num: i32) -> Result<Range, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = atspi_sys::atspi_text_get_selection(self.as_ref().to_glib_none().0, selection_num, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    #[cfg(any(feature = "v2_9_90", feature = "dox"))]
    fn get_string_at_offset(&self, offset: i32, granularity: TextGranularity) -> Result<TextRange, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = atspi_sys::atspi_text_get_string_at_offset(self.as_ref().to_glib_none().0, offset, granularity.to_glib(), &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    fn get_text(&self, start_offset: i32, end_offset: i32) -> Result<GString, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = atspi_sys::atspi_text_get_text(self.as_ref().to_glib_none().0, start_offset, end_offset, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    //fn get_text_after_offset(&self, offset: i32, type_: /*Ignored*/TextBoundaryType) -> Result<TextRange, Error> {
    //    unsafe { TODO: call atspi_sys:atspi_text_get_text_after_offset() }
    //}

    //fn get_text_at_offset(&self, offset: i32, type_: /*Ignored*/TextBoundaryType) -> Result<TextRange, Error> {
    //    unsafe { TODO: call atspi_sys:atspi_text_get_text_at_offset() }
    //}

    fn get_text_attribute_value(&self, offset: i32, attribute_name: &str) -> Result<Option<GString>, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = atspi_sys::atspi_text_get_text_attribute_value(self.as_ref().to_glib_none().0, offset, attribute_name.to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    //fn get_text_attributes(&self, offset: i32) -> Result<(/*Unknown conversion*//*Unimplemented*/HashTable TypeId { ns_id: 0, id: 28 }/TypeId { ns_id: 0, id: 28 }, i32, i32), Error> {
    //    unsafe { TODO: call atspi_sys:atspi_text_get_text_attributes() }
    //}

    //fn get_text_before_offset(&self, offset: i32, type_: /*Ignored*/TextBoundaryType) -> Result<TextRange, Error> {
    //    unsafe { TODO: call atspi_sys:atspi_text_get_text_before_offset() }
    //}

    fn remove_selection(&self, selection_num: i32) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = atspi_sys::atspi_text_remove_selection(self.as_ref().to_glib_none().0, selection_num, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn set_caret_offset(&self, new_offset: i32) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = atspi_sys::atspi_text_set_caret_offset(self.as_ref().to_glib_none().0, new_offset, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn set_selection(&self, selection_num: i32, start_offset: i32, end_offset: i32) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = atspi_sys::atspi_text_set_selection(self.as_ref().to_glib_none().0, selection_num, start_offset, end_offset, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }
}

impl fmt::Display for Text {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Text")
    }
}
