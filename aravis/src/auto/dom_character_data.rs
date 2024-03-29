// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ../gir-files
// DO NOT EDIT

use crate::DomNode;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;

glib::wrapper! {
	#[doc(alias = "ArvDomCharacterData")]
	pub struct DomCharacterData(Object<ffi::ArvDomCharacterData, ffi::ArvDomCharacterDataClass>) @extends DomNode;

	match fn {
		type_ => || ffi::arv_dom_character_data_get_type(),
	}
}

unsafe impl Send for DomCharacterData {}

pub const NONE_DOM_CHARACTER_DATA: Option<&DomCharacterData> = None;

/// Trait containing all [`struct@DomCharacterData`] methods.
///
/// # Implementors
///
/// [`DomCharacterData`][struct@crate::DomCharacterData], [`DomText`][struct@crate::DomText]
pub trait DomCharacterDataExt: 'static {
	#[doc(alias = "arv_dom_character_data_get_data")]
	#[doc(alias = "get_data")]
	fn data(&self) -> Option<glib::GString>;

	#[doc(alias = "arv_dom_character_data_set_data")]
	fn set_data(&self, value: &str);
}

impl<O: IsA<DomCharacterData>> DomCharacterDataExt for O {
	fn data(&self) -> Option<glib::GString> {
		unsafe {
			from_glib_none(ffi::arv_dom_character_data_get_data(
				self.as_ref().to_glib_none().0,
			))
		}
	}

	fn set_data(&self, value: &str) {
		unsafe {
			ffi::arv_dom_character_data_set_data(
				self.as_ref().to_glib_none().0,
				value.to_glib_none().0,
			);
		}
	}
}

impl fmt::Display for DomCharacterData {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.write_str("DomCharacterData")
	}
}
