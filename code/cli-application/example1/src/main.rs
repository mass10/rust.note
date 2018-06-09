extern crate clap;

fn main() {

	// creating an application
	let application = clap::App::new("example1");
	let application = application.version("1.0");
	let application = application.author("mass10 <mass10.github@gmail.com>");
	let application = application.about("このアプリケーションは clap のサンプルです。");

	// adding a option
	let argument = clap::Arg::with_name("path to configuration file")
		.short("c")
		.long("config")
		.value_name("PATH TO CONFIGURATION FILE")
		.help("Sets a custom config file")
		.takes_value(true);
	let application = application.arg(argument);

	// adding a option
	let argument = clap::Arg::with_name("verbose option")
		.short("v")
		.long("verbose")
		.help("verbose")
		.takes_value(false);
	let application = application.arg(argument);

	// retrieving
	let matches = application.get_matches();
	let configuration_path = matches.value_of("path to configuration file").unwrap_or(".settings.yaml");
	let verbose_option = matches.is_present("verbose option");
	println!("config: {}", configuration_path);
	println!("config: {}", verbose_option);
}
