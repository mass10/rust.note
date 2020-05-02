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
	println!("[TARCE] {}", json_text);
}

fn test1() {
	let data = r#"{
		"name": "John Doe",
		"age": 43,
		"phones": [
		"+44 1234567",
		"+44 2345678"
		]
	}"#;
	let v: Value = serde_json::from_str(data).unwrap();
	println!("[TRACE] {:?}", v);
	println!("[TRACE] Please call {} at the number {}", v["name"], v["phones"][0]);
}

fn main() {
	test0();
	test1();

	println!("Ok.");
}
