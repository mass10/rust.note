extern crate rand;

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

	/// std::error::Error としての振る舞いを実装します。
	impl std::error::Error for ApplicationError {
		fn description(&self) -> &str {
			&self.message
		}
	}

	///
	/// アプリケーションのエラーを扱うための構造体を定義します。
	///
	#[derive(Debug, Clone)]
	pub struct DatabaseError {
		/// エラーのメッセージ
		pub message: String,
	}

	impl DatabaseError {
		/// アプリケーションエラーを表すオブジェクトを返します。
		#[allow(unused)]
		pub fn new(message: &str) -> DatabaseError {
			return Self { message: message.to_string() };
		}
	}

	impl std::fmt::Display for DatabaseError {
		/// [std::fmt::Display] としての振る舞いを提供します。
		///
		/// # Examples
		///
		/// ```
		/// fn main() {
		///     let error = DatabaseError::new("error message");
		///     println!("{}", error);
		/// }
		/// ```
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
			write!(f, "{}", self.message)
		}
	}

	/// std::error::Error としての振る舞いを実装します。
	impl std::error::Error for DatabaseError {
		fn description(&self) -> &str {
			&self.message
		}

		fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
			None
		}
	}

	///
	/// アプリケーションのエラーを扱うための構造体を定義します。
	///
	#[derive(Debug, Clone)]
	pub struct MyFatal {
		/// エラーのメッセージ
		pub message: String,
	}

	impl MyFatal {
		/// アプリケーションエラーを表すオブジェクトを返します。
		#[allow(unused)]
		pub fn new(message: &str) -> MyFatal {
			return Self { message: message.to_string() };
		}
	}

	impl std::fmt::Display for MyFatal {
		/// [std::fmt::Display] としての振る舞いを提供します。
		///
		/// # Examples
		///
		/// ```
		/// fn main() {
		///     let error = MyFatal::new("error message");
		///     println!("{}", error);
		/// }
		/// ```
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
			write!(f, "{}", self.message)
		}
	}

	/// std::error::Error としての振る舞いを実装します。
	impl std::error::Error for MyFatal {
		fn description(&self) -> &str {
			&self.message
		}

		fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
			None
		}
	}
}

/// Result を利用して、実行時エラーのハンドリングを試みる
#[allow(unused)]
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

/// Error をキャッチする例
#[allow(unused)]
mod example_case_01 {

	/// 様々な要因により、ちょくちょく失敗する何らかの処理
	fn operation_fails() -> std::result::Result<(), Box<dyn std::error::Error>> {
		use rand::{thread_rng, Rng};

		let n = thread_rng().gen_range(0..3);
		if n == 0 {
			let error = super::error::ApplicationError::new("アプリケーションのエラーです。");
			return Err(Box::new(error));
		}
		return Ok(());
	}

	pub fn run() {
		let result = operation_fails();
		if result.is_err() {
			let error = result.unwrap_err();
			println!("[ERROR] エラー: [{}]", error);
			return;
		}
		println!("[TRACE] Ok.");
	}
}

/// enum を利用して、異なる Error のハンドリングを行う試み
#[allow(unused)]
mod example_case_02 {
	use std::any::Any;

	/// アプリケーションのエラーを扱うための列挙型を定義します。
	#[derive(Debug)]
	enum ApplicationErrorEnum {
		/// ユーザーがみつかりません。
		UserNotFound,
		/// 部署がみつかりません。
		DeptNotFound,
		/// 不正なメールアドレスです。
		InvalidEmail,
		/// 不正なエラーです。
		UnknownError,
	}

