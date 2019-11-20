use aravis::Aravis;

fn main() {
	let aravis = Aravis::initialize().unwrap();

	let devices = aravis.get_device_list();
	if devices.is_empty() {
		eprintln!("No devices found.");
		std::process::exit(1);
	} else {
		for device in &devices {
			println!("{:#?}", device);
		}
	}
}
