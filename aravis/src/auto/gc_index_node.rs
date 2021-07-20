// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ../gir-files
// DO NOT EDIT

use crate::DomElement;
use crate::DomNode;
use crate::GcNode;
use crate::GcPropertyNode;
use glib::object::Cast;
use glib::translate::*;
use std::fmt;
use std::ptr;

glib::wrapper! {
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
			let mut error = ptr::null_mut();
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

impl fmt::Display for GcIndexNode {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.write_str("GcIndexNode")
	}
}
