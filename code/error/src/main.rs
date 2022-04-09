///
/// アプリケーション内のエラー関連
///
mod error {
	///
	/// アプリケーションのエラーを扱うための構造体を定義します。
	///
	#[derive(Debug, Clone)]
	pub struct ApplicationError {
		/// エラーのメッセージ
		pub message: String,
		// pub line: usize,
		// pub column: usize,
	}

	impl ApplicationError {
		/// アプリケーションエラーを表すオブジェクトを返します。
		pub fn new(message: &str) -> ApplicationError {
			return Self { message: message.to_string() };
		}
	}

	impl std::fmt::Display for ApplicationError {
		/// [std::fmt::Display] としての振る舞いを提供します。
		///
		/// # Examples
		///
		/// ```
		/// fn main() {
		///     let error = ApplicationError::new("error message");
		///     println!("{}", error);
		/// }
		/// ```
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
			write!(f, "{}", self.message)
		}
	}

	impl std::error::Error for ApplicationError {
		fn description(&self) -> &str {
			&self.message
		}
	}
}

///
/// 例: Result のハンドリング
///
mod example_case_00 {

	pub fn run() {
		let result = std::fs::rename("left", "right");
		if result.is_err() {
			println!("{}", result.unwrap_err());
			return;
		}
		println!("[TRACE] ファイルをコピーしました。");
	}
}

///
/// 例
///
mod example_case_01 {
	/// 不正な処理を行う操作
	fn operation_fails() -> std::result::Result<(), std::boxed::Box<dyn std::error::Error>> {
		let reason = super::error::ApplicationError::new("アプリケーションのエラーです。要求はキャンセルされました。");
		return Err(Box::new(reason));
	}

	pub fn run() {
		// 不正な処理を行う操作
		let result = operation_fails();
		if result.is_err() {
			let error = result.err().unwrap();
			println!("[ERROR] 要求は中止します。理由: [{}]", error);
			return;
		}
		println!("[TRACE] Ok.");
	}
}

fn main() {
	// Result のハンドリング
	example_case_00::run();

	// 独自エラーの実装
	example_case_01::run();
}
