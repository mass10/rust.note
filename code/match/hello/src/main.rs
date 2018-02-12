use std::num::ParseIntError;

fn parse_int_unsafe(s: &str) -> std::result::Result<i32, ParseIntError> {

	match s.trim().parse::<i32>() {
		Ok(n) => Ok(n),
		Err(err) => Err(err),
	}
}

fn test(s: &str) {

	let result = parse_int_unsafe(&s);
	match result {
		Ok(value) => println!("\"{}\" >> ({})", s, value),
		Err(e) => println!("\"{}\" >> ({})", s, e),
	};
}

fn main() {

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
