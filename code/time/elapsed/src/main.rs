use std::time;
use std::thread;

fn duration(left: time::Instant, right: time::Instant) {

	let elapsed = right - left;

	println!("{:?}", elapsed);
	println!("{}", elapsed.as_secs());
	println!("{}", elapsed.subsec_nanos());
}

fn main() {

	let time1 = time::Instant::now();

	thread::sleep(time::Duration::new(2, 234));

	let time2 = time::Instant::now();

	duration(time1, time2);
}
