extern crate aravis_sys;
extern crate gio;
extern crate gio_sys;
#[macro_use]
extern crate glib;
extern crate glib_sys;
extern crate gobject_sys;

use std::ffi::CString;

macro_rules! assert_initialized_main_thread {
    () => {};
}

/// No-op.
macro_rules! skip_assert_initialized {
    () => {};
}

mod auto;
pub use auto::*;

#[derive(Debug)]
pub struct DeviceInfo {
    pub id: CString,
    pub physical_id: CString,
    pub vendor: CString,
    pub model: CString,
    pub protocol: CString,
    pub address: CString,
}

pub unsafe fn get_device_list() -> Vec<DeviceInfo> {
    aravis_sys::arv_update_device_list();
    let count = aravis_sys::arv_get_n_devices();

    (0..count).map(|i| DeviceInfo {
        id:          std::ffi::CStr::from_ptr(aravis_sys::arv_get_device_id(i)).into(),
        physical_id: std::ffi::CStr::from_ptr(aravis_sys::arv_get_device_physical_id(i)).into(),
        vendor:      std::ffi::CStr::from_ptr(aravis_sys::arv_get_device_vendor(i)).into(),
        model:       std::ffi::CStr::from_ptr(aravis_sys::arv_get_device_model(i)).into(),
        protocol:    std::ffi::CStr::from_ptr(aravis_sys::arv_get_device_protocol(i)).into(),
        address:     std::ffi::CStr::from_ptr(aravis_sys::arv_get_device_address(i)).into(),
    }).collect()
}
