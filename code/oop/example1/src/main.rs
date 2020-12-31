//!
//! 簡単なバッチ処理風プリケーションを例に、mut や borrow を練習する。
//!

extern crate chrono;

mod utils {

	#[allow(unused)]
	pub struct StringUtil {}

	impl StringUtil {}

	/// タイムスタンプを返します。
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
		pub fn enum_students() -> std::result::Result<Vec<Student>, Box<dyn std::error::Error>> {
			// 変更可能な vec を作成
			let mut v: Vec<Student> = vec![];

			// vec に要素を追加
			v.push(Student {
				name: "Jimi Hendrix".to_string(),
				email: "jimi.hendrix@gmail.com".to_string(),
				team: "".to_string(),
			});
			v.push(Student {
				name: "Paul Kossof".to_string(),
				email: "paul.kossoff@gmail.com".to_string(),
				team: "".to_string(),
			});
			v.push(Student {
				name: "Freddie Mercury".to_string(),
				email: "freddie.mercury@gmail.com".to_string(),
				team: "".to_string(),
			});
			v.push(Student {
				name: "Billy Preston".to_string(),
				email: "billy.preston@gmail.com".to_string(),
				team: "".to_string(),
			});
			v.push(Student {
				name: "Steve Marriot".to_string(),
				email: "steve.marriot@gmail.com".to_string(),
				team: "".to_string(),
			});

			// vec を返却
			return Ok(v);
		}
	}

	///
	/// 生徒クラス
	///
	#[derive(std::fmt::Debug)]
	pub struct Student {
		// 名前
		pub name: String,
		// メールアドレス
		pub email: String,
		// 所属チーム
		pub team: String,
	}

	/// クラスモデルの宣言
	pub struct ClassModel {
		students: Vec<Student>,
	}

	/// クラスモデルの実装
	impl ClassModel {
		/// 新しいインスタンスを返します。
		pub fn new() -> std::result::Result<ClassModel, Box<dyn std::error::Error>> {
			// vec を初期化
			let instance = ClassModel { students: StudentModel::enum_students()? };
			return Ok(instance);
		}

		/// 生徒一覧を返します。
		pub fn get_students(&mut self) -> &mut Vec<Student> {
			return &mut self.students;
		}

		/// クラスを解散します。★これは破壊的変更の発生するメソッドです。
		pub fn free(&mut self) {
			self.students.clear();
		}

		/// ダンプ
		pub fn dump(&self) {
			for e in &self.students {
				println!("[TRACE] {:?}", e);
			}
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
		/// アプリケーションバージョン
		version: String,
		/// アプリケーション開始日時
		start_timestamp: String,
		/// アプリケーション終了日時
		end_timestamp: String,
		/// クラスモデル
		class: models::ClassModel,
	}

	impl Application {
		/// 新しいインスタンスを返します。
		pub fn new() -> std::result::Result<Self, Box<dyn std::error::Error>> {
			let instance = Self {
				name: env!("CARGO_PKG_NAME").to_string(),
				version: env!("CARGO_PKG_VERSION").to_string(),
				start_timestamp: utils::get_timestamp(),
				end_timestamp: "".to_string(),
				class: models::ClassModel::new()?,
			};
			return Ok(instance);
		}

		/// アプリケーション名を返します。
		/// ※ここは clone の方がよい。参照を返すと、呼び出し側変数の生存期間が邪魔になる。
		#[allow(unused)]
		pub fn get_name(&self) -> String {
			return self.name.clone();
		}

		/// アプリケーションバージョンを返します。
		/// ※ここは clone の方がよい。参照を返すと、呼び出し側変数の生存期間が邪魔になる。
		#[allow(unused)]
		pub fn get_version(&self) -> String {
			return self.version.clone();
		}

		/// 処理開始日時を返します。
		/// ※ここは clone の方がよい。参照を返すと、呼び出し側変数の生存期間が邪魔になる。
		#[allow(unused)]
		pub fn get_start_timestamp(&self) -> String {
			return self.start_timestamp.clone();
		}

		/// 処理終了日時を返します。
		/// ※ここは clone の方がよい。参照を返すと、呼び出し側変数の生存期間が邪魔になる。
		#[allow(unused)]
		pub fn get_end_timestamp(&self) -> String {
			return self.end_timestamp.clone();
		}

		/// ランダムな文字列を生成します。
		#[allow(unused)]
		pub fn generate_random_string(&self) -> String {
			return utils::get_timestamp();
		}

		/// アプリケーションのコンフィギュレーションを行います。
		pub fn configure(&mut self) -> std::result::Result<(), Box<dyn std::error::Error>> {
			let _ = self.generate_random_string();
			return Ok(());
		}

		/// タスク1を実行します。
		pub fn update_paul(&mut self) -> std::result::Result<(), Box<dyn std::error::Error>> {
			// 生徒を列挙しつつ、一部の生徒情報に変更を加えます。
			for student in self.class.get_students() {
				if student.name.contains("Paul") {
					student.name.push_str("★");
				}
			}
			return Ok(());
		}

		/// タスク2を実行します。
		pub fn update_jimi(&mut self) -> std::result::Result<(), Box<dyn std::error::Error>> {
			for student in self.class.get_students() {
				if student.name.contains("Jimi") {
					student.name.push_str("■");
				}
			}
			return Ok(());
		}

		/// これまでの操作を確定します。
		fn commit_operation(&mut self) -> std::result::Result<(), Box<dyn std::error::Error>> {
			// ダンプ
			self.class.dump();

			// 解放
			self.class.free();

			return Ok(());
		}

		/// アプリケーションを実行します。
		pub fn run(&mut self) -> std::result::Result<(), Box<dyn std::error::Error>> {
			// アプリケーション名
			// こういう変数は複製として扱います。ここが &String になっていると参照が生存していて面倒。
			let app_name = self.get_name();

			println!("{} [TRACE] <{}> ### START ###", utils::get_timestamp(), app_name);

			// Paul の変更処理
			self.update_paul()?;

			// Jimi の変更処理
			self.update_jimi()?;

			// ここまでの操作を確定します。
			// ※アプリケーション本体に対する破壊的変更
			self.commit_operation()?;

			println!("{} [TRACE] <{}> --- END ---", utils::get_timestamp(), app_name);

			return Ok(());
		}
	}
}

///
/// エントリーポイント
///
fn main() {
	// アプリケーション本体クラスを初期化します。
	let result = application::Application::new();
	if result.is_err() {
		println!("[ERROR] アプリケーションはエラーで終了しました。理由: [{}]", result.err().unwrap());
		return;
	}
	let mut app = result.unwrap();

	// コンフィギュレーションを行います。
	let result = app.configure();
	if result.is_err() {
		println!("[ERROR] {}", result.err().unwrap());
	}

	// メイン処理を実行します。
	let result = app.run();
	if result.is_err() {
		println!("[ERROR] アプリケーションはエラーで終了しました。理由: [{}]", result.err().unwrap());
		return;
	}

	println!("Ok.");
}
