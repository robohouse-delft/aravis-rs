use aravis::Aravis;

fn main() {
	let aravis = Aravis::initialize().unwrap();

	let devices = aravis.get_device_list();
	if devices.is_empty() {
		println!("No devices found.");
	} else {
		for device in &devices {
			println!("{:#?}", device);
		}
	}
}
