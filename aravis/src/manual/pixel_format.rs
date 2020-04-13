/// Calculate the bits per pixel of a given format.
pub fn bits_per_pixel(format: PixelFormat) -> u8 {
	((format.raw() >> 16) & 0xFF) as u8
}

/// Calculated the size in bytes for a pixel format and count.
///
/// For packed formats, the returned size is rounded up to the nearest whole byte.
pub fn buffer_size(format: PixelFormat, count: usize) -> usize {
	(usize::from(bits_per_pixel(format)) * count + 7) / 8
}

/// Calculate the size in bytes for a image with a pixel format, width and height.
///
/// For packed formats, the returned size is rounded up to the nearest whole byte.
pub fn buffer_size_wh(format: PixelFormat, width: usize, height: usize) -> usize {
	buffer_size(format, width * height)
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_bits_per_pixel() {
		assert_eq!(bits_per_pixel(PixelFormat::MONO_8), 8);
		assert_eq!(bits_per_pixel(PixelFormat::RGB_8_PACKED), 24);
		assert_eq!(bits_per_pixel(PixelFormat::BAYER_RG_10), 16);
		assert_eq!(bits_per_pixel(PixelFormat::BAYER_RG_10P), 10);
		assert_eq!(bits_per_pixel(PixelFormat::BAYER_RG_12), 16);
		assert_eq!(bits_per_pixel(PixelFormat::BAYER_RG_12P), 12);
	}
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[repr(transparent)]
pub struct PixelFormat(u32);

impl glib::translate::FromGlib<u32> for PixelFormat {
	fn from_glib(other: u32) -> Self {
		Self::from_raw(other)
	}
}

impl glib::translate::ToGlib for PixelFormat {
	type GlibType = u32;

	fn to_glib(&self) -> Self::GlibType {
		self.raw()
	}
}

impl PixelFormat {
	pub const fn from_raw(raw: u32) -> Self {
		Self(raw)
	}

	pub const fn raw(self) -> u32 {
		self.0
	}

	pub const BAYER_BG_10: Self = Self::from_raw(aravis_sys::ARV_PIXEL_FORMAT_BAYER_BG_10);
	pub const BAYER_BG_10P: Self = Self::from_raw(aravis_sys::ARV_PIXEL_FORMAT_BAYER_BG_10P);
	pub const BAYER_BG_10_PACKED: Self =
		Self::from_raw(aravis_sys::ARV_PIXEL_FORMAT_BAYER_BG_10_PACKED);
	pub const BAYER_BG_12: Self = Self::from_raw(aravis_sys::ARV_PIXEL_FORMAT_BAYER_BG_12);
	pub const BAYER_BG_12P: Self = Self::from_raw(aravis_sys::ARV_PIXEL_FORMAT_BAYER_BG_12P);
	pub const BAYER_BG_12_PACKED: Self =
		Self::from_raw(aravis_sys::ARV_PIXEL_FORMAT_BAYER_BG_12_PACKED);
	pub const BAYER_BG_16: Self = Self::from_raw(aravis_sys::ARV_PIXEL_FORMAT_BAYER_BG_16);
	pub const BAYER_BG_8: Self = Self::from_raw(aravis_sys::ARV_PIXEL_FORMAT_BAYER_BG_8);
	pub const BAYER_GB_10: Self = Self::from_raw(aravis_sys::ARV_PIXEL_FORMAT_BAYER_GB_10);
	pub const BAYER_GB_10P: Self = Self::from_raw(aravis_sys::ARV_PIXEL_FORMAT_BAYER_GB_10P);
	pub const BAYER_GB_10_PACKED: Self =
		Self::from_raw(aravis_sys::ARV_PIXEL_FORMAT_BAYER_GB_10_PACKED);
	pub const BAYER_GB_12: Self = Self::from_raw(aravis_sys::ARV_PIXEL_FORMAT_BAYER_GB_12);
	pub const BAYER_GB_12P: Self = Self::from_raw(aravis_sys::ARV_PIXEL_FORMAT_BAYER_GB_12P);
	pub const BAYER_GB_12_PACKED: Self =
		Self::from_raw(aravis_sys::ARV_PIXEL_FORMAT_BAYER_GB_12_PACKED);
	pub const BAYER_GB_16: Self = Self::from_raw(aravis_sys::ARV_PIXEL_FORMAT_BAYER_GB_16);
	pub const BAYER_GB_8: Self = Self::from_raw(aravis_sys::ARV_PIXEL_FORMAT_BAYER_GB_8);
	pub const BAYER_GR_10: Self = Self::from_raw(aravis_sys::ARV_PIXEL_FORMAT_BAYER_GR_10);
	pub const BAYER_GR_10P: Self = Self::from_raw(aravis_sys::ARV_PIXEL_FORMAT_BAYER_GR_10P);
	pub const BAYER_GR_10_PACKED: Self =
		Self::from_raw(aravis_sys::ARV_PIXEL_FORMAT_BAYER_GR_10_PACKED);
	pub const BAYER_GR_12: Self = Self::from_raw(aravis_sys::ARV_PIXEL_FORMAT_BAYER_GR_12);
	pub const BAYER_GR_12P: Self = Self::from_raw(aravis_sys::ARV_PIXEL_FORMAT_BAYER_GR_12P);
	pub const BAYER_GR_12_PACKED: Self =
		Self::from_raw(aravis_sys::ARV_PIXEL_FORMAT_BAYER_GR_12_PACKED);
	pub const BAYER_GR_16: Self = Self::from_raw(aravis_sys::ARV_PIXEL_FORMAT_BAYER_GR_16);
	pub const BAYER_GR_8: Self = Self::from_raw(aravis_sys::ARV_PIXEL_FORMAT_BAYER_GR_8);
	pub const BAYER_RG_10: Self = Self::from_raw(aravis_sys::ARV_PIXEL_FORMAT_BAYER_RG_10);
	pub const BAYER_RG_10P: Self = Self::from_raw(aravis_sys::ARV_PIXEL_FORMAT_BAYER_RG_10P);
	pub const BAYER_RG_10_PACKED: Self =
		Self::from_raw(aravis_sys::ARV_PIXEL_FORMAT_BAYER_RG_10_PACKED);
	pub const BAYER_RG_12: Self = Self::from_raw(aravis_sys::ARV_PIXEL_FORMAT_BAYER_RG_12);
	pub const BAYER_RG_12P: Self = Self::from_raw(aravis_sys::ARV_PIXEL_FORMAT_BAYER_RG_12P);
	pub const BAYER_RG_12_PACKED: Self =
		Self::from_raw(aravis_sys::ARV_PIXEL_FORMAT_BAYER_RG_12_PACKED);
	pub const BAYER_RG_16: Self = Self::from_raw(aravis_sys::ARV_PIXEL_FORMAT_BAYER_RG_16);
	pub const BAYER_RG_8: Self = Self::from_raw(aravis_sys::ARV_PIXEL_FORMAT_BAYER_RG_8);
	pub const BGRA_8_PACKED: Self = Self::from_raw(aravis_sys::ARV_PIXEL_FORMAT_BGRA_8_PACKED);
	pub const BGR_10_PACKED: Self = Self::from_raw(aravis_sys::ARV_PIXEL_FORMAT_BGR_10_PACKED);
	pub const BGR_12_PACKED: Self = Self::from_raw(aravis_sys::ARV_PIXEL_FORMAT_BGR_12_PACKED);
	pub const BGR_8_PACKED: Self = Self::from_raw(aravis_sys::ARV_PIXEL_FORMAT_BGR_8_PACKED);
	pub const CUSTOM_BAYER_BG_12_PACKED: Self =
		Self::from_raw(aravis_sys::ARV_PIXEL_FORMAT_CUSTOM_BAYER_BG_12_PACKED);
	pub const CUSTOM_BAYER_BG_16: Self =
		Self::from_raw(aravis_sys::ARV_PIXEL_FORMAT_CUSTOM_BAYER_BG_16);
	pub const CUSTOM_BAYER_GB_12_PACKED: Self =
		Self::from_raw(aravis_sys::ARV_PIXEL_FORMAT_CUSTOM_BAYER_GB_12_PACKED);
	pub const CUSTOM_BAYER_GB_16: Self =
		Self::from_raw(aravis_sys::ARV_PIXEL_FORMAT_CUSTOM_BAYER_GB_16);
	pub const CUSTOM_BAYER_GR_12_PACKED: Self =
		Self::from_raw(aravis_sys::ARV_PIXEL_FORMAT_CUSTOM_BAYER_GR_12_PACKED);
	pub const CUSTOM_BAYER_GR_16: Self =
		Self::from_raw(aravis_sys::ARV_PIXEL_FORMAT_CUSTOM_BAYER_GR_16);
	pub const CUSTOM_BAYER_RG_12_PACKED: Self =
		Self::from_raw(aravis_sys::ARV_PIXEL_FORMAT_CUSTOM_BAYER_RG_12_PACKED);
	pub const CUSTOM_BAYER_RG_16: Self =
		Self::from_raw(aravis_sys::ARV_PIXEL_FORMAT_CUSTOM_BAYER_RG_16);
	pub const CUSTOM_YUV_422_YUYV_PACKED: Self =
		Self::from_raw(aravis_sys::ARV_PIXEL_FORMAT_CUSTOM_YUV_422_YUYV_PACKED);
	pub const MONO_10: Self = Self::from_raw(aravis_sys::ARV_PIXEL_FORMAT_MONO_10);
	pub const MONO_10_PACKED: Self = Self::from_raw(aravis_sys::ARV_PIXEL_FORMAT_MONO_10_PACKED);
	pub const MONO_12: Self = Self::from_raw(aravis_sys::ARV_PIXEL_FORMAT_MONO_12);
	pub const MONO_12_PACKED: Self = Self::from_raw(aravis_sys::ARV_PIXEL_FORMAT_MONO_12_PACKED);
	pub const MONO_14: Self = Self::from_raw(aravis_sys::ARV_PIXEL_FORMAT_MONO_14);
	pub const MONO_16: Self = Self::from_raw(aravis_sys::ARV_PIXEL_FORMAT_MONO_16);
	pub const MONO_8: Self = Self::from_raw(aravis_sys::ARV_PIXEL_FORMAT_MONO_8);
	pub const MONO_8_SIGNED: Self = Self::from_raw(aravis_sys::ARV_PIXEL_FORMAT_MONO_8_SIGNED);
	pub const RGBA_8_PACKED: Self = Self::from_raw(aravis_sys::ARV_PIXEL_FORMAT_RGBA_8_PACKED);
	pub const RGB_10_PACKED: Self = Self::from_raw(aravis_sys::ARV_PIXEL_FORMAT_RGB_10_PACKED);
	pub const RGB_10_PLANAR: Self = Self::from_raw(aravis_sys::ARV_PIXEL_FORMAT_RGB_10_PLANAR);
	pub const RGB_12_PACKED: Self = Self::from_raw(aravis_sys::ARV_PIXEL_FORMAT_RGB_12_PACKED);
	pub const RGB_12_PLANAR: Self = Self::from_raw(aravis_sys::ARV_PIXEL_FORMAT_RGB_12_PLANAR);
	pub const RGB_16_PLANAR: Self = Self::from_raw(aravis_sys::ARV_PIXEL_FORMAT_RGB_16_PLANAR);
	pub const RGB_8_PACKED: Self = Self::from_raw(aravis_sys::ARV_PIXEL_FORMAT_RGB_8_PACKED);
	pub const RGB_8_PLANAR: Self = Self::from_raw(aravis_sys::ARV_PIXEL_FORMAT_RGB_8_PLANAR);
	pub const YUV_411_PACKED: Self = Self::from_raw(aravis_sys::ARV_PIXEL_FORMAT_YUV_411_PACKED);
	pub const YUV_422_PACKED: Self = Self::from_raw(aravis_sys::ARV_PIXEL_FORMAT_YUV_422_PACKED);
	pub const YUV_422_YUYV_PACKED: Self =
		Self::from_raw(aravis_sys::ARV_PIXEL_FORMAT_YUV_422_YUYV_PACKED);
	pub const YUV_444_PACKED: Self = Self::from_raw(aravis_sys::ARV_PIXEL_FORMAT_YUV_444_PACKED);
}
