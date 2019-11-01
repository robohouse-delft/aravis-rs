use aravis::BufferExt;
use aravis::CameraExt;
use std::path::{Path, PathBuf};
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(setting = structopt::clap::AppSettings::ColoredHelp)]
#[structopt(setting = structopt::clap::AppSettings::UnifiedHelpMessage)]
struct Options {
	/// The IP address of the camera to connecto to.
	id: String,

	/// Where to save the output file.
	out: PathBuf,
}

fn main() {
	let options = Options::from_args();
	println!("Connecting to camera {}.", options.id);
	let camera = match aravis::Camera::new(Some(&options.id)) {
		Some(x) => x,
		None => {
			println!("Failed to connect to camera.");
			std::process::exit(1);
		},
	};

	println!("Connected");

	let start = std::time::Instant::now();

	let buffer = match camera.acquisition(3_000_000) {
		Some(x) => x,
		None => {
			println!("Failed to acquire image.");
			std::process::exit(1);
		}
	};

	println!("Capture time: {}", start.elapsed().as_secs_f64());

	let width = buffer.get_image_width();
	let height = buffer.get_image_height();
	println!("Recorded image of {}x{} pixel.", width, height);

	if let Err(err) = write_png(&options.out, &buffer) {
		println!("Failed to save image to {}: {}.", options.out.display(), err);
		std::process::exit(1);
	};
}

fn write_png(path: impl AsRef<Path>, buffer: &aravis::Buffer) -> std::io::Result<()> {
	let width = buffer.get_image_width() as u32;
	let height = buffer.get_image_height() as u32;

	let path = path.as_ref();
	let file = std::fs::File::create(path)?;
	let mut writer = std::io::BufWriter::new(file);

	let mut encoder = png::Encoder::new(writer, width, height);
	encoder.set_color(png::ColorType::Grayscale);
	encoder.set_depth(png::BitDepth::Eight);
	let mut writer = encoder.write_header()?;

	let data = buffer.get_data();

	writer.write_image_data(&data[0..(width * height) as usize])?;
	Ok(())
}
