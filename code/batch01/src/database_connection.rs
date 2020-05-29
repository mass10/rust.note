/// データベース接続
pub struct DatabaseConnection;

impl DatabaseConnection {
	/// インスタンスを返します。
	pub fn new() -> DatabaseConnection {
		let instance = DatabaseConnection {};
		return instance;
	}

	/// ユーザーの登録
	pub fn insert(&self, name: &str, email: &str) -> bool {
		println!("[TRACE] REGISTER... name: {}, email: {}", name, email);
		return true;
	}
}
