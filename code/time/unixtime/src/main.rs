extern crate chrono;

fn parse_int(s: &str) -> i64 {
	return match s.trim().parse::<i64>() {
		Ok(n) => n,
		Err(_) => 0,
	};
}

fn describe_timestamp(secs: i64) {
	let time = chrono::NaiveDateTime::from_timestamp(secs, 0);
	println!("{} is {:?}", secs, time);
}

fn describe_current_timestamp() {
	let now = chrono::Local::now();
	let timestamp = now.timestamp();
	describe_timestamp(timestamp);
}
fn main() {
	let args: Vec<String> = std::env::args().skip(1).collect();
	if args.len() == 0 {
		describe_current_timestamp();
		return;
	}

	let secs = parse_int(&args[0]);
	describe_timestamp(secs);
}
