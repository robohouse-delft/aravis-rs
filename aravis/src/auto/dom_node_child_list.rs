// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ../gir-files
// DO NOT EDIT

use crate::{ffi, DomNode, DomNodeList};
use glib::{prelude::*, translate::*};

glib::wrapper! {
///
///
/// # Implements
///
/// [`DomNodeListExt`][trait@crate::prelude::DomNodeListExt], [`trait@glib::ObjectExt`]
	#[doc(alias = "ArvDomNodeChildList")]
	pub struct DomNodeChildList(Object<ffi::ArvDomNodeChildList, ffi::ArvDomNodeChildListClass>) @extends DomNodeList;

	match fn {
		type_ => || ffi::arv_dom_node_child_list_get_type(),
	}
}

impl DomNodeChildList {
	#[doc(alias = "arv_dom_node_child_list_new")]
	pub fn new(parent_node: &impl IsA<DomNode>) -> DomNodeChildList {
		skip_assert_initialized!();
		unsafe {
			DomNodeList::from_glib_full(ffi::arv_dom_node_child_list_new(
				parent_node.as_ref().to_glib_none().0,
			))
			.unsafe_cast()
		}
	}
}

unsafe impl Send for DomNodeChildList {}
