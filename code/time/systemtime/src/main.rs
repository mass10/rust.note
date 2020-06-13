extern crate chrono;

fn timestamp0() -> String {
	let date = chrono::Local::now();
	return format!("{}", date.format("%Y-%m-%d %H:%M:%S%.3f"));
}

fn timestamp1() -> String {
	let date = chrono::Local::now();
	return format!("{}", date.format("%Y%m%d-%H%M%S"));
}

fn timestamp_with_timezone() -> String {
	let date = chrono::Local::now();
	return format!("{}", date.format("%+"));
}

fn main() {
	println!("# std::time::SystemTime");
	println!("{:?}", std::time::SystemTime::now());
	println!();

	println!("# timestamp with timezone (chrono::Local)");
	println!("{}", timestamp_with_timezone());
	println!();

	println!("# local timestamp");
	println!("{}", timestamp0());
	println!("{}", timestamp1());
}
