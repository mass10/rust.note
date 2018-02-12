fn test1(s: &String) {

	println!("<test1()> {}", s);
}

fn test2(s: &String) {

	println!("<test2()> {}", s);
}

fn main() {

	let s = String::from("こんにちは");

	// 文字列を関数に渡す(問題なし)
	test1(&s);
	
	// 文字列を関数に渡す(問題なし)
	test2(&s);
}
