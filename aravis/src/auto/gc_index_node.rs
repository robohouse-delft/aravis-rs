// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ../gir-files
// DO NOT EDIT

use crate::{ffi, DomElement, DomNode, GcNode, GcPropertyNode};
use glib::{prelude::*, translate::*};

glib::wrapper! {
///
///
/// # Implements
///
/// [`GcPropertyNodeExt`][trait@crate::prelude::GcPropertyNodeExt], [`GcNodeExt`][trait@crate::prelude::GcNodeExt], [`DomElementExt`][trait@crate::prelude::DomElementExt], [`DomNodeExt`][trait@crate::prelude::DomNodeExt], [`trait@glib::ObjectExt`]
	#[doc(alias = "ArvGcIndexNode")]
	pub struct GcIndexNode(Object<ffi::ArvGcIndexNode, ffi::ArvGcIndexNodeClass>) @extends GcPropertyNode, GcNode, DomElement, DomNode;

	match fn {
		type_ => || ffi::arv_gc_index_node_get_type(),
	}
}

impl GcIndexNode {
	#[doc(alias = "arv_gc_index_node_new")]
	pub fn new() -> GcIndexNode {
		assert_initialized_main_thread!();
		unsafe { GcNode::from_glib_full(ffi::arv_gc_index_node_new()).unsafe_cast() }
	}

	#[doc(alias = "arv_gc_index_node_get_index")]
	#[doc(alias = "get_index")]
	pub fn index(&self, default_offset: i64) -> Result<i64, glib::Error> {
		unsafe {
			let mut error = std::ptr::null_mut();
			let ret =
				ffi::arv_gc_index_node_get_index(self.to_glib_none().0, default_offset, &mut error);
			if error.is_null() {
				Ok(ret)
			} else {
				Err(from_glib_full(error))
			}
		}
	}
}

impl Default for GcIndexNode {
	fn default() -> Self {
		Self::new()
	}
}

unsafe impl Send for GcIndexNode {}
