// use std::net::TcpListener;

extern crate serde;
extern crate serde_derive;
extern crate serde_json;

mod application;
mod application_defined_errors;
mod client;
mod listener;
mod util;

fn usage() {}

fn configure() -> String {
	for e in std::env::args().skip(1) {
		if e == "--help" {
			usage();
			return "".to_string();
		}
		if e == "--server" {
			return "--server".to_string();
		}
	}
	return "".to_string();
}

/// アプリケーションのエントリーポイント
fn main() {
	let request = configure();

	// リッスン開始
	let result = application::Application::new().start(&request);
	if result.is_err() {
		println!("[ERROR] {}", result.err().unwrap());
		return;
	}
}
