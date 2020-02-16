use aravis::BufferExtManual;
use aravis::CameraExt;
use aravis::CameraExtManual;
use aravis::StreamExt;
use image::DynamicImage;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::mpsc;
use std::time::{Duration, Instant};
use structopt::StructOpt;

type ArcImage = Arc<image::DynamicImage>;
type ImageCallback = Box<dyn FnMut(usize, ArcImage) + Send>;

#[derive(StructOpt)]
#[structopt(setting = structopt::clap::AppSettings::ColoredHelp)]
#[structopt(setting = structopt::clap::AppSettings::UnifiedHelpMessage)]
struct Options {
	/// The IP address of the camera to connecto to.
	id: String,

	/// Show recorded images in a graphical window.
	#[structopt(long)]
	#[cfg(feature = "gui")]
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

	#[structopt(long)]
	#[structopt(conflicts_with = "count")]
	forever: bool,

	/// The frequency at which to record images.
	#[structopt(long, short)]
	#[structopt(default_value = "30")]
	frequency: f64,
}

fn main() {
	aravis_utils::init_logging();

	let options = Options::from_args();
	let camera_id = options.id;
	let count = if options.forever {
		0
	} else {
		options.count
	};
	let period = Duration::from_secs_f64(1.0 / options.frequency);
	let name_prefix = options.out_name;

	let mut senders = Vec::<ImageCallback>::with_capacity(2);

	// Start write thread if saving images.
	let write_thread = options.save.map(|path| {
		let (sender, receiver) = mpsc::channel::<(usize, ArcImage)>();
		senders.push(Box::new(move |i, image| if let Err(e) = sender.send((i, image)) {
			log::error!("Failed to send image to writer thread: {}.", e);
		}));
		std::thread::spawn(move || {
			for (i, image) in receiver {
				let path = path.join(format!("{}{:03}.png", name_prefix, i));
				if let Err(err) = image.save(&path) {
					log::error!("Failed to save image to {}: {}.", path.display(), err);
				};
			}
		})
	});

	let mut gui_thread = None;
	#[cfg(feature = "gui")] {
		if options.show {
			let (sender, receiver) = mpsc::channel::<(usize, ArcImage)>();
			gui_thread = Some(std::thread::spawn(|| {
				let window = show_image::make_window("image").unwrap();
				for (i, image) in receiver {
					let name = format!("image-{:03}", i);
					window.set_image(&*image, name).unwrap();
				}
			}));
			senders.push(Box::new(move |i, image| {
				if let Err(e) = sender.send((i, image)) {
					log::error!("Failed to send image to GUI thread: {}.", e);
				}
			}));
		}
	}

	let convert_color = gui_thread.is_some();
	let camera_thread = std::thread::spawn(move || {
		if let Err(e) = run_camera_loop(&camera_id, count, period, convert_color, &mut senders) {
			// Only log the error, let the write thread stop on by itself when the channel is empty.
			log::error!("{}", e);
		}
		// TODO: Stop other threads.
	});

	// Join all threads.
	let _ = camera_thread.join();
	if let Some(thread) = write_thread {
		let _ = thread.join();
	}
}

fn run_camera_loop(
	camera_id: &str,
	count: usize,
	period: Duration,
	convert_color: bool,
	callbacks: &mut [ImageCallback],
) -> Result<(), String> {
	log::info!("Connecting to camera {}.", camera_id);
	let camera = aravis::Camera::new(Some(&camera_id))
		.ok_or("Failed to connect to camera")?;
	log::info!("Connected.");

	let pixel_format = camera.get_pixel_format()
		.map_err(|e| format!("Failed to determine pixel format: {}", e))?;
	let (_, _, width, height) = camera.get_region().unwrap();
	let make_buffer = || aravis::Buffer::new_leaked_image(pixel_format, width as usize, height as usize);

	let stream = camera.create_stream();

	// Fill stream with 10 buffers.
	for _ in 0..10 {
		stream.push_buffer(&make_buffer())
	}

	let _ = camera.start_acquisition();

	let start = Instant::now();
	let mut next_frame = Instant::now() + period;

	for i in (0..).take_while(|i| count == 0 || *i < count) {

		let start = Instant::now();

		let buffer = match stream.timeout_pop_buffer(3_000_000) {
			Some(x) => x,
			None => {
				log::error!("Failed to acquire image.");
				continue;
			}
		};
		log::info!("Capture time: {}", start.elapsed().as_secs_f64());

		let image = match unsafe { buffer.into_image() } {
			Ok(x) => x,
			Err(e) => {
				log::error!("Failed to convert buffer into image: {}", e);
				continue;
			}
		};

		stream.push_buffer(&make_buffer());

		let image = if convert_color {
			Arc::new(DynamicImage::ImageRgb8(image.into_rgb()))
		} else {
			Arc::new(image)
		};

		for callback in callbacks.iter_mut() {
			callback(i, image.clone());
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
