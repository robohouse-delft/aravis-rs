use crate::{Device, Stream};

use glib::translate::ToGlibPtr;
use glib::IsA;

pub(crate) mod traits {
	use super::*;

	/// Trait containing additional [`Device`] methods.
	pub trait DeviceExtManual {
		fn create_stream(&self) -> Result<Stream, glib::Error>;
	}
}

impl<T: IsA<Device>> traits::DeviceExtManual for T {
	fn create_stream(&self) -> Result<Stream, glib::Error> {
		unsafe {
			let mut error = std::ptr::null_mut();
			let stream = aravis_sys::arv_device_create_stream(
				self.as_ref().to_glib_none().0,
				None,
				std::ptr::null_mut(),
				&mut error,
			);
			if error.is_null() {
				Ok(glib::translate::from_glib_full(stream))
			} else {
				Err(glib::translate::from_glib_full(error))
			}
		}
	}
}
