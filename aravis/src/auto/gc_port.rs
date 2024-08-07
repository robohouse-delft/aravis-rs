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
	#[doc(alias = "ArvGcPort")]
	pub struct GcPort(Object<ffi::ArvGcPort, ffi::ArvGcPortClass>) @extends GcFeatureNode, GcNode, DomElement, DomNode;

	match fn {
		type_ => || ffi::arv_gc_port_get_type(),
	}
}

impl GcPort {
	#[doc(alias = "arv_gc_port_new")]
	pub fn new() -> GcPort {
		assert_initialized_main_thread!();
		unsafe { GcNode::from_glib_full(ffi::arv_gc_port_new()).unsafe_cast() }
	}

	//#[doc(alias = "arv_gc_port_read")]
	//pub fn read(&self, buffer: /*Unimplemented*/Option<Basic: Pointer>, address: u64, length: u64) -> Result<(), glib::Error> {
	//    unsafe { TODO: call ffi:arv_gc_port_read() }
	//}

	//#[doc(alias = "arv_gc_port_write")]
	//pub fn write(&self, buffer: /*Unimplemented*/Option<Basic: Pointer>, address: u64, length: u64) -> Result<(), glib::Error> {
	//    unsafe { TODO: call ffi:arv_gc_port_write() }
	//}
}

impl Default for GcPort {
	fn default() -> Self {
		Self::new()
	}
}

unsafe impl Send for GcPort {}
