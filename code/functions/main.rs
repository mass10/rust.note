fn test1(s: &str) {
	println!("{}", s)
}

fn create_string() -> String {

	return String::from("Hello, Real Rust World!");
}

fn main() {

	test1("abc");
	println!("{:?}", create_string());
}

