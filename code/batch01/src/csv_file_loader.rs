/// CSV ファイルを読み込むクラス
pub struct CsvFileLoader {
	_file: Option<std::fs::File>,
}

impl CsvFileLoader {
	/// 新しいインすタンスを返します。
	pub fn new() -> CsvFileLoader {
		return CsvFileLoader { _file: None };
	}

	pub fn open_csv_file(&mut self, path: &str) -> std::result::Result<(), Box<dyn std::error::Error>> {
		let file = std::fs::File::open(path)?;
		self._file = Some(file);
		return Ok(());
	}

	pub fn close(&self) {
		if self._file.is_none() {
			return;
		}
	}
}

/// CsvFileLoader の解放
impl Drop for CsvFileLoader {
	fn drop(&mut self) {
		println!("(drop)");
	}
}