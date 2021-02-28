fn println_red<T: std::fmt::Display>(s: T) {
	println!("\x1b[31m{}\x1b[39m", s);
}

fn println_green<T: std::fmt::Display>(s: T) {
	println!("\x1b[32m{}\x1b[39m", s);
}

#[derive(Debug, Clone)]
struct ApplicationError {
	/// エラーメッセージ
	pub description: String,
}

impl ApplicationError {
	/// 新しいインスタンスを返します。
	///
	/// ### Arguments
	/// `description` アプリケーション定義のエラーメッセージ
	///
	/// ### Returns
	/// 新しいインスタンス
	pub fn new(description: &str) -> ApplicationError {
		return ApplicationError { description: description.to_string() };
	}
}

/// ApplicationError に [std::fmt::Display] としての振る舞いを実装します。
impl std::fmt::Display for ApplicationError {
	/// 文字列表現を返す既定の動作です。
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
		return write!(f, "{}", self.description);
	}
}

/// ApplicationError に [std::error::Error] としての振る舞いを実装します。
impl std::error::Error for ApplicationError {
	/// 文字列表現を返す既定の動作です。
	fn description(&self) -> &str {
		return &self.description;
	}
}

/// エントリーポイント
fn main() {
	// 緑の文字で表示します。
	println_green("Hello, world!");
	println_green(999);

	// 赤の文字で表示します。
	println_red(-1);
	println_red(ApplicationError::new("エラーが発生しました！！"));
}
