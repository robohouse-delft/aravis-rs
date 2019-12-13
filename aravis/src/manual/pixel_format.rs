use crate::PixelFormat;

/// Calculate the bits per pixel of a given format.
pub fn bits_per_pixel(format: PixelFormat) -> u8 {
	((format >> 16) & 0xFF) as u8
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
		assert_eq!(bits_per_pixel(MONO_8), 8);
		assert_eq!(bits_per_pixel(RGB_8_PACKED), 24);
		assert_eq!(bits_per_pixel(BAYER_RG_10), 16);
		assert_eq!(bits_per_pixel(BAYER_RG_10P), 10);
		assert_eq!(bits_per_pixel(BAYER_RG_12), 16);
		assert_eq!(bits_per_pixel(BAYER_RG_12P), 12);
	}
}

pub mod pixel_formats {
	use crate::PixelFormat;

	pub const BAYER_BG_10                : PixelFormat = aravis_sys::ARV_PIXEL_FORMAT_BAYER_BG_10;
	pub const BAYER_BG_10P               : PixelFormat = aravis_sys::ARV_PIXEL_FORMAT_BAYER_BG_10P;
	pub const BAYER_BG_10_PACKED         : PixelFormat = aravis_sys::ARV_PIXEL_FORMAT_BAYER_BG_10_PACKED;
	pub const BAYER_BG_12                : PixelFormat = aravis_sys::ARV_PIXEL_FORMAT_BAYER_BG_12;
	pub const BAYER_BG_12P               : PixelFormat = aravis_sys::ARV_PIXEL_FORMAT_BAYER_BG_12P;
	pub const BAYER_BG_12_PACKED         : PixelFormat = aravis_sys::ARV_PIXEL_FORMAT_BAYER_BG_12_PACKED;
	pub const BAYER_BG_16                : PixelFormat = aravis_sys::ARV_PIXEL_FORMAT_BAYER_BG_16;
	pub const BAYER_BG_8                 : PixelFormat = aravis_sys::ARV_PIXEL_FORMAT_BAYER_BG_8;
	pub const BAYER_GB_10                : PixelFormat = aravis_sys::ARV_PIXEL_FORMAT_BAYER_GB_10;
	pub const BAYER_GB_10P               : PixelFormat = aravis_sys::ARV_PIXEL_FORMAT_BAYER_GB_10P;
	pub const BAYER_GB_10_PACKED         : PixelFormat = aravis_sys::ARV_PIXEL_FORMAT_BAYER_GB_10_PACKED;
	pub const BAYER_GB_12                : PixelFormat = aravis_sys::ARV_PIXEL_FORMAT_BAYER_GB_12;
	pub const BAYER_GB_12P               : PixelFormat = aravis_sys::ARV_PIXEL_FORMAT_BAYER_GB_12P;
	pub const BAYER_GB_12_PACKED         : PixelFormat = aravis_sys::ARV_PIXEL_FORMAT_BAYER_GB_12_PACKED;
	pub const BAYER_GB_16                : PixelFormat = aravis_sys::ARV_PIXEL_FORMAT_BAYER_GB_16;
	pub const BAYER_GB_8                 : PixelFormat = aravis_sys::ARV_PIXEL_FORMAT_BAYER_GB_8;
	pub const BAYER_GR_10                : PixelFormat = aravis_sys::ARV_PIXEL_FORMAT_BAYER_GR_10;
	pub const BAYER_GR_10P               : PixelFormat = aravis_sys::ARV_PIXEL_FORMAT_BAYER_GR_10P;
	pub const BAYER_GR_10_PACKED         : PixelFormat = aravis_sys::ARV_PIXEL_FORMAT_BAYER_GR_10_PACKED;
	pub const BAYER_GR_12                : PixelFormat = aravis_sys::ARV_PIXEL_FORMAT_BAYER_GR_12;
	pub const BAYER_GR_12P               : PixelFormat = aravis_sys::ARV_PIXEL_FORMAT_BAYER_GR_12P;
	pub const BAYER_GR_12_PACKED         : PixelFormat = aravis_sys::ARV_PIXEL_FORMAT_BAYER_GR_12_PACKED;
	pub const BAYER_GR_16                : PixelFormat = aravis_sys::ARV_PIXEL_FORMAT_BAYER_GR_16;
	pub const BAYER_GR_8                 : PixelFormat = aravis_sys::ARV_PIXEL_FORMAT_BAYER_GR_8;
	pub const BAYER_RG_10                : PixelFormat = aravis_sys::ARV_PIXEL_FORMAT_BAYER_RG_10;
	pub const BAYER_RG_10P               : PixelFormat = aravis_sys::ARV_PIXEL_FORMAT_BAYER_RG_10P;
	pub const BAYER_RG_10_PACKED         : PixelFormat = aravis_sys::ARV_PIXEL_FORMAT_BAYER_RG_10_PACKED;
	pub const BAYER_RG_12                : PixelFormat = aravis_sys::ARV_PIXEL_FORMAT_BAYER_RG_12;
	pub const BAYER_RG_12P               : PixelFormat = aravis_sys::ARV_PIXEL_FORMAT_BAYER_RG_12P;
	pub const BAYER_RG_12_PACKED         : PixelFormat = aravis_sys::ARV_PIXEL_FORMAT_BAYER_RG_12_PACKED;
	pub const BAYER_RG_16                : PixelFormat = aravis_sys::ARV_PIXEL_FORMAT_BAYER_RG_16;
	pub const BAYER_RG_8                 : PixelFormat = aravis_sys::ARV_PIXEL_FORMAT_BAYER_RG_8;
	pub const BGRA_8_PACKED              : PixelFormat = aravis_sys::ARV_PIXEL_FORMAT_BGRA_8_PACKED;
	pub const BGR_10_PACKED              : PixelFormat = aravis_sys::ARV_PIXEL_FORMAT_BGR_10_PACKED;
	pub const BGR_12_PACKED              : PixelFormat = aravis_sys::ARV_PIXEL_FORMAT_BGR_12_PACKED;
	pub const BGR_8_PACKED               : PixelFormat = aravis_sys::ARV_PIXEL_FORMAT_BGR_8_PACKED;
	pub const CUSTOM_BAYER_BG_12_PACKED  : PixelFormat = aravis_sys::ARV_PIXEL_FORMAT_CUSTOM_BAYER_BG_12_PACKED;
	pub const CUSTOM_BAYER_BG_16         : PixelFormat = aravis_sys::ARV_PIXEL_FORMAT_CUSTOM_BAYER_BG_16;
	pub const CUSTOM_BAYER_GB_12_PACKED  : PixelFormat = aravis_sys::ARV_PIXEL_FORMAT_CUSTOM_BAYER_GB_12_PACKED;
	pub const CUSTOM_BAYER_GB_16         : PixelFormat = aravis_sys::ARV_PIXEL_FORMAT_CUSTOM_BAYER_GB_16;
	pub const CUSTOM_BAYER_GR_12_PACKED  : PixelFormat = aravis_sys::ARV_PIXEL_FORMAT_CUSTOM_BAYER_GR_12_PACKED;
	pub const CUSTOM_BAYER_GR_16         : PixelFormat = aravis_sys::ARV_PIXEL_FORMAT_CUSTOM_BAYER_GR_16;
	pub const CUSTOM_BAYER_RG_12_PACKED  : PixelFormat = aravis_sys::ARV_PIXEL_FORMAT_CUSTOM_BAYER_RG_12_PACKED;
	pub const CUSTOM_BAYER_RG_16         : PixelFormat = aravis_sys::ARV_PIXEL_FORMAT_CUSTOM_BAYER_RG_16;
	pub const CUSTOM_YUV_422_YUYV_PACKED : PixelFormat = aravis_sys::ARV_PIXEL_FORMAT_CUSTOM_YUV_422_YUYV_PACKED;
	pub const MONO_10                    : PixelFormat = aravis_sys::ARV_PIXEL_FORMAT_MONO_10;
	pub const MONO_10_PACKED             : PixelFormat = aravis_sys::ARV_PIXEL_FORMAT_MONO_10_PACKED;
	pub const MONO_12                    : PixelFormat = aravis_sys::ARV_PIXEL_FORMAT_MONO_12;
	pub const MONO_12_PACKED             : PixelFormat = aravis_sys::ARV_PIXEL_FORMAT_MONO_12_PACKED;
	pub const MONO_14                    : PixelFormat = aravis_sys::ARV_PIXEL_FORMAT_MONO_14;
	pub const MONO_16                    : PixelFormat = aravis_sys::ARV_PIXEL_FORMAT_MONO_16;
	pub const MONO_8                     : PixelFormat = aravis_sys::ARV_PIXEL_FORMAT_MONO_8;
	pub const MONO_8_SIGNED              : PixelFormat = aravis_sys::ARV_PIXEL_FORMAT_MONO_8_SIGNED;
	pub const RGBA_8_PACKED              : PixelFormat = aravis_sys::ARV_PIXEL_FORMAT_RGBA_8_PACKED;
	pub const RGB_10_PACKED              : PixelFormat = aravis_sys::ARV_PIXEL_FORMAT_RGB_10_PACKED;
	pub const RGB_10_PLANAR              : PixelFormat = aravis_sys::ARV_PIXEL_FORMAT_RGB_10_PLANAR;
	pub const RGB_12_PACKED              : PixelFormat = aravis_sys::ARV_PIXEL_FORMAT_RGB_12_PACKED;
	pub const RGB_12_PLANAR              : PixelFormat = aravis_sys::ARV_PIXEL_FORMAT_RGB_12_PLANAR;
	pub const RGB_16_PLANAR              : PixelFormat = aravis_sys::ARV_PIXEL_FORMAT_RGB_16_PLANAR;
	pub const RGB_8_PACKED               : PixelFormat = aravis_sys::ARV_PIXEL_FORMAT_RGB_8_PACKED;
	pub const RGB_8_PLANAR               : PixelFormat = aravis_sys::ARV_PIXEL_FORMAT_RGB_8_PLANAR;
	pub const YUV_411_PACKED             : PixelFormat = aravis_sys::ARV_PIXEL_FORMAT_YUV_411_PACKED;
	pub const YUV_422_PACKED             : PixelFormat = aravis_sys::ARV_PIXEL_FORMAT_YUV_422_PACKED;
	pub const YUV_422_YUYV_PACKED        : PixelFormat = aravis_sys::ARV_PIXEL_FORMAT_YUV_422_YUYV_PACKED;
	pub const YUV_444_PACKED             : PixelFormat = aravis_sys::ARV_PIXEL_FORMAT_YUV_444_PACKED;
}
