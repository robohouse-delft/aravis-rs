// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ../gir-files
// DO NOT EDIT

use crate::{ffi, DomElement, DomNode, Gc};
use glib::{prelude::*, translate::*};

glib::wrapper! {
/// [`GcNode`][crate::GcNode] provides a base class for the implementation of the different
/// types of Genicam nodes.
///
/// This is an Abstract Base Class, you cannot instantiate it.
///
/// # Implements
///
/// [`GcNodeExt`][trait@crate::prelude::GcNodeExt], [`DomElementExt`][trait@crate::prelude::DomElementExt], [`DomNodeExt`][trait@crate::prelude::DomNodeExt], [`trait@glib::ObjectExt`]
	#[doc(alias = "ArvGcNode")]
	pub struct GcNode(Object<ffi::ArvGcNode, ffi::ArvGcNodeClass>) @extends DomElement, DomNode;

	match fn {
		type_ => || ffi::arv_gc_node_get_type(),
	}
}

impl GcNode {
	pub const NONE: Option<&'static GcNode> = None;
}

unsafe impl Send for GcNode {}

mod sealed {
	pub trait Sealed {}
	impl<T: super::IsA<super::GcNode>> Sealed for T {}
}

/// Trait containing all [`struct@GcNode`] methods.
///
/// # Implementors
///
/// [`GcFeatureNode`][struct@crate::GcFeatureNode], [`GcNode`][struct@crate::GcNode], [`GcPropertyNode`][struct@crate::GcPropertyNode]
pub trait GcNodeExt: IsA<GcNode> + sealed::Sealed + 'static {
	#[doc(alias = "arv_gc_node_get_genicam")]
	#[doc(alias = "get_genicam")]
	fn genicam(&self) -> Option<Gc> {
		unsafe { from_glib_none(ffi::arv_gc_node_get_genicam(self.as_ref().to_glib_none().0)) }
	}
}

impl<O: IsA<GcNode>> GcNodeExt for O {}
