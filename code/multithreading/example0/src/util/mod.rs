/// タイムスタンプを生成します。
pub fn get_current_timestamp() -> String {
	let date = chrono::Local::now();
	return format!("{}", date.format("%Y-%m-%d %H:%M:%S%.3f"));
}

/// 簡易ロギング(debug)
#[macro_export]
macro_rules! debug {
	($($arg:tt)*) => ({
		let line = format!($($arg)*);
		println!("{} [DEBUG] {} ({:?}) {}", util::get_current_timestamp(), std::process::id(), std::thread::current().id(), line);
	})
}

/// 簡易ロギング(info)
#[macro_export]
macro_rules! info {
	($($arg:tt)*) => ({
		let line = format!($($arg)*);
		println!("{} [INFO] {} ({}) {}", util::get_current_timestamp(),
			std::process::id(), std::thread::current().id(), line);
	})
}

/// 簡易ロギング(error)
#[macro_export]
macro_rules! error {
	($($arg:tt)*) => ({
		let line = format!($($arg)*);
		println!("{} [ERROR] {} ({:?}) {}",
			util::get_current_timestamp(), std::process::id(), std::thread::current().id(), line);
	})
}
