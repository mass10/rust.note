extern crate reqwest;
use std::io;

fn main() {

	let http = reqwest::get("http://127.0.0.1:3000");
	if http.is_err() {
		let error = http.unwrap();
		println!("{:?}", error);
		return;
	}
	let mut res = http.unwrap();
	res.copy_to(&mut io::stdout()).unwrap();
}
