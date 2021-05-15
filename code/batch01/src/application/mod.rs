//!
//! アプリケーションのメイン
//!

use crate::io;
use crate::services;

///
/// アプリケーション本体の実装
///
pub struct Application {
	#[allow(unused)]
	reserved: i32,
}

impl Application {
	pub fn new() -> Application {
		return Application { reserved: 0 };
	}

	pub fn run(&mut self) -> std::result::Result<(), Box<dyn std::error::Error>> {
		// CSV ファイルを読み込み
		let mut loader = io::CsvFileLoader::new();
		loader.open_csv_file(".rustfmt.toml")?;

		// インポート
		loop {
			let name = "";
			let email = "";
			let user_manager = services::UserManager::new();
			user_manager.register_new_user(name, email)?;
			break;
		}

		loader.close();

		return Ok(());
	}
}
