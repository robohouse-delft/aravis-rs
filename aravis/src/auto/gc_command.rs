// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ../gir-files
// DO NOT EDIT

use crate::{ffi, DomElement, DomNode, GcFeatureNode, GcNode};
use glib::{prelude::*, translate::*};

glib::wrapper! {
///
///
/// # Implements
///
/// [`GcFeatureNodeExt`][trait@crate::prelude::GcFeatureNodeExt], [`GcNodeExt`][trait@crate::prelude::GcNodeExt], [`DomElementExt`][trait@crate::prelude::DomElementExt], [`DomNodeExt`][trait@crate::prelude::DomNodeExt], [`trait@glib::ObjectExt`]
	#[doc(alias = "ArvGcCommand")]
	pub struct GcCommand(Object<ffi::ArvGcCommand, ffi::ArvGcCommandClass>) @extends GcFeatureNode, GcNode, DomElement, DomNode;

	match fn {
		type_ => || ffi::arv_gc_command_get_type(),
	}
}

impl GcCommand {
	#[doc(alias = "arv_gc_command_new")]
	pub fn new() -> GcCommand {
		assert_initialized_main_thread!();
		unsafe { GcNode::from_glib_full(ffi::arv_gc_command_new()).unsafe_cast() }
	}

	#[doc(alias = "arv_gc_command_execute")]
	pub fn execute(&self) -> Result<(), glib::Error> {
		unsafe {
			let mut error = std::ptr::null_mut();
			let _ = ffi::arv_gc_command_execute(self.to_glib_none().0, &mut error);
			if error.is_null() {
				Ok(())
			} else {
				Err(from_glib_full(error))
			}
		}
	}
}

impl Default for GcCommand {
	fn default() -> Self {
		Self::new()
	}
}

unsafe impl Send for GcCommand {}
