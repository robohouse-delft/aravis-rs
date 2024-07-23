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
	#[doc(alias = "ArvGvInterface")]
	pub struct GvInterface(Object<ffi::ArvGvInterface, ffi::ArvGvInterfaceClass>) @extends Interface;

	match fn {
		type_ => || ffi::arv_gv_interface_get_type(),
	}
}

impl GvInterface {
/// Gets the unique instance of the GV interface.
///
/// # Returns
///
/// a [`Interface`][crate::Interface] singleton.
	#[doc(alias = "arv_gv_interface_get_instance")]
	#[doc(alias = "get_instance")]
	pub fn instance() -> Option<Interface> {
		assert_initialized_main_thread!();
		unsafe { from_glib_none(ffi::arv_gv_interface_get_instance()) }
	}
}

unsafe impl Send for GvInterface {}
