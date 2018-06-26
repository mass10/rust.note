extern crate chrono;

use std::time::SystemTime;

fn timestamp() -> String {
	let date = chrono::Local::now();
	return format!("{}", date.format("%Y-%m-%d %H:%M:%S%.3f"));
}

fn timestamp_with_timezone() -> String {
	let date = chrono::Local::now();
	return format!("{}", date.format("%+"));
}

fn main() {
	println!("{:?}", SystemTime::now());
	// timestamp with timezone
	println!("{}", timestamp_with_timezone());
	// timestamp ★★★
	println!("{}", timestamp());
}
