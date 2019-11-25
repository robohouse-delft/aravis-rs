use std::sync::mpsc;
use crate::image::ArcImage;

pub fn run_gui(channel: mpsc::Receiver<(usize, ArcImage)>) -> Result<(), String> {
	use gtk::DialogExt;
	gtk::init()
		.map_err(|e| format!("Failed to initialized GTK: {}", e))?;

	gtk::MessageDialog::new(None::<&gtk::Window>,
		gtk::DialogFlags::empty(),
		gtk::MessageType::Info,
		gtk::ButtonsType::Ok,
		"Hello World").run();

	Ok(())
}
