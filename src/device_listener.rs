use atspi_sys;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;

glib_wrapper! {
    pub struct DeviceListener(Object<atspi_sys::AtspiDeviceListener, atspi_sys::AtspiDeviceListenerClass, DeviceListenerClass>);

    match fn {
        get_type => || atspi_sys::atspi_device_listener_get_type(),
    }
}

impl DeviceListener {
    //pub fn new<P: Fn(&DeviceEvent) -> bool + 'static>(callback: P, user_data: Option<Fundamental: Pointer>) -> DeviceListener {
    //    unsafe { TODO: call atspi_sys:atspi_device_listener_new() }
    //}

    //pub fn new_simple<P: Fn(&DeviceEvent) -> bool + 'static>(callback: P) -> DeviceListener {
    //    unsafe { TODO: call atspi_sys:atspi_device_listener_new_simple() }
    //}
}

pub const NONE_DEVICE_LISTENER: Option<&DeviceListener> = None;

pub trait DeviceListenerExt: 'static {
    //fn add_callback<P: Fn(&DeviceEvent) -> bool + 'static>(&self, callback: P, user_data: /*Unimplemented*/Option<Fundamental: Pointer>);

    //fn remove_callback<P: FnMut(&DeviceEvent) -> bool>(&self, callback: P);
}

impl<O: IsA<DeviceListener>> DeviceListenerExt for O {
    //fn add_callback<P: Fn(&DeviceEvent) -> bool + 'static>(&self, callback: P, user_data: /*Unimplemented*/Option<Fundamental: Pointer>) {
    //    unsafe { TODO: call atspi_sys:atspi_device_listener_add_callback() }
    //}

    //fn remove_callback<P: FnMut(&DeviceEvent) -> bool>(&self, callback: P) {
    //    unsafe { TODO: call atspi_sys:atspi_device_listener_remove_callback() }
    //}
}

impl fmt::Display for DeviceListener {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DeviceListener")
    }
}
