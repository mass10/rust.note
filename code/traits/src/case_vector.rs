// 独自の型を定義。入れ子になった型は trait の型パラメータに指定できない。
type StringVectorType = std::vec::Vec<std::string::String>;

///
/// 文字列ベクタを操作するためのトレイトです。
///
trait StringVectorTrait<StringVectorType> {
	/// この T の `index` 番目の要素を返します。
	///
	/// # Arguments
	/// `index` 調べる位置
	///
	/// # Returns
	/// 文字列
	fn at(&self, index: usize) -> String;
}

impl StringVectorTrait<StringVectorType> for StringVectorType {
	fn at(&self, index: usize) -> String {
		if self.len() <= index {
			return "".to_string();
		}
		return self[index].to_string();
	}
}

pub fn execute() {
	println!("### vec をテストします ###");

	let v = vec![
		String::from(""),
		String::from("bbbbbb"),
		String::from("ジミヘン"),
	];

	println!("{}", v.at(0));
	println!("{}", v.at(2));
	println!("{}", v.at(99));
	println!();
}
