// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ../gir-files
// DO NOT EDIT

use crate::ffi;
use glib::translate::*;

glib::wrapper! {
///
///
/// # Implements
///
/// [`trait@glib::ObjectExt`]
	#[doc(alias = "ArvEvaluator")]
	pub struct Evaluator(Object<ffi::ArvEvaluator, ffi::ArvEvaluatorClass>);

	match fn {
		type_ => || ffi::arv_evaluator_get_type(),
	}
}

impl Evaluator {
	/// Creates a new [`Evaluator`][crate::Evaluator] object. The syntax is described in the genicam standard specification.
	/// ## `expression`
	/// an evaluator expression
	///
	/// # Returns
	///
	/// a new [`Evaluator`][crate::Evaluator] object.
	#[doc(alias = "arv_evaluator_new")]
	pub fn new(expression: Option<&str>) -> Evaluator {
		assert_initialized_main_thread!();
		unsafe { from_glib_full(ffi::arv_evaluator_new(expression.to_glib_none().0)) }
	}

	#[doc(alias = "arv_evaluator_evaluate_as_double")]
	pub fn evaluate_as_double(&self) -> Result<f64, glib::Error> {
		unsafe {
			let mut error = std::ptr::null_mut();
			let ret = ffi::arv_evaluator_evaluate_as_double(self.to_glib_none().0, &mut error);
			if error.is_null() {
				Ok(ret)
			} else {
				Err(from_glib_full(error))
			}
		}
	}

	#[doc(alias = "arv_evaluator_evaluate_as_int64")]
	pub fn evaluate_as_int64(&self) -> Result<i64, glib::Error> {
		unsafe {
			let mut error = std::ptr::null_mut();
			let ret = ffi::arv_evaluator_evaluate_as_int64(self.to_glib_none().0, &mut error);
			if error.is_null() {
				Ok(ret)
			} else {
				Err(from_glib_full(error))
			}
		}
	}

	/// ## `name`
	/// constant name
	///
	/// # Returns
	///
	/// The formula of the constant corresponding to `name`, [`None`] if not defined.
	#[doc(alias = "arv_evaluator_get_constant")]
	#[doc(alias = "get_constant")]
	pub fn constant(&self, name: &str) -> Option<glib::GString> {
		unsafe {
			from_glib_none(ffi::arv_evaluator_get_constant(
				self.to_glib_none().0,
				name.to_glib_none().0,
			))
		}
	}

	#[doc(alias = "arv_evaluator_get_expression")]
	#[doc(alias = "get_expression")]
	pub fn expression(&self) -> Option<glib::GString> {
		unsafe { from_glib_none(ffi::arv_evaluator_get_expression(self.to_glib_none().0)) }
	}

	/// ## `name`
	/// sub-expression name
	///
	/// # Returns
	///
	/// The formula of the sub-expression corresponding to `name`, [`None`] if not defined.
	#[doc(alias = "arv_evaluator_get_sub_expression")]
	#[doc(alias = "get_sub_expression")]
	pub fn sub_expression(&self, name: &str) -> Option<glib::GString> {
		unsafe {
			from_glib_none(ffi::arv_evaluator_get_sub_expression(
				self.to_glib_none().0,
				name.to_glib_none().0,
			))
		}
	}

	/// Assign a string to a constant. If `constant` == [`None`], the constant previously assigned to `name` will be removed.
	/// ## `name`
	/// constant name
	/// ## `constant`
	/// constant as a string
	#[doc(alias = "arv_evaluator_set_constant")]
	pub fn set_constant(&self, name: &str, constant: Option<&str>) {
		unsafe {
			ffi::arv_evaluator_set_constant(
				self.to_glib_none().0,
				name.to_glib_none().0,
				constant.to_glib_none().0,
			);
		}
	}

	#[doc(alias = "arv_evaluator_set_double_variable")]
	pub fn set_double_variable(&self, name: &str, v_double: f64) {
		unsafe {
			ffi::arv_evaluator_set_double_variable(
				self.to_glib_none().0,
				name.to_glib_none().0,
				v_double,
			);
		}
	}

	#[doc(alias = "arv_evaluator_set_expression")]
	pub fn set_expression(&self, expression: &str) {
		unsafe {
			ffi::arv_evaluator_set_expression(self.to_glib_none().0, expression.to_glib_none().0);
		}
	}

	#[doc(alias = "arv_evaluator_set_int64_variable")]
	pub fn set_int64_variable(&self, name: &str, v_int64: i64) {
		unsafe {
			ffi::arv_evaluator_set_int64_variable(
				self.to_glib_none().0,
				name.to_glib_none().0,
				v_int64,
			);
		}
	}

	/// Assign a formula to a sub-expression. If `expression` == [`None`], the sub-expression previously assigned to `name` will be removed.
	/// A sub-expression may not reference another sub-expression.
	/// ## `name`
	/// sub-expression name
	/// ## `expression`
	/// sub-pexression formula
	#[doc(alias = "arv_evaluator_set_sub_expression")]
	pub fn set_sub_expression(&self, name: &str, expression: Option<&str>) {
		unsafe {
			ffi::arv_evaluator_set_sub_expression(
				self.to_glib_none().0,
				name.to_glib_none().0,
				expression.to_glib_none().0,
			);
		}
	}
}

unsafe impl Send for Evaluator {}
