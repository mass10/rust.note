extern crate clap;

#[derive(Debug)]
pub struct Configuration {
	pub dry_run: bool,
	pub verbose: bool,
}

impl Configuration {
	/// コンフィギュレーション
	pub fn configure() -> Configuration {
		// creating an application
		let mut application = clap::App::new("xcopy").version("0.1");
		// adding a option
		{
			let arg_dry_run = clap::Arg::with_name("dry-run option")
				.long("dry-run")
				.help("dry run")
				.takes_value(false);
			application = application.arg(arg_dry_run);
		}
		// adding a option
		{
			let arg_verbose = clap::Arg::with_name("verbose option")
				.long("verbose")
				.help("verbose")
				.takes_value(false);
			application = application.arg(arg_verbose);
		}
		// retrieving
		let matches = application.get_matches();
		// configuration setting
		let conf = Configuration {
			dry_run: matches.is_present("dry-run option"),
			verbose: matches.is_present("verbose option"),
		};
		return conf;
	}
}
