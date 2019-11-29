use std::path::Path;
use std::sync::Arc;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum ImageFormat {
	Mono8,
	Mono16,
	Rgb8,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct ImageInfo {
	pub width: u32,
	pub height: u32,
	pub format: ImageFormat,
}

#[derive(Debug, Clone)]
pub struct BoxImage {
	pub info: ImageInfo,
	pub data: Box<[u8]>,
}

#[derive(Debug, Clone)]
pub struct ArcImage {
	pub info: ImageInfo,
	pub data: Arc<[u8]>,
}

impl BoxImage {
	pub fn write_png(&self, path: impl AsRef<Path>) -> std::io::Result<()> {
		write_png(path, &self.info, &self.data)
	}
}

impl ArcImage {
	pub fn write_png(&self, path: impl AsRef<Path>) -> std::io::Result<()> {
		write_png(path, &self.info, &self.data)
	}
}

fn write_png(path: impl AsRef<Path>, info: &ImageInfo, data: &[u8]) -> std::io::Result<()> {
	let path = path.as_ref();
	let file = std::fs::File::create(path)?;
	let writer = std::io::BufWriter::new(file);

	let mut encoder = png::Encoder::new(writer, info.width, info.height);

	let (color, depth) = match info.format {
		ImageFormat::Mono8  => (png::ColorType::Grayscale, png::BitDepth::Eight),
		ImageFormat::Mono16 => (png::ColorType::Grayscale, png::BitDepth::Sixteen),
		ImageFormat::Rgb8   => (png::ColorType::RGB,       png::BitDepth::Eight),
	};

	encoder.set_color(color);
	encoder.set_depth(depth);

	let mut writer = encoder.write_header()?;

	let length = (info.width * info.height) as usize;
	writer.write_image_data(&data[0..length])?;
	Ok(())
}

impl From<BoxImage> for ArcImage {
	fn from(other: BoxImage) -> Self {
		let BoxImage { info, data } = other;
		Self {
			info,
			data: Arc::from(data),
		}
	}
}
