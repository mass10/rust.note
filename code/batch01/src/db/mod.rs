//!
//! データベース関連の操作を提供します。
//!

///
/// データベース接続
///
pub struct DatabaseConnection;

impl DatabaseConnection {
	/// 新しいインスタンスを返します。
	///
	/// # Returns
	/// `DatabaseConnection` の新しいインスタンス
	pub fn new() -> DatabaseConnection {
		let instance = DatabaseConnection {};
		return instance;
	}

	/// ユーザーの登録
	///
	/// # Arguments
	/// `name` ユーザーの名前
	/// `email` ユーザーのメールアドレス
	///
	/// # Returns
	/// 成否
	pub fn insert(&self, name: &str, email: &str) -> bool {
		println!("[TRACE] REGISTER... name: {}, email: {}", name, email);
		return true;
	}
}
