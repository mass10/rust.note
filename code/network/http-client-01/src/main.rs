extern crate chrono;
extern crate ctrlc;
extern crate reqwest;

use std::io::Read;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

fn get_current_timestamp() -> String {
	let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S%.3f");
	return format!("{}", timestamp);
}

fn inner_main() {
	// let mut _headers = reqwest::header::Headers::new();
	// _headers.set(
	// 	reqwest::header::AcceptLanguage(vec![])
	// );

	println!("{} [TRACE] attack.", get_current_timestamp());
	let client = reqwest::Client::new();
	// let response = client.get("http://127.0.0.1:3000").header(reqwest::header::Connection::close()).send();
	let response = client.get("http://127.0.0.1:3000").send();
	if response.is_err() {
		let error = response.err().unwrap();
		println!("{} [ERROR] {:?}", get_current_timestamp(), error);
		std::thread::sleep(std::time::Duration::from_millis(100));
		return;
	}
	let mut response = response.unwrap();
	response.copy_to(&mut std::io::stdout()).unwrap();
	std::thread::sleep(std::time::Duration::from_millis(10));
}

fn run_test01() {
	let running = Arc::new(AtomicBool::new(true));
	let r = running.clone();
	ctrlc::set_handler(move || {
		r.store(false, Ordering::SeqCst);
	})
	.expect("Error setting Ctrl-C handler");

	while running.load(Ordering::SeqCst) {
		inner_main();
	}

	println!("Ok.");
}

fn get2() {
	println!("{} [TRACE] attack.", get_current_timestamp());
	let client = reqwest::Client::new();
	let response = client
		.get("https://www.google.co.jp/search?q=tokyo+olympic+2020")
		.header("Accept-Language", "ja")
		.header("Accept-Charset", "UTF-8")
		.send();
	if response.is_err() {
		let error = response.err().unwrap();
		println!("{} [ERROR] {:?}", get_current_timestamp(), error);
		std::thread::sleep(std::time::Duration::from_millis(100));
		return;
	}
	let mut response = response.unwrap();

	// String として全て読み込もうとするとダメ "stream did not contain valid UTF-8"
	let mut body = String::new();
	let status = response.read_to_string(&mut body);
	if status.is_err() {
		println!("[ERROR] {}", status.err().unwrap());
		return;
	}
}

fn get3() {
	println!("{} [TRACE] attack.", get_current_timestamp());
	let client = reqwest::Client::new();
	let response = client
		.get("https://www.google.co.jp/search?q=tokyo+olympic+2020")
		.header("Accept-Language", "ja")
		.header("Accept-Charset", "UTF-8")
		.send();
	if response.is_err() {
		let error = response.err().unwrap();
		println!("{} [ERROR] {:?}", get_current_timestamp(), error);
		std::thread::sleep(std::time::Duration::from_millis(100));
		return;
	}
	let mut response = response.unwrap();
	let mut buffer = std::vec::Vec::new();
	let status = response.read_to_end(&mut buffer);
	if status.is_err() {
		println!("[ERROR] {}", status.err().unwrap());
		return;
	}
	let response = String::from_utf8(buffer);
	if response.is_err() {
		println!("[ERROR] 読み込み失敗！ 理由: {}", response.err().unwrap());
		return;
	}
	let body = response.ok().unwrap();
	println!("{}", body);
}

fn _get(url: &str) -> String {
	let client = reqwest::Client::new();
	let response = client
		.get(url)
		.header("Accept-Language", "ja")
		.header("Accept-Charset", "UTF-8")
		.send();
	if response.is_err() {
		let error = response.err().unwrap();
		println!("{} [ERROR] {:?}", get_current_timestamp(), error);
		std::thread::sleep(std::time::Duration::from_millis(100));
		return String::new();
	}
	let content = response.unwrap().text();
	if content.is_err() {
		println!("[ERROR] 読み込み失敗！ 理由: {}", content.err().unwrap());
		return String::new();
	}
	let body = content.ok().unwrap();
	return body;
}

fn get4() {
	println!("{} [TRACE] attack.", get_current_timestamp());
	let client = reqwest::Client::new();
	let response = client
		.get("https://www.google.co.jp/search?q=tokyo+olympic+2020")
		.header("Accept-Language", "ja")
		.header("Accept-Charset", "UTF-8")
		.send();
	if response.is_err() {
		let error = response.err().unwrap();
		println!("{} [ERROR] {:?}", get_current_timestamp(), error);
		std::thread::sleep(std::time::Duration::from_millis(100));
		return;
	}
	let content = response.unwrap().text();
	if content.is_err() {
		println!("[ERROR] 読み込み失敗！ 理由: {}", content.err().unwrap());
		return;
	}
	let body = content.ok().unwrap();
	println!("{}", body);
}

fn run_test02() {
	get4();
}

fn main() {
	if false {
		run_test01();
	}
	if true {
		run_test02();
	}
	println!("Ok.");
}
