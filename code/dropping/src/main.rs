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

struct MyLifetime {
	pub name: String
}

impl Drop for MyLifetime {
	/// 解放の実装
	fn drop(&mut self) {
		println!("[{}] dropped.", self.name);
	}
}

/// アプリケーションの生存期間を示す構造体
struct ApplicationLifetime {
	pub exit_code: i32
}

/// アプリケーション解放の実装
impl Drop for ApplicationLifetime {
	fn drop(&mut self) {
		if self.exit_code != 0 {
			println!("[ApplicationLifetime] QUIT IMMEDIATELY. (with exit_code: {})", self.exit_code);
			// 残っているすべての構造体に対して Drop が呼び出されません。
			std::process::exit(self.exit_code);
		}
		println!("[ApplicationLifetime] QUIT NATURALLY. (with exit_code: {})", self.exit_code);
	}
}

/// エントリーポイント
fn main() -> Result<(), Box<dyn std::error::Error>> {
	let _longest_lifetime = MyLifetime { name: "longest".to_string() };
	let mut _application_lifetime = ApplicationLifetime { exit_code: 0 };
	let _medium_lifetime = MyLifetime { name: "medium".to_string() };
	let _shortest_lifetime = MyLifetime { name: "short".to_string() };

	println!("### START ###");

	let _dummy = Lock::new();

	// 現在のミリ秒
	let current_millisec = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH)?.as_millis() % 1000;

	// ランダムで終了コードを設定
	if 500 <= current_millisec {
		_application_lifetime.exit_code = 1;
	}

	println!("--- END ---");

	return Ok(());
}
