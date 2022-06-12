mod my_traits;
mod myutil;

fn test_integral_value(n: i32) {
	use my_traits::TestableNumber;

	let result = n.is_alphabet();
	println!("{:?}.is_a() -> [{}]", n, ::myutil::result(result));
}

fn test_all_999(n: i32) {
	use my_traits::TestableNumber;

	let result = n.is_999();
	println!("{:?}.is_999() -> [{}]", n, ::myutil::result(result));
}

/// 文字列の中身を診断します。
///
/// # Arguments
/// * `unknown` 次を実装する型 'T' を指定します。
///     * [self::TestableString]
///     * [std::fmt::Display]
///     * [std::fmt::Debug]
fn diagnose_unknown_parameter<T: crate::my_traits::TestableString<T> + std::fmt::Display + std::fmt::Debug>(unknown: T) {
	let result = unknown.is_alpha();

	println!("{:?}.is_alpha() -> [{}]", unknown, ::myutil::result(result));
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
	{
		diagnose_unknown_parameter(String::from("くぃうえお"));
		diagnose_unknown_parameter(String::from("A"));
		diagnose_unknown_parameter(String::from("_"));
		diagnose_unknown_parameter(String::from("my namE Is BABO"));
		diagnose_unknown_parameter(String::from(" A"));
		diagnose_unknown_parameter(String::from("Ｊｉｍｉ"));
		diagnose_unknown_parameter(String::from("Ａ"));
		diagnose_unknown_parameter("_");
		diagnose_unknown_parameter("あいうえお");
		diagnose_unknown_parameter("hhHgsAYTWhnxbA");
		diagnose_unknown_parameter("892137981240");
		diagnose_unknown_parameter("_");
		diagnose_unknown_parameter("Ｊｉｍｉ");
		diagnose_unknown_parameter("Ａ");
	}

	// Vec に trait を実装してみる
	{
		use crate::my_traits::StringVectorTrait;

		let v = vec![String::from(""), String::from("bbbbbb"), String::from("ジミヘン")];

		println!("{}", v.at(0));
		println!("{}", v.at(2));
		println!("{}", v.at(99));
	}

	println!("Ok.");
}
