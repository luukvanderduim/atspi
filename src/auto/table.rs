// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Accessible;
use crate::Error;
use atspi_sys;
use glib::GString;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct Table(Interface<atspi_sys::AtspiTable>);

    match fn {
        get_type => || atspi_sys::atspi_table_get_type(),
    }
}

pub const NONE_TABLE: Option<&Table> = None;

pub trait TableExt: 'static {
    fn add_column_selection(&self, column: i32) -> Result<(), Error>;

    fn add_row_selection(&self, row: i32) -> Result<(), Error>;

    fn get_accessible_at(&self, row: i32, column: i32) -> Result<Accessible, Error>;

    fn get_caption(&self) -> Result<Accessible, Error>;

    fn get_column_at_index(&self, index: i32) -> Result<i32, Error>;

    fn get_column_description(&self, column: i32) -> Result<GString, Error>;

    fn get_column_extent_at(&self, row: i32, column: i32) -> Result<i32, Error>;

    fn get_column_header(&self, column: i32) -> Result<Accessible, Error>;

    fn get_index_at(&self, row: i32, column: i32) -> Result<i32, Error>;

    fn get_n_columns(&self) -> Result<i32, Error>;

    fn get_n_rows(&self) -> Result<i32, Error>;

    fn get_n_selected_columns(&self) -> Result<i32, Error>;

    fn get_n_selected_rows(&self) -> Result<i32, Error>;

    fn get_row_at_index(&self, index: i32) -> Result<i32, Error>;

    fn get_row_column_extents_at_index(&self, index: i32) -> Result<(i32, i32, i32, i32, bool), Error>;

    fn get_row_description(&self, row: i32) -> Result<GString, Error>;

    fn get_row_extent_at(&self, row: i32, column: i32) -> Result<i32, Error>;

    fn get_row_header(&self, row: i32) -> Result<Accessible, Error>;

    //fn get_selected_columns(&self) -> Result</*Unknown conversion*//*Unimplemented*/Array TypeId { ns_id: 0, id: 14 }, Error>;

    //fn get_selected_rows(&self) -> Result</*Unknown conversion*//*Unimplemented*/Array TypeId { ns_id: 0, id: 14 }, Error>;

    fn get_summary(&self) -> Result<Accessible, Error>;

    fn is_column_selected(&self, column: i32) -> Result<(), Error>;

    fn is_row_selected(&self, row: i32) -> Result<(), Error>;

    fn is_selected(&self, row: i32, column: i32) -> Result<(), Error>;

    fn remove_column_selection(&self, column: i32) -> Result<(), Error>;

    fn remove_row_selection(&self, row: i32) -> Result<(), Error>;
}

impl<O: IsA<Table>> TableExt for O {
    fn add_column_selection(&self, column: i32) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = atspi_sys::atspi_table_add_column_selection(self.as_ref().to_glib_none().0, column, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn add_row_selection(&self, row: i32) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = atspi_sys::atspi_table_add_row_selection(self.as_ref().to_glib_none().0, row, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn get_accessible_at(&self, row: i32, column: i32) -> Result<Accessible, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = atspi_sys::atspi_table_get_accessible_at(self.as_ref().to_glib_none().0, row, column, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    fn get_caption(&self) -> Result<Accessible, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = atspi_sys::atspi_table_get_caption(self.as_ref().to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    fn get_column_at_index(&self, index: i32) -> Result<i32, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = atspi_sys::atspi_table_get_column_at_index(self.as_ref().to_glib_none().0, index, &mut error);
            if error.is_null() { Ok(ret) } else { Err(from_glib_full(error)) }
        }
    }

    fn get_column_description(&self, column: i32) -> Result<GString, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = atspi_sys::atspi_table_get_column_description(self.as_ref().to_glib_none().0, column, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    fn get_column_extent_at(&self, row: i32, column: i32) -> Result<i32, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = atspi_sys::atspi_table_get_column_extent_at(self.as_ref().to_glib_none().0, row, column, &mut error);
            if error.is_null() { Ok(ret) } else { Err(from_glib_full(error)) }
        }
    }

    fn get_column_header(&self, column: i32) -> Result<Accessible, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = atspi_sys::atspi_table_get_column_header(self.as_ref().to_glib_none().0, column, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    fn get_index_at(&self, row: i32, column: i32) -> Result<i32, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = atspi_sys::atspi_table_get_index_at(self.as_ref().to_glib_none().0, row, column, &mut error);
            if error.is_null() { Ok(ret) } else { Err(from_glib_full(error)) }
        }
    }

