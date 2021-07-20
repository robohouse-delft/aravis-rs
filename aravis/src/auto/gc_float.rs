// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ../gir-files
// DO NOT EDIT

use crate::GcDisplayNotation;
use crate::GcRepresentation;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;
use std::ptr;

glib::wrapper! {
	#[doc(alias = "ArvGcFloat")]
	pub struct GcFloat(Interface<ffi::ArvGcFloat, ffi::ArvGcFloatInterface>);

	match fn {
		type_ => || ffi::arv_gc_float_get_type(),
	}
}

unsafe impl Send for GcFloat {}

pub const NONE_GC_FLOAT: Option<&GcFloat> = None;

/// Trait containing all [`struct@GcFloat`] methods.
///
/// # Implementors
///
/// [`GcConverterNode`][struct@crate::GcConverterNode], [`GcFloatNode`][struct@crate::GcFloatNode], [`GcFloatRegNode`][struct@crate::GcFloatRegNode], [`GcFloat`][struct@crate::GcFloat], [`GcSwissKnifeNode`][struct@crate::GcSwissKnifeNode]
pub trait GcFloatExt: 'static {
	/// Get number display notation.
	///
	/// # Returns
	///
	/// Number display notation as [`GcDisplayNotation`][crate::GcDisplayNotation].
	#[doc(alias = "arv_gc_float_get_display_notation")]
	#[doc(alias = "get_display_notation")]
	fn display_notation(&self) -> GcDisplayNotation;

	/// Gets number of digits to show in user interface. This number should always be positive and represent
	/// total number of digits on left and right side of decimal.
	///
	/// # Returns
	///
	/// Number of digits to show.
	#[doc(alias = "arv_gc_float_get_display_precision")]
	#[doc(alias = "get_display_precision")]
	fn display_precision(&self) -> i64;

	#[doc(alias = "arv_gc_float_get_inc")]
	#[doc(alias = "get_inc")]
	fn inc(&self) -> Result<f64, glib::Error>;

	#[doc(alias = "arv_gc_float_get_max")]
	#[doc(alias = "get_max")]
	fn max(&self) -> Result<f64, glib::Error>;

	#[doc(alias = "arv_gc_float_get_min")]
	#[doc(alias = "get_min")]
	fn min(&self) -> Result<f64, glib::Error>;

	/// Get number representation format.
	///
	/// # Returns
	///
	/// Number representation format as [`GcRepresentation`][crate::GcRepresentation].
	#[doc(alias = "arv_gc_float_get_representation")]
	#[doc(alias = "get_representation")]
	fn representation(&self) -> GcRepresentation;

	#[doc(alias = "arv_gc_float_get_unit")]
	#[doc(alias = "get_unit")]
	fn unit(&self) -> Option<glib::GString>;

	#[doc(alias = "arv_gc_float_get_value")]
	#[doc(alias = "get_value")]
	fn value(&self) -> Result<f64, glib::Error>;

	#[doc(alias = "arv_gc_float_impose_max")]
	fn impose_max(&self, maximum: f64) -> Result<(), glib::Error>;

	#[doc(alias = "arv_gc_float_impose_min")]
	fn impose_min(&self, minimum: f64) -> Result<(), glib::Error>;

	#[doc(alias = "arv_gc_float_set_value")]
	fn set_value(&self, value: f64) -> Result<(), glib::Error>;
}

impl<O: IsA<GcFloat>> GcFloatExt for O {
	fn display_notation(&self) -> GcDisplayNotation {
		unsafe {
			from_glib(ffi::arv_gc_float_get_display_notation(
				self.as_ref().to_glib_none().0,
			))
		}
	}

	fn display_precision(&self) -> i64 {
		unsafe { ffi::arv_gc_float_get_display_precision(self.as_ref().to_glib_none().0) }
	}

	fn inc(&self) -> Result<f64, glib::Error> {
		unsafe {
			let mut error = ptr::null_mut();
			let ret = ffi::arv_gc_float_get_inc(self.as_ref().to_glib_none().0, &mut error);
			if error.is_null() {
				Ok(ret)
			} else {
				Err(from_glib_full(error))
			}
		}
	}

	fn max(&self) -> Result<f64, glib::Error> {
		unsafe {
			let mut error = ptr::null_mut();
			let ret = ffi::arv_gc_float_get_max(self.as_ref().to_glib_none().0, &mut error);
			if error.is_null() {
				Ok(ret)
			} else {
				Err(from_glib_full(error))
			}
		}
	}

	fn min(&self) -> Result<f64, glib::Error> {
		unsafe {
			let mut error = ptr::null_mut();
			let ret = ffi::arv_gc_float_get_min(self.as_ref().to_glib_none().0, &mut error);
			if error.is_null() {
				Ok(ret)
			} else {
				Err(from_glib_full(error))
			}
		}
	}

	fn representation(&self) -> GcRepresentation {
		unsafe {
			from_glib(ffi::arv_gc_float_get_representation(
				self.as_ref().to_glib_none().0,
			))
		}
	}

	fn unit(&self) -> Option<glib::GString> {
		unsafe { from_glib_none(ffi::arv_gc_float_get_unit(self.as_ref().to_glib_none().0)) }
	}

	fn value(&self) -> Result<f64, glib::Error> {
		unsafe {
			let mut error = ptr::null_mut();
			let ret = ffi::arv_gc_float_get_value(self.as_ref().to_glib_none().0, &mut error);
			if error.is_null() {
				Ok(ret)
			} else {
				Err(from_glib_full(error))
			}
		}
	}

	fn impose_max(&self, maximum: f64) -> Result<(), glib::Error> {
		unsafe {
			let mut error = ptr::null_mut();
			let _ =
				ffi::arv_gc_float_impose_max(self.as_ref().to_glib_none().0, maximum, &mut error);
			if error.is_null() {
				Ok(())
			} else {
				Err(from_glib_full(error))
			}
		}
	}

	fn impose_min(&self, minimum: f64) -> Result<(), glib::Error> {
		unsafe {
			let mut error = ptr::null_mut();
			let _ =
				ffi::arv_gc_float_impose_min(self.as_ref().to_glib_none().0, minimum, &mut error);
			if error.is_null() {
				Ok(())
			} else {
				Err(from_glib_full(error))
			}
		}
	}

	fn set_value(&self, value: f64) -> Result<(), glib::Error> {
		unsafe {
			let mut error = ptr::null_mut();
			let _ = ffi::arv_gc_float_set_value(self.as_ref().to_glib_none().0, value, &mut error);
			if error.is_null() {
				Ok(())
			} else {
				Err(from_glib_full(error))
			}
		}
	}
}

impl fmt::Display for GcFloat {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.write_str("GcFloat")
	}
}
