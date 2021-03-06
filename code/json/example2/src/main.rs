#[macro_use]

extern crate json;

use std::io::Read;

fn invalid_json() -> Result<(), json::Error> {

	println!("[INFO] $$$ 不正な JSON のパース $$$");

	// ファイルを読み込み
	let result = read_text_file("settings-invalid.json");
	if result.is_err() {
		println!("{:?}", result.err().unwrap());
		return Ok(());
	}
	let json_text = result.unwrap();

	// パース。ここで Err() が返るので、次の行へは進みません。
	json::parse(json_text.as_str())?;

	// unreachable
	return Ok(());
}

fn read_text_file(path: &str) -> Result<String, std::io::Error> {

	// 何かあればただちに std::io::Error が返ります。
	let mut file = std::fs::File::open(path)?;
	let mut s = String::new();
	// 何かあればただちに std::io::Error が返ります。
	file.read_to_string(&mut s)?;
	// Success(content: "JSON 文字列") のようなものが返ります。
	return Ok(s);
}

fn valid_json() -> Result<(), json::Error> {

	println!("[INFO] $$$ 正しい JSON のパース $$$");

	// ファイルを読み込み
	let result = read_text_file("settings-valid.json");
	if result.is_err() {
		println!("{:?}", result.err().unwrap());
		return Ok(());
	}
	let json_text = result.unwrap();

	// パース。
	let parsed_object = json::parse(json_text.as_str())?;

	// 検証
	let instantiated = object!{
		"code" => 200,
		"success" => true,
		"payload" => object!{
			"features" => array![
				"awesome",
				"easyAPI",
				"lowLearningCurve"
			]
		}
	};
	assert_eq!(parsed_object, instantiated);

	/* 特に何も返す必要がないので中身は空でよい。success(reason: null) みたいなイメージ。 */
	return Ok(());
}

fn main() {

	// ========== 不正な JSONファイルをパースするケース ==========
	{
		let result = invalid_json();
		if result.is_err() {
			println!("[ERROR] {:?}", result.err().unwrap());
		}
		else {
			println!("[INFO] {:?}", result.unwrap());
		}
	}

	// ========== 正しい JSONファイルをパースするケース ==========
	{
		let result = valid_json();
		if result.is_err() {
			println!("[ERROR] {:?}", result.err().unwrap());
		}
		else {
			println!("[INFO] {:?}", result.unwrap());
		}
	}
}
