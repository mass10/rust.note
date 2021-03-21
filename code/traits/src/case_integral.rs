use my_helpers::TestableNumber;

extern crate colored;

mod my_helpers {
	///
	/// 文字列を調べるトレイトの宣言
	///
	pub trait TestableNumber<T> {
		/// この T が 'A' もしくは 'a' と同等である場合に true を返します。
		fn is_a(&self) -> bool;
	}

	impl TestableNumber<i32> for i32 {
		/// この i32 が 'A' もしくは 'a' と同等である場合に true を返します。
		fn is_a(&self) -> bool {
			match *self {
				0x41 => true,
				0x61 => true,
				_ => false,
			}
		}
	}

	impl TestableNumber<u32> for u32 {
		/// この u32 が 'A' もしくは 'a' と同等である場合に true を返します。
		fn is_a(&self) -> bool {
			match *self {
				0x41 => true,
				0x61 => true,
				_ => false,
			}
		}
	}
}

fn test_integral_value(n: i32) {
	use self::my_helpers::TestableNumber;

	let result = ::myutil::result(n.is_a());
	println!("{:?}.is_a() -> [{}]", n, result);
}

pub fn execute() {
	println!("### 数値をテストします ###");

	test_integral_value(63);
	test_integral_value(64);
	test_integral_value(65);
	test_integral_value(95);
	test_integral_value(96);
	test_integral_value(97);

	println!();
}
