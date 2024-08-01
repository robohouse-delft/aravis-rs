// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ../gir-files
// DO NOT EDIT

use crate::{ffi, Interface};
use glib::translate::*;

glib::wrapper! {
///
///
/// # Implements
///
/// [`InterfaceExt`][trait@crate::prelude::InterfaceExt], [`trait@glib::ObjectExt`]
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
