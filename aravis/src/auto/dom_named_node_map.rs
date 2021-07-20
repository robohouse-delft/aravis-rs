// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ../gir-files
// DO NOT EDIT

use crate::DomNode;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;

glib::wrapper! {
	#[doc(alias = "ArvDomNamedNodeMap")]
	pub struct DomNamedNodeMap(Object<ffi::ArvDomNamedNodeMap, ffi::ArvDomNamedNodeMapClass>);

	match fn {
		type_ => || ffi::arv_dom_named_node_map_get_type(),
	}
}

unsafe impl Send for DomNamedNodeMap {}

pub const NONE_DOM_NAMED_NODE_MAP: Option<&DomNamedNodeMap> = None;

/// Trait containing all [`struct@DomNamedNodeMap`] methods.
///
/// # Implementors
///
/// [`DomNamedNodeMap`][struct@crate::DomNamedNodeMap]
pub trait DomNamedNodeMapExt: 'static {
	/// ## `index`
	/// an index
	///
	/// # Returns
	///
	/// the [`DomNode`][crate::DomNode] corresponding to `index`.
	#[doc(alias = "arv_dom_named_node_map_get_item")]
	#[doc(alias = "get_item")]
	fn item(&self, index: u32) -> Option<DomNode>;

	#[doc(alias = "arv_dom_named_node_map_get_length")]
	#[doc(alias = "get_length")]
	fn length(&self) -> u32;

	/// ## `name`
	/// name of the element to look for.
	///
	/// # Returns
	///
	/// a [`DomElement`][crate::DomElement].
	#[doc(alias = "arv_dom_named_node_map_get_named_item")]
	#[doc(alias = "get_named_item")]
	fn named_item(&self, name: &str) -> Option<DomNode>;

	/// ## `name`
	/// name of the node to remove
	///
	/// # Returns
	///
	/// the removed node.
	#[doc(alias = "arv_dom_named_node_map_remove_named_item")]
	fn remove_named_item(&self, name: &str) -> Option<DomNode>;

	/// ## `item`
	/// a node to insert
	///
	/// # Returns
	///
	/// same as `node` on success.
	#[doc(alias = "arv_dom_named_node_map_set_named_item")]
	fn set_named_item<P: IsA<DomNode>>(&self, item: &P) -> Option<DomNode>;
}

impl<O: IsA<DomNamedNodeMap>> DomNamedNodeMapExt for O {
	fn item(&self, index: u32) -> Option<DomNode> {
		unsafe {
			from_glib_none(ffi::arv_dom_named_node_map_get_item(
				self.as_ref().to_glib_none().0,
				index,
			))
		}
	}

	fn length(&self) -> u32 {
		unsafe { ffi::arv_dom_named_node_map_get_length(self.as_ref().to_glib_none().0) }
	}

	fn named_item(&self, name: &str) -> Option<DomNode> {
		unsafe {
			from_glib_none(ffi::arv_dom_named_node_map_get_named_item(
				self.as_ref().to_glib_none().0,
				name.to_glib_none().0,
			))
		}
	}

	fn remove_named_item(&self, name: &str) -> Option<DomNode> {
		unsafe {
			from_glib_none(ffi::arv_dom_named_node_map_remove_named_item(
				self.as_ref().to_glib_none().0,
				name.to_glib_none().0,
			))
		}
	}

	fn set_named_item<P: IsA<DomNode>>(&self, item: &P) -> Option<DomNode> {
		unsafe {
			from_glib_none(ffi::arv_dom_named_node_map_set_named_item(
				self.as_ref().to_glib_none().0,
				item.as_ref().to_glib_none().0,
			))
		}
	}
}

impl fmt::Display for DomNamedNodeMap {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.write_str("DomNamedNodeMap")
	}
}
