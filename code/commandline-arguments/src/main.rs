///
/// コンフィギュレーション struct
///
#[derive(Debug)]
struct MyConfigurationSettings {
	pub dry_run: bool,
	pub verbose: bool,
	pub file: String,
}

impl MyConfigurationSettings {
	pub fn new() -> MyConfigurationSettings {
		return MyConfigurationSettings {
			dry_run: false,
			verbose: false,
			file: "".to_string(),
		};
	}
}

/// getopts を使用したコマンドライン解析
fn configure_getopts() -> Option<MyConfigurationSettings> {
	// 提供するコマンドラインオプションを定義
	let mut getopt = getopts::Options::new();
	getopt.optopt("f", "file", "", "");
	getopt.optflag("h", "help", "");
	getopt.optflag("v", "verbose", "");
	getopt.optflag("", "dry-run", "");

	// 解析
	let result = getopt.parse(std::env::args().skip(1));
	if result.is_err() {
		println!("[TRACE] {}", result.err().unwrap());
		println!("{}", getopt.usage(""));
		return None;
	}
	let result = result.unwrap();

	// 読み取り
	if result.opt_present("help") {
		// Options:
		//     -f, --file
		//     -h, --help
		//     -v, --verbose
		//         --dry-run
		println!("{}", getopt.usage(""));
		return None;
	}

	let mut conf = MyConfigurationSettings::new();
	// file
	if result.opt_present("file") {
		conf.file = result.opt_str("file")?;
	}
	// dry run
	conf.dry_run = result.opt_present("dry-run");
	// verbose
	conf.verbose = result.opt_present("verbose");
	return Some(conf);
}

/// clap を使用したコマンドライン解析
fn configure() -> MyConfigurationSettings {
	extern crate clap;

	let pkg_name = env!("CARGO_PKG_NAME");
	// 初期化
	let mut application = clap::App::new(pkg_name).version("0.1");

	// option
	{
		let arg_dry_run = clap::Arg::with_name("dry-run option")
			.long("dry-run")
			.help("dry run")
			.takes_value(false);
		application = application.arg(arg_dry_run);
	}

	// option
	{
		let arg_verbose = clap::Arg::with_name("verbose option")
			.long("verbose")
			.help("verbose")
			.takes_value(false);
		application = application.arg(arg_verbose);
	}

	// 読み取り実行
	let matches = application.get_matches();

	// 独自 struct にして返却
	let mut conf = MyConfigurationSettings::new();
	conf.dry_run = matches.is_present("dry-run option");
	conf.verbose = matches.is_present("verbose option");
	return conf;
}

/// エントリーポイント
fn main() {
	// 一つずつ読む
	if false {
		let args: Vec<String> = std::env::args().collect();
		if args.len() < 2 {
			println!("what?");
			return;
		}
		for e in &args[1..] {
			println!("OPTION: [{}]", e);
		}
	}

	// コマンドラインオプション読み取り getopts を利用する
	if true {
		let conf = configure_getopts();
		println!("[TRACE] option: {:?}", conf);
	}

	// コマンドラインオプション読み取り crate "clap" を利用する
	if false {
		let conf = configure();
		println!("[TRACE] option: {:?}", conf);
	}
}
