pub struct TimeKeeper {
	/// 開始タイムスタンプ
	start: std::time::Instant,
}

impl TimeKeeper {
	/// 新しいインスタンスを返します。
	pub fn new() -> TimeKeeper {
		return TimeKeeper { start: std::time::Instant::now() };
	}

	/// 終了の判断
	pub fn is_over(&self) -> bool {
		let current_time = std::time::Instant::now();
		let elapsed = current_time - self.start;
		return 9 <= elapsed.as_secs();
	}
}
