#[allow(dead_code)]
fn parse_json_to_value(json_text: &str) -> std::result::Result<serde_json::Value, std::boxed::Box<dyn std::error::Error>> {
	let result = serde_json::from_str::<serde_json::Value>(json_text)?;
	return Ok(result);
}

/// 固定オブジェクトから JSON 文字列を生成します。
pub fn test0() {
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

pub fn test1() -> std::result::Result<(), Box<dyn std::error::Error>> {
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
