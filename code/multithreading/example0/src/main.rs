use std::thread;
use std::time;

fn main() {
	println!("### START ###");

	let (tx, rx): (std::sync::mpsc::Sender<String>, std::sync::mpsc::Receiver<String>) = std::sync::mpsc::channel();

	let handle = thread::spawn(move || {

		println!("$$$ begin thread $$$");

		let result = tx.send("START".to_string());
		if result.is_err() {
			let error = result.err().unwrap();
			println!("[ERROR] Unknown error. reason: [{}]", error);
		}

		std::thread::sleep(time::Duration::from_millis(1000 * 5));

		println!("--- exit thread ---");

		let result = tx.send("END".to_string());
		if result.is_err() {
			let error = result.err().unwrap();
			println!("[ERROR] Unknown error. reason: [{}]", error);
		}

		return "".to_string();
	});

	{
		let result = rx.recv();
		if result.is_err() {
			println!("[ERROR] reason: {}", result.err().unwrap());
		} else {
			let result = result.ok();
			println!("[TRACE] result: [{:?}]", result);
		}
	}

	let result = handle.join();
	match result {
		Err(error) => println!("error! {:?}", error),
		Ok(result) => println!("status: {:?}", result),
	}

	println!("--- END ---");
}
