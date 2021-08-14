extern crate yaml_rust;

use std::fs::File;
use std::io::Read;
use yaml_rust::yaml::YamlLoader;

fn configure() -> Vec<yaml_rust::Yaml> {

	let mut file = File::open("conf/settings.yml").unwrap();
	let mut content = String::new();
	file.read_to_string(&mut content).unwrap();
	let docs = YamlLoader::load_from_str(content.as_str()).unwrap();
	return docs;
}

fn main() {

	let docs = configure();
	let doc = &docs[0];

	// Hash になる要素
	if let Some(attributes) = doc["tree_attributes"].as_hash() {
		println!("[TRACE] tree_attributes is {:?}", attributes);
		println!("[TRACE] tree_attributes: {:?}", attributes[&yaml_rust::Yaml::String("email".to_string())]);
		println!("[TRACE] tree_attributes: {:?}", attributes[&yaml_rust::Yaml::String("address".to_string())]);
		println!("[TRACE] tree_attributes: {:?}", doc["tree_attributes"]["email"]);
		println!("[TRACE] tree_attributes: {:?}", doc["tree_attributes"]["address"]);
	}

	// 単純な文字列の要素
	println!("[TRACE] key2: {:?}", doc["key2"].as_str());

	// 単純な数値の要素
	println!("[TRACE] key3: {:?}", doc["key3"].as_f64());

	// Vec として取れる要素
	println!("[TRACE] array_od_entities");
	for e in doc["array_od_entities"].as_vec().unwrap() {
		let entry = e.as_hash().unwrap();
		println!("[TRACE] {:?}", &entry);
	}

	println!("[TRACE] key5: {:?}", doc["key5"].is_badvalue());
}
