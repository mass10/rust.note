//!
//! Rust の非同期処理のサンプル
//!

mod application;
mod thread1;
mod thread2;

/// 現在のタイムスタンプを返します。
pub fn get_current_timestamp() -> String {
	let now = chrono::Local::now();
	return now.format("%Y-%m-%d %H:%M:%S%.3f").to_string();
}

/// info
#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
		let line = format!($($arg)*);
		let current_timestamp = crate::get_current_timestamp();
		let pid = std::process::id();
		let tid = std::thread::current().id();
		println!("{} ({}, {:?}) [info] {}", current_timestamp, pid, tid, line);
	};
}

/// debug
#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {
		let line = format!($($arg)*);
		let current_timestamp = crate::get_current_timestamp();
		let pid = std::process::id();
		let tid = std::thread::current().id();
		println!("{} ({}, {:?}) [debug] {}", current_timestamp, pid, tid, line);
    };
}

/// error
#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
		let line = format!($($arg)*);
		let current_timestamp = crate::get_current_timestamp();
		let pid = std::process::id();
		let tid = std::thread::current().id();
		println!("{} ({}, {:?}) [error] {}", current_timestamp, pid, tid, line);
    };
}

/// Rust アプリケーションのエントリーポイント
fn main() {
	let app = application::Application::new();
	app.run();
}
