fn test1(s: &String) {

	println!("<test1()> {}", s);
}

fn test2(s: &String) {

	println!("<test2()> {}", s);
}

fn main() {

	let s = String::from("こんにちは");

	test1(&s);
	test2(&s);
}
