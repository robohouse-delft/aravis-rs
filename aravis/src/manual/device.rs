use crate::{Device, Stream};

use glib::IsA;
use glib::translate::ToGlibPtr;

pub trait DeviceExtManual {
	fn create_stream(&self) -> Stream;
}

impl<T: IsA<Device>> DeviceExtManual for T {
	fn create_stream(&self) -> Stream {
		unsafe {
			let stream = aravis_sys::arv_device_create_stream(self.as_ref().to_glib_none().0, None, std::ptr::null_mut());
			glib::translate::from_glib_full(stream)
		}
	}
}
