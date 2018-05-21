extern crate reqwest;
use std::io;

fn main() {

	let http = reqwest::get("https://google.co.jp?q=led%20zeppekin");
	if http.is_err() {
		let error = http.unwrap();
		println!("{:?}", error);
		return;
	}
	let mut res = http.unwrap();
	res.copy_to(&mut io::stdout()).unwrap();
}
