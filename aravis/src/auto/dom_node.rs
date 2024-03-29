// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ../gir-files
// DO NOT EDIT

use crate::DomDocument;
use crate::DomNodeList;
use crate::DomNodeType;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;

glib::wrapper! {
	#[doc(alias = "ArvDomNode")]
	pub struct DomNode(Object<ffi::ArvDomNode, ffi::ArvDomNodeClass>);

	match fn {
		type_ => || ffi::arv_dom_node_get_type(),
	}
}

unsafe impl Send for DomNode {}

pub const NONE_DOM_NODE: Option<&DomNode> = None;

/// Trait containing all [`struct@DomNode`] methods.
///
/// # Implementors
///
/// [`DomCharacterData`][struct@crate::DomCharacterData], [`DomDocumentFragment`][struct@crate::DomDocumentFragment], [`DomDocument`][struct@crate::DomDocument], [`DomElement`][struct@crate::DomElement], [`DomNode`][struct@crate::DomNode]
pub trait DomNodeExt: 'static {
	/// Adds the node `new_child` to the end of the list of children of this node.
	/// If the `new_child` is already in the tree, it is first removed.
	/// ## `new_child`
	/// node to append
	///
	/// # Returns
	///
	/// the added node.
	#[doc(alias = "arv_dom_node_append_child")]
	fn append_child<P: IsA<DomNode>>(&self, new_child: &P) -> Option<DomNode>;

	#[doc(alias = "arv_dom_node_changed")]
	fn changed(&self);

	///
	/// # Returns
	///
	/// a [`DomNodeList`][crate::DomNodeList], NULL on error.
	#[doc(alias = "arv_dom_node_get_child_nodes")]
	#[doc(alias = "get_child_nodes")]
	fn child_nodes(&self) -> Option<DomNodeList>;

	///
	/// # Returns
	///
	/// `self` first child.
	#[doc(alias = "arv_dom_node_get_first_child")]
	#[doc(alias = "get_first_child")]
	fn first_child(&self) -> Option<DomNode>;

	///
	/// # Returns
	///
	/// `self` last child.
	#[doc(alias = "arv_dom_node_get_last_child")]
	#[doc(alias = "get_last_child")]
	fn last_child(&self) -> Option<DomNode>;

	///
	/// # Returns
	///
	/// `self` next sibling.
	#[doc(alias = "arv_dom_node_get_next_sibling")]
	#[doc(alias = "get_next_sibling")]
	fn next_sibling(&self) -> Option<DomNode>;

	/// Gets the node name.
	///
	/// # Returns
	///
	/// the node name.
	#[doc(alias = "arv_dom_node_get_node_name")]
	#[doc(alias = "get_node_name")]
	fn node_name(&self) -> Option<glib::GString>;

	#[doc(alias = "arv_dom_node_get_node_type")]
	#[doc(alias = "get_node_type")]
	fn node_type(&self) -> DomNodeType;

	/// Gets the node value.
	///
	/// # Returns
	///
	/// the node value.
	#[doc(alias = "arv_dom_node_get_node_value")]
	#[doc(alias = "get_node_value")]
	fn node_value(&self) -> Option<glib::GString>;

	///
	/// # Returns
	///
	/// `self` owner document.
	#[doc(alias = "arv_dom_node_get_owner_document")]
	#[doc(alias = "get_owner_document")]
	fn owner_document(&self) -> Option<DomDocument>;

	/// Get the parent node of `self`.
	///
	/// # Returns
	///
	/// `self` parent.
	#[doc(alias = "arv_dom_node_get_parent_node")]
	#[doc(alias = "get_parent_node")]
	fn parent_node(&self) -> Option<DomNode>;

	///
	/// # Returns
	///
	/// `self` previous sibling.
	#[doc(alias = "arv_dom_node_get_previous_sibling")]
	#[doc(alias = "get_previous_sibling")]
	fn previous_sibling(&self) -> Option<DomNode>;

	#[doc(alias = "arv_dom_node_has_child_nodes")]
	fn has_child_nodes(&self) -> bool;

	/// Inserts the node `new_child` before the existing child node `ref_child`. If
	/// `ref_child` is null, insert `new_child` at the end of the list of children.
	/// If the `new_child` is already in the tree, it is first removed.
	/// ## `new_child`
	/// node to insert
	/// ## `ref_child`
	/// reference node, i.e., the node before which the new node must be inserted.
	///
	/// # Returns
	///
	/// the inserted node.
	#[doc(alias = "arv_dom_node_insert_before")]
	fn insert_before<P: IsA<DomNode>, Q: IsA<DomNode>>(
		&self,
		new_child: &P,
		ref_child: &Q,
	) -> Option<DomNode>;

	/// Removes the child node indicated by `old_child` from the list of children, and returns it.
	/// ## `old_child`
	/// node to remove.
	///
	/// # Returns
	///
	/// the removed node.
	#[doc(alias = "arv_dom_node_remove_child")]
	fn remove_child<P: IsA<DomNode>>(&self, old_child: &P) -> Option<DomNode>;

	/// Replaces the child node `old_child` with `new_child` in the list of children,
	/// and returns the `old_child` node.
	/// If the `new_child` is already in the tree, it is first removed.
	/// ## `new_child`
	/// a replacement node
	/// ## `old_child`
	/// node to replace
	///
	/// # Returns
	///
	/// the replaced node.
	#[doc(alias = "arv_dom_node_replace_child")]
	fn replace_child<P: IsA<DomNode>, Q: IsA<DomNode>>(
		&self,
		new_child: &P,
		old_child: &Q,
	) -> Option<DomNode>;

	#[doc(alias = "arv_dom_node_set_node_value")]
	fn set_node_value(&self, new_value: &str);
}

