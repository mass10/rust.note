/// CSV ファイルを読み込むクラス
pub struct CsvFileLoader {
	reader: Option<std::io::BufReader<std::fs::File>>,
}

impl CsvFileLoader {
	/// `CsvFileLoader` の新しいインスタンスを返します。
	///
	/// # Returns
	/// `CsvFileLoader` の新しいインスタンス
	pub fn new() -> CsvFileLoader {
		return CsvFileLoader { reader: None };
	}

	/// ファイルを開きます。
	///
	/// # Arguments
	/// `path` ファイルのパス
	pub fn open_tsv_file(&mut self, path: &str) -> std::result::Result<(), Box<dyn std::error::Error>> {
		let file = std::fs::File::open(path)?;
		self.reader = Some(std::io::BufReader::new(file));
		return Ok(());
	}

	fn reader(&mut self) -> &mut std::io::BufReader<std::fs::File> {
		return self.reader.as_mut().unwrap();
	}

	pub fn read_line(&mut self) -> std::result::Result<Option<Vec<String>>, Box<dyn std::error::Error>> {
		use std::io::BufRead;

		// 状態確認
		if self.reader.is_none() {
			let result: Vec<String> = vec![];
			return Ok(Some(result));
		}

		// リーダー
		let reader = self.reader();

		// 1行読み取り
		let mut line = String::new();
		let result = reader.read_line(&mut line)?;
		if result == 0 {
			return Ok(None);
		}
		let line = line.trim();
		if line == "" {
			return Ok(Some(vec![]));
		}

		// 0x09 で分割して返却します。
		let fields: Vec<String> = line.split("\t").map(String::from).collect();
		return Ok(Some(fields));
	}
}

/// CsvFileLoader の解放
impl Drop for CsvFileLoader {
	fn drop(&mut self) {}
}
