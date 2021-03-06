// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use aravis_sys;
use gio;
use glib::object::IsA;
use glib::translate::*;
use glib::GString;
use std::fmt;
use std::mem;
use Buffer;

glib_wrapper! {
	pub struct FakeCamera(Object<aravis_sys::ArvFakeCamera, aravis_sys::ArvFakeCameraClass, FakeCameraClass>);

	match fn {
		get_type => || aravis_sys::arv_fake_camera_get_type(),
	}
}

impl FakeCamera {
	pub fn new(serial_number: &str) -> FakeCamera {
		assert_initialized_main_thread!();
		unsafe {
			from_glib_full(aravis_sys::arv_fake_camera_new(
				serial_number.to_glib_none().0,
			))
		}
	}

	pub fn new_full(serial_number: &str, genicam_filename: &str) -> FakeCamera {
		assert_initialized_main_thread!();
		unsafe {
			from_glib_full(aravis_sys::arv_fake_camera_new_full(
				serial_number.to_glib_none().0,
				genicam_filename.to_glib_none().0,
			))
		}
	}
}

unsafe impl Send for FakeCamera {}

pub const NONE_FAKE_CAMERA: Option<&FakeCamera> = None;

/// Trait containing all `FakeCamera` methods.
///
/// # Implementors
///
/// [`FakeCamera`](struct.FakeCamera.html)
pub trait FakeCameraExt: 'static {
	/// Fill a buffer with data from the fake camera.
	/// ## `buffer`
	/// the `Buffer` to fill
	/// ## `packet_size`
	/// the packet size
	fn fill_buffer<P: IsA<Buffer>>(&self, buffer: &P) -> u32;

	fn get_acquisition_status(&self) -> u32;

	fn get_control_channel_privilege(&self) -> u32;

	/// ## `size`
	/// the size of the returned XML string
	///
	/// # Returns
	///
	/// the genicam XML description of the camera
	fn get_genicam_xml(&self) -> (GString, usize);

	fn get_heartbeat_timeout(&self) -> u32;

	fn get_payload(&self) -> usize;

	/// ## `next_timestamp_us`
	/// the timestamp for the next frame in microseconds
	///
	/// # Returns
	///
	/// the sleep time for the next frame
	fn get_sleep_time_for_next_frame(&self) -> (u64, u64);

	///
	/// # Returns
	///
	/// the data stream `gio::SocketAddress` for this camera
	fn get_stream_address(&self) -> Option<gio::SocketAddress>;

	//fn read_memory(&self, address: u32, size: u32, buffer: /*Unimplemented*/Option<Fundamental: Pointer>) -> bool;

	/// ## `address`
	/// the register address
	/// ## `value`
	/// the register value
	///
	/// # Returns
	///
	/// true if the read succeeded, false otherwise
	fn read_register(&self, address: u32) -> Option<u32>;

	fn set_control_channel_privilege(&self, privilege: u32);

	fn set_inet_address<P: IsA<gio::InetAddress>>(&self, address: &P);

	fn set_trigger_frequency(&self, frequency: f64);

	fn wait_for_next_frame(&self);

	//fn write_memory(&self, address: u32, size: u32, buffer: /*Unimplemented*/Option<Fundamental: Pointer>) -> bool;

	fn write_register(&self, address: u32, value: u32) -> bool;
}

impl<O: IsA<FakeCamera>> FakeCameraExt for O {
	fn fill_buffer<P: IsA<Buffer>>(&self, buffer: &P) -> u32 {
		unsafe {
			let mut packet_size = mem::MaybeUninit::uninit();
			aravis_sys::arv_fake_camera_fill_buffer(
				self.as_ref().to_glib_none().0,
				buffer.as_ref().to_glib_none().0,
				packet_size.as_mut_ptr(),
			);
			let packet_size = packet_size.assume_init();
			packet_size
		}
	}

	fn get_acquisition_status(&self) -> u32 {
		unsafe {
			aravis_sys::arv_fake_camera_get_acquisition_status(self.as_ref().to_glib_none().0)
		}
	}

