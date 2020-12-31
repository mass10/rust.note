// use std::thread;
// use std::time;

fn get_current_timestamp() -> String {
	let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S%.3f");
	return format!("{}", timestamp);
}

async fn slow_operation() {
	let current_thread_id = std::thread::current().id();
	let pid = std::process::id();

	println!("{} ({:?}) ({:?}) $$$ begin $$$", get_current_timestamp(), pid, current_thread_id);

	std::thread::sleep(std::time::Duration::from_millis(1000 * 3));

	println!("{} ({:?}) ({:?}) --- end ---", get_current_timestamp(), pid, current_thread_id);
}

fn main() {
	let current_thread_id = std::thread::current().id();
	let pid = std::process::id();

	println!("{} ({:?}) ({:?}) ### START ###", get_current_timestamp(), pid, current_thread_id);

	futures::executor::block_on(slow_operation());

	println!("{} ({:?}) ({:?}) --- END ---", get_current_timestamp(), pid, current_thread_id);
}
