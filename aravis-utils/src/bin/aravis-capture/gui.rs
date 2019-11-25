use std::sync::mpsc;
use crate::image::{ArcImage, ImageFormat};
use crate::ImageCallback;

use gdk_pixbuf::Pixbuf;
use gtk::BoxExt;
use gtk::ContainerExt;
use gtk::ImageExt;
use gtk::WidgetExt;
use glib::ObjectType;

struct SetImage {
	widget: usize,
}

impl SetImage {
	fn set_image(&self, _i: usize, image: ArcImage) {
		let widget = self.widget;
		gtk::idle_add(move || unsafe {
			let widget = widget as *mut gtk_sys::GtkImage;
			let widget : gtk::Image = glib::translate::from_glib_none(widget);
			let pixbuf = match Pixbuf::new(gdk_pixbuf::Colorspace::Rgb, false, 8, image.info.width as i32, image.info.height as i32) {
				Some(x) => x,
				None => {
					log::error!("Failed to allocate pixbuf for image.");
					return gtk::Continue(false);
				}
			};

			// TODO
			assert!(image.info.format == ImageFormat::Mono8);

			let pixels = pixbuf.get_pixels();
			let rowstride = pixbuf.get_rowstride() as usize;
			for y in 0..image.info.height as usize {
				for x in 0..image.info.width as usize {
					let value = image.data[image.info.width as usize * y + x];
					pixels[y * rowstride + x * 3 + 0] = value;
					pixels[y * rowstride + x * 3 + 1] = value;
					pixels[y * rowstride + x * 3 + 2] = value;
				}
			}

			widget.set_from_pixbuf(Some(&pixbuf));
			gtk::Continue(false)
		});
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

	let set_image = SetImage { widget: image_widget.as_ptr() as usize };
	let callback = Box::new(move |i, image| set_image.set_image(i, image));

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
