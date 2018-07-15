extern crate chrono;

use std::time::SystemTime;

fn timestamp() -> String {
	let date = chrono::Local::now();
	return format!("{}", date.format("%Y-%m-%d %H:%M:%S%.3f"));
}

fn timestamp2() -> String {
	let date = chrono::Local::now();
	return format!("{}", date.format("%Y%m%d-%H%M%S"));
}

fn timestamp_with_timezone() -> String {
	let date = chrono::Local::now();
	return format!("{}", date.format("%+"));
}

fn main() {

	println!("# struct SystemTime");
	println!("{:?}", SystemTime::now());
	println!();

	println!("# timestamp with timezone");
	println!("{}", timestamp_with_timezone());
	println!();

	println!("# local timestamp");
	println!("{}", timestamp());
	println!("{}", timestamp2());
}
