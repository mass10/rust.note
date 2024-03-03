fn find_dir(path: &str, handler: &mut dyn FnMut(&str)) -> Result<(), Box<dyn std::error::Error>> {
	let xpath = std::path::Path::new(path);
	if xpath.is_dir() {
		let entries = std::fs::read_dir(path)?;
		for entry in entries {
			let entry = entry?;
			let name = entry.file_name();
			if name == "node_modules" {
				continue;
			}
			if name == ".git" {
				continue;
			}
			if name == ".vscode" {
				continue;
			}
			let path = entry.path();
			let path = path.to_str().unwrap();
			handler(path);
		}
	} else if xpath.is_file() {
		handler(path);
	}
	return Ok(());
}

/// ファイルの情報を集計する構造体
struct FileInfoAggregator {
	count: u64,
}

impl FileInfoAggregator {
	fn new() -> Self {
		Self { count: 0 }
	}

	fn handle(&mut self, path: &str) {
		self.count += 1;
		println!("{}", path);
	}
}

/// Rust アプリケーションのエントリーポイント
fn main() {
    // コマンドライン引数
	let args: Vec<String> = std::env::args().skip(1).collect();

	// ファイルの情報を集計する構造体
	let mut aggregator = FileInfoAggregator::new();

	// ファイルハンドラー
	let mut handler = |path: &str| aggregator.handle(path);

	// ファイルの情報を集計する
	for path in args {
		let _ = find_dir(&path, &mut handler);
	}
}
