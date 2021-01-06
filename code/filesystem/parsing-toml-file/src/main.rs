extern crate serde_derive;

///
/// その他のテーブル属性
///
#[derive(serde_derive::Deserialize, std::fmt::Debug)]
struct Attribute {
	attribute01: Option<String>,
	attribute02: Option<String>,
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
	name: String,
	/// メール
	email: Option<String>,
}

///
/// グループ情報
///
#[derive(serde_derive::Deserialize, std::fmt::Debug)]
struct Groups {
	/// グループ名
	name: Option<String>,
	/// メンバー
	members: Option<Vec<Member>>,
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

///
/// テキストファイル全体を String で返します。
///
fn read_text_file_all(path: &str) -> std::result::Result<String, Box<dyn std::error::Error>> {
	use std::io::Read;

	let mut file = std::fs::File::open(path)?;
	let mut content = String::new();
	file.read_to_string(&mut content)?;

	return Ok(content);
}

///
/// TOML ファイルを解析します。
///
fn read_toml_file(path: &str) -> std::result::Result<Configuration, Box<dyn std::error::Error>> {
	extern crate toml;

	// ファイル全体を文字列として読み込みます。
	let content = read_text_file_all(path)?;

	// toml 文字列を解析します。
	let conf: Configuration = toml::from_str(&content)?;

	return Ok(conf);
}

///
/// コンフィギュレーションを行います。
///
fn configure(path: &str) -> std::result::Result<(), Box<dyn std::error::Error>> {
	// TOML ファイル読み込み
	let path = if path == "" { "settings.toml" } else { path };
	let conf = read_toml_file(path)?;

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

///
/// エントリーポイントの定義です。
///
fn main() {
	// コンフィギュレーション
	let result = configure("");
	if result.is_err() {
		let error = result.err().unwrap();
		println!("[ERROR] Configuration error! reason: [{}]", error);
	}
}
