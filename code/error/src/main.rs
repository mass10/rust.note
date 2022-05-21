mod util {

	/// Return system timestamp
	///
	/// # Returns
	/// タイムスタンプ
	#[allow(unused)]
	pub fn get_current_timestamp() -> String {
		let date = chrono::Local::now();
		return format!("{}", date.format("%Y-%m-%d %H:%M:%S%.3f"));
	}
}

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

	///
	/// アプリケーションのエラーを扱うための構造体を定義します。
	///
	#[derive(Debug, Clone)]
	pub struct MyErrorA {
		/// エラーのメッセージ
		pub message: String,
	}

	impl MyErrorA {
		/// アプリケーションエラーを表すオブジェクトを返します。
		pub fn new(message: &str) -> MyErrorA {
			return Self { message: message.to_string() };
		}
	}

	impl std::fmt::Display for MyErrorA {
		/// [std::fmt::Display] としての振る舞いを提供します。
		///
		/// # Examples
		///
		/// ```
		/// fn main() {
		///     let error = MyErrorA::new("error message");
		///     println!("{}", error);
		/// }
		/// ```
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
			write!(f, "{}", self.message)
		}
	}

	impl std::error::Error for MyErrorA {
		fn description(&self) -> &str {
			&self.message
		}
	}
}

///
/// 例: Result のハンドリング
///
mod example_case_00 {

	/// 存在しないファイルのリネームを試みるテスト
	pub fn run() {
		let result = std::fs::rename("left", "right");
		if result.is_err() {
			println!("[ERROR] {}", result.unwrap_err());
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
			println!("[ERROR] エラー: [{}]", error);
			return;
		}
		println!("[TRACE] Ok.");
	}
}

mod example_case_02 {

	pub fn run() {
		let result = execute();
		if result.is_err() {
			println!("[ERROR] {}", result.err().unwrap());
			return;
		}
	}

	fn execute() -> Result<(), Box<dyn std::error::Error>> {
		let time = std::time::SystemTime::now();
		let current = time.duration_since(std::time::UNIX_EPOCH).unwrap();
		let millisec = current.subsec_millis();

		println!("[TRACE] {}", millisec);

		if 800 <= millisec {
			let err = super::error::ApplicationError::new("500ミリ秒以上経過しています。");
			return Err(Box::new(err));
		} else if 100 <= millisec {
			let err = super::error::MyErrorA::new("200ミリ秒以上経過しています。");
			return Err(Box::new(err));
		}
		return Ok(());
	}
}

fn main() {
	// Result のハンドリング
	example_case_00::run();

	// 独自エラーの実装
	example_case_01::run();

	example_case_02::run();
}
