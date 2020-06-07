/// ユーティリティー
pub struct Util {}

impl Util {
	/// タイムスタンプを返します。
	pub fn timestamp0() -> String {
		let date = chrono::Local::now();
		return format!("{}", date.format("%Y-%m-%d %H:%M:%S%.3f"));
	}

	/// タイムスタンプを返します。
	pub fn timestamp1() -> String {
		let date = chrono::Local::now();
		return format!("{}", date.format("%Y%m%d-%H%M%S"));
	}
}
