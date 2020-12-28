//!
//! 簡単なバッチプリケーションを例に、mut や borrow を練習する。
//!

extern crate chrono;

mod utils {

	pub struct StringUtil {}

	impl StringUtil {}

	pub fn get_timestamp() -> String {
		let date = chrono::Local::now();
		return format!("{}", date.format("%Y-%m-%d %H:%M:%S%.3f"));
	}
}

mod models {

	/// データベース操作クラスの宣言
	pub struct StudentModel {}

	// データベース操作クラスの実装
	impl StudentModel {
		/// 複製された vec を返します。
		pub fn enum_students() -> Vec<String> {
			// 変更可能な vec を作成
			let mut v: Vec<String> = vec![];
			// vec に要素を追加
			v.push(String::from("jimi.hendrix@gmail.com"));
			v.push(String::from("paul.kossoff@gmail.com"));
			v.push(String::from("freddie.mercury@gmail.com"));
			v.push(String::from("billy.preston@gmail.com"));
			v.push(String::from("steve.marriot@gmail.com"));
			// vec の複製を返却
			return v;
		}
	}

	/// クラスモデルの宣言
	pub struct ClassModel {
		_vec: Vec<String>,
	}

	/// クラスモデルの実装
	impl ClassModel {
		/// 新しいインスタンスを返します。
		pub fn new() -> ClassModel {
			// vec を初期化
			return ClassModel {
				_vec: StudentModel::enum_students(),
			};
		}

		/// 生徒一覧を返します。
		fn enum_students(&self) -> &Vec<String> {
			return &self._vec;
		}

		/// クラスを解散します。★これは破壊的変更の発生するメソッドです。
		pub fn free(&mut self) {
			self._vec.clear();
		}

		#[allow(dead_code)]
		/// ダミー
		pub fn dummp1(&self) {
			// self に破壊的変更が発生するような操作は一切認められません。
			// self._vec.clear();
		}

		pub fn dump(&self) {
			println!("[TRACE] $$$ begin dump $$$");
			for e in self.enum_students() {
				println!("{}", e);
			}
			println!("[TRACE] --- end dump ---");
		}
	}
}

mod application {

	use super::models;
	use super::utils;

	///
	/// アプリケーション本体クラスの定義
	///
	pub struct Application {
		/// アプリケーション名
		name: String,
		/// アプリケーション開始日時
		start_timestamp: String,
		/// アプリケーション終了日時
		end_timestamp: String,
	}

	impl Application {
		/// 新しいインスタンスを返します。
		pub fn new() -> Self {
			return Self {
				name: env!("CARGO_PKG_NAME").to_string(),
				start_timestamp: utils::get_timestamp(),
				end_timestamp: "".to_string(),
			};
		}

		pub fn get_start_timestamp(&self) -> &String {
			return &self.start_timestamp;
		}

		pub fn get_end_timestamp(&self) -> &String {
			return &self.end_timestamp;
		}

		/// アプリケーションのコンフィギュレーションを行います。
		pub fn configure(&mut self) -> std::result::Result<(), Box<dyn std::error::Error>> {
			return Ok(());
		}

		pub fn task1(&self, c: &models::ClassModel) {
			// mutable として borrow してないので、破壊的な変更は認められません。
			// c.free();
			// ダンプ
			c.dump();
		}

		pub fn task2(&self, c: &mut models::ClassModel) {
			// mutable として borrow しているため破壊的な変更ができます。
			c.free();
			// 再度ダンプ
			c.dump();
		}

		pub fn run(&self) {
			// オブジェクトを作成
			let mut c = models::ClassModel::new();
			self.task1(&c);
			self.task2(&mut c);
		}
	}
}

/// エントリーポイント
fn main() {
	// アプリケーション本体クラスを初期化します。
	let mut app = application::Application::new();

	// コンフィギュレーションを行います。
	let result = app.configure();
	if result.is_err() {
		println!("[ERROR] {}", result.err().unwrap());
	}

	// メイン処理を実行します。
	app.run();

	println!("Ok.");
}
