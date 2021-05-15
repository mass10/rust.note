struct Lock {}

impl Lock {
	/// 新しいインスタンスを返します。
	pub fn new() -> Lock {
		println!("(lock)");
		return Lock {};
	}

	/// 何かしらの操作
	pub fn hello(&self) {
		println!("(hello)");
	}
}

impl Drop for Lock {
	/// 解放の実装
	fn drop(&mut self) {
		println!("(unlock)");
	}
}

/// エントリーポイント
fn main() -> Result<(), Box<dyn std::error::Error>> {
	println!("### START ###");
	let lock = Lock::new();
	lock.hello();
	println!("--- END ---");
	return Ok(());
}
