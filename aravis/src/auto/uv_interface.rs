// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ../gir-files
// DO NOT EDIT

use crate::Interface;
use glib::translate::*;
use std::fmt;

glib::wrapper! {
	#[doc(alias = "ArvUvInterface")]
	pub struct UvInterface(Object<ffi::ArvUvInterface, ffi::ArvUvInterfaceClass>) @extends Interface;

	match fn {
		type_ => || ffi::arv_uv_interface_get_type(),
	}
}

impl UvInterface {
	/// Gets the unique instance of the GV interface.
	///
	/// # Returns
	///
	/// a [`Interface`][crate::Interface] singleton.
	#[doc(alias = "arv_uv_interface_get_instance")]
	#[doc(alias = "get_instance")]
	pub fn instance() -> Option<Interface> {
		assert_initialized_main_thread!();
		unsafe { from_glib_none(ffi::arv_uv_interface_get_instance()) }
	}
}

unsafe impl Send for UvInterface {}

impl fmt::Display for UvInterface {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.write_str("UvInterface")
	}
}
