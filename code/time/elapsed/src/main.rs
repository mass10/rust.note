use std::time;
use std::thread;

trait formatting {
	fn to_string(&self) -> String;
}

impl formatting for time::Duration {
	fn to_string(&self) -> String {
		let mut sec = self.as_secs();
		let mut min = 0;
		let mut hour = 0;
		while 60 <= sec {
			min += 1;
			sec -= 60;
		}
		while 60 <= min {
			hour += 1;
			min -= 60;
		}
		let s = format!("{:02}:{:02}:{:02}", hour, min, sec);
		return s;
	}
}

fn duration(left: time::Instant, right: time::Instant) {

	let elapsed = right - left;

	println!("{:?}", elapsed);
	println!("{}", elapsed.to_string());
	println!("{}", elapsed.subsec_nanos());
}

fn main() {

	let time1 = time::Instant::now();

	thread::sleep(time::Duration::new(2, 234));

	let time2 = time::Instant::now();

	duration(time1, time2);
}
