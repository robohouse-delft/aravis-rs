// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ../gir-files
// DO NOT EDIT

use crate::{ffi, DomElement, DomNode, GcFeatureNode, GcFloat, GcNode, GcRegister, GcRegisterNode};
use glib::{prelude::*, translate::*};

glib::wrapper! {
///
///
/// # Implements
///
/// [`GcRegisterNodeExt`][trait@crate::prelude::GcRegisterNodeExt], [`GcFeatureNodeExt`][trait@crate::prelude::GcFeatureNodeExt], [`GcNodeExt`][trait@crate::prelude::GcNodeExt], [`DomElementExt`][trait@crate::prelude::DomElementExt], [`DomNodeExt`][trait@crate::prelude::DomNodeExt], [`trait@glib::ObjectExt`], [`GcRegisterExt`][trait@crate::prelude::GcRegisterExt], [`GcFloatExt`][trait@crate::prelude::GcFloatExt]
	#[doc(alias = "ArvGcFloatRegNode")]
	pub struct GcFloatRegNode(Object<ffi::ArvGcFloatRegNode, ffi::ArvGcFloatRegNodeClass>) @extends GcRegisterNode, GcFeatureNode, GcNode, DomElement, DomNode, @implements GcRegister, GcFloat;

	match fn {
		type_ => || ffi::arv_gc_float_reg_node_get_type(),
	}
}

impl GcFloatRegNode {
	pub const NONE: Option<&'static GcFloatRegNode> = None;

	///
	/// # Returns
	///
	/// a new FloatReg node
	#[doc(alias = "arv_gc_float_reg_node_new")]
	pub fn new() -> GcFloatRegNode {
		assert_initialized_main_thread!();
		unsafe { GcNode::from_glib_full(ffi::arv_gc_float_reg_node_new()).unsafe_cast() }
	}
}

impl Default for GcFloatRegNode {
	fn default() -> Self {
		Self::new()
	}
}

unsafe impl Send for GcFloatRegNode {}
