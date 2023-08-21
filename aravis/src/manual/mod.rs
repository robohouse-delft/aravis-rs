use std::ffi::CString;
use std::sync::atomic::{AtomicBool, Ordering};

mod buffer;
pub use self::buffer::*;

mod camera;
mod device;
mod register;

mod pixel_format;
pub use self::pixel_format::*;

#[doc(hidden)]
pub(crate) mod traits {
	pub use super::camera::traits::CameraExtManual;
	pub use super::device::traits::DeviceExtManual;
	pub use super::register::traits::GcRegisterExtManual;
}

pub type BoxImage<P> = image::ImageBuffer<P, Box<[u8]>>;
pub type RcImage<P> = image::ImageBuffer<P, std::rc::Rc<[u8]>>;
pub type ArcImage<P> = image::ImageBuffer<P, std::sync::Arc<[u8]>>;

static INITIALIZED: AtomicBool = AtomicBool::new(false);

/// Token representing access to the global state of the Aravis library.
pub struct Aravis {
	_phantom: (),
}

/// Error returned when [`Aravis::initialize`] is called more than once.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct AlreadyInitializedError;

/// Information identifying a GenICam camera.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DeviceInfo {
	pub id: CString,
	pub physical_id: CString,
	pub vendor: CString,
	pub model: CString,
	pub protocol: CString,
	pub address: CString,
}

impl Aravis {
	/// Retrieve a token to access global state of the Aravis library.
	///
	/// This may only be called once.
	/// Any subsequent invocation after the first will return a [`AlreadyInitializedError`].
	#[allow(clippy::bool_comparison)]
	pub fn initialize() -> Result<Aravis, AlreadyInitializedError> {
		if INITIALIZED.swap(true, Ordering::AcqRel) == true {
			Err(AlreadyInitializedError)
		} else {
			Ok(Aravis { _phantom: () })
		}
	}

	/// Enumerate all available GenICam devices.
	pub fn get_device_list(&self) -> Vec<DeviceInfo> {
		unsafe { get_device_list() }
	}
}

/// Enumerate all available GenICam devices without mutual exclusion.
///
/// # Safety
/// This function is unsafe because it accesses global state of the Aravis library,
/// without guarantee that this thread is the only one accessing it.
///
/// See [`Aravis::get_device_list`] for a safe alternative.
pub unsafe fn get_device_list() -> Vec<DeviceInfo> {
	aravis_sys::arv_update_device_list();
	let count = aravis_sys::arv_get_n_devices();

	(0..count)
		.map(|i| DeviceInfo {
			id: std::ffi::CStr::from_ptr(aravis_sys::arv_get_device_id(i)).into(),
			physical_id: std::ffi::CStr::from_ptr(aravis_sys::arv_get_device_physical_id(i)).into(),
			vendor: std::ffi::CStr::from_ptr(aravis_sys::arv_get_device_vendor(i)).into(),
			model: std::ffi::CStr::from_ptr(aravis_sys::arv_get_device_model(i)).into(),
			protocol: std::ffi::CStr::from_ptr(aravis_sys::arv_get_device_protocol(i)).into(),
			address: std::ffi::CStr::from_ptr(aravis_sys::arv_get_device_address(i)).into(),
		})
		.collect()
}

impl std::fmt::Display for AlreadyInitializedError {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "Aravis has already been initialized")
	}
}

impl std::error::Error for AlreadyInitializedError {}
