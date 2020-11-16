///
/// 例: Result のハンドリング
///
mod example00 {

	pub fn run() {
		let left = std::path::Path::new("left");
		let right = std::path::Path::new("right");

		// rename() はエラー情報を返すが、拾わなくても panic しない
		let result = std::fs::rename(left, right);
		match result {
			Ok(n) => println!("[TRACE] ファイルをコピーしました。{:?}", n),
			Err(err) => println!("[ERROR] ファイルをコピーできません。理由: {:?}", err),
		}
	}
}

///
/// 例
///
mod example01 {
	#[derive(Debug, Clone)]
	pub struct MyStringError {
		pub message: String,
		// pub line: usize,
		// pub column: usize,
	}

	impl std::fmt::Display for MyStringError {
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
			write!(f, "{}", self.message)
		}
	}

	impl std::error::Error for MyStringError {
		fn description(&self) -> &str {
			&self.message
		}
	}

	fn assertion_failure() -> std::result::Result<(), MyStringError> {
		return Err(MyStringError {
			message: "アプリケーションのエラーです。要求はキャンセルされました。".to_string(),
		});
	}

	pub fn run() {
		let result = assertion_failure();
		if result.is_err() {
			println!("[ERROR] 要求は中止します。理由: [{}]", result.err().unwrap());
			return;
		}
		println!("[TRACE] Ok.");
	}
}

fn main() {
	// Result のハンドリング
	example00::run();
	// 独自エラーの実装
	example01::run();
}
