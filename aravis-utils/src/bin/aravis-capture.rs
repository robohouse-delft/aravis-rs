use aravis::prelude::{CameraExt, CameraExtManual, StreamExt};
use image::DynamicImage;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::mpsc;
use std::time::{Duration, Instant, SystemTime};

type ArcImage = Arc<image::DynamicImage>;
type ImageCallback = Box<dyn FnMut(usize, SystemTime, ArcImage) + Send>;

#[derive(Debug, Copy, Clone, clap::ValueEnum)]
enum UsbMode {
	Sync,
	Async,
}

impl AsRef<aravis::UvUsbMode> for UsbMode {
	fn as_ref(&self) -> &aravis::UvUsbMode {
		match self {
			UsbMode::Sync => &aravis::UvUsbMode::Sync,
			UsbMode::Async => &aravis::UvUsbMode::Async,
		}
	}
}

#[derive(clap::Parser)]
struct Options {
	/// The IP address of the camera to connect to.
	id: String,

	/// Show recorded images in a graphical window.
	#[clap(long)]
	#[cfg(feature = "gui")]
	show: bool,

	/// Save recorded images to a folder.
	#[clap(long)]
	#[clap(value_name = "PATH")]
	save: Option<PathBuf>,

	/// The name prefix of the saved images.
	#[clap(long)]
	#[clap(value_name = "PREFIX")]
	#[clap(default_value = "image-")]
	out_name: String,

	/// The numer of images to record.
	#[clap(long, short)]
	#[clap(default_value = "1")]
	count: usize,

	#[clap(long)]
	#[clap(conflicts_with = "count")]
	forever: bool,

	/// The frequency at which to record images.
	#[clap(long, short)]
	#[clap(default_value = "30")]
	frequency: f64,

	/// The mode of communication for USB devices.
	#[clap(long, short)]
	#[clap(default_value = "sync")]
	#[clap(value_enum)]
	usb_mode: UsbMode,
}

#[show_image::main]
fn main() {
	aravis_utils::init_logging(&[env!("CARGO_CRATE_NAME")]);

	let options: Options = clap::Parser::parse();
	let camera_id = options.id;
	let count = if options.forever {
		0
	} else {
		options.count
	};
	let period = Duration::from_secs_f64(1.0 / options.frequency);

	let name_prefix = options.out_name;
	let format_name = move |_i, time: SystemTime, suffix| {
		let time : chrono::DateTime<chrono::Utc> = time.into();
		let time = time.format("%F-%H-%M-%S-%9f");
		format!("{}{}{}", name_prefix, time, suffix)
	};

	let mut senders = Vec::<ImageCallback>::with_capacity(2);

	// Start write thread if saving images.
	let write_thread = options.save.map(|path| {
		let (sender, receiver) = mpsc::channel::<(usize, SystemTime, ArcImage)>();
		senders.push(Box::new(move |i, time, image| if let Err(e) = sender.send((i, time, image)) {
			log::error!("Failed to send image to writer thread: {}.", e);
		}));

		let format_name = format_name.clone();
		std::thread::spawn(move || {
			for (i, time, image) in receiver {
				let path = path.join(format_name(i, time, ".png"));
				if let Err(err) = image.save(&path) {
					log::error!("Failed to save image to {}: {}.", path.display(), err);
				};
			}
		})
	});

	let mut gui_thread = None;
	#[cfg(feature = "gui")] {
		if options.show {
			let (sender, receiver) = mpsc::channel::<(usize, SystemTime, ArcImage)>();
			senders.push(Box::new(move |i, time, image| {
				if let Err(e) = sender.send((i, time, image)) {
					log::error!("Failed to send image to GUI thread: {}.", e);
				}
			}));

			let format_name = format_name.clone();
			gui_thread = Some(std::thread::spawn(move || {
				let window = show_image::create_window("image", Default::default()).unwrap();
				for (i, time, image) in receiver {
					window.set_image(format_name(i, time, ""), image).unwrap();
				}
			}));
		}
	}

	let convert_color = gui_thread.is_some();
	let usb_mode = options.usb_mode;
	let camera_thread = std::thread::spawn(move || {
		if let Err(e) = run_camera_loop(&camera_id, &usb_mode, count, period, convert_color, &mut senders) {
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
	usb_mode: &UsbMode,
	count: usize,
	period: Duration,
	convert_color: bool,
	callbacks: &mut [ImageCallback],
) -> Result<(), String> {
	log::info!("Connecting to camera {}.", camera_id);
	let camera = aravis::Camera::new(Some(&camera_id))
		.map_err(|e| format!("Failed to connect to camera: {}", e))?;
	log::info!("Connected.");

	let device = camera.device()
		.ok_or("no device associated with camera")?;
	if let Ok(device) = device.downcast::<aravis::UvDevice>() {
		device.set_usb_mode(*usb_mode.as_ref());
	}

	let pixel_format = camera.pixel_format()
		.map_err(|e| format!("Failed to determine pixel format: {}", e))?;
	let (_, _, width, height) = camera.region().unwrap();
	let make_buffer = || aravis::Buffer::new_leaked_image(pixel_format, width as usize, height as usize);

	let stream = camera.create_stream().map_err(|e| format!("Failed to open stream: {}", e))?;
	stream.push_buffer(&make_buffer());

	camera.set_acquisition_mode(aravis::AcquisitionMode::Continuous)
		.map_err(|e| format!("Failed to set acquisition mode to continuous: {}.", e))?;
	camera.set_trigger("Software")
		.map_err(|e| format!("Failed to set trigger source to software: {}.", e))?;
	camera.start_acquisition()
		.map_err(|e| format!("Failed to start acquisition: {}.", e))?;

	let start = Instant::now();
	let mut next_frame = Instant::now() + period;

	for i in (0..).take_while(|i| count == 0 || *i < count) {
		camera.software_trigger().map_err(|e| format!("Failed to trigger camera: {}", e))?;
		let trigger_time = SystemTime::now();

		let buffer = match stream.timeout_pop_buffer(3_000_000) {
			Some(x) => x,
			None => {
				log::error!("Failed to acquire image.");
				continue;
			}
		};

		stream.push_buffer(&make_buffer());

		let image = match unsafe { buffer.into_image() } {
			Ok(x) => x,
			Err(e) => {
				log::error!("Failed to convert buffer into image: {}", e);
				continue;
			}
		};

		let image = if convert_color {
			Arc::new(DynamicImage::ImageRgb8(image.into_rgb8()))
		} else {
			Arc::new(image)
		};

		for callback in callbacks.iter_mut() {
			callback(i, trigger_time, image.clone());
		}

		let now = Instant::now();
		if next_frame > now {
			std::thread::sleep(next_frame.duration_since(now));
		}

		next_frame += period;
		if next_frame < now {
			next_frame = now;
		}

	}

	let total_duration = start.elapsed().as_secs_f64();
	log::info!("Total record time: {}s, average FPS: {}", total_duration, count as f64 / total_duration);

	Ok(())
}
