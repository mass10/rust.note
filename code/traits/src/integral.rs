extern crate colored;

/// トレイトの宣言
trait TestableNumber<T> {
	/// この T が 'A' もしくは 'a' と同等である場合に true を返します。
	fn is_a(&self) -> bool;
}

impl TestableNumber<i32> for i32 {
	fn is_a(&self) -> bool {
		match *self {
			0x41 => true,
			0x61 => true,
			_ => false,
		}
	}
}

// fn red<T: std::fmt::Display>(s: T) -> String {
// 	return format!("\x1b[31m{}\x1b[39m", s);
// }

// fn green<T: std::fmt::Display>(s: T) -> String {
// 	return format!("\x1b[32m{}\x1b[39m", s);
// }

// fn result(b: bool) -> String {
// 	match b {
// 		true => green(b),
// 		false => red(b)
// 	}
// }

fn test_integral_value(n: i32) {
	let result = ::myutil::result(n.is_a());
	println!("{:?}.is_a() -> {}", n, result);
}

pub fn main() {
	println!("### 数値をテストします ###");
	test_integral_value(63);
	test_integral_value(64);
	test_integral_value(65);
	test_integral_value(95);
	test_integral_value(96);
	test_integral_value(97);
	println!();
}
