struct Lock {}

impl Lock {
	/// 新しいインスタンスを返します。
	pub fn new() -> Lock {
		println!("[Lock::new] locked.");
		return Lock {};
	}

	/// 何かしらの操作
	#[allow(unused)]
	pub fn hello(&self) {
		// println!("[Lock::new] hello)");
	}
}

impl Drop for Lock {
	/// 解放の実装
	fn drop(&mut self) {
		println!("[Lock::drop] unlocked.");
	}
}

/// エントリーポイント
fn main() -> Result<(), Box<dyn std::error::Error>> {
	println!("### START ###");

	let _dummy = Lock::new();

	println!("--- END ---");

	return Ok(());
}
