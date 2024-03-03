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
			let _ = find_dir(path, handler);
		}
	} else if xpath.is_file() {
		handler(path);
	}
	return Ok(());
}

/// ファイルの情報を集計する構造体
struct FileInfoAggregator {
	timestamp: String,
	path: String,
}

impl FileInfoAggregator {
	/// 初期化
	fn new() -> Self {
		Self {
			timestamp: "".to_string(),
			path: "".to_string(),
		}
	}

	/// ファイルの情報を追加
	fn handle(&mut self, path: &str) {
		if path == "" {
			return;
		}

		let xpath = std::path::Path::new(path);
		let file_name = xpath.file_name().unwrap().to_str().unwrap_or_default();
		if file_name.ends_with(".exe") {
		} else if file_name.ends_with(".dll") {
		} else {
			return;
		}
		let last_modified = xpath.metadata().unwrap().modified().unwrap();
		let last_modified = format_systemtime(&last_modified);

		if self.timestamp == "" {
			// REPLACE
			self.timestamp = last_modified.clone();
			self.path = path.to_string();
		} else if self.timestamp < last_modified {
			// REPLACE
			self.timestamp = last_modified.clone();
			self.path = path.to_string();
		} else {
			// IGNORE
			return;
		}
	}

	fn get_info(&self) -> (String, String) {
		return (self.path.clone(), self.timestamp.clone());
	}
}

fn format_systemtime(time: &std::time::SystemTime) -> String {
	let datetime: chrono::DateTime<chrono::Local> = time.clone().into();
	return datetime.format("%Y-%m-%d %H:%M:%S").to_string();
}

fn detect_shutdown_file() -> bool {
	let path = std::path::Path::new(".shutdown");
	return path.exists();
}

struct Application {}

impl Application {
	pub fn new() -> Self {
		Self {}
	}

	pub fn start(&self) {
		loop {
			if detect_shutdown_file() {
				break;
			}

			eprintln!("--- 最も新しいファイル ---");

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

			// 最も新しいファイルの情報
			let (path, timestamp) = aggregator.get_info();
			eprintln!("{}: {}", path, timestamp);

			// sleep
			std::thread::sleep(std::time::Duration::from_secs(5));
		}
	}
}

/// Rust アプリケーションのエントリーポイント
fn main() {
	Application::new().start();
}
