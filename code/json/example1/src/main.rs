extern crate serde;
extern crate serde_derive;
extern crate serde_json;

///
/// テスト
///
mod test00 {

	/// 固定オブジェクトから JSON 文字列を生成します。
	pub fn run() {
		let json_text = serde_json::json!({
			"code": 200,
			"success": true,
			"payload": {
				"features": [
					"serde",
					"json"
				]
			}
		});

		let json_text = format!("{}", json_text);

		println!("[TARCE] {}", json_text);
	}
}

///
/// テスト
///
mod test01 {

	#[allow(dead_code)]
	fn parse_json_to_value(json_text: &str) -> std::result::Result<serde_json::Value, std::boxed::Box<dyn std::error::Error>> {
		let result = serde_json::from_str::<serde_json::Value>(json_text)?;
		return Ok(result);
	}

	/// JSON 文字列からオブジェクトを生成します。
	pub fn run() -> std::result::Result<(), Box<dyn std::error::Error>> {
		let json_text = r#"{
			"name": "Jimi Hendrix",
			"age": 28,
			"phones": [
				"+81 09000000000",
				"+81 09000000001"
			]
		}"#;
		let value = parse_json_to_value(json_text)?;
		println!("[TRACE] {:?}", value);
		println!("[TRACE] Please call {} at the number {}", value["name"], value["phones"][0]);
		return Ok(());
	}
}

///
/// テスト
///
mod test02 {

	/// ユーザー情報構造体
	#[derive(Debug, serde_derive::Serialize, serde_derive::Deserialize)]
	struct User {
		email: String,
		name: String,
		birth: String,
	}

	/// JSON の文字列をパースしてオブジェクトを生成します。
	///
	/// ### Arguments
	/// * `s` JSON 文字列
	///
	/// ### Returns
	/// * ユーザー情報構造体
	#[allow(unused)]
	fn parse_user_json(s: &str) -> std::result::Result<User, std::boxed::Box<dyn std::error::Error>> {
		let mut user = User {
			email: "".to_string(),
			name: "".to_string(),
			birth: "".to_string(),
		};
		let result = serde_json::from_str::<User>(s);

		return Ok(user);
	}

	// テストを実行します。
	pub fn run() -> std::result::Result<(), std::boxed::Box<dyn std::error::Error>> {
		return Ok(());
	}
}

/// テストを実行します。
fn run_tests() -> std::result::Result<(), std::boxed::Box<dyn std::error::Error>> {
	if true {
		test00::run();
	}
	if true {
		test01::run()?;
	}
	if false {
		test02::run()?;
	}

	return Ok(());
}

/// エントリーポイント
fn main() {
	// テストを実行します。
	let result = run_tests();
	if result.is_err() {
		let error = result.err().unwrap();
		println!("[ERROR] {}", error);
		return;
	}

	println!("Ok.");
}
