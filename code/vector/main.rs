struct Class {
}

impl Class {

	fn enum_students(&self) -> Vec<String> {
		let mut v: Vec<String> = vec![];
		v.push(String::from("jimi.hendrix@gmail.com"));
		v.push(String::from("paul.kossoff@gmail.com"));
		v.push(String::from("freddie.mercury@gmail.com"));
		v.push(String::from("billy.preston@gmail.com"));
		v.push(String::from("steve.marriot@gmail.com"));
		return v;
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

fn main() {

	let v = vec![
		"こんにちは",
		"Real",
		"World",
		"Rust",
	];
	println!("{:?}", v);

	let mut v: Vec<i64> = vec![];
	v.push(-192);
	v.push(12);
	v.push(19823);
	test(&v);
	println!("{:?}", v);

	let v = vec![1, 3, -89, 67, 10092, -1029, -12, -10];
	println!("3つめの要素は {0} です。", v[2]);

	let class = Class{};
	println!("{:?}", class.enum_students());
	println!("{:?}", class.enum_students());
}
