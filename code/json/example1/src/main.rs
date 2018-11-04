#[macro_use]
extern crate serde_json;

// use serde_json::{Value, Error};

#[derive(Debug)]
enum Value {
	Null,
	Bool(bool),
	Number(serde_json::Number),
	String(String),
	Array(Vec<serde_json::Value>),
	// Object(serde_json::Map<String, Value>),
}

fn test0() {

	let _value = json!({
		"code": 200,
		"success": true,
		"payload": {
			"features": [
				"serde",
				"json"
			]
		}
	});
}

fn test1() -> Result<(), serde_json::Error> {
	let data = r#"{
		"name": "John Doe",
		"age": 43,
		"phones": [
		"+44 1234567",
		"+44 2345678"
		]
	}"#;
	let v: Value = serde_json::from_str(data).unwrap();
	// println!("{:?}", v);
	// println!("Please call {} at the number {}", v["name"], v["phones"][0]);
	Ok(())
}

fn main() {
	
	match test1() {
		Ok(result) => println!("[INFO] {:?}", result),
		Err(result) => println!("[ERROR] {:?}", result),
	}
    
    println!("Ok.");
}
