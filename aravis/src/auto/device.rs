// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use aravis_sys;
use glib;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::GString;
use glib_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem;
use std::mem::transmute;
use std::ptr;
use ChunkParser;
use Gc;
use GcNode;
use RegisterCachePolicy;

glib_wrapper! {
	pub struct Device(Object<aravis_sys::ArvDevice, aravis_sys::ArvDeviceClass, DeviceClass>);

	match fn {
		get_type => || aravis_sys::arv_device_get_type(),
	}
}

unsafe impl Send for Device {}

pub const NONE_DEVICE: Option<&Device> = None;

/// Trait containing all `Device` methods.
///
/// # Implementors
///
/// [`Device`](struct.Device.html), [`FakeDevice`](struct.FakeDevice.html), [`GvDevice`](struct.GvDevice.html), [`UvDevice`](struct.UvDevice.html)
pub trait DeviceExt: 'static {
	/// Create a `ChunkParser` object, to be used for chunk data extraction from `Buffer`.
	///
	/// # Returns
	///
	/// a new `ChunkParser` object, NULL on error.
	fn create_chunk_parser(&self) -> Option<ChunkParser>;

	/// Get all the available values of `feature`, as integers.
	/// ## `feature`
	/// feature name
	/// ## `n_values`
	/// placeholder for the number of returned values
	///
	/// # Returns
	///
	/// a newly created array of 64 bit integers, which must freed after use using g_free,
	/// or NULL on error.
	fn dup_available_enumeration_feature_values(
		&self,
		feature: &str,
	) -> Result<Vec<i64>, glib::Error>;

	/// Get display names of all the available entries of `feature`.
	/// ## `feature`
	/// feature name
	/// ## `n_values`
	/// placeholder for the number of returned values
	///
	/// # Returns
	///
	/// a newly created array of const strings, to be freed after use using g_free, or
	/// `None` on error.
	fn dup_available_enumeration_feature_values_as_display_names(
		&self,
		feature: &str,
	) -> Result<Vec<GString>, glib::Error>;

	/// Get all the available values of `feature`, as strings.
	/// ## `feature`
	/// feature name
	/// ## `n_values`
	/// placeholder for the number of returned values
	///
	/// # Returns
	///
	/// a newly created array of const strings, which must freed after use using g_free,
	/// or NULL on error.
	fn dup_available_enumeration_feature_values_as_strings(
		&self,
		feature: &str,
	) -> Result<Vec<GString>, glib::Error>;

	/// Execute a genicam command.
	/// ## `feature`
	/// feature name
	fn execute_command(&self, feature: &str) -> Result<(), glib::Error>;

	/// ## `feature`
	/// feature name
	///
	/// # Returns
	///
	/// the feature value, `false` on error.
	fn get_boolean_feature_value(&self, feature: &str) -> Result<bool, glib::Error>;

	/// ## `feature`
	/// feature name
	///
	/// # Returns
	///
	/// the genicam node corresponding to the feature name, NULL if not found.
	fn get_feature(&self, feature: &str) -> Option<GcNode>;

	/// Retrieves feature bounds.
	/// ## `feature`
	/// feature name
	/// ## `min`
	/// minimum feature value
	/// ## `max`
	/// maximum feature value
	fn get_float_feature_bounds(&self, feature: &str) -> Result<(f64, f64), glib::Error>;

	/// ## `feature`
	/// feature name
	///
	/// # Returns
	///
	/// the float feature value, 0.0 on error.
	fn get_float_feature_value(&self, feature: &str) -> Result<f64, glib::Error>;

	/// Retrieves the genicam interface of the given device.
	///
	/// # Returns
	///
	/// the genicam interface.
	fn get_genicam(&self) -> Option<Gc>;

	/// Gets the Genicam XML data stored in the device memory.
	/// ## `size`
	/// placeholder for the returned data size (bytes)
	///
	/// # Returns
	///
	/// a pointer to the Genicam XML data, owned by the device.
	fn get_genicam_xml(&self) -> (GString, usize);

	/// Retrieves feature bounds.
	/// ## `feature`
	/// feature name
	/// ## `min`
	/// minimum feature value
	/// ## `max`
	/// maximum feature value
	fn get_integer_feature_bounds(&self, feature: &str) -> Result<(i64, i64), glib::Error>;

	/// Not all integer features have evenly distributed allowed values, which means the returned increment may not reflect the allowed value
	/// set.
	/// ## `feature`
	/// feature name
	///
	/// # Returns
	///
	/// feature value increment, or 1 on error.
	fn get_integer_feature_increment(&self, feature: &str) -> Result<i64, glib::Error>;

	/// ## `feature`
	/// feature name
	///
	/// # Returns
	///
	/// the integer feature value, 0 on error.
	fn get_integer_feature_value(&self, feature: &str) -> Result<i64, glib::Error>;

	/// ## `feature`
	/// feature name
	///
	/// # Returns
	///
	/// the string feature value, `None` on error.
	fn get_string_feature_value(&self, feature: &str) -> Result<GString, glib::Error>;

	/// ## `feature`
	/// feature name
	///
	/// # Returns
	///
	/// `true` if feature is available, `false` if not or on error.
	fn is_feature_available(&self, feature: &str) -> Result<bool, glib::Error>;

	//fn read_memory(&self, address: u64, size: u32, buffer: /*Unimplemented*/Option<Fundamental: Pointer>) -> Result<bool, glib::Error>;

	/// Reads the value of a device register.
	/// ## `address`
	/// register address
	/// ## `value`
	/// a placeholder for the read value
	///
	/// # Returns
	///
	/// TRUE on success.
	fn read_register(&self, address: u64) -> Result<(bool, u32), glib::Error>;

	/// Set the value of a boolean feature.
	/// ## `feature`
	/// feature name
	/// ## `value`
	/// feature value
	fn set_boolean_feature_value(&self, feature: &str, value: bool) -> Result<(), glib::Error>;

	/// Set features from a string containing a list of space separated feature assignments or command names. For example:
	///
	///
	/// ```C
	/// arv_device_set_features_from_string (device, "Width=256 Height=256 PixelFormat='Mono8' TriggerStart", &error);
	/// ```
	/// ## `string`
	/// a space separated list of features assignments
	fn set_features_from_string(&self, string: &str) -> Result<(), glib::Error>;

	/// Set the float feature value.
	/// ## `feature`
	/// feature name
	/// ## `value`
	/// new feature value
	fn set_float_feature_value(&self, feature: &str, value: f64) -> Result<(), glib::Error>;

	/// Set the integer feature value.
	/// ## `feature`
	/// feature name
	/// ## `value`
	/// new feature value
	fn set_integer_feature_value(&self, feature: &str, value: i64) -> Result<(), glib::Error>;

	/// Sets the register cache policy.
	///
	/// `<warning>``<para>`Be aware that some camera may have wrong Cachable properties defined in their Genicam metadata, which may
	/// lead to incorrect readouts. Using the debug cache policy, and activating genicam debug output (export ARV_DEBUG=genicam), can help you to
	/// check the cache validity. In this mode, every time the cache content is not in sync with the actual register value, a debug message is
	/// printed on the console.`</para>``</warning>`
	/// ## `policy`
	/// cache policy
	fn set_register_cache_policy(&self, policy: RegisterCachePolicy);

	/// Set the string feature value.
	/// ## `feature`
	/// feature name
	/// ## `value`
	/// new feature value
	fn set_string_feature_value(&self, feature: &str, value: &str) -> Result<(), glib::Error>;

	//fn write_memory(&self, address: u64, size: u32, buffer: /*Unimplemented*/Option<Fundamental: Pointer>) -> Result<bool, glib::Error>;

	/// Writes `value` to a device register.
	/// ## `address`
	/// the register address
	/// ## `value`
	/// value to write
	///
	/// # Returns
	///
	/// TRUE on success.
	fn write_register(&self, address: u64, value: u32) -> Result<bool, glib::Error>;

	/// Signal that the control of the device is lost.
	///
	/// This signal may be emited from a thread different than the main one,
	/// so please take care to shared data access from the callback.
	fn connect_control_lost<F: Fn(&Self) + Send + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Device>> DeviceExt for O {
	fn create_chunk_parser(&self) -> Option<ChunkParser> {
		unsafe {
			from_glib_full(aravis_sys::arv_device_create_chunk_parser(
				self.as_ref().to_glib_none().0,
			))
		}
	}

	fn dup_available_enumeration_feature_values(
		&self,
		feature: &str,
	) -> Result<Vec<i64>, glib::Error> {
		unsafe {
			let mut n_values = mem::MaybeUninit::uninit();
			let mut error = ptr::null_mut();
			let ret = aravis_sys::arv_device_dup_available_enumeration_feature_values(
				self.as_ref().to_glib_none().0,
				feature.to_glib_none().0,
				n_values.as_mut_ptr(),
				&mut error,
			);
			if error.is_null() {
				Ok(FromGlibContainer::from_glib_container_num(
					ret,
					n_values.assume_init() as usize,
				))
			} else {
				Err(from_glib_full(error))
			}
		}
	}

	fn dup_available_enumeration_feature_values_as_display_names(
		&self,
		feature: &str,
	) -> Result<Vec<GString>, glib::Error> {
		unsafe {
			let mut n_values = mem::MaybeUninit::uninit();
			let mut error = ptr::null_mut();
			let ret =
				aravis_sys::arv_device_dup_available_enumeration_feature_values_as_display_names(
					self.as_ref().to_glib_none().0,
					feature.to_glib_none().0,
					n_values.as_mut_ptr(),
					&mut error,
				);
			if error.is_null() {
				Ok(FromGlibContainer::from_glib_container_num(
					ret,
					n_values.assume_init() as usize,
				))
			} else {
				Err(from_glib_full(error))
			}
		}
	}

	fn dup_available_enumeration_feature_values_as_strings(
		&self,
		feature: &str,
	) -> Result<Vec<GString>, glib::Error> {
		unsafe {
			let mut n_values = mem::MaybeUninit::uninit();
			let mut error = ptr::null_mut();
			let ret = aravis_sys::arv_device_dup_available_enumeration_feature_values_as_strings(
				self.as_ref().to_glib_none().0,
				feature.to_glib_none().0,
				n_values.as_mut_ptr(),
				&mut error,
			);
			if error.is_null() {
				Ok(FromGlibContainer::from_glib_container_num(
					ret,
					n_values.assume_init() as usize,
				))
			} else {
				Err(from_glib_full(error))
			}
		}
	}

	fn execute_command(&self, feature: &str) -> Result<(), glib::Error> {
		unsafe {
			let mut error = ptr::null_mut();
			let _ = aravis_sys::arv_device_execute_command(
				self.as_ref().to_glib_none().0,
				feature.to_glib_none().0,
				&mut error,
			);
			if error.is_null() {
				Ok(())
			} else {
				Err(from_glib_full(error))
			}
		}
	}

	fn get_boolean_feature_value(&self, feature: &str) -> Result<bool, glib::Error> {
		unsafe {
			let mut error = ptr::null_mut();
			let ret = aravis_sys::arv_device_get_boolean_feature_value(
				self.as_ref().to_glib_none().0,
				feature.to_glib_none().0,
				&mut error,
			);
			if error.is_null() {
				Ok(from_glib(ret))
			} else {
				Err(from_glib_full(error))
			}
		}
	}

	fn get_feature(&self, feature: &str) -> Option<GcNode> {
		unsafe {
			from_glib_none(aravis_sys::arv_device_get_feature(
				self.as_ref().to_glib_none().0,
				feature.to_glib_none().0,
			))
		}
	}

	fn get_float_feature_bounds(&self, feature: &str) -> Result<(f64, f64), glib::Error> {
		unsafe {
			let mut min = mem::MaybeUninit::uninit();
			let mut max = mem::MaybeUninit::uninit();
			let mut error = ptr::null_mut();
			let _ = aravis_sys::arv_device_get_float_feature_bounds(
				self.as_ref().to_glib_none().0,
				feature.to_glib_none().0,
				min.as_mut_ptr(),
				max.as_mut_ptr(),
				&mut error,
			);
			let min = min.assume_init();
			let max = max.assume_init();
			if error.is_null() {
				Ok((min, max))
			} else {
				Err(from_glib_full(error))
			}
		}
	}

	fn get_float_feature_value(&self, feature: &str) -> Result<f64, glib::Error> {
		unsafe {
			let mut error = ptr::null_mut();
			let ret = aravis_sys::arv_device_get_float_feature_value(
				self.as_ref().to_glib_none().0,
				feature.to_glib_none().0,
				&mut error,
			);
			if error.is_null() {
				Ok(ret)
			} else {
				Err(from_glib_full(error))
			}
		}
	}

	fn get_genicam(&self) -> Option<Gc> {
		unsafe {
			from_glib_none(aravis_sys::arv_device_get_genicam(
				self.as_ref().to_glib_none().0,
			))
		}
	}

	fn get_genicam_xml(&self) -> (GString, usize) {
		unsafe {
			let mut size = mem::MaybeUninit::uninit();
			let ret = from_glib_none(aravis_sys::arv_device_get_genicam_xml(
				self.as_ref().to_glib_none().0,
				size.as_mut_ptr(),
			));
			let size = size.assume_init();
			(ret, size)
		}
	}

	fn get_integer_feature_bounds(&self, feature: &str) -> Result<(i64, i64), glib::Error> {
		unsafe {
			let mut min = mem::MaybeUninit::uninit();
			let mut max = mem::MaybeUninit::uninit();
			let mut error = ptr::null_mut();
			let _ = aravis_sys::arv_device_get_integer_feature_bounds(
				self.as_ref().to_glib_none().0,
				feature.to_glib_none().0,
				min.as_mut_ptr(),
				max.as_mut_ptr(),
				&mut error,
			);
			let min = min.assume_init();
			let max = max.assume_init();
			if error.is_null() {
				Ok((min, max))
			} else {
				Err(from_glib_full(error))
			}
		}
	}

	fn get_integer_feature_increment(&self, feature: &str) -> Result<i64, glib::Error> {
		unsafe {
			let mut error = ptr::null_mut();
			let ret = aravis_sys::arv_device_get_integer_feature_increment(
				self.as_ref().to_glib_none().0,
				feature.to_glib_none().0,
				&mut error,
			);
			if error.is_null() {
				Ok(ret)
			} else {
				Err(from_glib_full(error))
			}
		}
	}

	fn get_integer_feature_value(&self, feature: &str) -> Result<i64, glib::Error> {
		unsafe {
			let mut error = ptr::null_mut();
			let ret = aravis_sys::arv_device_get_integer_feature_value(
				self.as_ref().to_glib_none().0,
				feature.to_glib_none().0,
				&mut error,
			);
			if error.is_null() {
				Ok(ret)
			} else {
				Err(from_glib_full(error))
			}
		}
	}

	fn get_string_feature_value(&self, feature: &str) -> Result<GString, glib::Error> {
		unsafe {
			let mut error = ptr::null_mut();
			let ret = aravis_sys::arv_device_get_string_feature_value(
				self.as_ref().to_glib_none().0,
				feature.to_glib_none().0,
				&mut error,
			);
			if error.is_null() {
				Ok(from_glib_none(ret))
			} else {
				Err(from_glib_full(error))
			}
		}
	}

	fn is_feature_available(&self, feature: &str) -> Result<bool, glib::Error> {
		unsafe {
			let mut error = ptr::null_mut();
			let ret = aravis_sys::arv_device_is_feature_available(
				self.as_ref().to_glib_none().0,
				feature.to_glib_none().0,
				&mut error,
			);
			if error.is_null() {
				Ok(from_glib(ret))
			} else {
				Err(from_glib_full(error))
			}
		}
	}

	//fn read_memory(&self, address: u64, size: u32, buffer: /*Unimplemented*/Option<Fundamental: Pointer>) -> Result<bool, glib::Error> {
	//    unsafe { TODO: call aravis_sys:arv_device_read_memory() }
	//}

	fn read_register(&self, address: u64) -> Result<(bool, u32), glib::Error> {
		unsafe {
			let mut value = mem::MaybeUninit::uninit();
			let mut error = ptr::null_mut();
			let ret = aravis_sys::arv_device_read_register(
				self.as_ref().to_glib_none().0,
				address,
				value.as_mut_ptr(),
				&mut error,
			);
			let value = value.assume_init();
			if error.is_null() {
				Ok((from_glib(ret), value))
			} else {
				Err(from_glib_full(error))
			}
		}
	}

	fn set_boolean_feature_value(&self, feature: &str, value: bool) -> Result<(), glib::Error> {
		unsafe {
			let mut error = ptr::null_mut();
			let _ = aravis_sys::arv_device_set_boolean_feature_value(
				self.as_ref().to_glib_none().0,
				feature.to_glib_none().0,
				value.to_glib(),
				&mut error,
			);
			if error.is_null() {
				Ok(())
			} else {
				Err(from_glib_full(error))
			}
		}
	}

	fn set_features_from_string(&self, string: &str) -> Result<(), glib::Error> {
		unsafe {
			let mut error = ptr::null_mut();
			let _ = aravis_sys::arv_device_set_features_from_string(
				self.as_ref().to_glib_none().0,
				string.to_glib_none().0,
				&mut error,
			);
			if error.is_null() {
				Ok(())
			} else {
				Err(from_glib_full(error))
			}
		}
	}

	fn set_float_feature_value(&self, feature: &str, value: f64) -> Result<(), glib::Error> {
		unsafe {
			let mut error = ptr::null_mut();
			let _ = aravis_sys::arv_device_set_float_feature_value(
				self.as_ref().to_glib_none().0,
				feature.to_glib_none().0,
				value,
				&mut error,
			);
			if error.is_null() {
				Ok(())
			} else {
				Err(from_glib_full(error))
			}
		}
	}

	fn set_integer_feature_value(&self, feature: &str, value: i64) -> Result<(), glib::Error> {
		unsafe {
			let mut error = ptr::null_mut();
			let _ = aravis_sys::arv_device_set_integer_feature_value(
				self.as_ref().to_glib_none().0,
				feature.to_glib_none().0,
				value,
				&mut error,
			);
			if error.is_null() {
				Ok(())
			} else {
				Err(from_glib_full(error))
			}
		}
	}

	fn set_register_cache_policy(&self, policy: RegisterCachePolicy) {
		unsafe {
			aravis_sys::arv_device_set_register_cache_policy(
				self.as_ref().to_glib_none().0,
				policy.to_glib(),
			);
		}
	}

	fn set_string_feature_value(&self, feature: &str, value: &str) -> Result<(), glib::Error> {
		unsafe {
			let mut error = ptr::null_mut();
			let _ = aravis_sys::arv_device_set_string_feature_value(
				self.as_ref().to_glib_none().0,
				feature.to_glib_none().0,
				value.to_glib_none().0,
				&mut error,
			);
			if error.is_null() {
				Ok(())
			} else {
				Err(from_glib_full(error))
			}
		}
	}

	//fn write_memory(&self, address: u64, size: u32, buffer: /*Unimplemented*/Option<Fundamental: Pointer>) -> Result<bool, glib::Error> {
	//    unsafe { TODO: call aravis_sys:arv_device_write_memory() }
	//}

	fn write_register(&self, address: u64, value: u32) -> Result<bool, glib::Error> {
		unsafe {
			let mut error = ptr::null_mut();
			let ret = aravis_sys::arv_device_write_register(
				self.as_ref().to_glib_none().0,
				address,
				value,
				&mut error,
			);
			if error.is_null() {
				Ok(from_glib(ret))
			} else {
				Err(from_glib_full(error))
			}
		}
	}

	fn connect_control_lost<F: Fn(&Self) + Send + 'static>(&self, f: F) -> SignalHandlerId {
		unsafe extern "C" fn control_lost_trampoline<P, F: Fn(&P) + Send + 'static>(
			this: *mut aravis_sys::ArvDevice,
			f: glib_sys::gpointer,
		) where
			P: IsA<Device>,
		{
			let f: &F = &*(f as *const F);
			f(&Device::from_glib_borrow(this).unsafe_cast_ref())
		}
		unsafe {
			let f: Box_<F> = Box_::new(f);
			connect_raw(
				self.as_ptr() as *mut _,
				b"control-lost\0".as_ptr() as *const _,
				Some(transmute::<_, unsafe extern "C" fn()>(
					control_lost_trampoline::<Self, F> as *const (),
				)),
				Box_::into_raw(f),
			)
		}
	}
}

impl fmt::Display for Device {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "Device")
	}
}
