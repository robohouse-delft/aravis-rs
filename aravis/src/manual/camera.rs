use crate::{Camera, Stream};

use glib::translate::ToGlibPtr;
use glib::IsA;

pub trait CameraExtManual {
	fn create_stream(&self) -> Stream;
}

impl<T: IsA<Camera>> CameraExtManual for T {
	fn create_stream(&self) -> Stream {
		unsafe {
			let stream = aravis_sys::arv_camera_create_stream(
				self.as_ref().to_glib_none().0,
				None,
				std::ptr::null_mut(),
			);
			glib::translate::from_glib_full(stream)
		}
	}
}
