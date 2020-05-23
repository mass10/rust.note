/// トレイトの宣言
trait TestableString<T> {
	/// この T が「アルファベットである」とみなすことができる場合に true を返します。
	fn is_alpha(&self) -> bool;
}

impl TestableString<String> for String {
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

impl TestableString<&'static str> for &'static str {
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

fn test_string(n: String) {
	let result = ::myutil::result(n.is_alpha());
	println!("{:?}.slphabet() -> [{}]", n, result);
}

fn test_str(n: &'static str) {
	let result = ::myutil::result(n.is_alpha());
	println!("{:?}.slphabet() -> [{}]", n, result);
}

pub fn main() {
	println!("### 文字列をテストします ###");
	test_string(String::from("くぃうえお"));
	test_string(String::from("A"));
	test_string(String::from("_"));
	test_string(String::from("my namE Is BABO"));
	test_string(String::from(" A"));
	test_string(String::from("Ｊｉｍｉ"));
	test_string(String::from("Ａ"));
	test_str("_");
	test_str("あいうえお");
	test_str("hhHgsAYTWhnxbA");
	test_str("892137981240");
	test_str("_");
	test_str("Ｊｉｍｉ");
	test_str("Ａ");
	println!();
}
