// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ../gir-files
// DO NOT EDIT

use crate::{ffi, Device, FakeCamera};
use glib::{prelude::*, translate::*};

glib::wrapper! {
///
///
/// ## Properties
///
///
/// #### `serial-number`
///  Writeable | Construct Only
///
/// # Implements
///
/// [`DeviceExt`][trait@crate::prelude::DeviceExt], [`trait@glib::ObjectExt`]
	#[doc(alias = "ArvFakeDevice")]
	pub struct FakeDevice(Object<ffi::ArvFakeDevice, ffi::ArvFakeDeviceClass>) @extends Device;

	match fn {
		type_ => || ffi::arv_fake_device_get_type(),
	}
}

impl FakeDevice {
	/// ## `serial_number`
	/// fake device serial number
	///
	/// # Returns
	///
	/// a newly created [`Device`][crate::Device] simulating a real device
	#[doc(alias = "arv_fake_device_new")]
	pub fn new(serial_number: &str) -> Result<FakeDevice, glib::Error> {
		assert_initialized_main_thread!();
		unsafe {
			let mut error = std::ptr::null_mut();
			let ret = ffi::arv_fake_device_new(serial_number.to_glib_none().0, &mut error);
			if error.is_null() {
				Ok(Device::from_glib_full(ret).unsafe_cast())
			} else {
				Err(from_glib_full(error))
			}
		}
	}

	///
	/// # Returns
	///
	/// the [`FakeCamera`][crate::FakeCamera] used by this device instance.
	#[doc(alias = "arv_fake_device_get_fake_camera")]
	#[doc(alias = "get_fake_camera")]
	pub fn fake_camera(&self) -> Option<FakeCamera> {
		unsafe { from_glib_none(ffi::arv_fake_device_get_fake_camera(self.to_glib_none().0)) }
	}
}

unsafe impl Send for FakeDevice {}
