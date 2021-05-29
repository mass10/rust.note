//!
//! エントリーポイントの実装
//!

mod application;
mod db;
mod io;
mod services;

/// エントリーポイント
fn main() {
	// アプリケーションを初期化
	let mut app = application::Application::new();

	// セットアップ(テスト)
	app.setup();

	// 実行
	let result = app.run();
	if result.is_err() {
		println!("[ERROR] ERROR! REASON: {:?}", result.err());
		return;
	}
}
