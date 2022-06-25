extern crate colored;

///
/// 文字列を調べるトレイトの宣言
///
pub trait MyHelper1<T> {
	/// この T が a-z もしくは A-Z とみなせる場合に true を返します。
	fn is_alphabet(&self) -> bool;
	/// この T が、オール9の場合に true を返します。
	fn is_999(&self) -> bool;
}

impl MyHelper1<i32> for i32 {
	/// この i32 が 'A' もしくは 'a' と同等である場合に true を返します。
	fn is_alphabet(&self) -> bool {
		const SMALL_A: i32 = 'a' as i32;
		const SMALL_Z: i32 = 'z' as i32;
		const LARGE_A: i32 = 'A' as i32;
		const LARGE_Z: i32 = 'Z' as i32;
		return match *self {
			SMALL_A..=SMALL_Z => true,
			LARGE_A..=LARGE_Z => true,
			_ => false,
		};
	}

	/// この i32 が 999 と同等である場合に true を返します。
	fn is_999(&self) -> bool {
		let text = self.to_string();
		if text.len() == 0 {
			return false;
		}
		for c in text.chars() {
			if c != '9' {
				return false;
			}
		}
		return true;
	}
}

impl MyHelper1<u32> for u32 {
	/// この u32 が 'A' もしくは 'a' と同等である場合に true を返します。
	fn is_alphabet(&self) -> bool {
		const SMALL_A: u32 = 'a' as u32;
		const SMALL_Z: u32 = 'z' as u32;
		const LARGE_A: u32 = 'A' as u32;
		const LARGE_Z: u32 = 'Z' as u32;
		return match *self {
			SMALL_A..=SMALL_Z => true,
			LARGE_A..=LARGE_Z => true,
			_ => false,
		};
	}

	/// この i32 が 999 と同等である場合に true を返します。
	fn is_999(&self) -> bool {
		let text = self.to_string();
		if text.len() == 0 {
			return false;
		}
		for c in text.chars() {
			if c != '9' {
				return false;
			}
		}
		return true;
	}
}

///
/// 文字列を診断するトレイトです。
///
pub trait TestableString<T> {
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

// 独自の型を定義。入れ子になった型は trait の型パラメータに指定できない。
type MyStringVector = std::vec::Vec<std::string::String>;

type MyU32Vector = std::vec::Vec<u32>;

///
/// 文字列ベクタを操作するためのトレイトです。
///
pub trait VectorHelper<T> {
	/// この T の `index` 番目の要素を返します。
	///
	/// # Arguments
	/// `index` 調べる位置
	///
	/// # Returns
	/// 文字列
	fn at(&self, index: usize) -> Option<T>;

	fn has(&self, e: T) -> bool;
}

impl VectorHelper<String> for MyStringVector {
	/// [MyStringVector] に at の振る舞いを実装します。
	fn at(&self, index: usize) -> Option<String> {
		if self.len() <= index {
			return None;
		}
		return Some(self[index].to_string());
	}

	/// [MyStringVector] に has の振る舞いを実装します。
	fn has(&self, e: String) -> bool {
		for i in 0..self.len() {
			if self[i] == *e {
				return true;
			}
		}
		return false;
	}
}

impl VectorHelper<u32> for MyU32Vector {
	fn at(&self, index: usize) -> Option<u32> {
		if self.len() <= index {
			return None;
		}
		return Some(self[index]);
	}

	fn has(&self, e: u32) -> bool {
		for i in 0..self.len() {
			if self[i] == e {
				return true;
			}
		}
		return false;
	}
}

type MyCharVector = std::vec::Vec<char>;

impl VectorHelper<char> for MyCharVector {
	fn at(&self, index: usize) -> Option<char> {
		if self.len() <= index {
			return None;
		}
		return Some(self[index]);
	}

	fn has(&self, e: char) -> bool {
		return self.contains(&e);
	}
}
