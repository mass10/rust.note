mod case_string;
mod case_string2;
mod case_vector;
mod my_traits;
mod myutil;

fn test_integral_value(n: i32) {
	use my_traits::TestableNumber;

	let result = ::myutil::result(n.is_alphabet());
	println!("{:?}.is_a() -> [{}]", n, result);
}

fn test_all_999(n: i32) {
	use my_traits::TestableNumber;

	let result = ::myutil::result(n.is_999());
	println!("{:?}.is_999() -> [{}]", n, result);
}

fn main() {
	// ==================== 数値にアプリケーション固有のメソッドを生やしてみる ====================
	{
		test_integral_value(63);
		test_integral_value(64);
		test_integral_value(65);
		test_integral_value(95);
		test_integral_value(96);
		test_integral_value(97);

		test_all_999(123);
		test_all_999(9944);
		test_all_999(-91244);
		test_all_999(-999999999);
		test_all_999(9);
		test_all_999(0);
	}

	// ==================== 文字列に trait を実装してみる ====================
	case_string::execute();

	// 文字列に trait を実装してみる
	case_string2::execute();

	// Vec に trait を実装してみる
	case_vector::execute();

	println!("Ok.");
}
