fn main() {
	let devices = unsafe { aravis::get_device_list() };
	if devices.is_empty() {
		println!("No devices found.");
	} else {
		for device in &devices {
			println!("{:#?}", device);
		}
	}
}
