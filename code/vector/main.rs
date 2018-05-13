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
}
