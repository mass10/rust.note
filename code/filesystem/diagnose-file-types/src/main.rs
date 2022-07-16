//!
//! ファイルの拡張子を分析する
//!

extern crate serde_derive;

mod configuration;
mod core;
mod io;

use configuration::Configuration;
use io::search;

/// エントリーポイント
fn main() {
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
	if conf.get_path() == "" {
		println!("Path to directory needed.");
		std::thread::sleep(std::time::Duration::from_millis(700));
		return;
	}
	let path = std::path::Path::new(conf.get_path());

	// 計算機
	let mut calculator = core::Calculator::new(&conf);

	// ハンドラー
	let mut handler = |arg: &std::path::Path| -> std::result::Result<(), std::boxed::Box<dyn std::error::Error>> {
		calculator.diagnose(arg)?;
		return Ok(());
	};

	// ファイル走査
	let result = search(&conf, &path, &mut handler);
	if result.is_err() {
		let error = result.unwrap_err();
		println!("[ERROR] 予期しない実行時のエラーです。理由: [{:?}]", error);
		std::thread::sleep(std::time::Duration::from_secs(3));
		return;
	}

	// サマリーを表示
	calculator.summary();
}
