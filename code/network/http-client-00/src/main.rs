extern crate reqwest;
// use std::io;
use std::io::Read;

fn _get(url: &str, out: &mut dyn std::io::Write) {
	let http = reqwest::get(url);
	if http.is_err() {
		let error = http.err();
		println!("{:?}", error);
		return;
	}
	let mut res = http.unwrap();
	let status = res.copy_to(out);
	if status.is_err() {
		println!("[ERROR] {:?}", status.err());
		return;
	}
}

fn print_body_all(res: &mut reqwest::Response, out: &mut dyn std::io::Write) {
	let status = res.copy_to(out);
	if status.is_err() {
		println!("[ERROR] {:?}", status.err());
		return;
	}
}

fn _read_body_all(res: &mut reqwest::Response) -> String {
	let mut body = String::new();
	let status = res.read_to_string(&mut body);
	if status.is_err() {
		println!("[ERROR] {:?}", status.err());
		return String::new();
	}
	return body;
}

fn main() {
	// Accept-Language を送らないと全然ダメ
	let http = reqwest::get("https://google.co.jp?q=led%20zeppekin");
	if http.is_err() {
		let error = http.err();
		println!("{:?}", error);
		return;
	}
	let mut res = http.unwrap();

	// 文字列に全部読む
	if false {
		let _ = _read_body_all(&mut res);
	}

	// 標準出力に全部出す
	print_body_all(&mut res, &mut std::io::stdout());
}
