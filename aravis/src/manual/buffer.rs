use crate::Buffer;

use glib::IsA;
use glib::translate::ToGlibPtr;

use std::ffi::c_void;

pub trait BufferExtManual {
	fn get_data(&self) -> (*mut u8, usize);
}

impl Buffer {
	/// Create an Aravis buffer from a pre-allocated raw buffer.
	///
	/// The created buffer has no registered user data or destroy callback,
	/// so management of the underlying buffer has to be done externally.
	/// The buffer can be identified later when it is returned by a stream only byt the data pointer.
	///
	/// # Safety
	/// The resulting buffer borrows the data, but it carries no lifetime.
	/// The user has to ensure the buffer stays valid.
	pub unsafe fn new_preallocated(data: *mut u8, len: usize) -> Self {
		let buffer = aravis_sys::arv_buffer_new_full(len, data as *mut c_void, std::ptr::null_mut(), None);
		glib::translate::from_glib_full(buffer)
	}
}

impl<T: IsA<Buffer>> BufferExtManual for T {
	fn get_data(&self) -> (*mut u8, usize) {
		unsafe {
			let mut size = 0usize;
			let data = aravis_sys::arv_buffer_get_data(self.as_ref().to_glib_none().0, &mut size as *mut usize);
			(data as *mut u8, size)
		}
	}
}
