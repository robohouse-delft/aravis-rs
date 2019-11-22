pub fn init_logging() {
	env_logger::Builder::new()
		.filter_level(log::LevelFilter::Info)
		.format(|buffer, record: &log::Record| {
			use std::io::Write;
			use env_logger::fmt::Color;

			let mut prefix_style = buffer.style();
			let prefix;

			match record.level() {
				log::Level::Trace => {
					prefix = "Trace: ";
					prefix_style.set_bold(true);
				}
				log::Level::Debug => {
					prefix = "";
				}
				log::Level::Info => {
					prefix = "";
				}
				log::Level::Warn => {
					prefix = "Warning: ";
					prefix_style.set_color(Color::Yellow).set_bold(true);
				}
				log::Level::Error => {
					prefix = "Error: ";
					prefix_style.set_color(Color::Red).set_bold(true);
				}
			};

			writeln!(
				buffer,
				"{}{}",
				prefix_style.value(prefix),
				record.args()
			)
		}).init();
}
