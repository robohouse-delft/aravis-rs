use ffi::arv_gc_register_get;
use glib::object::IsA;
use glib::translate::*;
use std::ffi::c_void;
use std::ptr;

use crate::GcRegister;

pub(crate) mod traits {
	pub trait GcRegisterExtManual {
		fn get(&self, buffer: &mut [u8]) -> Result<(), glib::Error>;
		fn set(&self, buffer: &[u8]) -> Result<(), glib::Error>;
	}
}

impl<T: IsA<GcRegister>> traits::GcRegisterExtManual for T {
	fn get(&self, buffer: &mut [u8]) -> Result<(), glib::Error> {
		unsafe {
			let mut error = ptr::null_mut();
			let len = buffer.len();
			let ptr = buffer.as_mut_ptr();
			arv_gc_register_get(self.as_ref().to_glib_none().0, ptr as *mut c_void, len as u64, &mut error);
			if error.is_null() {
				Ok(())
			} else {
				Err(from_glib_full(error))
			}
		}
	}

	fn set(&self, buffer: &[u8]) -> Result<(), glib::Error> {
		unsafe {
			let mut error = ptr::null_mut();
			let len = buffer.len();
			let ptr = buffer.as_ptr();
			ffi::arv_gc_register_set(self.as_ref().to_glib_none().0, ptr as *mut c_void, len as u64, &mut error);
			if error.is_null() {
				Ok(())
			} else {
				Err(from_glib_full(error))
			}
		}
	}
}
