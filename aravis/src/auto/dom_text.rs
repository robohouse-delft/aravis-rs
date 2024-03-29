// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ../gir-files
// DO NOT EDIT

use crate::DomCharacterData;
use crate::DomNode;
use glib::object::Cast;
use glib::translate::*;
use std::fmt;

glib::wrapper! {
	#[doc(alias = "ArvDomText")]
	pub struct DomText(Object<ffi::ArvDomText, ffi::ArvDomTextClass>) @extends DomCharacterData, DomNode;

	match fn {
		type_ => || ffi::arv_dom_text_get_type(),
	}
}

impl DomText {
	#[doc(alias = "arv_dom_text_new")]
	pub fn new(data: &str) -> DomText {
		assert_initialized_main_thread!();
		unsafe {
			DomNode::from_glib_full(ffi::arv_dom_text_new(data.to_glib_none().0)).unsafe_cast()
		}
	}
}

unsafe impl Send for DomText {}

pub const NONE_DOM_TEXT: Option<&DomText> = None;

impl fmt::Display for DomText {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.write_str("DomText")
	}
}
