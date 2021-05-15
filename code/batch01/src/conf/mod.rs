extern crate clap;

/// コンフィギュレーション
#[derive(Debug, Clone)]
pub struct Configuration {
	/// 予約
	reserved: i32,
	/// dry-run の指定
	pub dry_run: bool,
	/// 冗長モード
	pub verbose: bool,
}

impl Configuration {
	/// コンフィギュレーション
	pub fn commandline_arguments() -> Configuration {
		// creating an application
		let mut application = clap::App::new("xcopy").version("0.1");
		// adding a option
		{
			let arg_dry_run = clap::Arg::with_name("dry-run option").long("dry-run").help("dry run").takes_value(false);
			application = application.arg(arg_dry_run);
		}
		// adding a option
		{
			let arg_verbose = clap::Arg::with_name("verbose option").long("verbose").help("verbose").takes_value(false);
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
