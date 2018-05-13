mod application;

extern crate yaml_rust;
extern crate mysql;

// use std::fs::File;
// use std::io::Read;
// use yaml_rust::yaml::YamlLoader;

fn main() {

	let mut app = application::Application::new();
	app.run();
}
