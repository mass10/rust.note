/// 環境変数を設定します。
pub fn setenv(key: &str, value: &str) {
	std::env::set_var(key, value);
}

pub fn get_current_timestamp() -> String {
	let now = chrono::Local::now();
	let timestamp = now.format("%Y-%m-%d %H:%M:%S%.3f").to_string();
	return timestamp;
}

#[macro_export]
macro_rules! info {
	($($arg:tt)*) => ({
		let timestamp = crate::util::get_current_timestamp();
		eprintln!("{} [info] {}", timestamp, format!($($arg)*));
	})
}

#[macro_export]
macro_rules! warn {
	($($arg:tt)*) => ({
		let timestamp = crate::util::get_current_timestamp();
		eprintln!("{} [warn] {}", timestamp, format!($($arg)*));
	})
}

#[macro_export]
macro_rules! error {
	($($arg:tt)*) => ({
		let timestamp = crate::util::get_current_timestamp();
		eprintln!("{} [error] {}", timestamp, format!($($arg)*));
	})
}
