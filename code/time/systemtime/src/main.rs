extern crate chrono;

use std::time::SystemTime;

fn main() {

	{
		let now = SystemTime::now();
		println!("{:?}", now);
	}

	{
		let date = chrono::Local::now();
		// timestamp with timezone
		println!("{}", date.format("%+"));
		// timestamp
		println!("{}", date.format("%Y-%m-%d %H:%M:%S%.3f"));
	}
}