	/// `std::fmt::Display` としての振る舞いを実装しています。
	///
	/// * `std::error::Error` の実装のために必須です。
	impl std::fmt::Display for ApplicationErrorEnum {
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
			match self {
				ApplicationErrorEnum::UserNotFound => write!(f, "ユーザーが見つかりません。"),
				ApplicationErrorEnum::DeptNotFound => write!(f, "部署が見つかりません。"),
				ApplicationErrorEnum::InvalidEmail => write!(f, "不正なメールアドレスです。"),
				ApplicationErrorEnum::UnknownError => write!(f, "不明なエラーです。"),
			}
		}
	}

	/// std::error::Error としての振る舞いを実装します。
	///
	/// * trait `std::fmt::Display` の実装が必須です。
	impl std::error::Error for ApplicationErrorEnum {
		fn description(&self) -> &str {
			match self {
				ApplicationErrorEnum::UserNotFound => "ユーザーが見つかりません。",
				ApplicationErrorEnum::DeptNotFound => "部署が見つかりません。",
				ApplicationErrorEnum::InvalidEmail => "不正なメールアドレスです。",
				ApplicationErrorEnum::UnknownError => "不明なエラーです。",
			}
		}
	}

	/// 様々な要因により、ちょくちょく失敗する何らかの処理
	fn execute() -> std::result::Result<(), Box<dyn std::error::Error>> {
		use rand::{thread_rng, Rng};

		let n = thread_rng().gen_range(0..99);
		if 90 <= n {
			return Err(Box::new(ApplicationErrorEnum::UnknownError));
		}
		if 75 <= n {
			return Err(Box::new(ApplicationErrorEnum::UserNotFound));
		}
		if 50 <= n {
			return Err(Box::new(ApplicationErrorEnum::DeptNotFound));
		}
		if 20 <= n {
			return Err(Box::new(ApplicationErrorEnum::InvalidEmail));
		}

		return Ok(());
	}

	/// スタート
	pub fn run() {
		// タイミングによってよく失敗する何らかの処理
		let result = execute();
		if result.is_err() {
			// すべてのエラーがここでハンドルされます。
			let error = result.unwrap_err();
			if error.is::<ApplicationErrorEnum>() {
				// どうしても必要ならダウンキャストも可能です。
				let inner_type = error.downcast_ref::<ApplicationErrorEnum>().unwrap();
				// 必要ならパターンマッチ可能
				match inner_type {
					ApplicationErrorEnum::UserNotFound => println!("[ERROR] ユーザーが見つかりません。"),
					ApplicationErrorEnum::DeptNotFound => println!("[ERROR] 部署が見つかりません。"),
					ApplicationErrorEnum::InvalidEmail => println!("[ERROR] 不正なメールアドレスです。"),
					ApplicationErrorEnum::UnknownError => println!("[ERROR] 不明なエラーです。"),
				}
				// 列挙型の型名
				let inner_type_name = format!("{error:?}");
				// 文字列表現(fmt による)
				let source_message = format!("{inner_type}");
				println!("[ERROR] アプリケーションの定義済みエラーです。error: [{inner_type_name}], message: [{source_message}]");
			} else {
				// それ以外のすべてのエラー
				println!("[ERROR] 予期しない実行時のエラー: [{}]", error.to_string());
			}

			return;
		}

		println!("[INFO] 正常終了");
	}
}

/// enum ではなく、複数種類の Error でエラーハンドリングを試みる
mod example_case_03 {

	/// 様々な要因により、ちょくちょく失敗する何らかの処理
	fn execute() -> std::result::Result<(), Box<dyn std::error::Error>> {
		use rand::{thread_rng, Rng};

		let n = thread_rng().gen_range(0..9);
		if 7 <= n {
			let err = super::error::ApplicationError::new("対象の出荷指示票はただいま操作中です。");
			return Err(Box::new(err));
		} else if 3 <= n {
			let err = super::error::DatabaseError::new("指定された組織情報がみつかりません。");
			return Err(Box::new(err));
		} else if 1 <= n {
			let err = super::error::MyFatal::new("設定ファイルがみつかりませんでした。");
			return Err(Box::new(err));
		}
		return Ok(());
	}

	/// スタート
	pub fn run() {
		// 様々なきっかけで失敗する何らかの処理
		let result = execute();
		if result.is_err() {
			let err = result.unwrap_err();
			if err.is::<super::error::ApplicationError>() {
				println!("[ERROR] アプリケーションのエラーです。原因: [{}]", err);
			} else if err.is::<super::error::DatabaseError>() {
				println!("[ERROR] データベースのエラーです。原因: [{}]", err);
			} else if err.is::<super::error::MyFatal>() {
				println!("[ERROR] 致命的エラーです。原因: [{}]", err);
			} else {
				println!("[ERROR] 予期しないエラーです。原因: [{}]", err);
			}
			return;
		}
	}
}

fn main() {
	// Result のハンドリング
	// example_case_00::run();

	// 独自エラーの実装
	// example_case_01::run();

	// アプリケーション定義の enumn を用いた詳細なエラーハンドリング
	// example_case_02::run();

	// アプリケーション定義の Error trait を使用した、細かなエラーハンドリング
	example_case_03::run();
}
