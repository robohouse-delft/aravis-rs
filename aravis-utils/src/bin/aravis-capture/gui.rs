use std::sync::Arc;
use std::sync::Mutex;
use std::sync::mpsc;
use std::sync::atomic::{AtomicUsize, Ordering};
use crate::ArcImage;
use crate::ImageCallback;

use gdk_pixbuf::Pixbuf;
use gtk::BoxExt;
use gtk::ContainerExt;
use gtk::ImageExt;
use gtk::WidgetExt;

struct SetImage {
	next_image: Mutex<Option<(usize, ArcImage)>>,
	last_index: AtomicUsize,
	last_image: Mutex<Option<ArcImage>>,
}

impl SetImage {
	fn new() -> Self {
		Self {
			next_image: Mutex::new(None),
			last_index: AtomicUsize::new(0),
			last_image: Mutex::new(None),
		}
	}

	unsafe fn check(&self, widget: &gtk::Image) {
		let (index, image) = {
			let guard = self.next_image.lock().unwrap();
			match guard.as_ref() {
				Some(x) => x.clone(),
				None => return,
			}
		};

		let last_index = self.last_index.load(Ordering::Acquire);
		if index == last_index {
			return;
		}

		let rgb = image.as_rgb8().expect("invalid image type, expected rgb8");
		let data = rgb.as_flat_samples();
		let data = data.as_slice();
		let pixbuf = gdk_pixbuf_sys::gdk_pixbuf_new_from_data(
			data.as_ptr(),
			gdk_pixbuf_sys::GDK_COLORSPACE_RGB,
			0,
			8,
			rgb.width() as std::os::raw::c_int,
			rgb.height() as std::os::raw::c_int,
			rgb.width() as std::os::raw::c_int * 3,
			None,
			std::ptr::null_mut(),
		);

		let pixbuf = glib::translate::from_glib_full(pixbuf);
		widget.set_from_pixbuf(Some(&pixbuf));
		self.last_image.lock().unwrap().replace(image.clone());
	}

	fn set_next_image(&self, index: usize, image: ArcImage) {
		self.next_image.lock().unwrap().replace((index, image));
	}
}

fn build_gui() -> Result<(gtk::Window, ImageCallback), String> {
	let window = gtk::Window::new(gtk::WindowType::Toplevel);
	let main_box = gtk::Box::new(gtk::Orientation::Vertical, 0);
	let image_widget = gtk::Image::new();

	main_box.pack_start(&image_widget, true, true, 0);
	window.add(&main_box);
	window.show_all();

	let pixbuf = Pixbuf::new(gdk_pixbuf::Colorspace::Rgb, false, 8, 800, 600)
		.ok_or("Failed to create Pixbuf for image display.")?;
	pixbuf.fill(0);
	image_widget.set_from_pixbuf(Some(&pixbuf));

	let set_image = Arc::new(SetImage::new());

	let callback = {
		let set_image = set_image.clone();
		Box::new(move |i, image| set_image.set_next_image(i, image))
	};

	gtk::timeout_add(20, move || unsafe {
		set_image.check(&image_widget);
		gtk::Continue(true)
	});

	Ok((window, callback))
}

pub fn run_gui(sender: mpsc::Sender<ImageCallback>) -> Result<(), String> {
	gtk::init().map_err(|e| format!("Failed to initialize GTK: {}", e))?;
	let (window, callback) = build_gui()?;
	sender.send(callback).map_err(|e| format!("Failed to send callback back to caller thread: {}", e))?;
	window.show_all();
	gtk::main();
	Ok(())
}
