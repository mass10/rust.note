/// トレイトの宣言
trait TestableNumber<T> {
	/// この T が 'A' もしくは 'a' と同等である場合に true を返します。
	fn is_a(&self) -> bool;
	/// この T を返します。
	fn value(&self) -> T;
}

impl TestableNumber<i32> for i32 {
	fn is_a(&self) -> bool {
		match *self {
			0x41 => true,
			0x61 => true,
			_ => false,
		}
	}
	fn value(&self) -> i32 {
		return *self;
	}
}

fn test_integral_value(n: i32) {
	println!("{}.is_alphabet() -> {:?}", n.value(), n.is_a());
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
