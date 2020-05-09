type StringVectorType = std::vec::Vec<std::string::String>;

trait StringVectorTrait<StringVectorType> {
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

pub fn main() {
	let v = vec![
		String::from(""),
		String::from("bbbbbb"),
		String::from("ジミヘン"),
	];

	println!("### vec をテストします ###");
	println!("{}", v.at(0));
	println!("{}", v.at(2));
	println!("{}", v.at(99));
	println!();
}
