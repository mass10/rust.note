#![windows_subsystem = "windows"]

/// 呼び出されない関数です。
fn unused_function() {
	println!("[TRACE] Not implemened!");
}

/// アプリケーションのエントリーポイントです。
fn main() {
	println!("[TRACE] ### START ###");
	std::thread::sleep(std::time::Duration::from_secs(5));
	println!("[TRACE] --- END ---");
}
