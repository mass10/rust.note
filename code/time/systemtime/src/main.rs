extern crate chrono;

use std::time::SystemTime;

fn main() {

	let now = SystemTime::now();
	println!("{:?}", now);
	// println!("{:#X}", now);

	let date = chrono::Local::now();
	println!("{}", date.format("%Y-%m-%d %H:%M:%S%.3f"));
    println!("{}", date.format("%+"));
}

