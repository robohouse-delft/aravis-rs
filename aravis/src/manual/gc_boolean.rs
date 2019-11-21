use crate::GcBoolean;

use glib::IsA;
use glib::translate::ToGlibPtr;
use glib::translate::from_glib_full;

pub trait GcBooleanExtManual {
	fn get_value(&self) -> Result<bool, glib::Error>;
}

impl<T: IsA<GcBoolean>> GcBooleanExtManual for T {
	fn get_value(&self) -> Result<bool, glib::Error> {
		unsafe {
			let mut error = std::ptr::null_mut();
			let value = aravis_sys::arv_gc_boolean_get_value(self.as_ref().to_glib_none().0, &mut error);
			if error.is_null() { Ok(value != 0) } else { Err(from_glib_full(error)) }
		}
	}
}
