use crate::Buffer;

use std::ffi::c_void;

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
	pub unsafe fn new_preallocated(data: *mut c_void, len: usize) -> Self {
		let buffer = aravis_sys::arv_buffer_new_full(len, data, std::ptr::null_mut(), None);
		glib::translate::from_glib_full(buffer)
	}
}
