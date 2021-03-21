///
/// 文字列を診断するトレイトです。
///
trait TestableString<T> {
	/// この T を構成するすべての要素が「アルファベットである」とみなすことができる場合に true を返します。
	fn is_alpha(&self) -> bool;
}

impl TestableString<String> for String {
	/// この String を構成するすべての要素が「アルファベットである」とみなすことができる場合に true を返します。
	fn is_alpha(&self) -> bool {
		if self.len() == 0 {
			return false;
		}
		for ch in self.chars() {
			// 全角をはじく
			if ch.is_ascii_alphabetic() == false {
				return false;
			}
			// 大文字 "A"
			if ch.is_uppercase() {
				continue;
			}
			// 小文字 "a"
			if ch.is_lowercase() {
				continue;
			}
			return false;
		}
		return true;
	}
}

impl TestableString<&str> for &str {
	/// この &str を構成するすべての要素が「アルファベットである」とみなすことができる場合に true を返します。
	fn is_alpha(&self) -> bool {
		if self.len() == 0 {
			return false;
		}
		for ch in self.chars() {
			// 全角をはじく
			if ch.is_ascii_alphabetic() == false {
				return false;
			}
			// 大文字 "A"
			if ch.is_uppercase() {
				continue;
			}
			// 小文字 "a"
			if ch.is_lowercase() {
				continue;
			}
			return false;
		}
		return true;
	}
}

/// 文字列の中身を診断します。
///
/// # Arguments
/// * `unknown` 次を実装する型 'T' を指定します。
///     * [self::TestableString]
///     * [std::fmt::Display]
///     * [std::fmt::Debug]
fn diagnose_unknown_parameter<T: self::TestableString<T> + std::fmt::Display + std::fmt::Debug>(
	unknown: T,
) {
	let result = ::myutil::result(unknown.is_alpha());

	println!("{:?}.is_alpha() -> [{}]", unknown, result);
}

pub fn execute() {
	println!("### 文字列の中身を診断します ###");

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

	println!();
}
