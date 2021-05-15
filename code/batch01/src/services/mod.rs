//!
//! 各種ビジネスロジックを提供するサービス層です。
//!

use crate::db;

///
/// ユーザー管理
///
pub struct UserManager {
	#[allow(unused)]
	reserved: i32,
}

impl UserManager {
	/// インスタンスを返します。
	///
	/// # Returns
	/// `UserManager` の新しいインスタンス
	pub fn new() -> UserManager {
		return UserManager { reserved: 0 };
	}

	/// ユーザーの登録
	///
	/// # Arguments
	/// `name` ユーザーの名前
	/// `email` ユーザーのメールアドレス
	///
	/// # Returns
	/// 成否
	pub fn register_new_user(&self, name: &str, email: &str) -> std::result::Result<(), Box<dyn std::error::Error>> {
		// ユーザーを登録します。
		let connection = db::DatabaseConnection::new();
		connection.insert(name, email);

		return Ok(());
	}
}
