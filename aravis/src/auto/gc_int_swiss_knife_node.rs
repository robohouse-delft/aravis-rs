// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ../gir-files
// DO NOT EDIT

use crate::DomElement;
use crate::DomNode;
use crate::GcFeatureNode;
use crate::GcInteger;
use crate::GcNode;
use crate::GcSwissKnife;
use glib::object::Cast;
use glib::translate::*;
use std::fmt;

glib::wrapper! {
	#[doc(alias = "ArvGcIntSwissKnifeNode")]
	pub struct GcIntSwissKnifeNode(Object<ffi::ArvGcIntSwissKnifeNode, ffi::ArvGcIntSwissKnifeNodeClass>) @extends GcSwissKnife, GcFeatureNode, GcNode, DomElement, DomNode, @implements GcInteger;

	match fn {
		type_ => || ffi::arv_gc_int_swiss_knife_node_get_type(),
	}
}

impl GcIntSwissKnifeNode {
	#[doc(alias = "arv_gc_int_swiss_knife_node_new")]
	pub fn new() -> GcIntSwissKnifeNode {
		assert_initialized_main_thread!();
		unsafe { GcNode::from_glib_full(ffi::arv_gc_int_swiss_knife_node_new()).unsafe_cast() }
	}
}

impl Default for GcIntSwissKnifeNode {
	fn default() -> Self {
		Self::new()
	}
}

unsafe impl Send for GcIntSwissKnifeNode {}

pub const NONE_GC_INT_SWISS_KNIFE_NODE: Option<&GcIntSwissKnifeNode> = None;

impl fmt::Display for GcIntSwissKnifeNode {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.write_str("GcIntSwissKnifeNode")
	}
}
