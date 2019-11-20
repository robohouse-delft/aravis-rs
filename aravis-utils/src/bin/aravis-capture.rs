#![feature(new_uninit)]
#![feature(maybe_uninit_slice)]

use aravis::BufferExt;
use aravis::BufferExtManual;
use aravis::CameraExt;
use aravis::CameraExtManual;
use aravis::StreamExt;
use std::path::Path;
use std::sync::mpsc;
use std::time::{Duration, Instant};
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(setting = structopt::clap::AppSettings::ColoredHelp)]
#[structopt(setting = structopt::clap::AppSettings::UnifiedHelpMessage)]
struct Options {
	/// The IP address of the camera to connecto to.
	id: String,

	/// Where to save the output file.
	#[structopt(long)]
	#[structopt(default_value = "./image-")]
	out: String,

	/// The numer of images to record.
	#[structopt(long, short)]
	#[structopt(default_value = "1")]
	count: usize,

	/// The frequency at which to record images.
	#[structopt(long, short)]
	#[structopt(default_value = "30")]
	frequency: f64,
}

struct Image {
	width: u32,
	height: u32,
	data: Box<[u8]>,
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

	let (sender, receiver) = mpsc::sync_channel::<(String, Image)>(options.count);
	let write_thread = std::thread::spawn(move || {
		for (path, image) in receiver {
			if let Err(err) = write_png(&path, &image) {
				eprintln!("Failed to save image to {}: {}.", path, err);
			};
		}
	});

	println!("Connected.");

	let stream = camera.create_stream();

	// Fill stream with 10 buffers.
	let (_, _, width, height) = camera.get_region().unwrap();
	for _ in 0..10 {
		// TODO: use pixel depth to calculate size.
		stream.push_buffer(&make_buffer((width * height) as usize))
	}

	let period = Duration::from_secs_f64(1.0 / options.frequency);
	let mut next_frame = Instant::now() + period;

	for i in 0..options.count {
		let start = Instant::now();

		let buffer = match stream.timeout_pop_buffer(3_000_000) {
			Some(x) => x,
			None => {
				eprintln!("Failed to acquire image.");
				continue;
			}
		};

		let end = Instant::now();

		println!("Capture time: {}", end.duration_since(start).as_secs_f64());

		let path = format!("{}{:03}.png", &options.out, i);
		let image = unsafe { consume_buffer(buffer) };

		sender.send((path, image)).unwrap_or_else(|e| {
			eprintln!("Failed to send image to writer thread: {}.", e);
		});

		let now = Instant::now();
		if next_frame > now {
			std::thread::sleep(next_frame.duration_since(now));
		}
		next_frame += period;
	}

	drop(sender);
	let _ = write_thread.join();
}

fn write_png(path: impl AsRef<Path>, image: &Image) -> std::io::Result<()> {
	let path = path.as_ref();
	let file = std::fs::File::create(path)?;
	let writer = std::io::BufWriter::new(file);

	let mut encoder = png::Encoder::new(writer, image.width, image.height);
	encoder.set_color(png::ColorType::Grayscale);
	encoder.set_depth(png::BitDepth::Eight);
	let mut writer = encoder.write_header()?;

	let length = (image.width * image.height) as usize;
	writer.write_image_data(&image.data[0..length])?;
	Ok(())
}

fn make_buffer(len: usize) -> aravis::Buffer {
	let mut buffer = Box::<[u8]>::new_uninit_slice(len);
	let data = std::mem::MaybeUninit::first_ptr_mut(&mut buffer);
	let result = unsafe { aravis::Buffer::new_preallocated(data, len) };
	std::mem::forget(buffer);
	result
}

unsafe fn consume_buffer(buffer: aravis::Buffer) -> Image {
	// TODO: check buffer status
	let (data, len) = buffer.get_data();
	Image {
		width  : buffer.get_image_width()  as u32,
		height : buffer.get_image_height() as u32,
		data   : boxed_slice_from_raw(data, len),
	}
}

unsafe fn boxed_slice_from_raw<T>(data: *mut T, len: usize) -> Box<[T]> {
	Box::from_raw(std::slice::from_raw_parts_mut(data, len))
}
