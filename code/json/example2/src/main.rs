#[macro_use]

extern crate json;

fn function1() {

	let json_text = r#"
	{
		"code": 200,
		"success": true,
		"payload": {
			"features": [
				"awesome",
				"easyAPI",
				"lowLearningCurve"
			]
		}
	}
	"#;

	let parsed_object = json::parse(json_text).unwrap();

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
}

fn main() {

	function1();
	// println!("Ok.");
}