// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ../gir-files
// DO NOT EDIT

use crate::{
	ffi, DomElement, DomNode, GcFeatureNode, GcInteger, GcNode, GcRegister, GcRegisterNode,
	GcSelector,
};
use glib::{prelude::*, translate::*};

glib::wrapper! {
///
///
/// # Implements
///
/// [`GcRegisterNodeExt`][trait@crate::prelude::GcRegisterNodeExt], [`GcFeatureNodeExt`][trait@crate::prelude::GcFeatureNodeExt], [`GcNodeExt`][trait@crate::prelude::GcNodeExt], [`DomElementExt`][trait@crate::prelude::DomElementExt], [`DomNodeExt`][trait@crate::prelude::DomNodeExt], [`trait@glib::ObjectExt`], [`GcRegisterExt`][trait@crate::prelude::GcRegisterExt], [`GcIntegerExt`][trait@crate::prelude::GcIntegerExt], [`GcSelectorExt`][trait@crate::prelude::GcSelectorExt]
	#[doc(alias = "ArvGcIntRegNode")]
	pub struct GcIntRegNode(Object<ffi::ArvGcIntRegNode, ffi::ArvGcIntRegNodeClass>) @extends GcRegisterNode, GcFeatureNode, GcNode, DomElement, DomNode, @implements GcRegister, GcInteger, GcSelector;

	match fn {
		type_ => || ffi::arv_gc_int_reg_node_get_type(),
	}
}

impl GcIntRegNode {
	pub const NONE: Option<&'static GcIntRegNode> = None;

	///
	/// # Returns
	///
	/// a new IntReg node
	#[doc(alias = "arv_gc_int_reg_node_new")]
	pub fn new() -> GcIntRegNode {
		assert_initialized_main_thread!();
		unsafe { GcNode::from_glib_full(ffi::arv_gc_int_reg_node_new()).unsafe_cast() }
	}
}

impl Default for GcIntRegNode {
	fn default() -> Self {
		Self::new()
	}
}

unsafe impl Send for GcIntRegNode {}
