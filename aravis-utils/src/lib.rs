pub fn init_logging(modules: &[&str]) {
	let mut logger = env_logger::Builder::new();

	logger.format(|buffer, record: &log::Record| {
		use env_logger::fmt::Color;
		use std::io::Write;

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

		writeln!(buffer, "{}{}", prefix_style.value(prefix), record.args())
	});
	for module in modules {
		logger.filter_module(module, log::LevelFilter::Info);
	}
	logger.init();
}
