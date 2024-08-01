// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ../gir-files
// DO NOT EDIT

use crate::{ffi, DomElement, DomNode, GcFeatureNode, GcInteger, GcNode, GcSwissKnife};
use glib::{prelude::*, translate::*};

glib::wrapper! {
///
///
/// # Implements
///
/// [`GcSwissKnifeExt`][trait@crate::prelude::GcSwissKnifeExt], [`GcFeatureNodeExt`][trait@crate::prelude::GcFeatureNodeExt], [`GcNodeExt`][trait@crate::prelude::GcNodeExt], [`DomElementExt`][trait@crate::prelude::DomElementExt], [`DomNodeExt`][trait@crate::prelude::DomNodeExt], [`trait@glib::ObjectExt`], [`GcIntegerExt`][trait@crate::prelude::GcIntegerExt]
	#[doc(alias = "ArvGcIntSwissKnifeNode")]
	pub struct GcIntSwissKnifeNode(Object<ffi::ArvGcIntSwissKnifeNode, ffi::ArvGcIntSwissKnifeNodeClass>) @extends GcSwissKnife, GcFeatureNode, GcNode, DomElement, DomNode, @implements GcInteger;

	match fn {
		type_ => || ffi::arv_gc_int_swiss_knife_node_get_type(),
	}
}

impl GcIntSwissKnifeNode {
	pub const NONE: Option<&'static GcIntSwissKnifeNode> = None;

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
