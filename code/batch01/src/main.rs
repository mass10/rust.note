//!
//! エントリーポイントの実装
//!

mod application;
mod db;
mod io;
mod services;

/// エントリーポイント
fn main() {
	let mut app = application::Application::new();
	let result = app.run();
	if result.is_err() {
		println!("[ERROR] ERROR! REASON: {:?}", result.err());
		return;
	}
}
