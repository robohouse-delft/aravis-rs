// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ../gir-files
// DO NOT EDIT

use crate::{ffi, GvStreamPacketResend, GvStreamSocketBuffer, Stream};
use glib::{
	prelude::*,
	signal::{connect_raw, SignalHandlerId},
	translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
///
///
/// ## Properties
///
///
/// #### `frame-retention`
///  Amount of time Aravis is wating for frame completion after the last packet is received. A greater value will
/// also increase the maximum frame latency in case of missing packets.
///
/// Readable | Writeable | Construct
///
///
/// #### `initial-packet-timeout`
///  Delay before asking for a packet resend after the packet was detected missing for the first time. The reason
/// for this delay is, depending on the network topology, stream packets are not always received in increasing id
/// order. As the missing packet detection happens at each received packet, by verifying if each previous packet
/// has been received, we could emit useless packet resend requests if they are not ordered.
///
/// Readable | Writeable | Construct
///
///
/// #### `packet-request-ratio`
///  Maximum number of packet resend requests for a given frame, as a percentage of the number of packets per
/// frame.
///
/// Readable | Writeable | Construct
///
///
/// #### `packet-resend`
///  Packet resend policy. This only applies if the device supports packet resend.
///
/// Readable | Writeable | Construct
///
///
/// #### `packet-timeout`
///  Timeout while waiting for a packet after a resend request, before asking again.
///
/// Readable | Writeable | Construct
///
///
/// #### `socket-buffer`
///  Incoming socket buffer policy.
///
/// Readable | Writeable | Construct
///
///
/// #### `socket-buffer-size`
///  Size in bytes of the incoming socket buffer. A greater value helps to lower the number of missings packets,
/// as the expense of an increased memory usage.
///
/// Readable | Writeable | Construct
/// <details><summary><h4>Stream</h4></summary>
///
///
/// #### `callback`
///  Readable | Writeable | Construct Only
///
///
/// #### `callback-data`
///  Readable | Writeable | Construct Only
///
///
/// #### `destroy-notify`
///  Readable | Writeable | Construct Only
///
///
/// #### `device`
///  Readable | Writeable | Construct Only
///
///
/// #### `emit-signals`
///  Readable | Writeable
/// </details>
///
/// # Implements
///
/// [`StreamExt`][trait@crate::prelude::StreamExt], [`trait@glib::ObjectExt`]
	#[doc(alias = "ArvGvStream")]
	pub struct GvStream(Object<ffi::ArvGvStream, ffi::ArvGvStreamClass>) @extends Stream;

	match fn {
		type_ => || ffi::arv_gv_stream_get_type(),
	}
}

impl GvStream {
	#[doc(alias = "arv_gv_stream_get_port")]
	#[doc(alias = "get_port")]
	pub fn port(&self) -> u16 {
		unsafe { ffi::arv_gv_stream_get_port(self.to_glib_none().0) }
	}

	///
	/// # Returns
	///
	#[doc(alias = "arv_gv_stream_get_statistics")]
	#[doc(alias = "get_statistics")]
	pub fn statistics(&self) -> (u64, u64) {
		unsafe {
			let mut n_resent_packets = std::mem::MaybeUninit::uninit();
			let mut n_missing_packets = std::mem::MaybeUninit::uninit();
			ffi::arv_gv_stream_get_statistics(
				self.to_glib_none().0,
				n_resent_packets.as_mut_ptr(),
				n_missing_packets.as_mut_ptr(),
			);
			(
				n_resent_packets.assume_init(),
				n_missing_packets.assume_init(),
			)
		}
	}

	/// Amount of time Aravis is wating for frame completion after the last packet is received. A greater value will
	/// also increase the maximum frame latency in case of missing packets.
	#[doc(alias = "frame-retention")]
	pub fn frame_retention(&self) -> u32 {
		ObjectExt::property(self, "frame-retention")
	}

	/// Amount of time Aravis is wating for frame completion after the last packet is received. A greater value will
	/// also increase the maximum frame latency in case of missing packets.
	#[doc(alias = "frame-retention")]
	pub fn set_frame_retention(&self, frame_retention: u32) {
		ObjectExt::set_property(self, "frame-retention", frame_retention)
	}

	/// Delay before asking for a packet resend after the packet was detected missing for the first time. The reason
	/// for this delay is, depending on the network topology, stream packets are not always received in increasing id
	/// order. As the missing packet detection happens at each received packet, by verifying if each previous packet
	/// has been received, we could emit useless packet resend requests if they are not ordered.
	#[cfg(feature = "v0_8_15")]
	#[cfg_attr(docsrs, doc(cfg(feature = "v0_8_15")))]
	#[doc(alias = "initial-packet-timeout")]
	pub fn initial_packet_timeout(&self) -> u32 {
		ObjectExt::property(self, "initial-packet-timeout")
	}

