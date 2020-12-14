///
/// データベースクラス
///
struct DatabaseAccess {}
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
	pub fn enum_students(&self, classroom: &str) -> Box<Vec<String>> {
		if classroom == "" {}
		let mut students: Vec<String> = vec![];
		students.push(String::from("jimi.hendrix@gmail.com"));
		students.push(String::from("paul.kossoff@gmail.com"));
		students.push(String::from("freddie.mercury@gmail.com"));
		students.push(String::from("billy.preston@gmail.com"));
		students.push(String::from("steve.marriot@gmail.com"));
		return Box::new(students);
	}

	pub fn validate_registration_time(&self, students: &Vec<String>) -> bool {
		for e in students.into_iter() {
			if !e.contains("@") {
				return false;
			}
		}
		return true;
	}

	pub fn validate_registration_email(&self, students: &Vec<String>) -> bool {
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
struct Classroom {
	_x: i32,
	students: Vec<String>,
	classroom: String,
}

impl Classroom {
	/// 新しいインスタンスを返します。
	pub fn new(classroom: &str) -> Classroom {
		let mut instance = Classroom {
			_x: 0,
			classroom: String::from(classroom),
			students: vec![],
		};
		instance.initialize();
		return instance;
	}

	/// オブジェクトの初期化を行います。
	fn initialize(&mut self) {
		// 内部リストを初期化して
		self.students.clear();

		// データベースクラス
		let dba = DatabaseAccess::new();

		//生徒名簿を抽出
		let rows = dba.enum_students(&self.classroom);

		// 内部リストへ保管します。
		for e in rows.into_iter() {
			self.students.push(e);
		}
	}

	/// 生徒名簿を返します。返却されるオブジェクトは複製です。
	pub fn enum_students_clone(&self) -> Vec<String> {
		return self.students.clone();
	}

	/// 生徒名簿を返します。返却されるオブジェクトは参照です。
	pub fn enum_students_ref(&self) -> &Vec<String> {
		return &self.students;
	}
}

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

fn test01() {
	let v = vec!["こんにちは", "Real", "World", "Rust"];
	println!("{:?}", v);

	let mut v: Vec<i64> = vec![];
	v.push(-192);
	v.push(12);
	v.push(19823);
	test(&v);
	println!("{:?}", v);

	let v = vec![1, 3, -89, 67, 10092, -1029, -12, -10];
	println!("3つめの要素は {0} です。", v[2]);

	{
		let classroom = Classroom::new("A");
		println!("{:?}", classroom.enum_students_clone());
		println!("{:?}", classroom.enum_students_clone());
		println!("{:?}", classroom.enum_students_clone());
		println!("{:?}", classroom.enum_students_clone());
	}

	{
		// Classroom の生徒名簿を参照します。
		let classroom = Classroom::new("A");
		let students = classroom.enum_students_ref();

		// 検証っぽい何かの処理を行います。
		let db = DatabaseAccess::new();
		// 参照で見せるだけなら何も制限はありません。
		db.validate_registration_time(students);
		// 参照で見せるだけなら何も制限はありません。
		db.validate_registration_email(students);

		for student in students {
			println!("[TRACE] {}", student);
		}
	}
}

fn test02() {}

fn main() {
	test01();
	test02();
}
