/// データベース操作クラスの宣言
struct StudentModel {}

/// データベース操作クラスの実装
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
struct ClassModel {
	_vec: Vec<String>,
}

/// クラスモデルの実装
impl ClassModel {
	/// 新しいインスタンスを返します。
	fn new() -> ClassModel {
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
	fn free(&mut self) {
		self._vec.clear();
	}

	#[allow(dead_code)]
	/// ダミー
	fn dummp1(&self) {
		// self に破壊的変更が発生するような操作は一切認められません。
		// self._vec.clear();
	}

	fn dump(&self) {
		println!("[TRACE] $$$ begin dump $$$");
		for e in self.enum_students() {
			println!("{}", e);
		}
		println!("[TRACE] --- end dump ---");
	}
}

fn task1(c: &ClassModel) {
	// mutable として borrow してないので、破壊的な変更は認められません。
	// c.free();
	// ダンプ
	c.dump();
}

fn task2(c: &mut ClassModel) {
	// mutable として borrow しているため破壊的な変更ができます。
	c.free();
	// 再度ダンプ
	c.dump();
}

/// エントリーポイント
fn main() {
	// オブジェクトを作成
	let mut c = ClassModel::new();
	task1(&c);
	task2(&mut c);
	println!("Ok.");
}
