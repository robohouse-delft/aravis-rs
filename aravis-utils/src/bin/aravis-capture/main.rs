#![feature(get_mut_unchecked)]
#![feature(maybe_uninit_slice)]
#![feature(new_uninit)]

use aravis::BufferExt;
use aravis::BufferExtManual;
use aravis::CameraExt;
use aravis::CameraExtManual;
use aravis::StreamExt;
use std::path::PathBuf;
use std::sync::mpsc;
use std::sync::Arc;
use std::time::{Duration, Instant};
use structopt::StructOpt;

#[cfg(feature = "gtk")]
mod gui;
pub mod image;

use image::{ArcImage, ImageFormat, ImageInfo};

#[derive(StructOpt)]
#[structopt(setting = structopt::clap::AppSettings::ColoredHelp)]
#[structopt(setting = structopt::clap::AppSettings::UnifiedHelpMessage)]
struct Options {
	/// The IP address of the camera to connecto to.
	id: String,

	/// Show recorded images in a graphical window.
	#[structopt(long)]
	#[cfg(feature = "gtk")]
	show: bool,

	/// Save recorded images to a folder.
	#[structopt(long)]
	#[structopt(value_name = "PATH")]
	save: Option<PathBuf>,

	/// The name prefix of the saved images.
	#[structopt(long)]
	#[structopt(value_name = "PREFIX")]
	#[structopt(default_value = "image-")]
	out_name: String,

	/// The numer of images to record.
	#[structopt(long, short)]
	#[structopt(default_value = "1")]
	count: usize,

	/// The frequency at which to record images.
	#[structopt(long, short)]
	#[structopt(default_value = "30")]
	frequency: f64,
}

fn main() {
	aravis_utils::init_logging();

	let options = Options::from_args();
	let camera_id = options.id;
	let count = options.count;
	let period = Duration::from_secs_f64(1.0 / options.frequency);
	let name_prefix = options.out_name;

	let mut senders = Vec::with_capacity(2);

	// Start write thread if saving images.
	let write_thread = options.save.map(|path| {
		let (sender, receiver) = mpsc::channel::<(usize, ArcImage)>();
		senders.push(sender);
		std::thread::spawn(move || {
			for (i, image) in receiver {
				let path = path.join(format!("{}{:03}.png", name_prefix, i));
				if let Err(err) = image.write_png(&path) {
					log::error!("Failed to save image to {}: {}.", path.display(), err);
				};
			}
		})
	});

	#[cfg(feature = "gtk")]
	{
		if options.show {
			let (sender, receiver) = mpsc::channel::<(usize, ArcImage)>();
			senders.push(sender);
			if let Err(e) = gui::run_gui(receiver) {
				log::error!("{}", e);
			}
		}
	}

	let camera_thread = std::thread::spawn(move || {
		if let Err(e) = run_camera_loop(&camera_id, count, period, &senders) {
			// Only log the error, let the write thread stop on by itself when the channel is empty.
			log::error!("{}", e);
		}
	});

	// Join all threads.
	let _ = camera_thread.join();
	if let Some(write_thread) = write_thread {
		let _ = write_thread.join();
	}
}

fn run_camera_loop(
	camera_id: &str,
	count: usize,
	period: Duration,
	channels: &[mpsc::Sender<(usize, ArcImage)>],
) -> Result<(), String> {
	log::info!("Connecting to camera {}.", camera_id);
	let camera = aravis::Camera::new(Some(&camera_id))
		.ok_or("Failed to connect to camera")?;
	log::info!("Connected.");

	let stream = camera.create_stream();

	// Fill stream with 10 buffers.
	let (_, _, width, height) = camera.get_region().unwrap();
	for _ in 0..10 {
		// TODO: use pixel depth to calculate size.
		stream.push_buffer(&make_buffer((width * height) as usize))
	}

	let _ = camera.start_acquisition();

	let start = Instant::now();
	let mut next_frame = Instant::now() + period;

	for i in 0..count {
		let start = Instant::now();

		let buffer = match stream.timeout_pop_buffer(3_000_000) {
			Some(x) => x,
			None => {
				log::error!("Failed to acquire image.");
				continue;
			}
		};
		log::info!("Capture time: {}", start.elapsed().as_secs_f64());

		let image = unsafe { consume_buffer(buffer) };
		stream.push_buffer(&make_buffer((width * height) as usize));

		for channel in channels {
			channel.send((i, image.clone())).unwrap_or_else(|e| {
				log::error!("Failed to send image to writer thread: {}.", e);
			});
		}

		let now = Instant::now();
		if next_frame > now {
			std::thread::sleep(next_frame.duration_since(now));
		}

		next_frame += period;
	}

	let total_duration = start.elapsed().as_secs_f64();
	log::info!("Total record time: {}s, average FPS: {}", total_duration, count as f64 / total_duration);

	Ok(())
}

fn make_buffer(len: usize) -> aravis::Buffer {
	unsafe {
		let mut buffer = Arc::<[u8]>::new_uninit_slice(len);
		let data = std::mem::MaybeUninit::first_ptr_mut(Arc::get_mut_unchecked(&mut buffer));
		let result = aravis::Buffer::new_preallocated(data, len);
		std::mem::forget(buffer);
		result
	}
}

unsafe fn consume_buffer(buffer: aravis::Buffer) -> ArcImage {
	// TODO: check buffer status
	let (data, len) = buffer.get_data();
	ArcImage {
		info: ImageInfo {
			width: buffer.get_image_width()  as u32,
			height: buffer.get_image_height() as u32,
			format: ImageFormat::Mono8,
		},
		data: arc_slice_from_raw(data, len),
	}
}

unsafe fn arc_slice_from_raw<T>(data: *mut T, len: usize) -> Arc<[T]> {
	Arc::from_raw(std::slice::from_raw_parts_mut(data, len))
}
