extern crate clap;

/// コンフィギュレーション struct
#[derive(Debug)]
struct MyConfigurationSettings {
	pub dry_run: bool,
	pub verbose: bool,
}

fn configure() -> MyConfigurationSettings {
	// 初期化
	let mut application = clap::App::new("xcopy").version("0.1");

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
	return MyConfigurationSettings {
		dry_run: matches.is_present("dry-run option"),
		verbose: matches.is_present("verbose option"),
	};
}

/// エントリーポイント
fn main() {
	// 一つずつ読む
	{
		let args: Vec<String> = std::env::args().collect();
		if args.len() < 2 {
			println!("what?");
			return;
		}

		for e in &args[1..] {
			println!("OPTION: [{}]", e);
		}
	}

	// コマンドラインオプション読み取り crate "clap" を利用する
	{
		let _conf = configure();
		println!("[TRACE] option: {:?}", _conf);
	}
}