	/// Delay before asking for a packet resend after the packet was detected missing for the first time. The reason
	/// for this delay is, depending on the network topology, stream packets are not always received in increasing id
	/// order. As the missing packet detection happens at each received packet, by verifying if each previous packet
	/// has been received, we could emit useless packet resend requests if they are not ordered.
	#[cfg(feature = "v0_8_15")]
	#[cfg_attr(docsrs, doc(cfg(feature = "v0_8_15")))]
	#[doc(alias = "initial-packet-timeout")]
	pub fn set_initial_packet_timeout(&self, initial_packet_timeout: u32) {
		ObjectExt::set_property(self, "initial-packet-timeout", initial_packet_timeout)
	}

	/// Maximum number of packet resend requests for a given frame, as a percentage of the number of packets per
	/// frame.
	#[doc(alias = "packet-request-ratio")]
	pub fn packet_request_ratio(&self) -> f64 {
		ObjectExt::property(self, "packet-request-ratio")
	}

	/// Maximum number of packet resend requests for a given frame, as a percentage of the number of packets per
	/// frame.
	#[doc(alias = "packet-request-ratio")]
	pub fn set_packet_request_ratio(&self, packet_request_ratio: f64) {
		ObjectExt::set_property(self, "packet-request-ratio", packet_request_ratio)
	}

	/// Packet resend policy. This only applies if the device supports packet resend.
	#[doc(alias = "packet-resend")]
	pub fn packet_resend(&self) -> GvStreamPacketResend {
		ObjectExt::property(self, "packet-resend")
	}

	/// Packet resend policy. This only applies if the device supports packet resend.
	#[doc(alias = "packet-resend")]
	pub fn set_packet_resend(&self, packet_resend: GvStreamPacketResend) {
		ObjectExt::set_property(self, "packet-resend", packet_resend)
	}

	/// Timeout while waiting for a packet after a resend request, before asking again.
	#[doc(alias = "packet-timeout")]
	pub fn packet_timeout(&self) -> u32 {
		ObjectExt::property(self, "packet-timeout")
	}

	/// Timeout while waiting for a packet after a resend request, before asking again.
	#[doc(alias = "packet-timeout")]
	pub fn set_packet_timeout(&self, packet_timeout: u32) {
		ObjectExt::set_property(self, "packet-timeout", packet_timeout)
	}

	/// Incoming socket buffer policy.
	#[doc(alias = "socket-buffer")]
	pub fn socket_buffer(&self) -> GvStreamSocketBuffer {
		ObjectExt::property(self, "socket-buffer")
	}

	/// Incoming socket buffer policy.
	#[doc(alias = "socket-buffer")]
	pub fn set_socket_buffer(&self, socket_buffer: GvStreamSocketBuffer) {
		ObjectExt::set_property(self, "socket-buffer", socket_buffer)
	}

	/// Size in bytes of the incoming socket buffer. A greater value helps to lower the number of missings packets,
	/// as the expense of an increased memory usage.
	#[doc(alias = "socket-buffer-size")]
	pub fn socket_buffer_size(&self) -> i32 {
		ObjectExt::property(self, "socket-buffer-size")
	}

	/// Size in bytes of the incoming socket buffer. A greater value helps to lower the number of missings packets,
	/// as the expense of an increased memory usage.
	#[doc(alias = "socket-buffer-size")]
	pub fn set_socket_buffer_size(&self, socket_buffer_size: i32) {
		ObjectExt::set_property(self, "socket-buffer-size", socket_buffer_size)
	}

