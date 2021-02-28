extern crate chrono;

mod application;
mod thread;
mod util;

/// エントリーポイント
fn main() {
	// スタート
	let app = application::Application::new();
	app.start_app();
}
