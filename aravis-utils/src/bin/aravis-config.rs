use aravis::prelude::*;
use glib::{Cast, IsA};

use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(setting = structopt::clap::AppSettings::ColoredHelp)]
#[structopt(setting = structopt::clap::AppSettings::UnifiedHelpMessage)]
#[structopt(setting = structopt::clap::AppSettings::DeriveDisplayOrder)]
#[structopt(group = structopt::clap::ArgGroup::with_name("selector").required(true))]
struct Options {
	/// The IP address of the camera to connecto to.
	id: String,

	/// Show all options.
	#[structopt(long, short)]
	#[structopt(group = "selector")]
	all: bool,

	/// The option to get or set.
	#[structopt(long, short)]
	#[structopt(value_name = "NAME")]
	#[structopt(group = "selector")]
	feature: Option<String>,

	/// Set the value of the selected option.
	#[structopt(long, short)]
	#[structopt(value_name = "VALUE")]
	#[structopt(requires = "feature")]
	set: Option<String>,
}

fn main() {
	aravis_utils::init_logging();
	if let Err(e) = do_main() {
		log::error!("{}", e);
		std::process::exit(1);
	}
}

fn do_main() -> Result<(), String> {
	let options = Options::from_args();

	log::info!("Connecting to camera {}.", options.id);
	let camera = aravis::Camera::new(Some(&options.id))
		.ok_or("Failed to connecto to camera.")?;

	let genicam = camera.get_device().unwrap().get_genicam().unwrap();

	if options.all {
		walk_genicam(&genicam, "Root", "").map_err(|e| format!("{}", e))
	} else if let Some(feature) = options.feature {
		if let Some(set_value) = &options.set {
			set_feature(&genicam, &feature, set_value)
		} else {
			walk_genicam(&genicam, &feature, "").map_err(|e| format!("{}", e))
		}
	} else {
		unreachable!();
	}
}

fn walk_genicam<T: IsA<aravis::Gc>>(genicam: &T, feature: &str, indent: &str) -> Result<(), glib::Error> {
	let node = genicam.get_node(feature).unwrap();

	if let Some(node) = node.dynamic_cast_ref::<aravis::GcBoolean>() {
		println!("{}{}: boolean {}", indent, feature, node.get_value()?);
	} else if let Some(node) = node.dynamic_cast_ref::<aravis::GcCategory>() {
		if feature == "Root" {
			for feature in node.get_features() {
				walk_genicam(genicam, &feature, indent)?;
			}
		} else {
			println!("{}{}: category", indent, feature);
			for feature in node.get_features() {
				walk_genicam(genicam, &feature, &format!("  {}", indent))?;
			}
		}
	} else if let Some(_) = node.dynamic_cast_ref::<aravis::GcCommand>() {
		println!("{}{}: command", indent, feature);
	} else if let Some(node) = node.dynamic_cast_ref::<aravis::GcFloatRegNode>() {
		println!("{}{}: float {}", indent, feature, node.get_value()?);
	} else if let Some(node) = node.dynamic_cast_ref::<aravis::GcFloatNode>() {
		println!("{}{}: float {}", indent, feature, node.get_value()?);
	} else if let Some(node) = node.dynamic_cast_ref::<aravis::GcIntRegNode>() {
		println!("{}{}: integer {}", indent, feature, node.get_value()?);
	} else if let Some(node) = node.dynamic_cast_ref::<aravis::GcStringRegNode>() {
		println!("{}{}: string {}", indent, feature, node.get_value()?);
	} else if let Some(node) = node.dynamic_cast_ref::<aravis::GcIntegerNode>() {
		println!("{}{}: integer {}", indent, feature, node.get_value()?);
	} else if let Some(node) = node.dynamic_cast_ref::<aravis::GcEnumeration>() {
		println!("{}{}: enumeration {}", indent, feature, node.get_string_value()?);
	} else if let Some(node) = node.dynamic_cast_ref::<aravis::GcRegisterNode>() {
		println!("{}{}: register (0x{:02X}, {})", indent, feature, node.get_address()?, node.get_length()?);
	} else if let Some(_) = node.dynamic_cast_ref::<aravis::GcSwissKnife>() {
		println!("{}{}: swiss-knife", indent, feature);
	} else {
		println!("{:?}", node);
		unimplemented!();
	}

	Ok(())
}

fn set_feature<T: IsA<aravis::Gc>>(genicam: &T, feature: &str, value: &str) -> Result<(), String> {
	let node = genicam.get_node(feature).unwrap();

	if let Some(node) = node.dynamic_cast_ref::<aravis::GcBoolean>() {
		let value = value.parse().map_err(|_| "Invalid boolean value.")?;
		node.set_value(value).map_err(|e| format!("{}", e))?;
	} else if let Some(_) = node.dynamic_cast_ref::<aravis::GcCategory>() {
		return Err(format!("Can not set feature {}. It is a category.", feature));
	} else if let Some(node) = node.dynamic_cast_ref::<aravis::GcCommand>() {
		if value.eq_ignore_ascii_case("execute") {
			node.execute().map_err(|e| format!("{}", e))?;
		} else {
			return Err(format!("Invalid value for feature {}. Use `--set execute` to execute a command.", feature));
		}
	} else if let Some(node) = node.dynamic_cast_ref::<aravis::GcFloatRegNode>() {
		let value = value.parse().map_err(|_| "Invalid float value.")?;
		node.set_value(value).map_err(|e| format!("{}", e))?;
	} else if let Some(node) = node.dynamic_cast_ref::<aravis::GcFloatNode>() {
		let value = value.parse().map_err(|_| "Invalid float value.")?;
		node.set_value(value).map_err(|e| format!("{}", e))?;
	} else if let Some(node) = node.dynamic_cast_ref::<aravis::GcIntRegNode>() {
		let value = value.parse().map_err(|_| "Invalid integer value.")?;
		node.set_value(value).map_err(|e| format!("{}", e))?;
	} else if let Some(node) = node.dynamic_cast_ref::<aravis::GcStringRegNode>() {
		node.set_value(value).map_err(|e| format!("{}", e))?;
	} else if let Some(node) = node.dynamic_cast_ref::<aravis::GcIntegerNode>() {
		let value = value.parse().map_err(|_| "Invalid integer value.")?;
		node.set_value(value).map_err(|e| format!("{}", e))?;
	} else if let Some(_) = node.dynamic_cast_ref::<aravis::GcEnumeration>() {
		return Err(format!("Can not set feature {}. Setting enumeration nodes is unimplemented.", feature));
	} else if let Some(_) = node.dynamic_cast_ref::<aravis::GcRegisterNode>() {
		return Err(format!("Can not set feature {}. Setting register nodes is unimplemented.", feature));
	} else if let Some(_) = node.dynamic_cast_ref::<aravis::GcSwissKnife>() {
		return Err(format!("Can not set feature {}. Setting swiss knife nodes is unimplemented.", feature));
	} else {
		return Err(format!("Unimplemented node type: {:?}", node));
	}

	Ok(())
}
