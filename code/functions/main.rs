fn test1(s: &str) {
	println!("{}", s)
}

/// 文字列を返す関数
fn create_string() -> String {
	return String::from("Hello, Real Rust World!");
}

/// エントリーポイント
fn main() {
	test1("Hello, Real Rust World!");
	println!("{}", create_string());
	println!("{}", create_string().as_str());
	test1(create_string().as_str());
}
