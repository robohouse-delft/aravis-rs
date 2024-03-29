// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ../gir-files
// DO NOT EDIT

use crate::DomElement;
use crate::DomNode;
use crate::GcFeatureNode;
use crate::GcInteger;
use crate::GcNode;
use crate::GcSelector;
use glib::object::Cast;
use glib::translate::*;
use std::fmt;

glib::wrapper! {
	#[doc(alias = "ArvGcIntegerNode")]
	pub struct GcIntegerNode(Object<ffi::ArvGcIntegerNode, ffi::ArvGcIntegerNodeClass>) @extends GcFeatureNode, GcNode, DomElement, DomNode, @implements GcInteger, GcSelector;

	match fn {
		type_ => || ffi::arv_gc_integer_node_get_type(),
	}
}

impl GcIntegerNode {
	#[doc(alias = "arv_gc_integer_node_new")]
	pub fn new() -> GcIntegerNode {
		assert_initialized_main_thread!();
		unsafe { GcNode::from_glib_full(ffi::arv_gc_integer_node_new()).unsafe_cast() }
	}
}

impl Default for GcIntegerNode {
	fn default() -> Self {
		Self::new()
	}
}

unsafe impl Send for GcIntegerNode {}

impl fmt::Display for GcIntegerNode {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.write_str("GcIntegerNode")
	}
}
