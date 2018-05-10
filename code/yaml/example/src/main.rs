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

	// println!("{:?}", doc["key1"].as_str().unwrap());
	// println!("{:?}", doc["key2"].as_str().unwrap());
	println!("{:?}", doc["key2"].as_str());
	println!("{:?}", doc["key3"].as_f64());
	println!("{:?}", doc["key4"].is_badvalue());
}
