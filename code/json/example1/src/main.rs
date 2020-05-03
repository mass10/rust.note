#[macro_use]
extern crate serde_json;

use serde_json::Value;

fn test0() {
	let json_text = json!({
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
	// let json_text = serde_json::from_value::<String>(json_text).unwrap();
	println!("[TARCE] {}", json_text);
}

#[allow(dead_code)]
fn parse_json_to_value(json_text: &str) -> std::option::Option<Value> {
	let result = serde_json::from_str::<Value>(json_text);
	if result.is_err() {
		return None;
	}
	return Some(result.ok().unwrap());
}

fn test1() {
	let json_text = r#"{
		"name": "John Doe",
		"age": 43,
		"phones": [
		"+44 1234567",
		"+44 2345678"
		]
	}"#;
	let result = serde_json::from_str::<Value>(json_text);
	if result.is_err() {
		println!("[ERROR] {}", result.err().unwrap());
		return;
	}
	let v = result.ok().unwrap();
	println!("[TRACE] {:?}", v);
	println!("[TRACE] Please call {} at the number {}", v["name"], v["phones"][0]);
}

fn main() {
	test0();
	test1();

	println!("Ok.");
}
