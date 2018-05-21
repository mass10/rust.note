extern crate chrono;
extern crate ctrlc;
extern crate reqwest;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

fn inner_main() {

	println!("{} [TRACE] attack.", chrono::Local::now().format("%Y-%m-%d %H:%M:%S%.3f"));
	let response = reqwest::get("http://192.168.56.102:3000");
	if response.is_err() {
		let error = response.err().unwrap();
		println!("{} [ERROR] {:?}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S%.3f"), error);
		std::thread::sleep(std::time::Duration::from_millis(100));
        return;
	}
	let mut response = response.unwrap();
	response.copy_to(&mut std::io::stdout()).unwrap();
	std::thread::sleep(std::time::Duration::from_millis(10));
}

fn main() {

	let running = Arc::new(AtomicBool::new(true));
	let r = running.clone();
	ctrlc::set_handler(move || {
		r.store(false, Ordering::SeqCst);
	}).expect("Error setting Ctrl-C handler");

	while running.load(Ordering::SeqCst) {
		inner_main();
	}

	println!("Ok.");
}
