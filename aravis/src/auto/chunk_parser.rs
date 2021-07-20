// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use aravis_sys;
use glib;
use glib::object::IsA;
use glib::translate::*;
use glib::GString;
use glib::StaticType;
use glib::Value;
use gobject_sys;
use std::fmt;
use std::ptr;
use Buffer;
use Gc;

glib_wrapper! {
	pub struct ChunkParser(Object<aravis_sys::ArvChunkParser, aravis_sys::ArvChunkParserClass, ChunkParserClass>);

	match fn {
		get_type => || aravis_sys::arv_chunk_parser_get_type(),
	}
}

impl ChunkParser {
	/// Creates a new chunk_parser.
	/// ## `xml`
	/// XML genicam data
	/// ## `size`
	/// genicam data size, -1 if NULL terminated
	///
	/// # Returns
	///
	/// a new `ChunkParser` object
	pub fn new(xml: &str, size: usize) -> ChunkParser {
		assert_initialized_main_thread!();
		unsafe { from_glib_full(aravis_sys::arv_chunk_parser_new(xml.to_glib_none().0, size)) }
	}
}

unsafe impl Send for ChunkParser {}

pub const NONE_CHUNK_PARSER: Option<&ChunkParser> = None;

/// Trait containing all `ChunkParser` methods.
///
/// # Implementors
///
/// [`ChunkParser`](struct.ChunkParser.html)
pub trait ChunkParserExt: 'static {
	/// ## `buffer`
	/// a `Buffer` with a `BufferPayloadType::ChunkData` payload
	/// ## `chunk`
	/// chunk data name
	///
	/// # Returns
	///
	/// the boolean chunk data value.
	fn get_boolean_value<P: IsA<Buffer>>(&self, buffer: &P, chunk: &str)
		-> Result<(), glib::Error>;

	/// ## `buffer`
	/// a `Buffer` with a `BufferPayloadType::ChunkData` payload
	/// ## `chunk`
	/// chunk data name
	///
	/// # Returns
	///
	/// the float chunk data value.
	fn get_float_value<P: IsA<Buffer>>(&self, buffer: &P, chunk: &str) -> Result<f64, glib::Error>;

	/// ## `buffer`
	/// a `Buffer` with a `BufferPayloadType::ChunkData` payload
	/// ## `chunk`
	/// chunk data name
	///
	/// # Returns
	///
	/// the integer chunk data integer.
	fn get_integer_value<P: IsA<Buffer>>(
		&self,
		buffer: &P,
		chunk: &str,
	) -> Result<i64, glib::Error>;

	/// ## `buffer`
	/// a `Buffer` with a `BufferPayloadType::ChunkData` payload
	/// ## `chunk`
	/// chunk data name
	///
	/// # Returns
	///
	/// the string chunk data value.
	fn get_string_value<P: IsA<Buffer>>(
		&self,
		buffer: &P,
		chunk: &str,
	) -> Result<GString, glib::Error>;

	fn get_property_genicam(&self) -> Option<Gc>;
}

impl<O: IsA<ChunkParser>> ChunkParserExt for O {
	fn get_boolean_value<P: IsA<Buffer>>(
		&self,
		buffer: &P,
		chunk: &str,
	) -> Result<(), glib::Error> {
		unsafe {
			let mut error = ptr::null_mut();
			let _ = aravis_sys::arv_chunk_parser_get_boolean_value(
				self.as_ref().to_glib_none().0,
				buffer.as_ref().to_glib_none().0,
				chunk.to_glib_none().0,
				&mut error,
			);
			if error.is_null() {
				Ok(())
			} else {
				Err(from_glib_full(error))
			}
		}
	}

	fn get_float_value<P: IsA<Buffer>>(&self, buffer: &P, chunk: &str) -> Result<f64, glib::Error> {
		unsafe {
			let mut error = ptr::null_mut();
			let ret = aravis_sys::arv_chunk_parser_get_float_value(
				self.as_ref().to_glib_none().0,
				buffer.as_ref().to_glib_none().0,
				chunk.to_glib_none().0,
				&mut error,
			);
			if error.is_null() {
				Ok(ret)
			} else {
				Err(from_glib_full(error))
			}
		}
	}

	fn get_integer_value<P: IsA<Buffer>>(
		&self,
		buffer: &P,
		chunk: &str,
	) -> Result<i64, glib::Error> {
		unsafe {
			let mut error = ptr::null_mut();
			let ret = aravis_sys::arv_chunk_parser_get_integer_value(
				self.as_ref().to_glib_none().0,
				buffer.as_ref().to_glib_none().0,
				chunk.to_glib_none().0,
				&mut error,
			);
			if error.is_null() {
				Ok(ret)
			} else {
				Err(from_glib_full(error))
			}
		}
	}

	fn get_string_value<P: IsA<Buffer>>(
		&self,
		buffer: &P,
		chunk: &str,
	) -> Result<GString, glib::Error> {
		unsafe {
			let mut error = ptr::null_mut();
			let ret = aravis_sys::arv_chunk_parser_get_string_value(
				self.as_ref().to_glib_none().0,
				buffer.as_ref().to_glib_none().0,
				chunk.to_glib_none().0,
				&mut error,
			);
			if error.is_null() {
				Ok(from_glib_none(ret))
			} else {
				Err(from_glib_full(error))
			}
		}
	}

	fn get_property_genicam(&self) -> Option<Gc> {
		unsafe {
			let mut value = Value::from_type(<Gc as StaticType>::static_type());
			gobject_sys::g_object_get_property(
				self.to_glib_none().0 as *mut gobject_sys::GObject,
				b"genicam\0".as_ptr() as *const _,
				value.to_glib_none_mut().0,
			);
			value
				.get()
				.expect("Return Value for property `genicam` getter")
		}
	}
}

impl fmt::Display for ChunkParser {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "ChunkParser")
	}
}