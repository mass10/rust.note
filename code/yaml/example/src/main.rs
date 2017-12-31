
use std::fs::File;
use std::io::Read;

extern crate yaml_rust;
// use yaml_rust::{YamlLoader, YamlEmitter};
use yaml_rust::{YamlLoader};

#[allow(unused)]
fn main() {

	let mut file = File::open("conf/settings.yml").unwrap();
	let mut content = String::new();
	file.read_to_string(&mut content).unwrap();
	println!("{}", content);

	let docs = YamlLoader::load_from_str(content.as_str()).unwrap();
	let doc = &docs[0];
	println!("{:?}", doc);
	println!("{:?}", doc["key2"].as_str().unwrap());
	println!("{:?}", doc["unknown-key"].as_str());

	println!("Ok.");
}

