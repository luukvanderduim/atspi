// Generated on 2021-03-08
// Re-added self written methods

use glib::translate::*;

glib::wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Event(Boxed<ffi::AtspiEvent>);

    match fn {
        copy => |ptr| glib::gobject_ffi::g_boxed_copy(ffi::atspi_event_get_type(), ptr as *mut _) as *mut ffi::AtspiEvent,
        free => |ptr| glib::gobject_ffi::g_boxed_free(ffi::atspi_event_get_type(), ptr as *mut _),
        get_type => || ffi::atspi_event_get_type(),
    }
}

impl Event {
    #[doc(alias = "atspi_event_main")]
    pub fn main() {
        assert_initialized_main_thread!();
        unsafe {
            ffi::atspi_event_main();
        }
    }

    #[doc(alias = "atspi_event_quit")]
    pub fn quit() {
        assert_initialized_main_thread!();
        unsafe {
            ffi::atspi_event_quit();
        }
    }


      // FIXME This is likely incorrect
      pub fn get_source(&self) -> Option<super::Accessible> {
        assert_initialized_main_thread!(); // Needed?

        unsafe {
            if !(self.0).source.is_null() {
                Some(from_glib_none((self.0).source) )
            } else {
                None
            }
        }
    }

    pub fn get_detail1(&self) -> i32 {
        assert_initialized_main_thread!(); // Needed?
        self.0.detail1
    }

    pub fn get_detail2(&self) -> i32 {
        assert_initialized_main_thread!(); // Needed?
        self.0.detail2
    }

    /// In the event struct 'sender' is a v2.34> feature.
    /// Accessibility client programs may want to discern between the cause of events.
    /// Did the client itself cause the event or was this an externally caused event?
    /// While 'source' points to the Accessible that sent the Event,
    ///
    /// 'sender' will point to 'source' event, unless the event is invoked
    /// by the accessibility client. Then sender will point to the a11y
    /// client.
    ///

    // FIXME This is likely incorrect
    pub fn get_sender(self) -> Option<crate::Accessible> {
        assert_initialized_main_thread!(); // Needed?

        unsafe {
            if !(self.0).sender.is_null() {
                Some(from_glib_none((self.0).sender))
            } else {
                None
            }
        }
    }
}