	#[doc(alias = "frame-retention")]
	pub fn connect_frame_retention_notify<F: Fn(&Self) + Send + 'static>(
		&self,
		f: F,
	) -> SignalHandlerId {
		unsafe extern "C" fn notify_frame_retention_trampoline<
			F: Fn(&GvStream) + Send + 'static,
		>(
			this: *mut ffi::ArvGvStream,
			_param_spec: glib::ffi::gpointer,
			f: glib::ffi::gpointer,
		) {
			let f: &F = &*(f as *const F);
			f(&from_glib_borrow(this))
		}
		unsafe {
			let f: Box_<F> = Box_::new(f);
			connect_raw(
				self.as_ptr() as *mut _,
				b"notify::frame-retention\0".as_ptr() as *const _,
				Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
					notify_frame_retention_trampoline::<F> as *const (),
				)),
				Box_::into_raw(f),
			)
		}
	}

	#[cfg(feature = "v0_8_15")]
	#[cfg_attr(docsrs, doc(cfg(feature = "v0_8_15")))]
	#[doc(alias = "initial-packet-timeout")]
	pub fn connect_initial_packet_timeout_notify<F: Fn(&Self) + Send + 'static>(
		&self,
		f: F,
	) -> SignalHandlerId {
		unsafe extern "C" fn notify_initial_packet_timeout_trampoline<
			F: Fn(&GvStream) + Send + 'static,
		>(
			this: *mut ffi::ArvGvStream,
			_param_spec: glib::ffi::gpointer,
			f: glib::ffi::gpointer,
		) {
			let f: &F = &*(f as *const F);
			f(&from_glib_borrow(this))
		}
		unsafe {
			let f: Box_<F> = Box_::new(f);
			connect_raw(
				self.as_ptr() as *mut _,
				b"notify::initial-packet-timeout\0".as_ptr() as *const _,
				Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
					notify_initial_packet_timeout_trampoline::<F> as *const (),
				)),
				Box_::into_raw(f),
			)
		}
	}

	#[doc(alias = "packet-request-ratio")]
	pub fn connect_packet_request_ratio_notify<F: Fn(&Self) + Send + 'static>(
		&self,
		f: F,
	) -> SignalHandlerId {
		unsafe extern "C" fn notify_packet_request_ratio_trampoline<
			F: Fn(&GvStream) + Send + 'static,
		>(
			this: *mut ffi::ArvGvStream,
			_param_spec: glib::ffi::gpointer,
			f: glib::ffi::gpointer,
		) {
			let f: &F = &*(f as *const F);
			f(&from_glib_borrow(this))
		}
		unsafe {
			let f: Box_<F> = Box_::new(f);
			connect_raw(
				self.as_ptr() as *mut _,
				b"notify::packet-request-ratio\0".as_ptr() as *const _,
				Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
					notify_packet_request_ratio_trampoline::<F> as *const (),
				)),
				Box_::into_raw(f),
			)
		}
	}

	#[doc(alias = "packet-resend")]
	pub fn connect_packet_resend_notify<F: Fn(&Self) + Send + 'static>(
		&self,
		f: F,
	) -> SignalHandlerId {
		unsafe extern "C" fn notify_packet_resend_trampoline<F: Fn(&GvStream) + Send + 'static>(
			this: *mut ffi::ArvGvStream,
			_param_spec: glib::ffi::gpointer,
			f: glib::ffi::gpointer,
		) {
			let f: &F = &*(f as *const F);
			f(&from_glib_borrow(this))
		}
		unsafe {
			let f: Box_<F> = Box_::new(f);
			connect_raw(
				self.as_ptr() as *mut _,
				b"notify::packet-resend\0".as_ptr() as *const _,
				Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
					notify_packet_resend_trampoline::<F> as *const (),
				)),
				Box_::into_raw(f),
			)
		}
	}

	#[doc(alias = "packet-timeout")]
	pub fn connect_packet_timeout_notify<F: Fn(&Self) + Send + 'static>(
		&self,
		f: F,
	) -> SignalHandlerId {
		unsafe extern "C" fn notify_packet_timeout_trampoline<F: Fn(&GvStream) + Send + 'static>(
			this: *mut ffi::ArvGvStream,
			_param_spec: glib::ffi::gpointer,
			f: glib::ffi::gpointer,
		) {
			let f: &F = &*(f as *const F);
			f(&from_glib_borrow(this))
		}
		unsafe {
			let f: Box_<F> = Box_::new(f);
			connect_raw(
				self.as_ptr() as *mut _,
				b"notify::packet-timeout\0".as_ptr() as *const _,
				Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
					notify_packet_timeout_trampoline::<F> as *const (),
				)),
				Box_::into_raw(f),
			)
		}
	}

	#[doc(alias = "socket-buffer")]
	pub fn connect_socket_buffer_notify<F: Fn(&Self) + Send + 'static>(
		&self,
		f: F,
	) -> SignalHandlerId {
		unsafe extern "C" fn notify_socket_buffer_trampoline<F: Fn(&GvStream) + Send + 'static>(
			this: *mut ffi::ArvGvStream,
			_param_spec: glib::ffi::gpointer,
			f: glib::ffi::gpointer,
		) {
			let f: &F = &*(f as *const F);
			f(&from_glib_borrow(this))
		}
		unsafe {
			let f: Box_<F> = Box_::new(f);
			connect_raw(
				self.as_ptr() as *mut _,
				b"notify::socket-buffer\0".as_ptr() as *const _,
				Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
					notify_socket_buffer_trampoline::<F> as *const (),
				)),
				Box_::into_raw(f),
			)
		}
	}

	#[doc(alias = "socket-buffer-size")]
	pub fn connect_socket_buffer_size_notify<F: Fn(&Self) + Send + 'static>(
		&self,
		f: F,
	) -> SignalHandlerId {
		unsafe extern "C" fn notify_socket_buffer_size_trampoline<
			F: Fn(&GvStream) + Send + 'static,
		>(
			this: *mut ffi::ArvGvStream,
			_param_spec: glib::ffi::gpointer,
			f: glib::ffi::gpointer,
		) {
			let f: &F = &*(f as *const F);
			f(&from_glib_borrow(this))
		}
		unsafe {
			let f: Box_<F> = Box_::new(f);
			connect_raw(
				self.as_ptr() as *mut _,
				b"notify::socket-buffer-size\0".as_ptr() as *const _,
				Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
					notify_socket_buffer_size_trampoline::<F> as *const (),
				)),
				Box_::into_raw(f),
			)
		}
	}
}

unsafe impl Send for GvStream {}
