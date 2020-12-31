use std::thread;
use std::time;

fn main() {

	println!("### START ###");

	let handle = thread::spawn(|| {
		println!("$$$ begin thread $$$");
		std::thread::sleep(time::Duration::from_millis(1000 * 5));
		println!("--- exit thread ---");
		return 999;
	});

	let result = handle.join();
	match result {
		Err(error) => println!("error! {:?}", error),
		Ok(result) => println!("status: {:?}", result),
	}

	println!("--- END ---");
}
