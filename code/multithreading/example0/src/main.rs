//! アプリケーションのエントリーポイント
extern crate chrono;

mod application;
mod thread;
mod util;

/// アプリケーションのエントリーポイントです。
fn main() {
	// アプリケーションを初期化して起動します。
	let app = application::Application::new();
	let result = app.run();
	if result.is_err() {
		error!("{}", result.err().unwrap());
		return;
	}
}
