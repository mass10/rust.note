
fn parse_int_unsafe(s: &str) -> std::result::Result<i32, std::num::ParseIntError> {
	return s.trim().parse::<i32>();
}

#[allow(unused)]
fn parse_int(s: &str) -> i32 {
	match s.trim().parse::<i32>() {
		Ok(n) => n,
		Err(err) => 0,
	}
}

#[allow(unused)]
fn test1(s: &str) {
	let result = parse_int_unsafe(&s);
	match result {
		Ok(value) => println!("\"{}\" >> ({})", s, value),
		Err(e) => println!("\"{}\" >> ({})", s, e),
	};
}

#[allow(unused)]
fn test2(s: &str) {
	let value = parse_int(&s);
	println!("\"{}\" >> ({})", s, value);
}

fn main() {
	let test = test1;
	// let test = test2;
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