impl<O: IsA<DomNode>> DomNodeExt for O {
	fn append_child<P: IsA<DomNode>>(&self, new_child: &P) -> Option<DomNode> {
		unsafe {
			from_glib_none(ffi::arv_dom_node_append_child(
				self.as_ref().to_glib_none().0,
				new_child.as_ref().to_glib_full(),
			))
		}
	}

	fn changed(&self) {
		unsafe {
			ffi::arv_dom_node_changed(self.as_ref().to_glib_none().0);
		}
	}

	fn child_nodes(&self) -> Option<DomNodeList> {
		unsafe {
			from_glib_none(ffi::arv_dom_node_get_child_nodes(
				self.as_ref().to_glib_none().0,
			))
		}
	}

	fn first_child(&self) -> Option<DomNode> {
		unsafe {
			from_glib_none(ffi::arv_dom_node_get_first_child(
				self.as_ref().to_glib_none().0,
			))
		}
	}

	fn last_child(&self) -> Option<DomNode> {
		unsafe {
			from_glib_none(ffi::arv_dom_node_get_last_child(
				self.as_ref().to_glib_none().0,
			))
		}
	}

	fn next_sibling(&self) -> Option<DomNode> {
		unsafe {
			from_glib_none(ffi::arv_dom_node_get_next_sibling(
				self.as_ref().to_glib_none().0,
			))
		}
	}

	fn node_name(&self) -> Option<glib::GString> {
		unsafe {
			from_glib_none(ffi::arv_dom_node_get_node_name(
				self.as_ref().to_glib_none().0,
			))
		}
	}

	fn node_type(&self) -> DomNodeType {
		unsafe {
			from_glib(ffi::arv_dom_node_get_node_type(
				self.as_ref().to_glib_none().0,
			))
		}
	}

	fn node_value(&self) -> Option<glib::GString> {
		unsafe {
			from_glib_none(ffi::arv_dom_node_get_node_value(
				self.as_ref().to_glib_none().0,
			))
		}
	}

	fn owner_document(&self) -> Option<DomDocument> {
		unsafe {
			from_glib_none(ffi::arv_dom_node_get_owner_document(
				self.as_ref().to_glib_none().0,
			))
		}
	}

	fn parent_node(&self) -> Option<DomNode> {
		unsafe {
			from_glib_none(ffi::arv_dom_node_get_parent_node(
				self.as_ref().to_glib_none().0,
			))
		}
	}

	fn previous_sibling(&self) -> Option<DomNode> {
		unsafe {
			from_glib_none(ffi::arv_dom_node_get_previous_sibling(
				self.as_ref().to_glib_none().0,
			))
		}
	}

	fn has_child_nodes(&self) -> bool {
		unsafe {
			from_glib(ffi::arv_dom_node_has_child_nodes(
				self.as_ref().to_glib_none().0,
			))
		}
	}

	fn insert_before<P: IsA<DomNode>, Q: IsA<DomNode>>(
		&self,
		new_child: &P,
		ref_child: &Q,
	) -> Option<DomNode> {
		unsafe {
			from_glib_none(ffi::arv_dom_node_insert_before(
				self.as_ref().to_glib_none().0,
				new_child.as_ref().to_glib_full(),
				ref_child.as_ref().to_glib_none().0,
			))
		}
	}

	fn remove_child<P: IsA<DomNode>>(&self, old_child: &P) -> Option<DomNode> {
		unsafe {
			from_glib_full(ffi::arv_dom_node_remove_child(
				self.as_ref().to_glib_none().0,
				old_child.as_ref().to_glib_none().0,
			))
		}
	}

	fn replace_child<P: IsA<DomNode>, Q: IsA<DomNode>>(
		&self,
		new_child: &P,
		old_child: &Q,
	) -> Option<DomNode> {
		unsafe {
			from_glib_full(ffi::arv_dom_node_replace_child(
				self.as_ref().to_glib_none().0,
				new_child.as_ref().to_glib_full(),
				old_child.as_ref().to_glib_none().0,
			))
		}
	}

	fn set_node_value(&self, new_value: &str) {
		unsafe {
			ffi::arv_dom_node_set_node_value(
				self.as_ref().to_glib_none().0,
				new_value.to_glib_none().0,
			);
		}
	}
}

impl fmt::Display for DomNode {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.write_str("DomNode")
	}
}
