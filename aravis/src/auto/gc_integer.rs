// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ../gir-files
// DO NOT EDIT

use crate::{ffi, GcRepresentation};
use glib::{prelude::*, translate::*};

glib::wrapper! {
///
///
/// # Implements
///
/// [`GcIntegerExt`][trait@crate::prelude::GcIntegerExt]
	#[doc(alias = "ArvGcInteger")]
	pub struct GcInteger(Interface<ffi::ArvGcInteger, ffi::ArvGcIntegerInterface>);

	match fn {
		type_ => || ffi::arv_gc_integer_get_type(),
	}
}

impl GcInteger {
	pub const NONE: Option<&'static GcInteger> = None;
}

unsafe impl Send for GcInteger {}

mod sealed {
	pub trait Sealed {}
	impl<T: super::IsA<super::GcInteger>> Sealed for T {}
}

/// Trait containing all [`struct@GcInteger`] methods.
///
/// # Implementors
///
/// [`GcEnumeration`][struct@crate::GcEnumeration], [`GcIntConverterNode`][struct@crate::GcIntConverterNode], [`GcIntRegNode`][struct@crate::GcIntRegNode], [`GcIntSwissKnifeNode`][struct@crate::GcIntSwissKnifeNode], [`GcIntegerNode`][struct@crate::GcIntegerNode], [`GcInteger`][struct@crate::GcInteger], [`GcMaskedIntRegNode`][struct@crate::GcMaskedIntRegNode], [`GcStructEntryNode`][struct@crate::GcStructEntryNode]
pub trait GcIntegerExt: IsA<GcInteger> + sealed::Sealed + 'static {
	#[doc(alias = "arv_gc_integer_get_inc")]
	#[doc(alias = "get_inc")]
	fn inc(&self) -> Result<i64, glib::Error> {
		unsafe {
			let mut error = std::ptr::null_mut();
			let ret = ffi::arv_gc_integer_get_inc(self.as_ref().to_glib_none().0, &mut error);
			if error.is_null() {
				Ok(ret)
			} else {
				Err(from_glib_full(error))
			}
		}
	}

	#[doc(alias = "arv_gc_integer_get_max")]
	#[doc(alias = "get_max")]
	fn max(&self) -> Result<i64, glib::Error> {
		unsafe {
			let mut error = std::ptr::null_mut();
			let ret = ffi::arv_gc_integer_get_max(self.as_ref().to_glib_none().0, &mut error);
			if error.is_null() {
				Ok(ret)
			} else {
				Err(from_glib_full(error))
			}
		}
	}

	#[doc(alias = "arv_gc_integer_get_min")]
	#[doc(alias = "get_min")]
	fn min(&self) -> Result<i64, glib::Error> {
		unsafe {
			let mut error = std::ptr::null_mut();
			let ret = ffi::arv_gc_integer_get_min(self.as_ref().to_glib_none().0, &mut error);
			if error.is_null() {
				Ok(ret)
			} else {
				Err(from_glib_full(error))
			}
		}
	}

	#[doc(alias = "arv_gc_integer_get_representation")]
	#[doc(alias = "get_representation")]
	fn representation(&self) -> GcRepresentation {
		unsafe {
			from_glib(ffi::arv_gc_integer_get_representation(
				self.as_ref().to_glib_none().0,
			))
		}
	}

	#[doc(alias = "arv_gc_integer_get_unit")]
	#[doc(alias = "get_unit")]
	fn unit(&self) -> Option<glib::GString> {
		unsafe { from_glib_none(ffi::arv_gc_integer_get_unit(self.as_ref().to_glib_none().0)) }
	}

	#[doc(alias = "arv_gc_integer_get_value")]
	#[doc(alias = "get_value")]
	fn value(&self) -> Result<i64, glib::Error> {
		unsafe {
			let mut error = std::ptr::null_mut();
			let ret = ffi::arv_gc_integer_get_value(self.as_ref().to_glib_none().0, &mut error);
			if error.is_null() {
				Ok(ret)
			} else {
				Err(from_glib_full(error))
			}
		}
	}

	#[doc(alias = "arv_gc_integer_impose_max")]
	fn impose_max(&self, maximum: i64) -> Result<(), glib::Error> {
		unsafe {
			let mut error = std::ptr::null_mut();
			let _ =
				ffi::arv_gc_integer_impose_max(self.as_ref().to_glib_none().0, maximum, &mut error);
			if error.is_null() {
				Ok(())
			} else {
				Err(from_glib_full(error))
			}
		}
	}

	#[doc(alias = "arv_gc_integer_impose_min")]
	fn impose_min(&self, minimum: i64) -> Result<(), glib::Error> {
		unsafe {
			let mut error = std::ptr::null_mut();
			let _ =
				ffi::arv_gc_integer_impose_min(self.as_ref().to_glib_none().0, minimum, &mut error);
			if error.is_null() {
				Ok(())
			} else {
				Err(from_glib_full(error))
			}
		}
	}

	#[doc(alias = "arv_gc_integer_set_value")]
	fn set_value(&self, value: i64) -> Result<(), glib::Error> {
		unsafe {
			let mut error = std::ptr::null_mut();
			let _ =
				ffi::arv_gc_integer_set_value(self.as_ref().to_glib_none().0, value, &mut error);
			if error.is_null() {
				Ok(())
			} else {
				Err(from_glib_full(error))
			}
		}
	}
}

impl<O: IsA<GcInteger>> GcIntegerExt for O {}
