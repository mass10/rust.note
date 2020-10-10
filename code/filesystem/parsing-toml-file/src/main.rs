extern crate serde_derive;

#[derive(serde_derive::Deserialize, Debug)]
struct Attribute {
	#[allow(unused)]
	attribute01: Option<String>,
	#[allow(unused)]
	attribute02: Option<String>,
}

#[derive(serde_derive::Deserialize)]
struct Configuration {
	#[allow(unused)]
	email: String,
	#[allow(unused)]
	threshold: Option<u32>,
	#[allow(unused)]
	attributes: Option<Attribute>,
}

fn read_text_file_all(path: &str) -> std::result::Result<String, Box<dyn std::error::Error>> {
	use std::io::Read;

	let mut file = std::fs::File::open(path).unwrap();
	let mut s = String::new();
	file.read_to_string(&mut s)?;
	return Ok(s);
}

fn read_toml_file(path: &str) -> std::result::Result<Configuration, Box<dyn std::error::Error>> {
	extern crate toml;

	// ファイル全体を文字列として読み込みます。
	let content = read_text_file_all(path)?;

	// toml 文字列を解析します。
	let conf: Configuration = toml::from_str(&content)?;

	return Ok(conf);
}

fn configure() -> std::result::Result<(), Box<dyn std::error::Error>> {
	// TOML ファイル読み込み
	let conf = read_toml_file("settings.toml")?;

	// ダンプ
	println!("[TRACE] DUMP");
	println!("[TRACE] email: {:?}", conf.email);
	println!("[TRACE] threshold: {:?}", conf.threshold);
	println!("[TRACE] attributes: {:?}", conf.attributes);
	println!("[TRACE] Ok.");

	return Ok(());
}

fn main() {
	let result = configure();
	if result.is_err() {
		let error = result.err().unwrap();
		println!("[ERROR] reason: {:?}", error);
	}
}
