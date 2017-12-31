use std::fs::File;
use std::io::Read;

extern crate yaml_rust;
use yaml_rust::yaml;

// #[allow(unused)]
fn main() {

	let mut file = File::open("conf/settings.yml").unwrap();
	let mut content = String::new();
	file.read_to_string(&mut content).unwrap();
	let docs = yaml::YamlLoader::load_from_str(content.as_str()).unwrap();
	let doc = &docs[0];

	// println!("{:?}", doc["key1"].as_str().unwrap());
	// println!("{:?}", doc["key2"].as_str().unwrap());
	println!("{:?}", doc["key2"]);
	println!("{:?}", doc["key3"]);
	println!("{:?}", doc["key4"]);
}
