extern crate reqwest;
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
	// stream did not contain valid UTF-8
	if false {
		println!("[TRACE] try 1: read_to_string");
		let mut body = String::new();
		let status = res.read_to_string(&mut body);
		if status.is_err() {
			println!("[ERROR] {:?}", status.err().unwrap());
			return;
		}
		return;
	}

	// 標準出力に全部出す
	// Windows だとコケる (Windows stdio in console mode does not support writing non-UTF-8 byte sequences)
	if true {
		println!("try 2: read to STDOUT");
		let mut out = std::io::stdout();
		let status = res.copy_to(&mut out);
		if status.is_err() {
			println!("[ERROR] コンテンツの読み出し失敗！ 理由: {:?}", status.err().unwrap());
			return;
		}
		return;
	}
}
