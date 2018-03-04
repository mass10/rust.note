use std::time;

fn main() {

	{
		let now = time::Instant::now();
		println!("{:?}", now);
	}

	{
		let now = time::SystemTime::now();
		println!("{:?}", now);
		// println!("{:?}", time::strftime(now).unwrap());
	}
}
