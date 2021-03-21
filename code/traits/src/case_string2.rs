/// トレイトの宣言
trait TestableString<T> {
	/// この T が「アルファベットである」とみなすことができる場合に true を返します。
	fn is_alpha(&self) -> bool;
}

impl TestableString<&str> for &str {
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

fn test_str(n: &str) {
	let result = ::myutil::result(n.is_alpha());
	println!("{:?}.is_alpha() -> [{}]", n, result);
}

pub fn execute() {
	println!("### 文字列をテストします ###");
	test_str("_");
	test_str("あいうえお");
	test_str("hhHgsAYTWhnxbA");
	test_str("892137981240");
	test_str("_");
	test_str("Ｊｉｍｉ");
	test_str("Ａ");
	println!();
}
