//!
//! ファイルの拡張子を分析する
//!

extern crate serde_derive;

mod configuration;
mod core;
mod io;

use configuration::Configuration;

/// エントリーポイント
fn main() {
	// ========== コンフィギュレーション ==========

	// コンフィギュレーション
	let result = Configuration::new();
	if result.is_err() {
		let error = result.unwrap_err();
		println!("[ERROR] コンフィギュレーションのエラーです。理由: [{:?}]", error);
		std::thread::sleep(std::time::Duration::from_millis(700));
		return;
	}
	let conf = result.unwrap();

	// パスの確認
	if conf.path_to_run == "" {
		println!("Path to directory needed.");
		std::thread::sleep(std::time::Duration::from_millis(700));
		return;
	}

	// ========== アプリケーションを実行 ==========

	// アプリケーションを初期化
	let mut calculator = core::Application::new(&conf);

	// 実行
	let result = calculator.run();
	if result.is_err() {
		let error = result.unwrap_err();
		println!("[ERROR] 予期しない実行時のエラーです。理由: [{:?}]", error);
		std::thread::sleep(std::time::Duration::from_secs(3));
		return;
	}
}
