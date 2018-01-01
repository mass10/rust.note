use std::thread;
use std::time;

fn main() {

	println!("### START ###");

	let h = thread::spawn(|| {
		println!("$$$ begin thread $$$");
		std::thread::sleep(time::Duration::from_millis(1000 * 5));
		println!("--- exit thread ---");
		return 999;
	});

	let result = h.join();
	match result {
		Err(error) => println!("error! {:?}", error),
		Ok(result) => println!("status: {:?}", result),
	}

	println!("--- END ---");
}
