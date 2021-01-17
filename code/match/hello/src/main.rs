mod util {
	pub fn parse_int_unsafe(s: &str) -> std::result::Result<i32, std::num::ParseIntError> {
		return s.trim().parse::<i32>();
	}

	#[allow(unused)]
	pub fn parse_int(s: &str) -> i32 {
		match s.trim().parse::<i32>() {
			Ok(n) => n,
			Err(err) => 0,
		}
	}
}

mod application {
	use util;

	/// 数値変換を行うテストです。
	///
	/// 変換結果を Result で表現する、安全な関数を使用しています。
	#[allow(unused)]
	pub fn test_parsing_with_result(s: &str) {
		let result = util::parse_int_unsafe(&s);
		match result {
			Ok(value) => println!("\"{}\" >> ({})", s, value),
			Err(e) => println!("\"{}\" >> ({})", s, e),
		};
	}

	/// 数値変換を行うテストです。
	///
	/// 強制的に数値変換を行う安全な関数を使用しています。
	#[allow(unused)]
	pub fn test_parsing_with_value(s: &str) {
		let value = util::parse_int(&s);
		println!("\"{}\" >> ({})", s, value);
	}
}

fn main() {
	let test = application::test_parsing_with_result;
	// let test = application::test_parsing_with_value;
	test("");
	test("123");
	test("0.0");
	test("1.0");
	test("-500");
	test("-500.0");
	test("2147483647");
	test("2147483648");
	test("-2147483648");
	test("-2147483649");
	test("123.9");
	test("0x09");
	test(" 981 ");
	test("アイウエオ");
	test("狩野永徳");
}
