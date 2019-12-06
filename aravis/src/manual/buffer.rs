use crate::Buffer;
use crate::BufferExt;

use glib::IsA;
use glib::translate::ToGlibPtr;

use std::ffi::c_void;


pub trait BufferExtManual {
	/// Get a pointer to the buffer data and the length of the buffer.
	fn get_data(&self) -> (*mut u8, usize);

	/// Convert the buffer into an image.
	///
	/// # Safety
	/// The data is assumed to have been allocated by a Box,
	/// and is unsafely turned back into one.
	unsafe fn into_image(self) -> Result<image::DynamicImage, ImageError>;
}

impl Buffer {
	/// Create an Aravis buffer from a pre-allocated raw buffer.
	///
	/// The created buffer has no registered user data or destroy callback,
	/// so management of the underlying buffer has to be done externally.
	/// The buffer can be identified later when it is returned by a stream only byt the data pointer.
	///
	/// # Safety
	/// The resulting buffer borrows the data, but it carries no lifetime.
	/// The user has to ensure the buffer stays valid.
	pub unsafe fn new_preallocated(data: *mut u8, len: usize) -> Self {
		let buffer = aravis_sys::arv_buffer_new_full(len, data as *mut c_void, std::ptr::null_mut(), None);
		glib::translate::from_glib_full(buffer)
	}
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ImageError {
	InvalidStatus(crate::BufferStatus),
	InvalidPayloadType(crate::BufferPayloadType),
	UnsupportedPixelFormat(u32),
}

impl<T: IsA<Buffer>> BufferExtManual for T {
	fn get_data(&self) -> (*mut u8, usize) {
		unsafe {
			let mut size = 0usize;
			let data = aravis_sys::arv_buffer_get_data(self.as_ref().to_glib_none().0, &mut size as *mut usize);
			(data as *mut u8, size)
		}
	}

	unsafe fn into_image(self) -> Result<image::DynamicImage, ImageError> {
		use image::DynamicImage;
		use image::ImageBuffer;

		let (data, len) = self.get_data();
		let data = Vec::from(box_slice_from_raw(data, len));

		let status = self.get_status();
		if status != crate::BufferStatus::Success {
			return Err(ImageError::InvalidStatus(status));
		}

		let payload = self.get_payload_type();
		if payload != crate::BufferPayloadType::Image {
			return Err(ImageError::InvalidPayloadType(payload));
		}

		let width  = self.get_image_width() as u32;
		let height = self.get_image_height() as u32;

		let image = match self.get_image_pixel_format() {
			aravis_sys::ARV_PIXEL_FORMAT_RGB_8_PACKED => DynamicImage::ImageRgb8(ImageBuffer::from_raw(width, height, data).unwrap()),
			aravis_sys::ARV_PIXEL_FORMAT_BGR_8_PACKED => DynamicImage::ImageBgr8(ImageBuffer::from_raw(width, height, data).unwrap()),
			aravis_sys::ARV_PIXEL_FORMAT_MONO_8       => DynamicImage::ImageLuma8(ImageBuffer::from_raw(width, height, data).unwrap()),
			x => return Err(ImageError::UnsupportedPixelFormat(x)),
		};

		Ok(image)
	}
}

unsafe fn box_slice_from_raw<T>(data: *mut T, len: usize) -> Box<[T]> {
	Box::from_raw(std::slice::from_raw_parts_mut(data, len))
}

impl std::fmt::Display for ImageError {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		match self {
			Self::InvalidStatus(x)          => write!(f, "invalid buffer status: {}", x),
			Self::InvalidPayloadType(x)     => write!(f, "invalid buffer payload type: {}", x),
			Self::UnsupportedPixelFormat(x) => write!(f, "unsupported pixel format: {}", x),
		}
	}
}

impl std::error::Error for ImageError {}
