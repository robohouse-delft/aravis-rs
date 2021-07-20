// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ../gir-files
// DO NOT EDIT

use crate::Buffer;
use crate::Device;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem;
use std::mem::transmute;

glib::wrapper! {
	#[doc(alias = "ArvStream")]
	pub struct Stream(Object<ffi::ArvStream, ffi::ArvStreamClass>);

	match fn {
		type_ => || ffi::arv_stream_get_type(),
	}
}

unsafe impl Send for Stream {}

pub const NONE_STREAM: Option<&Stream> = None;

/// Trait containing all [`struct@Stream`] methods.
///
/// # Implementors
///
/// [`FakeStream`][struct@crate::FakeStream], [`GvStream`][struct@crate::GvStream], [`Stream`][struct@crate::Stream], [`UvStream`][struct@crate::UvStream]
pub trait StreamExt: 'static {
	/// Check if stream will emit its signals.
	///
	/// # Returns
	///
	/// [`true`] if `self` is emiting its signals.
	#[doc(alias = "arv_stream_get_emit_signals")]
	#[doc(alias = "get_emit_signals")]
	fn emits_signals(&self) -> bool;

	/// ## `id`
	/// info id
	///
	/// # Returns
	///
	/// the value of the corresponding stream information, as a double.
	#[cfg(any(feature = "v0_8_11", feature = "dox"))]
	#[cfg_attr(feature = "dox", doc(cfg(feature = "v0_8_11")))]
	#[doc(alias = "arv_stream_get_info_double")]
	#[doc(alias = "get_info_double")]
	fn info_double(&self, id: u32) -> f64;

	/// ## `name`
	/// info name
	///
	/// # Returns
	///
	/// the value of the corresponding stream information, as a double.
	#[cfg(any(feature = "v0_8_11", feature = "dox"))]
	#[cfg_attr(feature = "dox", doc(cfg(feature = "v0_8_11")))]
	#[doc(alias = "arv_stream_get_info_double_by_name")]
	#[doc(alias = "get_info_double_by_name")]
	fn info_double_by_name(&self, name: &str) -> f64;

	/// ## `id`
	/// info id
	///
	/// # Returns
	///
	/// the name of the corresponding stream information.
	#[cfg(any(feature = "v0_8_11", feature = "dox"))]
	#[cfg_attr(feature = "dox", doc(cfg(feature = "v0_8_11")))]
	#[doc(alias = "arv_stream_get_info_name")]
	#[doc(alias = "get_info_name")]
	fn info_name(&self, id: u32) -> Option<glib::GString>;

	/// ## `id`
	/// info id
	///
	/// # Returns
	///
	/// the `GType` of the corresponding stream information, which indicates what API to use to retrieve the
	/// information value ([`info_uint64()`][Self::info_uint64()] or [`info_double()`][Self::info_double()]).
	#[cfg(any(feature = "v0_8_11", feature = "dox"))]
	#[cfg_attr(feature = "dox", doc(cfg(feature = "v0_8_11")))]
	#[doc(alias = "arv_stream_get_info_type")]
	#[doc(alias = "get_info_type")]
	fn info_type(&self, id: u32) -> glib::types::Type;

	/// ## `id`
	/// info id
	///
	/// # Returns
	///
	/// the value of the corresponding stream information, as a 64 bit unsigned integer.
	#[cfg(any(feature = "v0_8_11", feature = "dox"))]
	#[cfg_attr(feature = "dox", doc(cfg(feature = "v0_8_11")))]
	#[doc(alias = "arv_stream_get_info_uint64")]
	#[doc(alias = "get_info_uint64")]
	fn info_uint64(&self, id: u32) -> u64;

	/// ## `name`
	/// info name
	///
	/// # Returns
	///
	/// the value of the corresponding stream information, as a 64 bit unsigned integer.
	#[cfg(any(feature = "v0_8_11", feature = "dox"))]
	#[cfg_attr(feature = "dox", doc(cfg(feature = "v0_8_11")))]
	#[doc(alias = "arv_stream_get_info_uint64_by_name")]
	#[doc(alias = "get_info_uint64_by_name")]
	fn info_uint64_by_name(&self, name: &str) -> u64;

	/// An accessor to the stream buffer queue lengths.
	///
	/// # Returns
	///
	///
	/// ## `n_input_buffers`
	/// input queue length
	///
	/// ## `n_output_buffers`
	/// output queue length
	#[doc(alias = "arv_stream_get_n_buffers")]
	#[doc(alias = "get_n_buffers")]
	fn n_buffers(&self) -> (i32, i32);

	///
	/// # Returns
	///
	/// the number of stream informations. These informations contain useful numbers about data transfer quality.
	#[cfg(any(feature = "v0_8_11", feature = "dox"))]
	#[cfg_attr(feature = "dox", doc(cfg(feature = "v0_8_11")))]
	#[doc(alias = "arv_stream_get_n_infos")]
	#[doc(alias = "get_n_infos")]
	fn n_infos(&self) -> u32;

	/// An accessor to the stream statistics.
	///
	/// # Returns
	///
	///
	/// ## `n_completed_buffers`
	/// number of complete received buffers
	///
	/// ## `n_failures`
	/// number of reception failures
	///
	/// ## `n_underruns`
	/// number of input buffer underruns
	#[doc(alias = "arv_stream_get_statistics")]
	#[doc(alias = "get_statistics")]
	fn statistics(&self) -> (u64, u64, u64);

	/// Pops a buffer from the output queue of `self`. The retrieved buffer
	/// may contain an invalid image. Caller should check the buffer status before using it.
	/// This function blocks until a buffer is available.
	///
	/// This method is thread safe.
	///
	/// # Returns
	///
	/// a [`Buffer`][crate::Buffer]
	#[doc(alias = "arv_stream_pop_buffer")]
	fn pop_buffer(&self) -> Option<Buffer>;

	/// Pushes a [`Buffer`][crate::Buffer] to the `self` thread. The `self` takes ownership of `buffer`,
	/// and will free all the buffers still in its queues when destroyed.
	///
	/// This method is thread safe.
	/// ## `buffer`
	/// buffer to push
	#[doc(alias = "arv_stream_push_buffer")]
	fn push_buffer(&self, buffer: &Buffer);

	/// Make `self` emit signals. This option is
	/// by default disabled because signal emission is expensive and unneeded when
	/// the application prefers to operate in pull mode.
	/// ## `emit_signals`
	/// the new state
	#[doc(alias = "arv_stream_set_emit_signals")]
	fn set_emit_signals(&self, emit_signals: bool);

	/// Start the stream receiving thread. The thread is automatically started when
	/// the [`Stream`][crate::Stream] object is instantiated, so this function is only useful if
	/// the thread was stopped using [`stop_thread()`][Self::stop_thread()].
	#[doc(alias = "arv_stream_start_thread")]
	fn start_thread(&self);

	/// Stop the stream receiving thread, and optionally delete all the [`Buffer`][crate::Buffer]
	/// stored in the stream object queues. Main use of this function is to be able
	/// to quickly change an acquisition parameter that changes the payload size,
	/// without deleting/recreating the stream object.
	/// ## `delete_buffers`
	/// enable buffer deletion
	///
	/// # Returns
	///
	/// the number of deleted buffers if `delete_buffers` == [`true`], 0 otherwise.
	#[doc(alias = "arv_stream_stop_thread")]
	fn stop_thread(&self, delete_buffers: bool) -> u32;

	/// Pops a buffer from the output queue of `self`, waiting no more than `timeout`. The retrieved buffer
	/// may contain an invalid image. Caller should check the buffer status before using it.
	///
	/// This method is thread safe.
	/// ## `timeout`
	/// timeout, in µs
	///
	/// # Returns
	///
	/// a [`Buffer`][crate::Buffer], NULL if no buffer is available until the timeout occurs.
	#[doc(alias = "arv_stream_timeout_pop_buffer")]
	fn timeout_pop_buffer(&self, timeout: u64) -> Option<Buffer>;

	/// Pops a buffer from the output queue of `self`. The retrieved buffer
	/// may contain an invalid image. Caller should check the buffer status before using it.
	/// This is the non blocking version of pop_buffer.
	///
	/// This method is thread safe.
	///
	/// # Returns
	///
	/// a [`Buffer`][crate::Buffer], NULL if no buffer is available.
	#[doc(alias = "arv_stream_try_pop_buffer")]
	fn try_pop_buffer(&self) -> Option<Buffer>;

	//fn callback(&self) -> /*Unimplemented*/Fundamental: Pointer;

	//#[doc(alias = "callback-data")]
	//fn callback_data(&self) -> /*Unimplemented*/Fundamental: Pointer;

	fn device(&self) -> Option<Device>;

	/// Signal that a new buffer is available.
	///
	/// This signal is emited from the stream receive thread and only when the
	/// "emit-signals" property is [`true`].
	///
	/// The new buffer can be retrieved with [`pop_buffer()`][Self::pop_buffer()].
	///
	/// Note that this signal is only emited when the "emit-signals" property is
	/// set to [`true`], which it is not by default for performance reasons.
	#[doc(alias = "new-buffer")]
	fn connect_new_buffer<F: Fn(&Self) + Send + 'static>(&self, f: F) -> SignalHandlerId;

	#[doc(alias = "emit-signals")]
	fn connect_emit_signals_notify<F: Fn(&Self) + Send + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Stream>> StreamExt for O {
	fn emits_signals(&self) -> bool {
		unsafe {
			from_glib(ffi::arv_stream_get_emit_signals(
				self.as_ref().to_glib_none().0,
			))
		}
	}

	#[cfg(any(feature = "v0_8_11", feature = "dox"))]
	#[cfg_attr(feature = "dox", doc(cfg(feature = "v0_8_11")))]
	fn info_double(&self, id: u32) -> f64 {
		unsafe { ffi::arv_stream_get_info_double(self.as_ref().to_glib_none().0, id) }
	}

	#[cfg(any(feature = "v0_8_11", feature = "dox"))]
	#[cfg_attr(feature = "dox", doc(cfg(feature = "v0_8_11")))]
	fn info_double_by_name(&self, name: &str) -> f64 {
		unsafe {
			ffi::arv_stream_get_info_double_by_name(
				self.as_ref().to_glib_none().0,
				name.to_glib_none().0,
			)
		}
	}

	#[cfg(any(feature = "v0_8_11", feature = "dox"))]
	#[cfg_attr(feature = "dox", doc(cfg(feature = "v0_8_11")))]
	fn info_name(&self, id: u32) -> Option<glib::GString> {
		unsafe {
			from_glib_none(ffi::arv_stream_get_info_name(
				self.as_ref().to_glib_none().0,
				id,
			))
		}
	}

	#[cfg(any(feature = "v0_8_11", feature = "dox"))]
	#[cfg_attr(feature = "dox", doc(cfg(feature = "v0_8_11")))]
	fn info_type(&self, id: u32) -> glib::types::Type {
		unsafe {
			from_glib(ffi::arv_stream_get_info_type(
				self.as_ref().to_glib_none().0,
				id,
			))
		}
	}

	#[cfg(any(feature = "v0_8_11", feature = "dox"))]
	#[cfg_attr(feature = "dox", doc(cfg(feature = "v0_8_11")))]
	fn info_uint64(&self, id: u32) -> u64 {
		unsafe { ffi::arv_stream_get_info_uint64(self.as_ref().to_glib_none().0, id) }
	}

	#[cfg(any(feature = "v0_8_11", feature = "dox"))]
	#[cfg_attr(feature = "dox", doc(cfg(feature = "v0_8_11")))]
	fn info_uint64_by_name(&self, name: &str) -> u64 {
		unsafe {
			ffi::arv_stream_get_info_uint64_by_name(
				self.as_ref().to_glib_none().0,
				name.to_glib_none().0,
			)
		}
	}

	fn n_buffers(&self) -> (i32, i32) {
		unsafe {
			let mut n_input_buffers = mem::MaybeUninit::uninit();
			let mut n_output_buffers = mem::MaybeUninit::uninit();
			ffi::arv_stream_get_n_buffers(
				self.as_ref().to_glib_none().0,
				n_input_buffers.as_mut_ptr(),
				n_output_buffers.as_mut_ptr(),
			);
			let n_input_buffers = n_input_buffers.assume_init();
			let n_output_buffers = n_output_buffers.assume_init();
			(n_input_buffers, n_output_buffers)
		}
	}

	#[cfg(any(feature = "v0_8_11", feature = "dox"))]
	#[cfg_attr(feature = "dox", doc(cfg(feature = "v0_8_11")))]
	fn n_infos(&self) -> u32 {
		unsafe { ffi::arv_stream_get_n_infos(self.as_ref().to_glib_none().0) }
	}

	fn statistics(&self) -> (u64, u64, u64) {
		unsafe {
			let mut n_completed_buffers = mem::MaybeUninit::uninit();
			let mut n_failures = mem::MaybeUninit::uninit();
			let mut n_underruns = mem::MaybeUninit::uninit();
			ffi::arv_stream_get_statistics(
				self.as_ref().to_glib_none().0,
				n_completed_buffers.as_mut_ptr(),
				n_failures.as_mut_ptr(),
				n_underruns.as_mut_ptr(),
			);
			let n_completed_buffers = n_completed_buffers.assume_init();
			let n_failures = n_failures.assume_init();
			let n_underruns = n_underruns.assume_init();
			(n_completed_buffers, n_failures, n_underruns)
		}
	}

	fn pop_buffer(&self) -> Option<Buffer> {
		unsafe { from_glib_full(ffi::arv_stream_pop_buffer(self.as_ref().to_glib_none().0)) }
	}

	fn push_buffer(&self, buffer: &Buffer) {
		unsafe {
			ffi::arv_stream_push_buffer(self.as_ref().to_glib_none().0, buffer.to_glib_full());
		}
	}

	fn set_emit_signals(&self, emit_signals: bool) {
		unsafe {
			ffi::arv_stream_set_emit_signals(
				self.as_ref().to_glib_none().0,
				emit_signals.into_glib(),
			);
		}
	}

	fn start_thread(&self) {
		unsafe {
			ffi::arv_stream_start_thread(self.as_ref().to_glib_none().0);
		}
	}

	fn stop_thread(&self, delete_buffers: bool) -> u32 {
		unsafe {
			ffi::arv_stream_stop_thread(self.as_ref().to_glib_none().0, delete_buffers.into_glib())
		}
	}

	fn timeout_pop_buffer(&self, timeout: u64) -> Option<Buffer> {
		unsafe {
			from_glib_full(ffi::arv_stream_timeout_pop_buffer(
				self.as_ref().to_glib_none().0,
				timeout,
			))
		}
	}

	fn try_pop_buffer(&self) -> Option<Buffer> {
		unsafe {
			from_glib_full(ffi::arv_stream_try_pop_buffer(
				self.as_ref().to_glib_none().0,
			))
		}
	}

	//fn callback(&self) -> /*Unimplemented*/Fundamental: Pointer {
	//    unsafe {
	//        let mut value = glib::Value::from_type(</*Unknown type*/ as StaticType>::static_type());
	//        glib::gobject_ffi::g_object_get_property(self.to_glib_none().0 as *mut glib::gobject_ffi::GObject, b"callback\0".as_ptr() as *const _, value.to_glib_none_mut().0);
	//        value.get().expect("Return Value for property `callback` getter")
	//    }
	//}

	//fn callback_data(&self) -> /*Unimplemented*/Fundamental: Pointer {
	//    unsafe {
	//        let mut value = glib::Value::from_type(</*Unknown type*/ as StaticType>::static_type());
	//        glib::gobject_ffi::g_object_get_property(self.to_glib_none().0 as *mut glib::gobject_ffi::GObject, b"callback-data\0".as_ptr() as *const _, value.to_glib_none_mut().0);
	//        value.get().expect("Return Value for property `callback-data` getter")
	//    }
	//}

	fn device(&self) -> Option<Device> {
		unsafe {
			let mut value = glib::Value::from_type(<Device as StaticType>::static_type());
			glib::gobject_ffi::g_object_get_property(
				self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
				b"device\0".as_ptr() as *const _,
				value.to_glib_none_mut().0,
			);
			value
				.get()
				.expect("Return Value for property `device` getter")
		}
	}

	fn connect_new_buffer<F: Fn(&Self) + Send + 'static>(&self, f: F) -> SignalHandlerId {
		unsafe extern "C" fn new_buffer_trampoline<P: IsA<Stream>, F: Fn(&P) + Send + 'static>(
			this: *mut ffi::ArvStream,
			f: glib::ffi::gpointer,
		) {
			let f: &F = &*(f as *const F);
			f(Stream::from_glib_borrow(this).unsafe_cast_ref())
		}
		unsafe {
			let f: Box_<F> = Box_::new(f);
			connect_raw(
				self.as_ptr() as *mut _,
				b"new-buffer\0".as_ptr() as *const _,
				Some(transmute::<_, unsafe extern "C" fn()>(
					new_buffer_trampoline::<Self, F> as *const (),
				)),
				Box_::into_raw(f),
			)
		}
	}

	fn connect_emit_signals_notify<F: Fn(&Self) + Send + 'static>(&self, f: F) -> SignalHandlerId {
		unsafe extern "C" fn notify_emit_signals_trampoline<
			P: IsA<Stream>,
			F: Fn(&P) + Send + 'static,
		>(
			this: *mut ffi::ArvStream,
			_param_spec: glib::ffi::gpointer,
			f: glib::ffi::gpointer,
		) {
			let f: &F = &*(f as *const F);
			f(Stream::from_glib_borrow(this).unsafe_cast_ref())
		}
		unsafe {
			let f: Box_<F> = Box_::new(f);
			connect_raw(
				self.as_ptr() as *mut _,
				b"notify::emit-signals\0".as_ptr() as *const _,
				Some(transmute::<_, unsafe extern "C" fn()>(
					notify_emit_signals_trampoline::<Self, F> as *const (),
				)),
				Box_::into_raw(f),
			)
		}
	}
}

impl fmt::Display for Stream {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.write_str("Stream")
	}
}
