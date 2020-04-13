use crate::Buffer;
use crate::BufferExt;
use crate::PixelFormat;

use glib::translate::ToGlibPtr;
use glib::IsA;

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
		let buffer =
			aravis_sys::arv_buffer_new_full(len, data as *mut c_void, std::ptr::null_mut(), None);
		glib::translate::from_glib_full(buffer)
	}
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ImageError {
	InvalidStatus(crate::BufferStatus),
	InvalidPayloadType(crate::BufferPayloadType),
	UnsupportedPixelFormat(PixelFormat),
}

impl Buffer {
	/// Create a new buffer backed by a leaked `Box<[u8]>`.
	///
	/// The buffer can later be turned into an image using `[Self::into_image]`.
	/// If the buffer is dropped without taking ownership of the data again, the memory is leaked.
	pub fn new_leaked_box(len: usize) -> Self {
		let mut buffer = Box::<[u8]>::new_uninit_slice(len);
		let data = std::mem::MaybeUninit::first_ptr_mut(&mut buffer);
		let result = unsafe { Buffer::new_preallocated(data, len) };
		std::mem::forget(buffer);
		result
	}

	/// Create a new buffer for an image of the specified format and dimensions, backed by a leaked `Box<[u8]>`.
	///
	/// The buffer can later be turned into an image using `[Self::into_image]`.
	/// If the buffer is dropped without taking ownership of the data again, the memory is leaked.
	pub fn new_leaked_image(format: crate::PixelFormat, width: usize, height: usize) -> Self {
		let byte_len = crate::buffer_size_wh(format, width, height);
		Self::new_leaked_box(byte_len)
	}
}

impl<T: IsA<Buffer>> BufferExtManual for T {
	/// Get a pointer to the raw data and the length of the buffer.
	fn get_data(&self) -> (*mut u8, usize) {
		unsafe {
			let mut size = 0usize;
			let data = aravis_sys::arv_buffer_get_data(
				self.as_ref().to_glib_none().0,
				&mut size as *mut usize,
			);
			(data as *mut u8, size)
		}
	}

	/// Convert the buffer into an image.
	///
	/// # Safety
	/// This function assumes the buffer is backed by a leaked box,
	/// such as created by [`Buffer::new_leaked_box`].
	///
	/// This function takes ownership of the leaked box,
	/// so the memory will be freed when the resulting image is dropped.
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

		let width = self.get_image_width() as u32;
		let height = self.get_image_height() as u32;
		let format = self.get_image_pixel_format();

		match format {
			PixelFormat::RGB_8_PACKED => {
				return Ok(DynamicImage::ImageRgb8(
					ImageBuffer::from_raw(width, height, data).unwrap(),
				))
			}
			PixelFormat::BGR_8_PACKED => {
				return Ok(DynamicImage::ImageBgr8(
					ImageBuffer::from_raw(width, height, data).unwrap(),
				))
			}
			PixelFormat::MONO_8 => {
				return Ok(DynamicImage::ImageLuma8(
					ImageBuffer::from_raw(width, height, data).unwrap(),
				))
			}
			_ => (),
		};

		#[cfg(feature = "bayer")]
		{
			if let Some(filter) = debayer::filter(format) {
				let start = std::time::Instant::now();
				let result = debayer::debayer(width, height, filter, &data);
				eprintln!("debayer time: {}", start.elapsed().as_secs_f64());
				return result;
			}
		}

		Err(ImageError::UnsupportedPixelFormat(format))
	}
}

#[cfg(feature = "bayer")]
mod debayer {
	use crate::ImageError;
	use crate::PixelFormat;
	use image::DynamicImage;
	use image::ImageBuffer;

	pub fn filter(format: PixelFormat) -> Option<bayer::CFA> {
		match format {
			PixelFormat::BAYER_BG_8 => Some(bayer::CFA::BGGR),
			PixelFormat::BAYER_GB_8 => Some(bayer::CFA::GBRG),
			PixelFormat::BAYER_GR_8 => Some(bayer::CFA::GRBG),
			PixelFormat::BAYER_RG_8 => Some(bayer::CFA::RGGB),
			_ => None,
		}
	}

	pub fn debayer(
		width: u32,
		height: u32,
		filter: bayer::CFA,
		mut data: &[u8],
	) -> Result<DynamicImage, ImageError> {
		let mut buffer = vec![0u8; width as usize * height as usize * 3];
		let mut dest = bayer::RasterMut::new(
			width as usize,
			height as usize,
			bayer::RasterDepth::Depth8,
			&mut buffer,
		);
		bayer::run_demosaic(
			&mut data,
			bayer::BayerDepth::Depth8,
			filter,
			bayer::Demosaic::Linear,
			&mut dest,
		)
		.unwrap();
		Ok(DynamicImage::ImageRgb8(
			ImageBuffer::from_raw(width, height, buffer).unwrap(),
		))
	}
}

unsafe fn box_slice_from_raw<T>(data: *mut T, len: usize) -> Box<[T]> {
	Box::from_raw(std::slice::from_raw_parts_mut(data, len))
}

impl std::fmt::Display for ImageError {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		match self {
			Self::InvalidStatus(x) => write!(f, "invalid buffer status: {}", x),
			Self::InvalidPayloadType(x) => write!(f, "invalid buffer payload type: {}", x),
			Self::UnsupportedPixelFormat(x) => write!(f, "unsupported pixel format: {}", x.raw()),
		}
	}
}

impl std::error::Error for ImageError {}
