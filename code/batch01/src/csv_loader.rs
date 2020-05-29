pub struct CsvLoader {
	_file: Option<std::fs::File>,
}

impl CsvLoader {
	pub fn new() -> CsvLoader {
		return CsvLoader { _file: None };
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

impl Drop for CsvLoader {
	fn drop(&mut self) {
		println!("(drop)");
	}
}
