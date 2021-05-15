//!
//! 各種ビジネスロジックを提供するサービス層です。
//!

use crate::db;

///
/// ユーザー管理
///
pub struct UserManager;

impl UserManager {
	/// インスタンスを返します。
	pub fn new() -> UserManager {
		return UserManager {};
	}

	/// ユーザーの登録
	pub fn register_new_user(&self, name: &str, email: &str) -> std::result::Result<(), Box<dyn std::error::Error>> {
		// ユーザーを登録します。
		let connection = db::DatabaseConnection::new();
		connection.insert(name, email);

		return Ok(());
	}
}