    fn get_n_columns(&self) -> Result<i32, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = atspi_sys::atspi_table_get_n_columns(self.as_ref().to_glib_none().0, &mut error);
            if error.is_null() { Ok(ret) } else { Err(from_glib_full(error)) }
        }
    }

    fn get_n_rows(&self) -> Result<i32, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = atspi_sys::atspi_table_get_n_rows(self.as_ref().to_glib_none().0, &mut error);
            if error.is_null() { Ok(ret) } else { Err(from_glib_full(error)) }
        }
    }

    fn get_n_selected_columns(&self) -> Result<i32, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = atspi_sys::atspi_table_get_n_selected_columns(self.as_ref().to_glib_none().0, &mut error);
            if error.is_null() { Ok(ret) } else { Err(from_glib_full(error)) }
        }
    }

    fn get_n_selected_rows(&self) -> Result<i32, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = atspi_sys::atspi_table_get_n_selected_rows(self.as_ref().to_glib_none().0, &mut error);
            if error.is_null() { Ok(ret) } else { Err(from_glib_full(error)) }
        }
    }

    fn get_row_at_index(&self, index: i32) -> Result<i32, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = atspi_sys::atspi_table_get_row_at_index(self.as_ref().to_glib_none().0, index, &mut error);
            if error.is_null() { Ok(ret) } else { Err(from_glib_full(error)) }
        }
    }

    fn get_row_column_extents_at_index(&self, index: i32) -> Result<(i32, i32, i32, i32, bool), Error> {
        unsafe {
            let mut row = mem::MaybeUninit::uninit();
            let mut col = mem::MaybeUninit::uninit();
            let mut row_extents = mem::MaybeUninit::uninit();
            let mut col_extents = mem::MaybeUninit::uninit();
            let mut is_selected = mem::MaybeUninit::uninit();
            let mut error = ptr::null_mut();
            let _ = atspi_sys::atspi_table_get_row_column_extents_at_index(self.as_ref().to_glib_none().0, index, row.as_mut_ptr(), col.as_mut_ptr(), row_extents.as_mut_ptr(), col_extents.as_mut_ptr(), is_selected.as_mut_ptr(), &mut error);
            let row = row.assume_init();
            let col = col.assume_init();
            let row_extents = row_extents.assume_init();
            let col_extents = col_extents.assume_init();
            let is_selected = is_selected.assume_init();
            if error.is_null() { Ok((row, col, row_extents, col_extents, from_glib(is_selected))) } else { Err(from_glib_full(error)) }
        }
    }

    fn get_row_description(&self, row: i32) -> Result<GString, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = atspi_sys::atspi_table_get_row_description(self.as_ref().to_glib_none().0, row, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    fn get_row_extent_at(&self, row: i32, column: i32) -> Result<i32, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = atspi_sys::atspi_table_get_row_extent_at(self.as_ref().to_glib_none().0, row, column, &mut error);
            if error.is_null() { Ok(ret) } else { Err(from_glib_full(error)) }
        }
    }

    fn get_row_header(&self, row: i32) -> Result<Accessible, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = atspi_sys::atspi_table_get_row_header(self.as_ref().to_glib_none().0, row, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    //fn get_selected_columns(&self) -> Result</*Unknown conversion*//*Unimplemented*/Array TypeId { ns_id: 0, id: 14 }, Error> {
    //    unsafe { TODO: call atspi_sys:atspi_table_get_selected_columns() }
    //}

    //fn get_selected_rows(&self) -> Result</*Unknown conversion*//*Unimplemented*/Array TypeId { ns_id: 0, id: 14 }, Error> {
    //    unsafe { TODO: call atspi_sys:atspi_table_get_selected_rows() }
    //}

    fn get_summary(&self) -> Result<Accessible, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = atspi_sys::atspi_table_get_summary(self.as_ref().to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    fn is_column_selected(&self, column: i32) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = atspi_sys::atspi_table_is_column_selected(self.as_ref().to_glib_none().0, column, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn is_row_selected(&self, row: i32) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = atspi_sys::atspi_table_is_row_selected(self.as_ref().to_glib_none().0, row, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn is_selected(&self, row: i32, column: i32) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = atspi_sys::atspi_table_is_selected(self.as_ref().to_glib_none().0, row, column, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn remove_column_selection(&self, column: i32) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = atspi_sys::atspi_table_remove_column_selection(self.as_ref().to_glib_none().0, column, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn remove_row_selection(&self, row: i32) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = atspi_sys::atspi_table_remove_row_selection(self.as_ref().to_glib_none().0, row, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }
}

impl fmt::Display for Table {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Table")
    }
}
