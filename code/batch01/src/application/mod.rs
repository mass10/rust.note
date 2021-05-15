//!
//! アプリケーションのメイン
//!

pub mod errors;

use crate::application;
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
	/// `Application` の新しいインスタンスを返します。
	pub fn new() -> Application {
		return Application { reserved: 0 };
	}

	/// アプリケーションを実行します。
	pub fn run(&mut self) -> std::result::Result<(), Box<dyn std::error::Error>> {
		// TSV ファイルを読み込み
		let mut loader = io::CsvFileLoader::new();
		loader.open_tsv_file("DATA.tsv")?;

		// ========== インポート ==========
		let user_manager = services::UserManager::new();
		loop {
			let result = loader.read_line()?;
			if result.is_none() {
				break;
			}
			let row = result.unwrap();
			if row.len() == 0 {
				continue;
			}
			if row.len() != 2 {
				let error = application::errors::ApplicationError::new("入力検査のエラー(フィールド数が正しくない)");
				return Err(Box::new(error));
			}

			// レコードからフィールドを取り出し
			let (name, email) = (&row[0], &row[1]);

			if name == "NAME" {
				// これはヘッダー行です。
				continue;
			}

			// ========== ユーザー登録 ==========
			user_manager.register_new_user(name, email)?;
		}

		return Ok(());
	}
}
