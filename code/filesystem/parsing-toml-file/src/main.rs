extern crate serde_derive;

///
/// その他のテーブル属性
///
#[derive(serde_derive::Deserialize, std::fmt::Debug)]
struct Attribute {
	pub attribute01: Option<String>,
	pub attribute02: Option<String>,
}

///
/// settings
///
#[derive(serde_derive::Deserialize, std::fmt::Debug)]
struct Settings {
	/// 文字列属性
	string_value: Option<String>,
	/// 数値属性
	integral_value: Option<u32>,
	/// 真偽属性
	boolean_attribute: Option<bool>,
	/// その他のテーブル属性
	attributes: Option<Attribute>,
}

///
/// メンバー情報
///
#[derive(serde_derive::Deserialize, std::fmt::Debug)]
struct Member {
	/// 名前
	pub name: String,
	/// メール
	pub email: Option<String>,
}

///
/// グループ情報
///
#[derive(serde_derive::Deserialize, std::fmt::Debug)]
struct Groups {
	/// グループ名
	pub name: Option<String>,
	/// メンバー
	pub members: Option<Vec<Member>>,
}

///
/// コンフィギュレーション
///
#[derive(serde_derive::Deserialize, std::fmt::Debug)]
struct Configuration {
	/// settings
	settings: Settings,
	/// groups
	groups: Vec<Groups>,
}

/// テキストファイル全体を String で返します。
fn read_text_file_all(path: &str) -> std::result::Result<String, Box<dyn std::error::Error>> {
	use std::io::Read;

	let mut file = std::fs::File::open(path)?;
	let mut content = String::new();
	file.read_to_string(&mut content)?;

	return Ok(content);
}

/// TOML ファイルを解析します。
fn read_toml_file(path: &str) -> std::result::Result<Configuration, Box<dyn std::error::Error>> {
	extern crate toml;

	// ファイル全体を文字列として読み込みます。
	let content = read_text_file_all(path)?;

	// toml 文字列を解析します。
	let conf: Configuration = toml::from_str(&content)?;

	return Ok(conf);
}

/// ファイルパスの補完
fn fix_conf_path(path: &str) -> String {
	if path != "" {
		return path.to_string();
	}
	return "settings.toml".to_string();
}

/// コンフィギュレーションを行います。
///
/// 決められた struct を用いて、コンフィギュレーションを行う方法です。
fn configure1(path: &str) -> std::result::Result<(), Box<dyn std::error::Error>> {
	// TOML ファイル読み込み
	let path = fix_conf_path(path);
	let conf = read_toml_file(&path)?;

	// [settings] の内容を表示します。
	{
		println!("--- HEADER ---");
		let string_value = conf.settings.string_value.unwrap_or(String::new());
		let threashold = conf.settings.integral_value.unwrap_or(0);
		let attributes = conf.settings.attributes;
		let boolean_attribute = conf.settings.boolean_attribute;

		println!("[TRACE] boolean_attribute: [{:?}]", boolean_attribute);
		println!("[TRACE] integral_value: [{}]", threashold);
		println!("[TRACE] string_value: [{}]", string_value);
		println!("[TRACE] attributes: [{:?}]", attributes);
		println!();
	}

	// [[groups]] の内容を表示します。
	{
		for group in conf.groups {
			println!("[TRACE] {}", group.name.unwrap());
			if group.members.is_some() {
				for member in &group.members.unwrap() {
					println!("[TRACE] {:?}", member);
				}
			}
			println!();
		}
	}

	return Ok(());
}

/// コンフィギュレーションを行います。
///
/// 決められた struct を用いず、コレクションを引っ張りまわす方法です。
fn configure2(path: &str) -> std::result::Result<(), Box<dyn std::error::Error>> {
	// TOML ファイル読み込み
	let path = fix_conf_path(path);
	let content = read_text_file_all(&path)?;

	// toml 文字列を解析します。
	let conf = toml::from_str::<toml::Value>(&content)?;

	let settings = conf.get("settings");
	println!("--- settings ---");
	println!("{:?}", &settings);

	println!("--- table ---");
	let table = conf.as_table();
	if table.is_none() {
		return Ok(());
	}
	let table = table.unwrap();
	for (key, value) in table {
		println!("{:?}: {:?}", key, value);
	}

	return Ok(());
}

/// エントリーポイントの定義です。
fn main() {
	// コマンドライン引数からパスを受け取る
	let args: std::vec::Vec<String> = std::env::args().skip(1).collect();

	let path_to_toml = if 0 < args.len() { &args[0] } else { "" };

	if path_to_toml == "" {
		return;
	}

	// コンフィギュレーション(1)
	let result = configure1(path_to_toml);
	if result.is_err() {
		let error = result.err().unwrap();
		println!("[ERROR] Configuration error! reason: [{}]", error);
	}

	// コンフィギュレーション(2)
	let result = configure2(path_to_toml);
	if result.is_err() {
		let error = result.err().unwrap();
		println!("[ERROR] Configuration error! reason: [{}]", error);
	}
}
