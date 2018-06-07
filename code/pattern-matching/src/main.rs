
fn to_string(n: i64) -> String {

	if n < 0 {
		return String::from("");
	}
	match n {
		0...19 => String::from("10代"),
		20...29 => String::from("20代"),
		30...39 => String::from("30代"),
		40...49 => String::from("40代"),
		50...59 => String::from("50代"),
		60...69 => String::from("60代"),
		70...79 => String::from("70代"),
		80...89 => String::from("80代"),
		_ => String::from("90歳以上")
	}
}

fn test(n: i64) {

	let description = to_string(n);
	println!("{} 歳は {:?} です。", n, description);
}

fn main() {

	test(-123);
	test(-1);
	test(0);
	test(1);
	test(12);
	test(19);
	test(20);
	test(21);
	test(59);
	test(90);
	test(100);
	test(300);
}
