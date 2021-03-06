mod application_errors {
	#[derive(Debug, Clone)]
	pub struct ApplicationError {
		pub description: String,
	}

	impl ApplicationError {
		#[allow(unused)]
		pub fn new(description: String) -> ApplicationError {
			return ApplicationError { description: description };
		}
	}

	impl std::fmt::Display for ApplicationError {
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
			write!(f, "{}", self.description)
		}
	}

	impl std::error::Error for ApplicationError {
		fn description(&self) -> &str {
			&self.description
		}
	}
}

///
/// データベース関連
///
mod db {
	///
	/// データベースクラス
	///
	pub struct DatabaseAccess {}
	impl DatabaseAccess {
		///
		/// 新しいオブジェクトのインスタンスを返します。
		///
		pub fn new() -> Box<DatabaseAccess> {
			let instance = DatabaseAccess {};
			return Box::new(instance);
		}

		/// 生徒名簿を抽出します。
		/// Heap 上のオブジェクトへの参照を返します。
		pub fn enum_students_boxed_type(&self, _unused1: &str) -> Box<Vec<Box<String>>> {
			let mut students: Vec<Box<String>> = vec![];
			students.push(Box::new(String::from("jimi.hendrix@gmail.com")));
			students.push(Box::new(String::from("paul.kossoff@gmail.com")));
			students.push(Box::new(String::from("freddie.mercury@gmail.com")));
			students.push(Box::new(String::from("billy.preston@gmail.com")));
			students.push(Box::new(String::from("steve.marriot@gmail.com")));
			return Box::new(students);
		}

		pub fn validate_registration_time(&self, students: &Vec<Box<String>>) -> std::result::Result<(), Box<dyn std::error::Error>> {
			use super::application_errors;

			for e in students.into_iter() {
				if !e.contains("@") {
					let error = application_errors::ApplicationError::new("何らかのエラー".to_string());
					return Err(Box::new(error));
				}
			}
			return Ok(());
		}

		pub fn validate_registration_email(&self, students: &Vec<Box<String>>) -> bool {
			for e in students.into_iter() {
				if e.len() == 0 {
					return false;
				}
			}
			return true;
		}
	}

	///
	/// クラスルームのクラス
	///
	pub struct Classroom {
		// 1 と 2 どっちが扱いやすいか
		students1: Vec<Box<String>>,
		students2: Vec<String>,
		name: String,
	}

	impl Classroom {
		/// 新しいインスタンスを返します。
		pub fn new(classroom: &str) -> Classroom {
			let mut instance = Classroom {
				name: String::from(classroom),
				students1: vec![],
				students2: vec![],
			};
			instance.initialize();
			return instance;
		}

		/// オブジェクトの初期化を行います。
		fn initialize(&mut self) {
			use super::db;

			// データベースクラス
			let dbaccess = db::DatabaseAccess::new();

			//生徒名簿を抽出
			let students = dbaccess.enum_students_boxed_type(&self.name);
			self.students1 = students.to_vec();

			self.students2.clear();
			for e in students.into_iter() {
				self.students2.push(*e);
			}
		}

		#[allow(unused)]
		/// 生徒名簿を返します。返却されるオブジェクトは複製です。
		pub fn enum_students_clone(&self) -> Vec<Box<String>> {
			return self.students1.clone();
		}

		#[allow(unused)]
		/// 生徒名簿を返します。返却されるオブジェクトは参照です。
		pub fn enum_students_ref(&mut self) -> &mut Vec<Box<String>> {
			return self.students1.as_mut();
		}
	}
}

///
/// test00
///
mod test00 {
	#[allow(unused)]
	fn test(v: &Vec<i64>) {
		print!("[TRACE] ");
		for (i, e) in v.iter().enumerate() {
			if 0 < i {
				print!(", ");
			}
			print!("{}", e);
		}
		println!("");
	}

	/// シンプルな Vec のテスト
	#[allow(unused)]
	pub fn run() {
		let v = vec!["こんにちは", "Real", "World", "Rust"];
		println!("{:?}", v);

		let mut v: Vec<i64> = vec![];
		v.push(-192);
		v.push(12);
		v.push(19823);
		test(&v);
		println!("{:?}", v);

		let v = vec![1, 3, -89, 67, 10092, -1029, -12, -10];
		println!("[TRACE] 3つめの要素は {0} です。", v[2]);
	}
}

mod test01 {
	#[allow(unused)]
	pub fn run() -> std::result::Result<(), Box<dyn std::error::Error>> {
		use super::db;

		// Classroom を参照します。
		let classroom = db::Classroom::new("A");

		// 生徒名簿
		let students = classroom.enum_students_clone();

		// データベース接続
		let dbaccess = db::DatabaseAccess::new();

		// Vec を読み取り専用アクセス(Rust では「借用」、よく「参照」と言われる仕組み)
		dbaccess.validate_registration_time(&students)?;
		dbaccess.validate_registration_time(&students)?;
		dbaccess.validate_registration_time(&students)?;

		for e in classroom.enum_students_clone() {
			println!("[TRACE] {}", e);
		}

		return Ok(());
	}
}

mod test02 {
	/// Vec に破壊的な変更を加える何らかの操作
	fn modify_vector_entries(students: &mut Vec<Box<String>>) {
		for e in students {
			if e.contains("jimi") {
				e.extend(" ★".chars());
			}
		}
	}

	#[allow(unused)]
	pub fn run() -> std::result::Result<(), Box<dyn std::error::Error>> {
		use super::db;

		// Classroom を参照します。
		let mut classroom = db::Classroom::new("A");

		// 生徒名簿
		let students = classroom.enum_students_ref();

		// データベース接続
		let dbaccess = db::DatabaseAccess::new();

		// 何らかのバリデーション。読み取り専用の「借用」(=参照)アクセス。
		dbaccess.validate_registration_time(students)?;

		// 要素に変更を加える操作を呼びだします。
		modify_vector_entries(students);

		// 何らかのバリデーション。読み取り専用の「借用」(=参照)アクセス。
		dbaccess.validate_registration_email(students);

		for e in students {
			println!("[TRACE] {}", e);
		}

		return Ok(());
	}
}

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
	let args: Vec<String> = std::env::args().skip(1).collect();
	let _request = if 0 < args.len() { args[0].as_str() } else { "" };

	// シンプルな Vec のテスト
	if false {
		test00::run();
	}
	if false {
		test01::run()?;
	}
	if true {
		test02::run()?;
	}

	return Ok(());
}
