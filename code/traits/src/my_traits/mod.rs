extern crate colored;

///
/// 文字列を調べるトレイトの宣言
///
pub trait TestableNumber<T> {
	/// この T が a-z もしくは A-Z とみなせる場合に true を返します。
	fn is_alphabet(&self) -> bool;
	/// この T が、オール9の場合に true を返します。
	fn is_999(&self) -> bool;
}

impl TestableNumber<i32> for i32 {
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

impl TestableNumber<u32> for u32 {
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