	fn get_control_channel_privilege(&self) -> u32 {
		unsafe {
			aravis_sys::arv_fake_camera_get_control_channel_privilege(
				self.as_ref().to_glib_none().0,
			)
		}
	}

	fn get_genicam_xml(&self) -> (GString, usize) {
		unsafe {
			let mut size = mem::MaybeUninit::uninit();
			let ret = from_glib_none(aravis_sys::arv_fake_camera_get_genicam_xml(
				self.as_ref().to_glib_none().0,
				size.as_mut_ptr(),
			));
			let size = size.assume_init();
			(ret, size)
		}
	}

	fn get_heartbeat_timeout(&self) -> u32 {
		unsafe { aravis_sys::arv_fake_camera_get_heartbeat_timeout(self.as_ref().to_glib_none().0) }
	}

	fn get_payload(&self) -> usize {
		unsafe { aravis_sys::arv_fake_camera_get_payload(self.as_ref().to_glib_none().0) }
	}

	fn get_sleep_time_for_next_frame(&self) -> (u64, u64) {
		unsafe {
			let mut next_timestamp_us = mem::MaybeUninit::uninit();
			let ret = aravis_sys::arv_fake_camera_get_sleep_time_for_next_frame(
				self.as_ref().to_glib_none().0,
				next_timestamp_us.as_mut_ptr(),
			);
			let next_timestamp_us = next_timestamp_us.assume_init();
			(ret, next_timestamp_us)
		}
	}

	fn get_stream_address(&self) -> Option<gio::SocketAddress> {
		unsafe {
			from_glib_full(aravis_sys::arv_fake_camera_get_stream_address(
				self.as_ref().to_glib_none().0,
			))
		}
	}

	//fn read_memory(&self, address: u32, size: u32, buffer: /*Unimplemented*/Option<Fundamental: Pointer>) -> bool {
	//    unsafe { TODO: call aravis_sys:arv_fake_camera_read_memory() }
	//}

	fn read_register(&self, address: u32) -> Option<u32> {
		unsafe {
			let mut value = mem::MaybeUninit::uninit();
			let ret = from_glib(aravis_sys::arv_fake_camera_read_register(
				self.as_ref().to_glib_none().0,
				address,
				value.as_mut_ptr(),
			));
			let value = value.assume_init();
			if ret {
				Some(value)
			} else {
				None
			}
		}
	}

	fn set_control_channel_privilege(&self, privilege: u32) {
		unsafe {
			aravis_sys::arv_fake_camera_set_control_channel_privilege(
				self.as_ref().to_glib_none().0,
				privilege,
			);
		}
	}

	fn set_inet_address<P: IsA<gio::InetAddress>>(&self, address: &P) {
		unsafe {
			aravis_sys::arv_fake_camera_set_inet_address(
				self.as_ref().to_glib_none().0,
				address.as_ref().to_glib_none().0,
			);
		}
	}

	fn set_trigger_frequency(&self, frequency: f64) {
		unsafe {
			aravis_sys::arv_fake_camera_set_trigger_frequency(
				self.as_ref().to_glib_none().0,
				frequency,
			);
		}
	}

	fn wait_for_next_frame(&self) {
		unsafe {
			aravis_sys::arv_fake_camera_wait_for_next_frame(self.as_ref().to_glib_none().0);
		}
	}

	//fn write_memory(&self, address: u32, size: u32, buffer: /*Unimplemented*/Option<Fundamental: Pointer>) -> bool {
	//    unsafe { TODO: call aravis_sys:arv_fake_camera_write_memory() }
	//}

	fn write_register(&self, address: u32, value: u32) -> bool {
		unsafe {
			from_glib(aravis_sys::arv_fake_camera_write_register(
				self.as_ref().to_glib_none().0,
				address,
				value,
			))
		}
	}
}

impl fmt::Display for FakeCamera {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "FakeCamera")
	}
}
