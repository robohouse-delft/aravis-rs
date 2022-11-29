use crate::Buffer;
use crate::PixelFormat;

use glib::translate::ToGlibPtr;
use glib_sys::GDestroyNotify;

use std::ffi::c_void;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ImageError {
	InvalidStatus(crate::BufferStatus),
	InvalidPayloadType(crate::BufferPayloadType),
	UnsupportedPixelFormat(PixelFormat),
}

impl Buffer {

	/// Create an Aravis buffer that owns its own data from a pre-allocated raw buffer.
	///
	/// The created buffer's user data and destory callback are generated such that the provided
	/// `destroy_callback` argument is called when the buffer is dropped.
	///
	/// # Safety
	/// The resulting buffer borrows the data, but it carries no lifetime.
	/// The user has to ensure the buffer stays valid.
	pub fn new_owned_preallocated<F: FnOnce()>(data: *mut u8, len: usize, destroy_callback: F) -> Self {
		extern "C" fn run_callback<F: FnOnce()>(user_data: *mut c_void) {
			unsafe {
				let function = Box::from_raw(user_data as *mut F);
				function()
			}
		}

		let user_data = Box::leak(Box::new(destroy_callback)) as *mut F as *mut c_void;
		unsafe {
			Self::preallocated(data as *mut c_void, len, user_data, Some(run_callback::<F>))
		}
	}

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
		Self::preallocated(data as *mut c_void, len, std::ptr::null_mut(), None)
	}

	unsafe fn preallocated(data: *mut c_void, len: usize, user_data: *mut c_void, destory_callback: GDestroyNotify) -> Self {
		let buffer = aravis_sys::arv_buffer_new_full(len, data as *mut c_void, user_data, destory_callback);
		glib::translate::from_glib_full(buffer)
	}

	/// Create a new buffer backed by a leaked `Box<[u8]>`.
	///
	/// The buffer can later be turned into an image using `[Self::into_image]`.
	/// If the buffer is dropped without taking ownership of the data again, the memory is leaked.
	pub fn new_leaked_box(len: usize) -> Self {
		#[cfg(feature = "nightly")]
		{
			let mut buffer = Box::<[u8]>::new_uninit_slice(len);
			let data = std::mem::MaybeUninit::slice_as_mut_ptr(&mut buffer);
			let result = unsafe { Buffer::new_preallocated(data, len) };
			std::mem::forget(buffer);
			result
		}
		#[cfg(not(feature = "nightly"))]
		{
			let mut buffer = vec![0u8; len].into_boxed_slice();
			let result = unsafe { Buffer::new_preallocated(buffer.as_mut_ptr(), len) };
			std::mem::forget(buffer);
			result
		}
	}

	/// Create a new buffer for an image of the specified format and dimensions, backed by a leaked `Box<[u8]>`.
	///
	/// The buffer can later be turned into an image using `[Self::into_image]`.
	/// If the buffer is dropped without taking ownership of the data again, the memory is leaked.
	pub fn new_leaked_image(format: crate::PixelFormat, width: usize, height: usize) -> Self {
		let byte_len = crate::buffer_size_wh(format, width, height);
		Self::new_leaked_box(byte_len)
	}

	/// Get a pointer to the raw data and the length of the buffer.
	pub fn data(&self) -> (*mut u8, usize) {
		unsafe {
			let mut size = 0usize;
			let data = aravis_sys::arv_buffer_get_data(
				self.to_glib_none().0,
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
	pub unsafe fn into_image(self) -> Result<image::DynamicImage, ImageError> {
		use image::DynamicImage;
		use image::ImageBuffer;

		let (data, len) = self.data();
		let data = Vec::from(box_slice_from_raw(data, len));

		let status = self.status();
		if status != crate::BufferStatus::Success {
			return Err(ImageError::InvalidStatus(status));
		}

		let payload = self.payload_type();
		if payload != crate::BufferPayloadType::Image {
			return Err(ImageError::InvalidPayloadType(payload));
		}

		let width = self.image_width() as u32;
		let height = self.image_height() as u32;
		let format = self.image_pixel_format();

		match format {
			PixelFormat::RGB_8_PACKED => {
				return Ok(DynamicImage::ImageRgb8(
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
				return debayer::debayer(width, height, filter, &data);
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
