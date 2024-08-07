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
	#[doc(alias = "ArvGcBoolean")]
	pub struct GcBoolean(Object<ffi::ArvGcBoolean, ffi::ArvGcBooleanClass>) @extends GcFeatureNode, GcNode, DomElement, DomNode;

	match fn {
		type_ => || ffi::arv_gc_boolean_get_type(),
	}
}

impl GcBoolean {
	#[doc(alias = "arv_gc_boolean_new")]
	pub fn new() -> GcBoolean {
		assert_initialized_main_thread!();
		unsafe { GcNode::from_glib_full(ffi::arv_gc_boolean_new()).unsafe_cast() }
	}

	///
	/// # Returns
	///
	/// the feature value.
	#[doc(alias = "arv_gc_boolean_get_value")]
	#[doc(alias = "get_value")]
	pub fn value(&self) -> Result<bool, glib::Error> {
		unsafe {
			let mut error = std::ptr::null_mut();
			let ret = ffi::arv_gc_boolean_get_value(self.to_glib_none().0, &mut error);
			if error.is_null() {
				Ok(from_glib(ret))
			} else {
				Err(from_glib_full(error))
			}
		}
	}

	#[doc(alias = "arv_gc_boolean_set_value")]
	pub fn set_value(&self, v_boolean: bool) -> Result<(), glib::Error> {
		unsafe {
			let mut error = std::ptr::null_mut();
			let _ = ffi::arv_gc_boolean_set_value(
				self.to_glib_none().0,
				v_boolean.into_glib(),
				&mut error,
			);
			if error.is_null() {
				Ok(())
			} else {
				Err(from_glib_full(error))
			}
		}
	}
}

impl Default for GcBoolean {
	fn default() -> Self {
		Self::new()
	}
}

unsafe impl Send for GcBoolean {}
