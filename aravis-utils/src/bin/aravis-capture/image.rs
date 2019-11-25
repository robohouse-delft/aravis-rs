use std::path::Path;

pub struct Image {
	pub width: u32,
	pub height: u32,
	pub data: Box<[u8]>,
}

impl Image {
	pub fn write_png(&self, path: impl AsRef<Path>) -> std::io::Result<()> {
		let path = path.as_ref();
		let file = std::fs::File::create(path)?;
		let writer = std::io::BufWriter::new(file);

		let mut encoder = png::Encoder::new(writer, self.width, self.height);
		encoder.set_color(png::ColorType::Grayscale);
		encoder.set_depth(png::BitDepth::Eight);
		let mut writer = encoder.write_header()?;

		let length = (self.width * self.height) as usize;
		writer.write_image_data(&self.data[0..length])?;
		Ok(())
	}
}
