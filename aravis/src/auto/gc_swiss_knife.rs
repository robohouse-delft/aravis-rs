// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use aravis_sys;
use glib::translate::*;
use std::fmt;
use DomElement;
use DomNode;
use GcFeatureNode;
use GcNode;

glib_wrapper! {
	pub struct GcSwissKnife(Object<aravis_sys::ArvGcSwissKnife, aravis_sys::ArvGcSwissKnifeClass, GcSwissKnifeClass>) @extends GcFeatureNode, GcNode, DomElement, DomNode;

	match fn {
		get_type => || aravis_sys::arv_gc_swiss_knife_get_type(),
	}
}

impl GcSwissKnife {}

unsafe impl Send for GcSwissKnife {}

pub const NONE_GC_SWISS_KNIFE: Option<&GcSwissKnife> = None;

impl fmt::Display for GcSwissKnife {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "GcSwissKnife")
	}
}
