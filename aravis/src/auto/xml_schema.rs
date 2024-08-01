// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ../gir-files
// DO NOT EDIT

use crate::ffi;
use glib::{prelude::*, translate::*};

glib::wrapper! {
///
///
/// # Implements
///
/// [`trait@glib::ObjectExt`]
	#[doc(alias = "ArvXmlSchema")]
	pub struct XmlSchema(Object<ffi::ArvXmlSchema, ffi::ArvXmlSchemaClass>);

	match fn {
		type_ => || ffi::arv_xml_schema_get_type(),
	}
}

impl XmlSchema {
	#[doc(alias = "arv_xml_schema_new_from_file")]
	#[doc(alias = "new_from_file")]
	pub fn from_file(file: &impl IsA<gio::File>) -> XmlSchema {
		assert_initialized_main_thread!();
		unsafe {
			from_glib_full(ffi::arv_xml_schema_new_from_file(
				file.as_ref().to_glib_none().0,
			))
		}
	}

	#[doc(alias = "arv_xml_schema_new_from_memory")]
	#[doc(alias = "new_from_memory")]
	pub fn from_memory(buffer: &str, size: usize) -> XmlSchema {
		assert_initialized_main_thread!();
		unsafe {
			from_glib_full(ffi::arv_xml_schema_new_from_memory(
				buffer.to_glib_none().0,
				size,
			))
		}
	}

	#[doc(alias = "arv_xml_schema_new_from_path")]
	#[doc(alias = "new_from_path")]
	pub fn from_path(path: &str) -> XmlSchema {
		assert_initialized_main_thread!();
		unsafe { from_glib_full(ffi::arv_xml_schema_new_from_path(path.to_glib_none().0)) }
	}

	//#[doc(alias = "arv_xml_schema_validate")]
	//pub fn validate(&self, xml: /*Unimplemented*/Option<Basic: Pointer>, size: usize, line: i32, column: i32) -> Result<(), glib::Error> {
	//    unsafe { TODO: call ffi:arv_xml_schema_validate() }
	//}
}

unsafe impl Send for XmlSchema {}
